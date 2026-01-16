// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use tracing_subscriber::{fmt, prelude::*, registry, EnvFilter};
use tracing_appender::{rolling, non_blocking};
use tracing_appender::non_blocking::WorkerGuard;
use std::path::PathBuf;
use anyhow::Result;

/// 日志 Worker Guards，必须在程序生命周期中保持存活
pub struct LogWorkerGuards {
    _file_guard: WorkerGuard,
    _console_guard: Option<WorkerGuard>,
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
        }
    }
}

fn default_console() -> bool {
    true
}

fn default_file() -> bool {
    true
}

/// 初始化日志系统
///
/// **重要**: 返回的 WorkerGuards 必须在程序整个生命周期中保持存活，
/// 否则日志后台线程会停止，导致日志丢失。
pub fn init_logging(config: &LogConfig) -> Result<LogWorkerGuards> {
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

    // 返回 guards，必须在程序整个生命周期中保持存活
    Ok(LogWorkerGuards {
        _file_guard: file_guard,
        _console_guard: console_guard,
    })
}

/// 从配置文件加载日志配置
///
/// 根据编译模式自动应用不同的日志策略：
/// - **debug 模式**：日志级别为 debug，输出到控制台和文件
/// - **release 模式**：日志级别为 error，仅输出到文件
///
/// 配置文件中的设置会被编译模式自动覆盖
pub fn load_config() -> LogConfig {
    // 确定编译模式并应用相应的默认配置
    let mut config = if cfg!(debug_assertions) {
        // Debug 模式：日志级别 debug，输出到控制台和文件
        LogConfig {
            level: LogLevel::Debug,
            console: true,
            file: true,
            ..Default::default()
        }
    } else {
        // Release 模式：日志级别 error，仅输出到文件
        LogConfig {
            level: LogLevel::Error,
            console: false,
            file: true,
            ..Default::default()
        }
    };

    // 首先尝试从用户数据目录读取配置文件
    let config_path = if let Some(data_dir) = dirs::data_local_dir() {
        let mut path = data_dir;
        path.push("rtodo");
        path.push("log-config.toml");
        path
    } else {
        return config;
    };

    // 如果配置文件存在，读取并解析
    if let Ok(content) = std::fs::read_to_string(&config_path) {
        if let Ok(file_config) = toml::from_str::<LogConfig>(&content) {
            // 合并配置：编译模式的设置优先级高于配置文件
            // 这确保 debug 模式总是 debug 级别且输出到控制台
            // release 模式总是 error 级别且不输出到控制台
            config.log_dir = file_config.log_dir;
            config.format = file_config.format;
            config.rolling = file_config.rolling;
            config.max_file_size_mb = file_config.max_file_size_mb;
            // 注意：level、console、file 由编译模式决定，不从配置文件读取
            // 不在这里记录日志，因为日志系统还未初始化
            return config;
        }
    }

    // 否则创建默认配置文件
    if let Some(data_dir) = dirs::data_local_dir() {
        let config_dir = data_dir.join("rtodo");
        std::fs::create_dir_all(&config_dir).ok();

        // 创建配置文件模板（注释说明由编译模式控制）
        let config_template = r#"# RTodo 日志配置文件
#
# 注意：以下配置由编译模式自动控制，配置文件中的设置不会生效：
# - level: Debug 模式固定为 debug，Release 模式固定为 error
# - console: Debug 模式固定为 true，Release 模式固定为 false
# - file: 始终为 true
#
# 可配置项：
# 日志目录（留空则使用默认路径）
# log_dir = "C:\\path\\to\\logs"

# 日志格式：full, compact, json
format = "full"

# 文件回滚方式：daily, hourly, minutely, never
rolling = "daily"

# 日志文件大小限制（MB），仅当 rolling = "never" 时生效
max_file_size_mb = 100
"#;

        std::fs::write(&config_path, config_template).ok();
    }

    config
}
