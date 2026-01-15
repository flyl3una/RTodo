// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use crate::database::Database;
use crate::database::repositories::TodoRepository;
use crate::models::{Todo, UpdateTodoRequest};
use crate::models::todo::GetTodosRequest;

/// 获取任务列表
#[tauri::command]
pub async fn get_todos(
    payload: GetTodosRequest,
    db: tauri::State<'_, Database>,
) -> Result<Vec<Todo>, String> {
    tracing::info!("get_todos called: group_id={:?}, tag_id={:?}, status={:?}, search={:?}, priority={:?}, start_date={:?}, end_date={:?}",
        payload.group_id, payload.tag_id, payload.status, payload.search, payload.priority, payload.start_date, payload.end_date);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    let result = TodoRepository::list(
        inner,
        payload.group_id,
        payload.tag_id,
        payload.status,
        payload.search.as_deref(),
        payload.priority,
        payload.start_date,
        payload.end_date,
    ).map_err(|e| {
        tracing::error!("get_todos failed: {}", e);
        format!("Failed to get todos: {}", e)
    })?;

    tracing::info!("get_todos returned {} todos", result.len());
    Ok(result)
}

/// 获取单个任务详情
#[tauri::command]
pub async fn get_todo(
    id: i64,
    db: tauri::State<'_, Database>,
) -> Result<Todo, String> {
    tracing::info!("get_todo called: id={}", id);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    let result = TodoRepository::get(inner, id)
        .map_err(|e| {
            tracing::error!("get_todo failed for id={}: {}", id, e);
            format!("Failed to get todo: {}", e)
        })?
        .ok_or_else(|| {
            tracing::warn!("get_todo: todo not found: {}", id);
            format!("Todo not found: {}", id)
        })?;

    tracing::info!("get_todo returned todo: id={}, title={}", result.id, result.title);
    Ok(result)
}

/// 创建任务
#[tauri::command]
pub async fn create_todo(
    title: String,
    description: Option<String>,
    group_id: Option<i64>,
    start_date: Option<i64>,
    due_date: Option<i64>,
    priority: Option<i32>,
    tag_ids: Option<Vec<i64>>,
    db: tauri::State<'_, Database>,
) -> Result<Todo, String> {
    tracing::info!("create_todo called: title={}, start_date={:?}, due_date={:?}, group_id={:?}, tag_ids={:?}",
        title, start_date, due_date, group_id, tag_ids);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    let result = TodoRepository::create(
        inner,
        &title,
        description.as_deref(),
        group_id,
        start_date,
        due_date,
        priority.unwrap_or(0),
        tag_ids,
    ).map_err(|e| {
        tracing::error!("create_todo failed: {}", e);
        format!("Failed to create todo: {}", e)
    })?;

    tracing::info!("create_todo succeeded: id={}, title={}", result.id, result.title);
    Ok(result)
}

/// 更新任务
#[tauri::command]
pub async fn update_todo(
    payload: UpdateTodoRequest,
    db: tauri::State<'_, Database>,
) -> Result<Todo, String> {
    tracing::info!("update_todo called: id={}, title={:?}, start_date={:?}, due_date={:?}, status={:?}",
        payload.id, payload.title, payload.start_date, payload.due_date, payload.status);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    // 将 Option<String> 转换为 Option<Option<String>> 用于表示是否需要更新
    let desc_opt: Option<Option<String>> = payload.description.map(Some);
    let group_opt: Option<Option<i64>> = payload.group_id.map(Some);
    let assignee_opt: Option<Option<String>> = payload.assignee.map(Some);
    let start_opt: Option<Option<i64>> = payload.start_date.map(Some);
    let due_opt: Option<Option<i64>> = payload.due_date.map(Some);

    let result = TodoRepository::update(
        inner,
        payload.id,
        payload.title.as_deref(),
        desc_opt,
        payload.status.map(|s| s as i32),
        payload.priority,
        group_opt,
        assignee_opt,
        start_opt,
        due_opt,
        payload.tag_ids,
    ).map_err(|e| {
        tracing::error!("update_todo failed for id={}: {}", payload.id, e);
        format!("Failed to update todo: {}", e)
    })?;

    tracing::info!("update_todo succeeded: id={}, start_date={:?}, due_date={:?}",
        result.id, result.start_date, result.due_date);
    Ok(result)
}

/// 删除任务
#[tauri::command]
pub async fn delete_todo(
    id: i64,
    db: tauri::State<'_, Database>,
) -> Result<(), String> {
    tracing::info!("delete_todo called: id={}", id);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    TodoRepository::delete(inner, id)
        .map_err(|e| {
            tracing::error!("delete_todo failed for id={}: {}", id, e);
            format!("Failed to delete todo: {}", e)
        })?;

    tracing::info!("delete_todo succeeded: id={}", id);
    Ok(())
}

/// 更新任务状态
#[tauri::command]
pub async fn update_todo_status(
    id: i64,
    status: i32,
    db: tauri::State<'_, Database>,
) -> Result<Todo, String> {
    tracing::info!("update_todo_status called: id={}, status={}", id, status);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    let result = TodoRepository::update_status(inner, id, status)
        .map_err(|e| {
            tracing::error!("update_todo_status failed for id={}: {}", id, e);
            format!("Failed to update todo status: {}", e)
        })?;

    tracing::info!("update_todo_status succeeded: id={}, status={}", result.id, result.status as i32);
    Ok(result)
}
