// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use tracing_subscriber::{fmt, prelude::*, registry, EnvFilter, Layer};
use tracing_appender::{rolling, non_blocking};
use tracing_appender::non_blocking::WorkerGuard;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;

/// 日志 Worker Guards，必须在程序生命周期中保持存活
pub struct LogWorkerGuards {
    _file_guard: WorkerGuard,
    _console_guard: Option<WorkerGuard>,
}

/// 日志重新加载句柄
/// 用于在运行时动态修改日志级别
#[derive(Clone)]
pub struct LogReloadHandle {
    log_level: Arc<RwLock<tracing::Level>>,
}

impl LogReloadHandle {
    pub fn new(log_level: tracing::Level) -> Self {
        Self {
            log_level: Arc::new(RwLock::new(log_level)),
        }
    }

    pub async fn set_level(&self, level: tracing::Level) -> Result<()> {
        let mut guard = self.log_level.write().await;
        *guard = level;
        Ok(())
    }

    pub async fn get_level(&self) -> tracing::Level {
        *self.log_level.read().await
    }
}

/// 日志级别配置
#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    #[default]
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl From<LogLevel> for tracing::Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => tracing::Level::TRACE,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Error => tracing::Level::ERROR,
        }
    }
}

impl std::str::FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "trace" => Ok(LogLevel::Trace),
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            _ => Err(format!("Invalid log level: {}", s)),
        }
    }
}

/// 日志回滚策略
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum RollingKind {
    #[default]
    Daily,   // 按天回滚
    Hourly,  // 按小时回滚
    Minutely, // 按分钟回滚
    Never,   // 不回滚，单文件
}

/// 日志输出格式
#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    #[default]
    Full,    // 完整格式：包含时间戳、级别、模块、文件、行号、消息
    Compact, // 紧凑格式：只包含级别、消息
    Json,    // JSON 格式
}

/// 日志配置
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct LogConfig {
    /// 日志级别：trace, debug, info, warn, error
    #[serde(default)]
    pub level: LogLevel,

    /// 是否输出到控制台
    #[serde(default = "default_console")]
    pub console: bool,

    /// 是否输出到文件
    #[serde(default = "default_file")]
    pub file: bool,

    /// 日志目录
    #[serde(default)]
    pub log_dir: Option<String>,

    /// 日志输出格式
    #[serde(default)]
    pub format: LogFormat,

    /// 文件回滚方式
    #[serde(default)]
    pub rolling: RollingKind,

    /// 日志文件大小限制（MB），0 表示不限制
    /// 注意：这个选项只在 rolling 为 never 时生效
    #[serde(default)]
    pub max_file_size_mb: usize,

    /// 是否自动压缩旧日志文件
    #[serde(default = "default_compress")]
    pub compress: bool,

    /// 日志保留天数
    #[serde(default = "default_retention_days")]
    pub retention_days: usize,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            console: true,
            file: true,
            log_dir: None,
            format: LogFormat::Full,
            rolling: RollingKind::Daily,
            max_file_size_mb: 100,
            compress: true,
            retention_days: 30,
        }
    }
}

fn default_console() -> bool {
    true
}

fn default_file() -> bool {
    true
}

fn default_compress() -> bool {
    true
}

fn default_retention_days() -> usize {
    30
}

