// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use rusqlite::{Connection, params, OptionalExtension};
use anyhow::{Result, Context};
use chrono::Utc;

use crate::models::TaskGroup;

/// TaskGroup 仓库
pub struct GroupRepository;

impl GroupRepository {
    /// 获取所有任务组
    pub fn list(conn: &Connection) -> Result<Vec<TaskGroup>> {
        let mut stmt = conn.prepare(
            "SELECT id, name, parent_id, icon, color, sort_order, created_at, updated_at
             FROM task_groups
             ORDER BY sort_order ASC, name ASC"
        )
        .context("Failed to prepare list groups query")?;

        let groups = stmt.query_map([], |row| {
            Ok(TaskGroup {
                id: row.get(0)?,
                name: row.get(1)?,
                parent_id: row.get(2)?,
                icon: row.get(3)?,
                color: row.get(4)?,
                sort_order: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })
        .context("Failed to execute list groups query")?
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to parse groups")?;

        Ok(groups)
    }

    /// 根据 ID 获取单个任务组
    pub fn get(conn: &Connection, id: &str) -> Result<Option<TaskGroup>> {
        let group = conn.query_row(
            "SELECT id, name, parent_id, icon, color, sort_order, created_at, updated_at
             FROM task_groups WHERE id = ?",
            params![id],
            |row| {
                Ok(TaskGroup {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    parent_id: row.get(2)?,
                    icon: row.get(3)?,
                    color: row.get(4)?,
                    sort_order: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            },
        )
        .optional()
        .context("Failed to execute get group query")?;

        Ok(group)
    }

    /// 创建任务组
    pub fn create(
        conn: &Connection,
        name: &str,
        parent_id: Option<&str>,
        icon: Option<&str>,
        color: Option<&str>,
    ) -> Result<TaskGroup> {
        // 生成新的 sort_order (当前最大值 + 10)
        let sort_order: i32 = conn
            .query_row("SELECT COALESCE(MAX(sort_order), 0) + 10 FROM task_groups", [], |row| {
                row.get(0)
            })
            .unwrap_or(10);

        let now = Utc::now().timestamp_millis();
        let id = uuid::Uuid::new_v4().to_string();
        let icon_value = icon.unwrap_or("📁");
        let color_value = color.unwrap_or("#409EFF");

        conn.execute(
            "INSERT INTO task_groups (id, name, parent_id, icon, color, sort_order, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                &id,
                name,
                parent_id,
                icon_value,
                color_value,
                sort_order,
                now,
                now,
            ],
        )
        .context("Failed to insert group")?;

        Ok(TaskGroup {
            id,
            name: name.to_string(),
            parent_id: parent_id.map(|s| s.to_string()),
            icon: Some(icon_value.to_string()),
            color: Some(color_value.to_string()),
            sort_order,
            created_at: now,
            updated_at: now,
        })
    }

    /// 更新任务组
    pub fn update(
        conn: &Connection,
        id: &str,
        name: Option<&str>,
        parent_id: Option<&str>,
        icon: Option<&str>,
        color: Option<&str>,
    ) -> Result<TaskGroup> {
        // 首先获取现有任务组
        let existing = Self::get(conn, id)?
            .ok_or_else(|| anyhow::anyhow!("Group not found: {}", id))?;

        let new_name = name.unwrap_or(existing.name.as_str());
        let new_icon = icon.or(existing.icon.as_deref());
        let new_color = color.or(existing.color.as_deref());
        let new_parent_id = parent_id.or(existing.parent_id.as_deref());

        let now = Utc::now().timestamp_millis();

        conn.execute(
            "UPDATE task_groups
             SET name = ?1, parent_id = ?2, icon = ?3, color = ?4, updated_at = ?5
             WHERE id = ?6",
            params![new_name, new_parent_id, new_icon, new_color, now, id],
        )
        .context("Failed to update group")?;

        Ok(TaskGroup {
            id: id.to_string(),
            name: new_name.to_string(),
            parent_id: new_parent_id.map(|s| s.to_string()),
            icon: new_icon.map(|s| s.to_string()),
            color: new_color.map(|s| s.to_string()),
            sort_order: existing.sort_order,
            created_at: existing.created_at,
            updated_at: now,
        })
    }

    /// 删除任务组
    /// 注意：由于外键约束 ON DELETE SET NULL，删除任务组后，关联的任务 group_id 会自动设置为 NULL
    /// 这里我们显式执行以确保数据一致性
    pub fn delete(conn: &Connection, id: &str) -> Result<()> {
        // 显式将关联任务的 group_id 设置为 NULL
        conn.execute(
            "UPDATE todos SET group_id = NULL WHERE group_id = ?",
            params![id],
        )
        .context("Failed to update todos group_id")?;

        // 删除任务组
        let rows_affected = conn.execute(
            "DELETE FROM task_groups WHERE id = ?",
            params![id],
        )
        .context("Failed to delete group")?;

        if rows_affected == 0 {
            anyhow::bail!("Group not found");
        }

        Ok(())
    }
}
