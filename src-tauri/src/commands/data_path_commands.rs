// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

//! 数据路径管理命令
//! 提供获取、设置、重置数据路径以及数据迁移功能

use crate::database::DbConnection;
use crate::config::AppConfig;
use tauri::{AppHandle, Emitter, Manager};
use std::fs;
use std::path::PathBuf;

/// 获取当前数据路径
#[tauri::command]
pub async fn get_data_path() -> Result<String, String> {
    DbConnection::get_data_dir()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| format!("Failed to get data path: {}", e))
}

/// 设置数据路径
#[tauri::command]
pub async fn set_data_path(
    new_path: String,
    app: AppHandle,
) -> Result<(), String> {
    tracing::info!("set_data_path called with: {}", new_path);

    // 验证路径可写
    let path = PathBuf::from(&new_path);
    if !path.exists() {
        fs::create_dir_all(&path)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // 测试写入权限
    let test_file = path.join(".write_test");
    fs::write(&test_file, b"test")
        .map_err(|e| format!("Path not writable: {}", e))?;
    fs::remove_file(&test_file)
        .map_err(|e| format!("Failed to cleanup test file: {}", e))?;

    // 更新配置
    if let Some(config_state) = app.try_state::<std::sync::Mutex<AppConfig>>() {
        if let Ok(mut config) = config_state.lock() {
            config.data_path = Some(new_path.clone());
            config.save(&app)?;
            tracing::info!("Data path updated to: {}", new_path);
            Ok(())
        } else {
            Err("Failed to acquire config lock".to_string())
        }
    } else {
        Err("Config state not available".to_string())
    }
}

/// 重置为默认数据路径
#[tauri::command]
pub async fn reset_data_path(app: AppHandle) -> Result<(), String> {
    tracing::info!("reset_data_path called");

    // 更新配置，移除自定义路径
    if let Some(config_state) = app.try_state::<std::sync::Mutex<AppConfig>>() {
        if let Ok(mut config) = config_state.lock() {
            config.data_path = None;
            config.save(&app)?;
            tracing::info!("Data path reset to default");
            Ok(())
        } else {
            Err("Failed to acquire config lock".to_string())
        }
    } else {
        Err("Config state not available".to_string())
    }
}

/// 迁移数据到新路径
#[tauri::command]
pub async fn migrate_data(
    new_path: String,
    app: AppHandle,
) -> Result<(), String> {
    use serde_json::json;

    tracing::info!("migrate_data called with: {}", new_path);

    // 1. 获取当前路径
    let current_path = DbConnection::get_data_dir()
        .map_err(|e| format!("Failed to get data dir: {}", e))?;
    let target_path = PathBuf::from(&new_path);

    // 发送进度：开始迁移
    app.emit("migrate-progress", json!({
        "status": "started",
        "message": "开始迁移数据..."
    })).map_err(|e| e.to_string())?;

    // 2. 验证目标路径
    if target_path == current_path {
        return Err("目标路径与当前路径相同".to_string());
    }

    // 3. 创建临时迁移目录
    let temp_path = target_path.join(".migrating");
    fs::create_dir_all(&temp_path)
        .map_err(|e| format!("创建临时目录失败: {}", e))?;

    // 确保迁移成功或失败时清理临时目录
    let cleanup_temp = || {
        if temp_path.exists() {
            let _ = fs::remove_dir_all(&temp_path);
        }
    };

    // 4. 复制数据库文件
    app.emit("migrate-progress", json!({
        "status": "copying_db",
        "message": "正在复制数据库..."
    })).map_err(|e| e.to_string())?;

    let db_source = current_path.join("rtodo.db");
    if !db_source.exists() {
        cleanup_temp();
        return Err("数据库文件不存在".to_string());
    }

    let db_target = temp_path.join("rtodo.db");
    fs::copy(&db_source, &db_target)
        .map_err(|e| {
            cleanup_temp();
            format!("复制数据库失败: {}", e)
        })?;

    // 5. 复制附件目录（如果存在）
    let attachments_source = current_path.join("attachments");
    if attachments_source.exists() {
        app.emit("migrate-progress", json!({
            "status": "copying_attachments",
            "message": "正在复制附件..."
        })).map_err(|e| e.to_string())?;

        let attachments_target = temp_path.join("attachments");
        copy_dir_recursive(&attachments_source, &attachments_target)
            .map_err(|e| {
                cleanup_temp();
                format!("复制附件失败: {}", e)
            })?;
    }

    // 6. 验证新数据库
    app.emit("migrate-progress", json!({
        "status": "validating",
        "message": "正在验证数据..."
    })).map_err(|e| e.to_string())?;

    rusqlite::Connection::open(&db_target)
        .map_err(|e| {
            cleanup_temp();
            format!("新数据库验证失败: {}", e)
        })?;

    // 7. 原子性替换
    app.emit("migrate-progress", json!({
        "status": "finalizing",
        "message": "正在完成迁移..."
    })).map_err(|e| e.to_string())?;

    // 如果目标路径已存在，先重命名为备份
    if target_path.exists() {
        let backup_path = target_path.with_extension(".bak");
        fs::rename(&target_path, &backup_path)
            .map_err(|e| {
                cleanup_temp();
                format!("创建备份失败: {}", e)
            })?;
    }

    fs::rename(&temp_path, &target_path)
        .map_err(|e| {
            cleanup_temp();
            format!("移动数据失败: {}", e)
        })?;

    // 8. 更新配置
    if let Some(config_state) = app.try_state::<std::sync::Mutex<AppConfig>>() {
        if let Ok(mut config) = config_state.lock() {
            config.data_path = Some(new_path);
            config.save(&app)?;
        }
    }

    // 发送完成事件
    app.emit("migrate-progress", json!({
        "status": "completed",
        "message": "迁移完成！"
    })).map_err(|e| e.to_string())?;

    tracing::info!("Data migration completed successfully");
    Ok(())
}

/// 递归复制目录
fn copy_dir_recursive(source: &PathBuf, target: &PathBuf) -> std::io::Result<()> {
    fs::create_dir_all(target)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());

        if filetype.is_dir() {
            copy_dir_recursive(&source_path, &target_path)?;
        } else {
            fs::copy(&source_path, &target_path)?;
        }
    }
    Ok(())
}