/// 初始化日志系统
///
/// **重要**: 返回的 WorkerGuards 必须在程序整个生命周期中保持存活，
/// 否则日志后台线程会停止，导致日志丢失。
///
/// 返回 (LogWorkerGuards, LogReloadHandle)
pub fn init_logging(config: &LogConfig) -> Result<(LogWorkerGuards, LogReloadHandle)> {
    // 确定日志目录
    let log_dir = if let Some(dir) = &config.log_dir {
        PathBuf::from(dir)
    } else {
        // 默认使用用户数据目录下的 logs 文件夹
        let mut path = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."));
        path.push("rtodo");
        path.push("logs");
        path
    };

    // 创建日志目录
    std::fs::create_dir_all(&log_dir)?;

    // println!("Log directory: {:?}", log_dir);

    // 根据配置选择日志格式
    let format_config = config.format;

    // 文件日志 appender
    let file_appender = match config.rolling {
        RollingKind::Daily => {
            rolling::daily(log_dir.clone(), "rtodo.log")
        }
        RollingKind::Hourly => {
            rolling::hourly(log_dir.clone(), "rtodo.log")
        }
        RollingKind::Minutely => {
            rolling::minutely(log_dir.clone(), "rtodo.log")
        }
        RollingKind::Never => {
            rolling::never(log_dir.clone(), "rtodo.log")
        }
    };

    let (non_blocking_file, file_guard) = non_blocking(file_appender);

    // 控制台日志
    let (non_blocking_console, console_guard) = if config.console {
        let (appender, guard) = non_blocking(std::io::stdout());
        (Some(appender), Some(guard))
    } else {
        (None, None)
    };

    // 构建订阅器
    let registry = registry::Registry::default();

    // 根据格式创建文件层
    let file_layer = match format_config {
        LogFormat::Full => fmt::layer()
            .with_writer(non_blocking_file)
            .with_ansi(false)
            .with_target(true)
            .with_file(true)
            .with_line_number(true)
            .boxed(),
        LogFormat::Compact => fmt::layer()
            .with_writer(non_blocking_file)
            .with_ansi(false)
            .with_target(false)
            .with_file(false)
            .with_line_number(false)
            .boxed(),
        LogFormat::Json => {
            fmt::layer()
                .json()
                .with_writer(non_blocking_file)
                .with_file(true)
                .with_line_number(true)
                .boxed()
        }
    };

    // 控制台层始终使用 Full 格式带颜色
    let console_layer = if let Some(console_writer) = non_blocking_console {
        Some(
            fmt::layer()
                .with_writer(console_writer)
                .with_ansi(true)
                .with_target(true)
                .with_file(true)
                .with_line_number(true)
                .boxed()
        )
    } else {
        None
    };

    // 组合所有层
    let subscriber = registry
        .with(console_layer)
        .with(file_layer);

    // 设置全局默认级别和过滤器
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            let level: tracing::Level = config.level.into();
            EnvFilter::new(format!("rtodo={}", level.as_str()))
        });

    subscriber.with(env_filter).init();

    tracing::info!("Logging initialized successfully");
    tracing::info!("  - Level: {:?}", config.level);
    tracing::info!("  - Directory: {:?}", log_dir);
    tracing::info!("  - Format: {:?}", format_config);
    tracing::info!("  - Rolling: {:?}", config.rolling);
    tracing::info!("  - Compress: {:?}", config.compress);
    tracing::info!("  - Retention Days: {:?}", config.retention_days);

    // 返回 guards 和 reload handle，必须在程序整个生命周期中保持存活
    let guards = LogWorkerGuards {
        _file_guard: file_guard,
        _console_guard: console_guard,
    };

    let reload_handle = LogReloadHandle::new(config.level.into());

    Ok((guards, reload_handle))
}

/// 从配置文件加载日志配置
///
/// 如果配置文件不存在，则创建默认配置文件并返回默认配置
/// 配置文件的优先级高于编译模式的默认值
///
/// 配置文件默认存储在程序安装目录下（可执行文件同目录）
pub fn load_config() -> LogConfig {
    // 使用程序安装目录（可执行文件所在目录）
    let config_path = if let Ok(exe_path) = std::env::current_exe() {
        // 获取可执行文件所在的目录
        if let Some(exe_dir) = exe_path.parent() {
            exe_dir.join("log-config.toml")
        } else {
            // 如果无法获取父目录，回退到用户数据目录
            if let Some(data_dir) = dirs::data_local_dir() {
                let mut path = data_dir;
                path.push("rtodo");
                path.push("log-config.toml");
                path
            } else {
                return LogConfig::default();
            }
        }
    } else {
        // 如果无法获取可执行文件路径，回退到用户数据目录
        if let Some(data_dir) = dirs::data_local_dir() {
            let mut path = data_dir;
            path.push("rtodo");
            path.push("log-config.toml");
            path
        } else {
            return LogConfig::default();
        }
    };

    // 如果配置文件存在，读取并解析
    if let Ok(content) = std::fs::read_to_string(&config_path) {
        if let Ok(file_config) = toml::from_str::<LogConfig>(&content) {
            // 配置文件存在且有效，直接使用
            return file_config;
        }
    }

    // 否则创建默认配置文件
    let config = LogConfig::default();
    save_config(&config);

    config
}

