// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use crate::database::Database;
use crate::database::repositories::{
    TodoRepository, GroupRepository, TagRepository,
};
use crate::models::ExportData;
use chrono::Utc;
use rusqlite::params;

/// 导出所有数据
#[tauri::command]
pub async fn export_all_data(
    db: tauri::State<'_, Database>,
) -> Result<ExportData, String> {
    tracing::info!("export_all_data called");

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    // 导出任务组
    let task_groups = GroupRepository::list(inner)
        .map_err(|e| format!("Failed to export task groups: {}", e))?;

    // 导出标签
    let tags = TagRepository::list(inner)
        .map_err(|e| format!("Failed to export tags: {}", e))?;

    // 导出所有任务（不过滤）
    let todos = TodoRepository::list(
        inner,
        None,  // group_id
        None,  // tag_id
        None,  // status
        None,  // search
        None,  // priority
        None,  // start_date
        None,  // end_date
    ).map_err(|e| format!("Failed to export todos: {}", e))?;

    let exported_at = Utc::now().timestamp_millis();

    Ok(ExportData {
        version: "1.0".to_string(),
        exported_at,
        task_groups,
        tags,
        todos,
    })
}

/// 导入数据
#[tauri::command]
pub async fn import_data(
    data: ExportData,
    db: tauri::State<'_, Database>,
) -> Result<(), String> {
    tracing::info!("import_data called: version={}, items={}", data.version, data.todos.len());

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    // 开始事务
    let transaction = inner.unchecked_transaction()
        .map_err(|e| format!("Failed to start transaction: {}", e))?;

    // 导入任务组（跳过已存在的）
    for group in &data.task_groups {
        // 检查是否已存在同名任务组
        let existing = transaction.query_row(
            "SELECT id FROM task_groups WHERE name = ?",
            params![group.name],
            |row| row.get::<_, String>(0)
        );

        if existing.is_ok() {
            // 已存在，跳过
            continue;
        }

        // 插入新任务组
        transaction.execute(
            "INSERT INTO task_groups (id, name, parent_id, icon, color, sort_order, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                group.id,
                group.name,
                group.parent_id,
                group.icon,
                group.color,
                group.sort_order,
                group.created_at,
                group.updated_at,
            ],
        ).map_err(|e| format!("Failed to import task group {}: {}", group.name, e))?;
    }

    // 导入标签（跳过已存在的）
    for tag in &data.tags {
        // 检查是否已存在同名标签
        let existing = transaction.query_row(
            "SELECT id FROM tags WHERE name = ?",
            params![tag.name],
            |row| row.get::<_, String>(0)
        );

        if existing.is_ok() {
            // 已存在，跳过
            continue;
        }

        // 插入新标签（将 i64 转换为 String）
        transaction.execute(
            "INSERT INTO tags (id, name, color, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![tag.id, tag.name, tag.color, tag.created_at.to_string()],
        ).map_err(|e| format!("Failed to import tag {}: {}", tag.name, e))?;
    }

    // 导入任务
    for todo in &data.todos {
        // 检查是否已存在（根据ID）
        let existing = transaction.query_row(
            "SELECT id FROM todos WHERE id = ?",
            params![todo.id],
            |row| row.get::<_, String>(0)
        );

        if existing.is_ok() {
            // 已存在，更新
            transaction.execute(
                "UPDATE todos SET
                    title = ?1, description = ?2, status = ?3, priority = ?4,
                    group_id = ?5, assignee = ?6,
                    start_date = ?7, due_date = ?8, completed_at = ?9,
                    updated_at = ?10
                 WHERE id = ?11",
                params![
                    todo.title,
                    todo.description,
                    todo.status as i32,
                    todo.priority,
                    todo.group_id,
                    todo.assignee,
                    todo.start_date,
                    todo.due_date,
                    todo.completed_at,
                    todo.updated_at,
                    todo.id,
                ],
            ).map_err(|e| format!("Failed to update todo {}: {}", todo.title, e))?;
        } else {
            // 不存在，插入
            transaction.execute(
                "INSERT INTO todos (
                    id, title, description, status, priority,
                    group_id, assignee, start_date, due_date, completed_at,
                    created_at, updated_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                params![
                    todo.id,
                    todo.title,
                    todo.description,
                    todo.status as i32,
                    todo.priority,
                    todo.group_id,
                    todo.assignee,
                    todo.start_date,
                    todo.due_date,
                    todo.completed_at,
                    todo.created_at,
                    todo.updated_at,
                ],
            ).map_err(|e| format!("Failed to insert todo {}: {}", todo.title, e))?;
        }

        // 导入任务-标签关联
        if let Some(tags) = &todo.tags {
            // 先删除旧的关联
            transaction.execute(
                "DELETE FROM todo_tags WHERE todo_id = ?",
                params![todo.id],
            ).ok();

            // 插入新的关联
            for tag in tags {
                transaction.execute(
                    "INSERT INTO todo_tags (todo_id, tag_id) VALUES (?1, ?2)",
                    params![todo.id, tag.id],
                ).map_err(|e| format!("Failed to link tag to todo {}: {}", todo.title, e))?;
            }
        }
    }

    // 提交事务
    transaction.commit()
        .map_err(|e| format!("Failed to commit import transaction: {}", e))?;

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

    // 开始事务
    let transaction = inner.unchecked_transaction()
        .map_err(|e| format!("Failed to start transaction: {}", e))?;

    // 删除所有数据（按依赖关系倒序）
    transaction.execute("DELETE FROM todo_tags", [])
        .map_err(|e| format!("Failed to clear todo_tags: {}", e))?;

    transaction.execute("DELETE FROM todo_steps", [])
        .map_err(|e| format!("Failed to clear todo_steps: {}", e))?;

    transaction.execute("DELETE FROM attachments", [])
        .map_err(|e| format!("Failed to clear attachments: {}", e))?;

    transaction.execute("DELETE FROM todos", [])
        .map_err(|e| format!("Failed to clear todos: {}", e))?;

    transaction.execute("DELETE FROM tags", [])
        .map_err(|e| format!("Failed to clear tags: {}", e))?;

    transaction.execute("DELETE FROM task_groups", [])
        .map_err(|e| format!("Failed to clear task_groups: {}", e))?;

    // 提交事务
    transaction.commit()
        .map_err(|e| format!("Failed to commit clear transaction: {}", e))?;

    Ok(())
}
