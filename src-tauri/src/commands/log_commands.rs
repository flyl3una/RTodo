// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use tauri::{AppHandle, State};
use std::sync::Mutex;
use crate::logging::{self, LogConfig, LogReloadHandle, LogLevel, RollingKind, load_config, save_config};
use std::path::PathBuf;

/// 日志状态，存储在应用 state 中
pub struct LogState {
    pub reload_handle: Mutex<Option<LogReloadHandle>>,
    pub current_config: Mutex<LogConfig>,
}

impl LogState {
    pub fn new(reload_handle: LogReloadHandle, config: LogConfig) -> Self {
        Self {
            reload_handle: Mutex::new(Some(reload_handle)),
            current_config: Mutex::new(config),
        }
    }
}

/// 获取当前日志配置
#[tauri::command]
pub async fn get_log_config(state: State<'_, LogState>) -> Result<logging::LogConfig, String> {
    let config = state.current_config.lock()
        .map_err(|e| format!("Failed to acquire lock: {}", e))?;

    // 如果 log_dir 为 None，计算默认路径并返回修改后的配置
    let mut result_config = config.clone();
    if result_config.log_dir.is_none() {
        if let Some(data_dir) = dirs::data_local_dir() {
            let default_path = data_dir.join("rtodo").join("logs");
            result_config.log_dir = default_path.to_str().map(|s| s.to_string());
        }
    }

    Ok(result_config)
}

/// 设置日志级别（立即生效）
#[tauri::command]
pub async fn set_log_level(
    level: String,
    app: AppHandle,
    state: State<'_, LogState>,
) -> Result<(), String> {
    // 解析日志级别
    let log_level = level.parse::<LogLevel>()
        .map_err(|e| format!("Invalid log level: {}", e))?;

    // 更新配置
    {
        let mut config = state.current_config.lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;
        config.level = log_level;
        save_config(&config);
    }

    // 重新加载日志级别 - 在 await 前提取并释放锁
    let handle_opt = {
        let reload_handle = state.reload_handle.lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;
        reload_handle.clone()
    };

    if let Some(handle) = handle_opt.as_ref() {
        logging::reload_log_level(handle, log_level).await
            .map_err(|e| format!("Failed to reload log level: {}", e))?;
    }

    tracing::info!("Log level changed to: {:?}", log_level);
    Ok(())
}

/// 设置日志目录（需要重启应用才能生效）
#[tauri::command]
pub async fn set_log_directory(
    directory: String,
    state: State<'_, LogState>,
) -> Result<(), String> {
    let mut config = state.current_config.lock()
        .map_err(|e| format!("Failed to acquire lock: {}", e))?;

    config.log_dir = if directory.is_empty() {
        None
    } else {
        Some(directory)
    };

    save_config(&config);

    tracing::info!("Log directory changed to: {:?}. Restart required.", config.log_dir);
    Ok(())
}

/// 设置日志回滚策略（需要重启应用才能生效）
#[tauri::command]
pub async fn set_log_rolling(
    rolling: String,
    state: State<'_, LogState>,
) -> Result<(), String> {
    let rolling_kind = match rolling.as_str() {
        "daily" => RollingKind::Daily,
        "hourly" => RollingKind::Hourly,
        "minutely" => RollingKind::Minutely,
        "never" => RollingKind::Never,
        _ => return Err("Invalid rolling kind".to_string()),
    };

    let mut config = state.current_config.lock()
        .map_err(|e| format!("Failed to acquire lock: {}", e))?;

    tracing::info!("Log rolling changed to: {:?}. Restart required.", rolling_kind);
    config.rolling = rolling_kind;
    save_config(&config);
    Ok(())
}

/// 设置日志压缩（需要重启应用才能生效）
#[tauri::command]
pub async fn set_log_compress(
    compress: bool,
    state: State<'_, LogState>,
) -> Result<(), String> {
    let mut config = state.current_config.lock()
        .map_err(|e| format!("Failed to acquire lock: {}", e))?;

    config.compress = compress;
    save_config(&config);

    tracing::info!("Log compress changed to: {}. Restart required.", compress);
    Ok(())
}

/// 设置日志保留天数（需要重启应用才能生效）
#[tauri::command]
pub async fn set_log_retention_days(
    days: usize,
    state: State<'_, LogState>,
) -> Result<(), String> {
    let mut config = state.current_config.lock()
        .map_err(|e| format!("Failed to acquire lock: {}", e))?;

    config.retention_days = days;
    save_config(&config);

    tracing::info!("Log retention days changed to: {}. Restart required.", days);
    Ok(())
}

/// 获取日志文件列表
#[tauri::command]
pub async fn get_log_files(state: State<'_, LogState>) -> Result<Vec<String>, String> {
    let config = state.current_config.lock()
        .map_err(|e| format!("Failed to acquire lock: {}", e))?;

    logging::get_log_files(&config)
        .map_err(|e| format!("Failed to get log files: {}", e))
}

/// 手动触发日志压缩
#[tauri::command]
pub async fn compress_logs(state: State<'_, LogState>) -> Result<(), String> {
    // 在 async 操作前提取需要的值并释放锁
    let (log_dir_option, retention_days) = {
        let config = state.current_config.lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;
        (config.log_dir.clone(), config.retention_days)
    };

    let log_dir = if let Some(dir) = log_dir_option {
        std::path::PathBuf::from(dir)
    } else {
        let mut path = dirs::data_local_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."));
        path.push("rtodo");
        path.push("logs");
        path
    };

    logging::compress_old_logs(&log_dir, retention_days).await
        .map_err(|e| format!("Failed to compress logs: {}", e))?;

    tracing::info!("Manual log compression completed");
    Ok(())
}