/// 保存配置到文件
///
/// 配置文件默认存储在程序安装目录下（可执行文件同目录）
pub fn save_config(config: &LogConfig) -> LogConfig {
    // 使用程序安装目录（可执行文件所在目录）
    let config_path = if let Ok(exe_path) = std::env::current_exe() {
        // 获取可执行文件所在的目录
        if let Some(exe_dir) = exe_path.parent() {
            exe_dir.join("log-config.toml")
        } else {
            // 如果无法获取父目录，回退到用户数据目录
            if let Some(data_dir) = dirs::data_local_dir() {
                data_dir.join("rtodo").join("log-config.toml")
            } else {
                return config.clone();
            }
        }
    } else {
        // 如果无法获取可执行文件路径，回退到用户数据目录
        if let Some(data_dir) = dirs::data_local_dir() {
            data_dir.join("rtodo").join("log-config.toml")
        } else {
            return config.clone();
        }
    };

    // 序列化配置
    if let Ok(toml_string) = toml::to_string_pretty(config) {
        std::fs::write(&config_path, toml_string).ok();
    }

    config.clone()
}

/// 重新加载日志级别
/// 注意：由于 tracing-subscriber 的限制，此函数只更新配置但不立即生效
/// 需要重启应用才能完全生效
pub async fn reload_log_level(handle: &LogReloadHandle, level: LogLevel) -> Result<()> {
    handle.set_level(level.into()).await?;
    Ok(())
}

/// 压缩旧的日志文件
///
/// 压缩指定天数之前的日志文件（非 .gz 文件）
pub async fn compress_old_logs(log_dir: &PathBuf, retention_days: usize) -> Result<()> {
    let mut compressed_count = 0;

    let mut entries = std::fs::read_dir(log_dir)?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    // 按修改时间排序，旧文件在前
    entries.sort_by_key(|entry| {
        entry.metadata()
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
    });

    let now = chrono::Utc::now();
    let retention_duration = chrono::Duration::days(retention_days as i64);

    for entry in entries {
        let path = entry.path();

        // 跳过已压缩的文件
        if path.extension().and_then(|s| s.to_str()) == Some("gz") {
            continue;
        }

        // 只处理日志文件
        if path.extension().and_then(|s| s.to_str()) != Some("log") {
            continue;
        }

        // 获取文件修改时间
        if let Ok(metadata) = entry.metadata() {
            if let Ok(modified) = metadata.modified() {
                let modified_datetime = chrono::DateTime::<chrono::Utc>::from(modified);
                let age = now.signed_duration_since(modified_datetime);

                // 如果文件超过保留天数，进行压缩
                if age > retention_duration {
                    compress_log_file(&path)?;
                    compressed_count += 1;
                }
            }
        }
    }

    tracing::info!("Compressed {} old log files", compressed_count);
    Ok(())
}

/// 压缩单个日志文件
fn compress_log_file(log_path: &PathBuf) -> Result<()> {
    let gz_path = log_path.with_extension("log.gz");

    // 读取原始文件
    let content = std::fs::read(log_path)?;

    // 创建压缩文件
    let file = std::fs::File::create(&gz_path)?;
    let mut encoder = flate2::write::GzEncoder::new(file, flate2::Compression::default());
    std::io::Write::write_all(&mut encoder, &content)?;
    encoder.finish()?;

    // 删除原始文件
    std::fs::remove_file(log_path)?;

    Ok(())
}

/// 获取日志目录下的所有日志文件
pub fn get_log_files(config: &LogConfig) -> Result<Vec<String>> {
    let log_dir = if let Some(dir) = &config.log_dir {
        PathBuf::from(dir)
    } else {
        let mut path = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."));
        path.push("rtodo");
        path.push("logs");
        path
    };

    let mut files = Vec::new();

    if let Ok(entries) = std::fs::read_dir(&log_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                // 只包含日志文件和压缩的日志文件
                if file_name.ends_with(".log") || file_name.ends_with(".gz") {
                    files.push(file_name.to_string());
                }
            }
        }
    }

    // 按名称排序
    files.sort();
    files.reverse();

    Ok(files)
}
