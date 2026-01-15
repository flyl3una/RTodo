// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use rusqlite::{Connection, params, OptionalExtension};
use anyhow::{Result, Context};
use chrono::Utc;

use crate::models::Attachment;

/// Attachment 仓库
pub struct AttachmentRepository;

impl AttachmentRepository {
    /// 获取任务的所有附件
    pub fn list_by_todo(conn: &Connection, todo_id: i64) -> Result<Vec<Attachment>> {
        let mut stmt = conn.prepare(
            "SELECT id, todo_id, name, file_path, file_size, mime_type, created_at
             FROM attachments
             WHERE todo_id = ?
             ORDER BY created_at DESC"
        )
        .context("Failed to prepare list attachments query")?;

        let attachments = stmt.query_map(params![todo_id], |row| {
            Ok(Attachment {
                id: row.get(0)?,
                todo_id: row.get(1)?,
                name: row.get(2)?,
                file_path: row.get(3)?,
                file_size: row.get(4)?,
                mime_type: row.get(5)?,
                created_at: row.get(6)?,
            })
        })
        .context("Failed to execute list attachments query")?
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to parse attachments")?;

        Ok(attachments)
    }

    /// 根据 ID 获取单个附件
    pub fn get(conn: &Connection, id: i64) -> Result<Option<Attachment>> {
        let attachment = conn.query_row(
            "SELECT id, todo_id, name, file_path, file_size, mime_type, created_at
             FROM attachments WHERE id = ?",
            params![id],
            |row| {
                Ok(Attachment {
                    id: row.get(0)?,
                    todo_id: row.get(1)?,
                    name: row.get(2)?,
                    file_path: row.get(3)?,
                    file_size: row.get(4)?,
                    mime_type: row.get(5)?,
                    created_at: row.get(6)?,
                })
            },
        )
        .optional()
        .context("Failed to execute get attachment query")?;

        Ok(attachment)
    }

    /// 创建附件
    pub fn create(
        conn: &Connection,
        todo_id: i64,
        name: &str,
        file_path: &str,
        file_size: i64,
        mime_type: Option<&str>,
    ) -> Result<Attachment> {
        let now = Utc::now().timestamp_millis();

        conn.execute(
            "INSERT INTO attachments (todo_id, name, file_path, file_size, mime_type, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![todo_id, name, file_path, file_size, mime_type, now],
        )
        .context("Failed to insert attachment")?;

        // 获取新插入的ID
        let id: i64 = conn.last_insert_rowid();

        Ok(Attachment {
            id,
            todo_id,
            name: name.to_string(),
            file_path: file_path.to_string(),
            file_size,
            mime_type: mime_type.map(|s| s.to_string()),
            created_at: now,
        })
    }

    /// 删除附件
    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        let rows_affected = conn.execute(
            "DELETE FROM attachments WHERE id = ?",
            params![id],
        )
        .context("Failed to delete attachment")?;

        if rows_affected == 0 {
            anyhow::bail!("Attachment not found");
        }

        Ok(())
    }
}
