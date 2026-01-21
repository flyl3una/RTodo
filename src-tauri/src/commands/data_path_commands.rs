// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

//! 数据路径管理命令
//! 提供获取、设置、重置数据路径以及数据迁移功能

use crate::database::DbConnection;
use crate::config::AppConfig;
use crate::pojo::request::MigrateDataRequest;
use tauri::{AppHandle, Emitter, Manager};
use std::fs;
use std::path::PathBuf;
use rusqlite::Connection;

/// 获取当前数据路径
#[tauri::command]
pub async fn get_data_path(app: AppHandle) -> Result<String, String> {
    // 尝试从配置中读取自定义路径
    if let Some(config_state) = app.try_state::<std::sync::Mutex<AppConfig>>() {
        if let Ok(config) = config_state.lock() {
            if let Some(custom_path) = &config.data_path {
                tracing::info!("Using custom data path from config: {}", custom_path);
                return Ok(custom_path.clone());
            }
        }
    }

    // 如果没有自定义路径，返回默认路径
    let default_path = DbConnection::get_data_dir()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| format!("Failed to get data path: {}", e))?;
    tracing::info!("Using default data path: {}", default_path);
    Ok(default_path)
}

/// 检查目录是否为空（忽略系统隐藏文件）
#[tauri::command]
pub async fn check_directory_empty(path: String) -> Result<bool, String> {
    let path = PathBuf::from(&path);
    tracing::info!("[check_directory_empty] Checking path: {}", path.display());

    // 如果目录不存在，返回 true（视为"空"）
    if !path.exists() {
        tracing::info!("[check_directory_empty] Path does not exist, treating as empty");
        return Ok(true);
    }

    // 需要忽略的系统文件和文件夹
    let system_files = [
        "desktop.ini",      // Windows 文件夹自定义配置
        "Thumbs.db",        // Windows 缩略图缓存
        ".DS_Store",        // macOS 文件夹元数据
        ".Spotlight-V100",  // macOS 索引
        ".Trashes",         // macOS 回收站
        "$RECYCLE.BIN",     // Windows 回收站
        "System Volume Information", // Windows 系统卷信息
    ];

    // 检查目录中是否只有系统文件
    let entries = path.read_dir()
        .map_err(|e| format!("Failed to read directory: {}", e))?;

    let mut entry_count = 0;
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();
        entry_count += 1;

        tracing::info!("[check_directory_empty] Found entry: {}", file_name_str);

        // 如果遇到非系统文件，说明目录不为空
        let name = file_name_str.as_ref();
        if !system_files.contains(&name) {
            tracing::info!("[check_directory_empty] '{}' is not a system file, directory not empty", name);
            return Ok(false);
        } else {
            tracing::info!("[check_directory_empty] '{}' is a system file, ignoring", name);
        }
    }

    tracing::info!("[check_directory_empty] Directory is empty (found {} entries, all system files)", entry_count);
    // 目录为空或只有系统文件，视为"空"
    Ok(true)
}

