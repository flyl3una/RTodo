// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use crate::database::Database;
use crate::database::repositories::TodoRepository;
use crate::models::Todo;

/// 获取任务列表
#[tauri::command]
pub async fn get_todos(
    group_id: Option<String>,
    tag_id: Option<String>,
    status: Option<String>,
    search: Option<String>,
    is_marked: Option<bool>,
    priority: Option<i32>,
    start_date: Option<i64>,
    end_date: Option<i64>,
    db: tauri::State<'_, Database>,
) -> Result<Vec<Todo>, String> {
    tracing::info!("get_todos called: group_id={:?}, tag_id={:?}, status={:?}, search={:?}, is_marked={:?}, priority={:?}, start_date={:?}, end_date={:?}",
        group_id, tag_id, status, search, is_marked, priority, start_date, end_date);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    TodoRepository::list(
        inner,
        group_id.as_deref(),
        tag_id.as_deref(),
        status.as_deref(),
        search.as_deref(),
        is_marked,
        priority,
        start_date,
        end_date,
    ).map_err(|e| format!("Failed to get todos: {}", e))
}

/// 获取单个任务详情
#[tauri::command]
pub async fn get_todo(
    id: String,
    db: tauri::State<'_, Database>,
) -> Result<Todo, String> {
    tracing::info!("get_todo called: id={}", id);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    TodoRepository::get(inner, &id)
        .map_err(|e| format!("Failed to get todo: {}", e))?
        .ok_or_else(|| format!("Todo not found: {}", id))
}

/// 创建任务
#[tauri::command]
pub async fn create_todo(
    title: String,
    description: Option<String>,
    group_id: Option<String>,
    start_date: Option<i64>,
    due_date: Option<i64>,
    priority: Option<i32>,
    tag_ids: Option<Vec<String>>,
    db: tauri::State<'_, Database>,
) -> Result<Todo, String> {
    tracing::info!("create_todo called: title={}", title);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    TodoRepository::create(
        inner,
        &title,
        description.as_deref(),
        group_id.as_deref(),
        start_date,
        due_date,
        priority.unwrap_or(0),
        tag_ids,
    ).map_err(|e| format!("Failed to create todo: {}", e))
}

/// 更新任务
#[tauri::command]
pub async fn update_todo(
    id: String,
    title: Option<String>,
    description: Option<String>,
    status: Option<String>,
    priority: Option<i32>,
    is_marked: Option<bool>,
    group_id: Option<String>,
    assignee: Option<String>,
    start_date: Option<i64>,
    due_date: Option<i64>,
    tag_ids: Option<Vec<String>>,
    db: tauri::State<'_, Database>,
) -> Result<Todo, String> {
    tracing::info!("update_todo called: id={}", id);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    // 将 Option<String> 转换为 Option<Option<String>> 用于表示是否需要更新
    let desc_opt: Option<Option<String>> = description.map(Some);
    let group_opt: Option<Option<String>> = group_id.map(Some);
    let assignee_opt: Option<Option<String>> = assignee.map(Some);
    let start_opt: Option<Option<i64>> = start_date.map(Some);
    let due_opt: Option<Option<i64>> = due_date.map(Some);

    TodoRepository::update(
        inner,
        &id,
        title.as_deref(),
        desc_opt,
        status.as_deref(),
        priority,
        is_marked,
        group_opt,
        assignee_opt,
        start_opt,
        due_opt,
        tag_ids,
    ).map_err(|e| format!("Failed to update todo: {}", e))
}

/// 删除任务
#[tauri::command]
pub async fn delete_todo(
    id: String,
    db: tauri::State<'_, Database>,
) -> Result<(), String> {
    tracing::info!("delete_todo called: id={}", id);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    TodoRepository::delete(inner, &id)
        .map_err(|e| format!("Failed to delete todo: {}", e))
}

/// 更新任务状态
#[tauri::command]
pub async fn update_todo_status(
    id: String,
    status: String,
    db: tauri::State<'_, Database>,
) -> Result<Todo, String> {
    tracing::info!("update_todo_status called: id={}, status={}", id, status);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    TodoRepository::update_status(inner, &id, &status)
        .map_err(|e| format!("Failed to update todo status: {}", e))
}

/// 切换任务重要标记
#[tauri::command]
pub async fn toggle_todo_mark(
    id: String,
    db: tauri::State<'_, Database>,
) -> Result<Todo, String> {
    tracing::info!("toggle_todo_mark called: id={}", id);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    TodoRepository::toggle_mark(inner, &id)
        .map_err(|e| format!("Failed to toggle todo mark: {}", e))
}
