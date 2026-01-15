// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use crate::database::Database;
use crate::database::repositories::TagRepository;
use crate::models::Tag;

/// 获取所有标签
#[tauri::command]
pub async fn get_tags(
    db: tauri::State<'_, Database>,
) -> Result<Vec<Tag>, String> {
    tracing::info!("get_tags called");

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    TagRepository::list(inner)
        .map_err(|e| format!("Failed to get tags: {}", e))
}

/// 创建标签
#[tauri::command]
pub async fn create_tag(
    name: String,
    color: String,
    db: tauri::State<'_, Database>,
) -> Result<Tag, String> {
    tracing::info!("create_tag called: name={}", name);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    TagRepository::create(inner, &name, &color)
        .map_err(|e| format!("Failed to create tag: {}", e))
}

/// 更新标签
#[tauri::command]
pub async fn update_tag(
    id: i64,
    name: Option<String>,
    color: Option<String>,
    db: tauri::State<'_, Database>,
) -> Result<Tag, String> {
    tracing::info!("update_tag called: id={}", id);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    TagRepository::update(
        inner,
        id,
        name.as_deref(),
        color.as_deref(),
    )
    .map_err(|e| format!("Failed to update tag: {}", e))
}

/// 删除标签
#[tauri::command]
pub async fn delete_tag(
    id: i64,
    db: tauri::State<'_, Database>,
) -> Result<(), String> {
    tracing::info!("delete_tag called: id={}", id);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    TagRepository::delete(inner, id)
        .map_err(|e| format!("Failed to delete tag: {}", e))
}
