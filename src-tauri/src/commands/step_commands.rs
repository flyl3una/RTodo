// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use crate::database::Database;
use crate::database::repositories::StepRepository;
use crate::models::TodoStep;
use crate::pojo::request::{CreateStepRequest, UpdateStepRequest};

/// 获取任务的所有步骤
#[tauri::command]
pub async fn get_todo_steps(
    todo_id: i64,
    db: tauri::State<'_, Database>,
) -> Result<Vec<TodoStep>, String> {
    tracing::info!("get_todo_steps called: todo_id={}", todo_id);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    StepRepository::list_by_todo(inner, todo_id)
        .map_err(|e| format!("Failed to get steps: {}", e))
}

/// 创建步骤
#[tauri::command]
pub async fn create_step(
    payload: CreateStepRequest,
    db: tauri::State<'_, Database>,
) -> Result<TodoStep, String> {
    tracing::info!("create_step called: todo_id={}, title={}", payload.todo_id, payload.title);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    StepRepository::create(inner, payload.todo_id, &payload.title)
        .map_err(|e| format!("Failed to create step: {}", e))
}

/// 切换步骤状态
#[tauri::command]
pub async fn toggle_step(
    id: i64,
    db: tauri::State<'_, Database>,
) -> Result<TodoStep, String> {
    tracing::info!("toggle_step called: id={}", id);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    StepRepository::toggle(inner, id)
        .map_err(|e| format!("Failed to toggle step: {}", e))
}

/// 更新步骤标题
#[tauri::command]
pub async fn update_step(
    payload: UpdateStepRequest,
    db: tauri::State<'_, Database>,
) -> Result<TodoStep, String> {
    let id = payload.id;
    let title = payload.title.unwrap_or_default();
    tracing::info!("update_step called: id={}, title={}", id, title);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    StepRepository::update(inner, id, &title)
        .map_err(|e| format!("Failed to update step: {}", e))
}

/// 删除步骤
#[tauri::command]
pub async fn delete_step(
    id: i64,
    db: tauri::State<'_, Database>,
) -> Result<(), String> {
    tracing::info!("delete_step called: id={}", id);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    StepRepository::delete(inner, id)
        .map_err(|e| format!("Failed to delete step: {}", e))
}
