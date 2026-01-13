// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use rusqlite::{Connection, params, OptionalExtension};
use anyhow::{Result, Context};
use chrono::Utc;

use crate::models::Tag;

/// Tag 仓库
pub struct TagRepository;

impl TagRepository {
    /// 获取所有标签
    pub fn list(conn: &Connection) -> Result<Vec<Tag>> {
        let mut stmt = conn.prepare(
            "SELECT id, name, color, created_at
             FROM tags
             ORDER BY name ASC"
        )
        .context("Failed to prepare list tags query")?;

        let tags = stmt.query_map([], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
                created_at: row.get(3)?,
            })
        })
        .context("Failed to execute list tags query")?
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to parse tags")?;

        Ok(tags)
    }

    /// 根据 ID 获取单个标签
    pub fn get(conn: &Connection, id: &str) -> Result<Option<Tag>> {
        let tag = conn.query_row(
            "SELECT id, name, color, created_at FROM tags WHERE id = ?",
            params![id],
            |row| {
                Ok(Tag {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    color: row.get(2)?,
                    created_at: row.get(3)?,
                })
            },
        )
        .optional()
        .context("Failed to execute get tag query")?;

        Ok(tag)
    }

    /// 创建标签
    pub fn create(
        conn: &Connection,
        name: &str,
        color: &str,
    ) -> Result<Tag> {
        let now = Utc::now().timestamp_millis();
        let id = uuid::Uuid::new_v4().to_string();

        conn.execute(
            "INSERT INTO tags (id, name, color, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![&id, name, color, now],
        )
        .context("Failed to insert tag")?;

        Ok(Tag {
            id,
            name: name.to_string(),
            color: color.to_string(),
            created_at: now,
        })
    }

    /// 更新标签
    pub fn update(
        conn: &Connection,
        id: &str,
        name: Option<&str>,
        color: Option<&str>,
    ) -> Result<Tag> {
        // 首先获取现有标签
        let existing = Self::get(conn, id)?
            .ok_or_else(|| anyhow::anyhow!("Tag not found: {}", id))?;

        let new_name = name.unwrap_or(existing.name.as_str());
        let new_color = color.unwrap_or(existing.color.as_str());

        conn.execute(
            "UPDATE tags SET name = ?1, color = ?2 WHERE id = ?3",
            params![new_name, new_color, id],
        )
        .context("Failed to update tag")?;

        Ok(Tag {
            id: id.to_string(),
            name: new_name.to_string(),
            color: new_color.to_string(),
            created_at: existing.created_at,
        })
    }

    /// 删除标签
    /// 注意：由于外键约束 ON DELETE CASCADE，删除标签后，todo_tags 关联会自动删除
    /// 这里我们显式执行以确保数据一致性
    pub fn delete(conn: &Connection, id: &str) -> Result<()> {
        // 显式删除标签与任务的关联
        conn.execute(
            "DELETE FROM todo_tags WHERE tag_id = ?",
            params![id],
        )
        .context("Failed to delete todo_tags associations")?;

        // 删除标签
        let rows_affected = conn.execute(
            "DELETE FROM tags WHERE id = ?",
            params![id],
        )
        .context("Failed to delete tag")?;

        if rows_affected == 0 {
            anyhow::bail!("Tag not found");
        }

        Ok(())
    }
}
