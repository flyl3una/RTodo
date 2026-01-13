// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use crate::database::Database;
use crate::database::repositories::AttachmentRepository;
use crate::models::Attachment;
use std::path::Path;

/// 获取任务的所有附件
#[tauri::command]
pub async fn get_attachments(
    todo_id: String,
    db: tauri::State<'_, Database>,
) -> Result<Vec<Attachment>, String> {
    tracing::info!("get_attachments called: todo_id={}", todo_id);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    AttachmentRepository::list_by_todo(inner, &todo_id)
        .map_err(|e| format!("Failed to get attachments: {}", e))
}

/// 上传附件
#[tauri::command]
pub async fn upload_attachment(
    todo_id: String,
    file_path: String,
    file_name: String,
    db: tauri::State<'_, Database>,
) -> Result<Attachment, String> {
    tracing::info!("upload_attachment called: todo_id={}, file_name={}", todo_id, file_name);

    // 获取文件大小
    let path = Path::new(&file_path);
    let file_size = std::fs::metadata(path)
        .map_err(|e| format!("Failed to get file metadata: {}", e))?
        .len();

    // MIME 类型（暂不实现）
    let mime_type: Option<String> = None;

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    AttachmentRepository::create(inner, &todo_id, &file_name, &file_path, file_size as i64, mime_type.as_deref())
        .map_err(|e| format!("Failed to upload attachment: {}", e))
}

/// 删除附件
#[tauri::command]
pub async fn delete_attachment(
    id: String,
    db: tauri::State<'_, Database>,
) -> Result<(), String> {
    tracing::info!("delete_attachment called: id={}", id);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    AttachmentRepository::delete(inner, &id)
        .map_err(|e| format!("Failed to delete attachment: {}", e))
}
