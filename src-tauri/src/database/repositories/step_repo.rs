// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use rusqlite::{Connection, params, OptionalExtension};
use anyhow::{Result, Context};
use chrono::Utc;

use crate::models::TodoStep;

/// TodoStep 仓库
pub struct StepRepository;

impl StepRepository {
    /// 获取任务的所有步骤
    pub fn list_by_todo(conn: &Connection, todo_id: &str) -> Result<Vec<TodoStep>> {
        let mut stmt = conn.prepare(
            "SELECT id, todo_id, title, is_completed, sort_order, created_at
             FROM todo_steps
             WHERE todo_id = ?
             ORDER BY sort_order ASC"
        )
        .context("Failed to prepare list steps query")?;

        let steps = stmt.query_map(params![todo_id], |row| {
            Ok(TodoStep {
                id: row.get(0)?,
                todo_id: row.get(1)?,
                title: row.get(2)?,
                is_completed: row.get::<_, i32>(3)? == 1,
                sort_order: row.get(4)?,
                created_at: row.get(5)?,
            })
        })
        .context("Failed to execute list steps query")?
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to parse steps")?;

        Ok(steps)
    }

    /// 根据 ID 获取单个步骤
    pub fn get(conn: &Connection, id: &str) -> Result<Option<TodoStep>> {
        let step = conn.query_row(
            "SELECT id, todo_id, title, is_completed, sort_order, created_at
             FROM todo_steps WHERE id = ?",
            params![id],
            |row| {
                Ok(TodoStep {
                    id: row.get(0)?,
                    todo_id: row.get(1)?,
                    title: row.get(2)?,
                    is_completed: row.get::<_, i32>(3)? == 1,
                    sort_order: row.get(4)?,
                    created_at: row.get(5)?,
                })
            },
        )
        .optional()
        .context("Failed to execute get step query")?;

        Ok(step)
    }

    /// 创建步骤
    pub fn create(
        conn: &Connection,
        todo_id: &str,
        title: &str,
    ) -> Result<TodoStep> {
        // 获取当前最大 sort_order
        let sort_order: i32 = conn.query_row(
            "SELECT COALESCE(MAX(sort_order), 0) + 10 FROM todo_steps WHERE todo_id = ?",
            params![todo_id],
            |row| row.get(0),
        ).unwrap_or(10);

        let now = Utc::now().timestamp_millis();
        let id = uuid::Uuid::new_v4().to_string();

        conn.execute(
            "INSERT INTO todo_steps (id, todo_id, title, is_completed, sort_order, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![&id, todo_id, title, 0, sort_order, now],
        )
        .context("Failed to insert step")?;

        Ok(TodoStep {
            id,
            todo_id: todo_id.to_string(),
            title: title.to_string(),
            is_completed: false,
            sort_order,
            created_at: now,
        })
    }

    /// 切换步骤状态
    pub fn toggle(conn: &Connection, id: &str) -> Result<TodoStep> {
        // 先获取当前状态
        let current: i32 = conn.query_row(
            "SELECT is_completed FROM todo_steps WHERE id = ?",
            params![id],
            |row| row.get(0),
        )
        .context("Failed to get current step status")?;

        let new_completed = if current == 0 { 1 } else { 0 };

        conn.execute(
            "UPDATE todo_steps SET is_completed = ?1 WHERE id = ?2",
            params![new_completed, id],
        )
        .context("Failed to toggle step status")?;

        Self::get(conn, id)?.context("Step not found after toggle")
    }

    /// 删除步骤
    pub fn delete(conn: &Connection, id: &str) -> Result<()> {
        let rows_affected = conn.execute(
            "DELETE FROM todo_steps WHERE id = ?",
            params![id],
        )
        .context("Failed to delete step")?;

        if rows_affected == 0 {
            anyhow::bail!("Step not found");
        }

        Ok(())
    }
}
