// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

//! 附件管理命令
//! 处理附件的上传、删除和查询功能

use crate::database::Database;
use crate::database::repositories::AttachmentRepository;
use crate::database::DbConnection;
use crate::models::Attachment;
use std::path::Path;
use std::fs;
use tauri_plugin_shell::ShellExt;

/// 获取任务的所有附件
#[tauri::command]
pub async fn get_attachments(
    todo_id: i64,
    db: tauri::State<'_, Database>,
) -> Result<Vec<Attachment>, String> {
    tracing::info!("get_attachments called: todo_id={}", todo_id);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    AttachmentRepository::list_by_todo(inner, todo_id)
        .map_err(|e| format!("Failed to get attachments: {}", e))
}

/// 上传附件
#[tauri::command]
pub async fn upload_attachment(
    todo_id: i64,
    file_path: String,
    file_name: String,
    db: tauri::State<'_, Database>,
) -> Result<Attachment, String> {
    tracing::info!("upload_attachment called: todo_id={}, file_name={}", todo_id, file_name);

    let source_path = Path::new(&file_path);

    // 验证源文件存在
    if !source_path.exists() {
        return Err("源文件不存在".to_string());
    }

    // 获取文件元数据
    let file_size = fs::metadata(source_path)
        .map_err(|e| format!("无法读取文件元数据: {}", e))?
        .len();

    // 验证文件大小 (50MB 限制)
    const MAX_FILE_SIZE: u64 = 50 * 1024 * 1024; // 50MB
    if file_size > MAX_FILE_SIZE {
        // 格式化文件大小显示
        let size_mb = file_size as f64 / (1024.0 * 1024.0);
        let limit_mb = MAX_FILE_SIZE as f64 / (1024.0 * 1024.0);
        return Err(format!(
            "文件大小超过限制：当前文件 {:.2} MB，限制大小 {:.0} MB",
            size_mb, limit_mb
        ));
    }

    // 获取附件目录
    let attachments_dir = DbConnection::get_attachments_dir()
        .map_err(|e| format!("无法获取数据目录: {}", e))?;

    // 确保附件目录存在
    fs::create_dir_all(&attachments_dir)
        .map_err(|e| format!("无法创建附件目录: {}", e))?;

    // 生成hash文件名（使用UUID）
    let file_ext = source_path.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    
    // 生成唯一的hash文件名
    let hash_filename = if file_ext.is_empty() {
        format!("{}", uuid::Uuid::new_v4())
    } else {
        format!("{}.{}", uuid::Uuid::new_v4(), file_ext)
    };
    
    let target_path = attachments_dir.join(&hash_filename);

    // 复制文件到附件目录
    fs::copy(&source_path, &target_path)
        .map_err(|e| format!("复制文件失败: {}", e))?;

    // 存储相对路径（使用hash文件名）
    let relative_path = format!("attachments/{}", hash_filename);

    // MIME 类型检测（简化版）
    let mime_type = mime_guess::from_path(&file_name)
        .first()
        .map(|m| m.to_string());

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    AttachmentRepository::create(
        inner,
        todo_id,
        &file_name,  // 数据库中存储真实文件名
        &relative_path,  // 文件系统中使用hash文件名
        file_size as i64,
        mime_type.as_deref()
    ).map_err(|e| {
        // 失败时删除已复制的文件
        let _ = fs::remove_file(&target_path);
        format!("Failed to create attachment record: {}", e)
    })
}

/// 删除附件
#[tauri::command]
pub async fn delete_attachment(
    id: i64,
    db: tauri::State<'_, Database>,
) -> Result<(), String> {
    tracing::info!("delete_attachment called: id={}", id);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    // 先查询附件信息，获取文件路径
    let attachment = AttachmentRepository::get(inner, id)
        .map_err(|e| format!("查询附件失败: {}", e))?
        .ok_or_else(|| "附件不存在".to_string())?;

    // 删除数据库记录
    AttachmentRepository::delete(inner, id)
        .map_err(|e| format!("删除数据库记录失败: {}", e))?;

    // 删除物理文件
    let data_dir = DbConnection::get_data_dir()
        .map_err(|e| format!("无法获取数据目录: {}", e))?;
    let file_path = data_dir.join(&attachment.file_path);

    if file_path.exists() {
        if let Err(e) = fs::remove_file(&file_path) {
            tracing::warn!("删除物理文件失败: {} (已删除数据库记录)", e);
            // 即使文件删除失败，也不中断流程（数据库记录已删除）
        }
    }

    tracing::info!("Attachment deleted successfully: id={}", id);
    Ok(())
}

/// 打开附件
#[tauri::command]
pub async fn open_attachment(
    attachment_id: i64,
    db: tauri::State<'_, Database>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    tracing::info!("open_attachment called: id={}", attachment_id);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    let attachment = AttachmentRepository::get(inner, attachment_id)
        .map_err(|e| format!("查询附件失败: {}", e))?
        .ok_or_else(|| "附件不存在".to_string())?;

    let data_dir = DbConnection::get_data_dir()
        .map_err(|e| format!("无法获取数据目录: {}", e))?;
    let full_path = data_dir.join(&attachment.file_path);

    if !full_path.exists() {
        return Err("附件文件不存在".to_string());
    }

    // 使用 shell 插件打开文件
    let path_str = full_path.to_string_lossy().to_string();
    app.shell().open(path_str, None)
        .map_err(|e| format!("无法打开文件: {}", e))?;

    Ok(())
}

/// 下载附件到指定位置
#[tauri::command]
pub async fn download_attachment(
    attachment_id: i64,
    target_path: String,
    db: tauri::State<'_, Database>,
) -> Result<(), String> {
    tracing::info!("download_attachment called: id={}, target={}", attachment_id, target_path);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    let attachment = AttachmentRepository::get(inner, attachment_id)
        .map_err(|e| format!("查询附件失败: {}", e))?
        .ok_or_else(|| "附件不存在".to_string())?;

    let data_dir = DbConnection::get_data_dir()
        .map_err(|e| format!("无法获取数据目录: {}", e))?;
    let source_path = data_dir.join(&attachment.file_path);

    if !source_path.exists() {
        return Err("附件文件不存在".to_string());
    }

    // 复制文件到目标位置
    fs::copy(&source_path, &target_path)
        .map_err(|e| format!("复制文件失败: {}", e))?;

    tracing::info!("Attachment downloaded successfully: id={} to {}", attachment_id, target_path);
    Ok(())
}
