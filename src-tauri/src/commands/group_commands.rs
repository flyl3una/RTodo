// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use crate::database::Database;
use crate::database::repositories::GroupRepository;
use crate::models::TaskGroup;

/// 获取所有任务组
#[tauri::command]
pub async fn get_task_groups(
    db: tauri::State<'_, Database>,
) -> Result<Vec<TaskGroup>, String> {
    tracing::info!("get_task_groups called");

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    GroupRepository::list(inner)
        .map_err(|e| format!("Failed to get task groups: {}", e))
}

/// 创建任务组
#[tauri::command]
pub async fn create_task_group(
    name: String,
    parent_id: Option<i64>,
    icon: Option<String>,
    color: Option<String>,
    db: tauri::State<'_, Database>,
) -> Result<TaskGroup, String> {
    tracing::info!("create_task_group called: name={}", name);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    GroupRepository::create(
        inner,
        &name,
        parent_id,
        icon.as_deref(),
        color.as_deref(),
    )
    .map_err(|e| format!("Failed to create task group: {}", e))
}

/// 更新任务组
#[tauri::command]
pub async fn update_task_group(
    id: i64,
    name: Option<String>,
    parent_id: Option<i64>,
    icon: Option<String>,
    color: Option<String>,
    db: tauri::State<'_, Database>,
) -> Result<TaskGroup, String> {
    tracing::info!("update_task_group called: id={}", id);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    GroupRepository::update(
        inner,
        id,
        name.as_deref(),
        parent_id,
        icon.as_deref(),
        color.as_deref(),
    )
    .map_err(|e| format!("Failed to update task group: {}", e))
}

/// 删除任务组
#[tauri::command]
pub async fn delete_task_group(
    id: i64,
    db: tauri::State<'_, Database>,
) -> Result<(), String> {
    tracing::info!("delete_task_group called: id={}", id);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    GroupRepository::delete(inner, id)
        .map_err(|e| format!("Failed to delete task group: {}", e))
}