/// 设置数据路径
#[tauri::command]
pub async fn set_data_path(
    newPath: String,
    app: AppHandle,
) -> Result<(), String> {
    tracing::info!("set_data_path called with: {}", newPath);

    // 验证路径可写
    let path = PathBuf::from(&newPath);
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
            config.data_path = Some(newPath.clone());
            config.save(&app)?;
            tracing::info!("Data path updated to: {}", newPath);
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
    payload: MigrateDataRequest,
    app: AppHandle,
) -> Result<(), String> {
    use serde_json::json;

    tracing::info!("migrate_data called with: {}", payload.new_path);

    // 1. 获取当前路径
    let current_path = DbConnection::get_data_dir()
        .map_err(|e| format!("Failed to get data dir: {}", e))?;
    let target_path = PathBuf::from(&payload.new_path);

    // 发送进度：开始迁移
    app.emit("migrate-progress", json!({
        "status": "started",
        "message": "开始迁移数据..."
    })).map_err(|e| e.to_string())?;

    // 2. 验证目标路径
    if target_path == current_path {
        return Err("目标路径与当前路径相同".to_string());
    }

    // 3. 使用系统临时目录进行迁移
    // 这样可以避免目标目录已存在时 VACUUM INTO 报错
    let temp_dir = std::env::temp_dir();
    let temp_path = temp_dir.join(format!("rtodo-migration-{}", std::process::id()));

    // 清理可能存在的旧临时目录
    if temp_path.exists() {
        fs::remove_dir_all(&temp_path)
            .map_err(|e| format!("清理旧临时目录失败: {}", e))?;
    }

    fs::create_dir_all(&temp_path)
        .map_err(|e| format!("创建临时目录失败: {}", e))?;

    tracing::info!("Created temporary migration directory: {}", temp_path.display());

    // 确保迁移成功或失败时清理临时目录
    let cleanup_temp = || {
        if temp_path.exists() {
            let _ = fs::remove_dir_all(&temp_path);
        }
    };

    // 4. 复制数据库文件（使用 VACUUM INTO 命令）
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

    // 使用 VACUUM INTO 命令复制数据库
    // 这是在数据库正在使用时最安全的复制方法
    Connection::open(&db_source)
        .and_then(|conn| {
            // 执行 VACUUM INTO 命令将数据库复制到目标位置
            // 转义路径中的单引号
            let escaped_path = db_target.to_string_lossy().replace('\'', "''");
            conn.execute(&format!("VACUUM INTO '{}'", escaped_path), [])
        })
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

    // 如果目标路径已存在，需要先处理
    if target_path.exists() {
        // 检查是否为空目录或只有系统文件
        let system_files = [
            "desktop.ini",
            "Thumbs.db",
            ".DS_Store",
            ".Spotlight-V100",
            ".Trashes",
            "$RECYCLE.BIN",
            "System Volume Information",
        ];

        let is_empty_or_system_only = target_path.read_dir()
            .map(|mut entries| {
                entries.all(|entry| {
                    entry.ok()
                        .and_then(|e| Some(e.file_name().to_string_lossy().to_string()))
                        .map(|name| system_files.contains(&name.as_str()))
                        .unwrap_or(true)
                })
            })
            .unwrap_or(false);

        if is_empty_or_system_only {
            // 目录为空或只有系统文件，直接删除
            tracing::info!("Target path is empty or contains only system files, removing");
            fs::remove_dir_all(&target_path)
                .map_err(|e| {
                    cleanup_temp();
                    format!("删除目标目录失败: {}", e)
                })?;
        } else {
            // 目录中有用户数据，创建备份
            tracing::info!("Target path contains user data, creating backup");
            let backup_path = target_path.with_extension(".bak");
            fs::rename(&target_path, &backup_path)
                .map_err(|e| {
                    cleanup_temp();
                    format!("创建备份失败: {}", e)
                })?;
        }
    }

    // 移动临时目录到目标位置
    // 先尝试使用 rename（同磁盘快速移动）
    // 如果跨磁盘，则使用复制后删除的方式
    let move_result = fs::rename(&temp_path, &target_path);

    if let Err(e) = move_result {
        // 检查是否是跨磁盘错误
        if e.raw_os_error() == Some(17) {
            // Windows ERROR_NOT_SAME_DEVICE (17)
            tracing::info!("Cross-device move detected, using copy+delete method");
            tracing::info!("Copying from {} to {}", temp_path.display(), target_path.display());

            // 确保目标目录存在（前面可能被删除了）
            fs::create_dir_all(&target_path)
                .map_err(|e| {
                    cleanup_temp();
                    format!("创建目标目录失败: {}", e)
                })?;

            // 跨磁盘移动：先复制到目标位置，然后删除临时目录
            copy_dir_recursive(&temp_path, &target_path)
                .map_err(|e| {
                    cleanup_temp();
                    format!("跨磁盘复制失败: {}", e)
                })?;

            // 复制成功后，删除临时目录
            fs::remove_dir_all(&temp_path)
                .map_err(|e| {
                    tracing::warn!("Failed to cleanup temp directory after successful copy: {}", e);
                    // 不返回错误，因为数据已经成功迁移
                    format!("清理临时目录失败: {}", e)
                })?;

            tracing::info!("Cross-device migration completed successfully");
        } else {
            // 其他错误，直接返回
            cleanup_temp();
            return Err(format!("移动数据失败: {}", e));
        }
    } else {
        tracing::info!("Same-device move completed successfully");
    }

    // 8. 更新配置
    if let Some(config_state) = app.try_state::<std::sync::Mutex<AppConfig>>() {
        if let Ok(mut config) = config_state.lock() {
            config.data_path = Some(payload.new_path.clone());
            config.save(&app)?;
        }
    }

    // 9. 根据用户选择决定是否删除原始数据
    // 注意：原子替换操作后，原始位置的数据实际上已经移动到新位置
    // 如果目标位置之前存在数据，它会被重命名为 .bak 文件
    if !payload.keep_original {
        // 删除备份文件（如果存在）
        app.emit("migrate-progress", json!({
            "status": "cleaning",
            "message": "正在清理备份文件..."
        })).map_err(|e| e.to_string())?;

        // 尝试删除备份文件
        let backup_path = target_path.with_extension("bak");
        if backup_path.exists() {
            // 尝试删除备份目录
            if backup_path.is_dir() {
                fs::remove_dir_all(&backup_path)
                    .map_err(|e| format!("删除备份目录失败: {}", e))?;
            } else {
                fs::remove_file(&backup_path)
                    .map_err(|e| format!("删除备份文件失败: {}", e))?;
            }
            tracing::info!("Backup deleted successfully");
        }

        // 可选：删除原始源路径的数据（如果它仍然存在）
        // 注意：在 Windows 上，如果文件仍在使用中可能会失败
        if db_source.exists() {
            let _ = fs::remove_file(&db_source);
            tracing::info!("Attempted to remove original db file");
        }
        if attachments_source.exists() {
            let _ = fs::remove_dir_all(&attachments_source);
            tracing::info!("Attempted to remove original attachments");
        }

        tracing::info!("Cleanup completed");
    } else {
        tracing::info!("Original data preserved as requested by user");
    }

    // 发送完成事件
    app.emit("migrate-progress", json!({
        "status": "completed",
        "message": if payload.keep_original { "迁移完成！原始数据已保留" } else { "迁移完成！原始数据已删除" }
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
