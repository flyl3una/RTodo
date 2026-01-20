// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

//! 数据管理命令
//! 处理数据的导入、导出和清理操作

use crate::database::Database;
use crate::database::repositories::DataRepository;
use crate::utils::data_export::{
    export_groups_to_csv,
    export_tags_to_csv,
    export_todos_to_csv,
    export_todo_tags_to_csv,
    create_zip_archive_with_attachments,
    extract_csv_from_zip,
    extract_attachments_from_zip,
};
use std::path::PathBuf;

/// 导出所有数据为 JSON 格式
#[tauri::command]
pub async fn export_all_data(
    db: tauri::State<'_, Database>,
) -> Result<crate::models::ExportData, String> {
    tracing::info!("export_all_data called");

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    DataRepository::export_all(inner)
        .map_err(|e| format!("Failed to export data: {}", e))
}

/// 导入数据（JSON 格式）
#[tauri::command]
pub async fn import_data(
    data: crate::models::ExportData,
    db: tauri::State<'_, Database>,
) -> Result<(), String> {
    tracing::info!("import_data called: version={}, items={}", data.version, data.todos.len());

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    DataRepository::import_data(inner, &data)
        .map_err(|e| format!("Failed to import data: {}", e))
}

/// 导出所有数据为 CSV 格式并保存到指定文件（包含附件）
#[tauri::command]
pub async fn export_data_as_csv(
    file_path: String,
    db: tauri::State<'_, Database>,
) -> Result<(), String> {
    tracing::info!("export_data_as_csv called: path={}", file_path);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    // 从数据库导出数据
    let export_data = DataRepository::export_all(inner)
        .map_err(|e| format!("Failed to export data: {}", e))?;

    // 转换为 CSV 格式
    let groups_csv = export_groups_to_csv(&export_data.task_groups)
        .map_err(|e| format!("Failed to export groups to CSV: {}", e))?;
    let tags_csv = export_tags_to_csv(&export_data.tags)
        .map_err(|e| format!("Failed to export tags to CSV: {}", e))?;
    let todos_csv = export_todos_to_csv(&export_data.todos)
        .map_err(|e| format!("Failed to export todos to CSV: {}", e))?;
    let todo_tags_csv = export_todo_tags_to_csv(&export_data.todos)
        .map_err(|e| format!("Failed to export todo tags to CSV: {}", e))?;

    // 获取附件目录路径
    let attachments_dir = crate::database::DbConnection::get_attachments_dir()
        .unwrap_or_else(|_| PathBuf::from("attachments"));

    let attachments_path = if attachments_dir.exists() {
        Some(attachments_dir.as_path())
    } else {
        tracing::info!("Attachments directory does not exist: {}", attachments_dir.display());
        None
    };

    // 创建包含附件的 ZIP 压缩包
    let zip_data = create_zip_archive_with_attachments(
        groups_csv,
        tags_csv,
        todos_csv,
        todo_tags_csv,
        attachments_path,
    )
    .map_err(|e| format!("Failed to create ZIP: {}", e))?;

    // 直接写入文件
    std::fs::write(&file_path, &zip_data)
        .map_err(|e| format!("Failed to write export file: {}", e))?;

    tracing::info!("Export completed: {} bytes written to {}", zip_data.len(), file_path);

    Ok(())
}

/// 从 CSV 压缩包导入数据（通过文件路径，包含附件）
#[tauri::command]
pub async fn import_data_from_csv(
    file_path: String,
    db: tauri::State<'_, Database>,
) -> Result<(), String> {
    tracing::info!("import_data_from_csv called: path={}", file_path);

    // 读取文件
    let file_data = std::fs::read(&file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    tracing::info!("File read successfully: {} bytes", file_data.len());

    // 解析 ZIP 文件中的 CSV 数据
    let csv_data = extract_csv_from_zip(file_data.clone())
        .map_err(|e| format!("Failed to extract CSV: {}", e))?;

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    // 使用 repository 层的方法导入数据
    DataRepository::import_from_csv(inner, &csv_data)
        .map_err(|e| format!("Failed to import CSV data: {}", e))?;

    tracing::info!("CSV data imported successfully");

    // 导入附件文件
    let attachments_dir = crate::database::DbConnection::get_attachments_dir()
        .map_err(|e| format!("Failed to get attachments directory: {}", e))?;

    tracing::info!("Extracting attachments to: {}", attachments_dir.display());

    match extract_attachments_from_zip(file_data, &attachments_dir) {
        Ok(_) => {
            tracing::info!("Attachments imported successfully");
        }
        Err(e) => {
            // 附件导入失败不应阻止整体导入，记录警告即可
            tracing::warn!("Failed to import attachments (non-critical): {}", e);
        }
    }

    Ok(())
}

/// 清空所有数据
#[tauri::command]
pub async fn clear_all_data(
    db: tauri::State<'_, Database>,
) -> Result<(), String> {
    tracing::info!("clear_all_data called");

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    DataRepository::clear_all(inner)
        .map_err(|e| format!("Failed to clear data: {}", e))
}
