// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

//! 数据管理仓库
//! 处理数据的导入、导出和清理操作

use rusqlite::{Connection, params, Transaction};
use anyhow::{Result, Context};
use crate::models::ExportData;
use crate::database::repositories::{GroupRepository, TagRepository, TodoRepository};
use crate::utils::data_export::{
    parse_task_group_csv,
    parse_tag_csv,
    parse_todo_csv,
    parse_todo_tag_csv,
    parse_step_csv,
    parse_attachment_csv,
};

/// 数据管理仓库
pub struct DataRepository;

impl DataRepository {
    /// 导出所有数据
    pub fn export_all(conn: &Connection) -> Result<ExportData> {
        // 导出任务组
        let task_groups = GroupRepository::list(conn)?;

        // 导出标签
        let tags = TagRepository::list(conn)?;

        // 导出所有任务（不过滤）
        let todos = TodoRepository::list(
            conn,
            None,  // group_id
            None,  // tag_id
            None,  // status
            None,  // search
            None,  // priority
            None,  // start_date
            None,  // end_date
        )?;

        let exported_at = chrono::Utc::now().timestamp_millis();

        Ok(ExportData {
            version: "1.0".to_string(),
            exported_at,
            task_groups,
            tags,
            todos,
        })
    }

    /// 从 CSV 导入任务组
    fn import_groups_from_csv(conn: &Transaction<'_>, groups_csv: &str) -> Result<()> {
        if groups_csv.is_empty() {
            return Ok(());
        }

        let mut rdr = csv::Reader::from_reader(groups_csv.as_bytes());

        for result in rdr.records() {
            let record = result.map_err(|e| anyhow::anyhow!("Failed to read CSV record: {}", e))?;

            let (id, name, parent_id, icon, color, sort_order, created_at, updated_at) =
                parse_task_group_csv(&record)?;

            // 检查是否已存在
            let existing = conn.query_row(
                "SELECT id FROM task_groups WHERE id = ?",
                params![id],
                |row| row.get::<_, i64>(0)
            );

            if existing.is_err() {
                // 不存在，插入
                conn.execute(
                    "INSERT INTO task_groups (id, name, parent_id, icon, color, sort_order, created_at, updated_at)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                    params![id, name, parent_id, icon, color, sort_order, created_at, updated_at],
                ).context(format!("Failed to import task group {}", name))?;
            }
        }

        Ok(())
    }

    /// 从 CSV 导入标签
    fn import_tags_from_csv(conn: &Transaction<'_>, tags_csv: &str) -> Result<()> {
        if tags_csv.is_empty() {
            return Ok(());
        }

        let mut rdr = csv::Reader::from_reader(tags_csv.as_bytes());

        for result in rdr.records() {
            let record = result.map_err(|e| anyhow::anyhow!("Failed to read CSV record: {}", e))?;

            let (id, name, color, created_at) = parse_tag_csv(&record)?;

            // 检查是否已存在
            let existing = conn.query_row(
                "SELECT id FROM tags WHERE id = ?",
                params![id],
                |row| row.get::<_, i64>(0)
            );

            if existing.is_err() {
                // 不存在，插入
                conn.execute(
                    "INSERT INTO tags (id, name, color, created_at) VALUES (?1, ?2, ?3, ?4)",
                    params![id, name, color, created_at.to_string()],
                ).context(format!("Failed to import tag {}", name))?;
            }
        }

        Ok(())
    }

    /// 从 CSV 导入任务
    fn import_todos_from_csv(conn: &Transaction<'_>, todos_csv: &str) -> Result<()> {
        if todos_csv.is_empty() {
            return Ok(());
        }

        let mut rdr = csv::Reader::from_reader(todos_csv.as_bytes());

        for result in rdr.records() {
            let record = result.map_err(|e| anyhow::anyhow!("Failed to read CSV record: {}", e))?;

            let (id, title, description, status, priority, group_id, assignee,
                 start_date, due_date, completed_at, created_at, updated_at) =
                parse_todo_csv(&record)?;

            // 检查是否已存在
            let existing = conn.query_row(
                "SELECT id FROM todos WHERE id = ?",
                params![id],
                |row| row.get::<_, i64>(0)
            );

            if existing.is_ok() {
                // 已存在，更新
                conn.execute(
                    "UPDATE todos SET title = ?1, description = ?2, status = ?3, priority = ?4,
                     group_id = ?5, assignee = ?6, start_date = ?7, due_date = ?8, completed_at = ?9,
                     updated_at = ?10 WHERE id = ?11",
                    params![title, description, status, priority, group_id, assignee,
                            start_date, due_date, completed_at, updated_at, id],
                ).context(format!("Failed to update todo {}", title))?;
            } else {
                // 不存在，插入
                conn.execute(
                    "INSERT INTO todos (id, title, description, status, priority,
                     group_id, assignee, start_date, due_date, completed_at, created_at, updated_at)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                    params![id, title, description, status, priority, group_id, assignee,
                            start_date, due_date, completed_at, created_at, updated_at],
                ).context(format!("Failed to insert todo {}", title))?;
            }
        }

        Ok(())
    }

    /// 从 CSV 导入任务-标签关联
    fn import_todo_tags_from_csv(conn: &Transaction<'_>, todo_tags_csv: &str) -> Result<()> {
        if todo_tags_csv.is_empty() {
            return Ok(());
        }

        let mut rdr = csv::Reader::from_reader(todo_tags_csv.as_bytes());

        // 先删除所有旧的关联
        conn.execute("DELETE FROM todo_tags", [])
            .context("Failed to clear old todo_tags")?;

        for result in rdr.records() {
            let record = result.map_err(|e| anyhow::anyhow!("Failed to read CSV record: {}", e))?;

            let (todo_id, tag_id) = parse_todo_tag_csv(&record)?;

            conn.execute(
                "INSERT INTO todo_tags (todo_id, tag_id) VALUES (?1, ?2)",
                params![todo_id, tag_id],
            ).context(format!("Failed to link tag {} to todo {}", tag_id, todo_id))?;
        }

        Ok(())
    }

    /// 从 CSV 导入步骤
    fn import_steps_from_csv(conn: &Transaction<'_>, steps_csv: &str) -> Result<()> {
        if steps_csv.is_empty() {
            return Ok(());
        }

        let mut rdr = csv::Reader::from_reader(steps_csv.as_bytes());

        // 先删除所有旧的步骤
        conn.execute("DELETE FROM todo_steps", [])
            .context("Failed to clear old steps")?;

        for result in rdr.records() {
            let record = result.map_err(|e| anyhow::anyhow!("Failed to read CSV record: {}", e))?;

            let (id, todo_id, title, is_completed, sort_order, created_at) =
                parse_step_csv(&record)?;

            conn.execute(
                "INSERT INTO todo_steps (id, todo_id, title, is_completed, sort_order, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![id, todo_id, title, is_completed as i32, sort_order, created_at],
            ).context(format!("Failed to import step {} for todo {}", id, todo_id))?;
        }

        Ok(())
    }

    /// 从 CSV 导入附件
    fn import_attachments_from_csv(conn: &Transaction<'_>, attachments_csv: &str) -> Result<()> {
        if attachments_csv.is_empty() {
            return Ok(());
        }

        let mut rdr = csv::Reader::from_reader(attachments_csv.as_bytes());

        // 先删除所有旧的附件
        conn.execute("DELETE FROM attachments", [])
            .context("Failed to clear old attachments")?;

        for result in rdr.records() {
            let record = result.map_err(|e| anyhow::anyhow!("Failed to read CSV record: {}", e))?;

            let (id, todo_id, name, file_path, file_size, mime_type, created_at) =
                parse_attachment_csv(&record)?;

            conn.execute(
                "INSERT INTO attachments (id, todo_id, name, file_path, file_size, mime_type, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![id, todo_id, name, file_path, file_size, mime_type, created_at],
            ).context(format!("Failed to import attachment {} for todo {}", id, todo_id))?;
        }

        Ok(())
    }

    /// 从 CSV 数据导入所有数据（在事务中执行）
    pub fn import_from_csv(conn: &Connection, csv_data: &crate::utils::data_export::ZipCsvData) -> Result<()> {
        // 开始事务
        let transaction = conn.unchecked_transaction()?;

        // 导入任务组
        Self::import_groups_from_csv(&transaction, &csv_data.groups_csv)
            .context("Failed to import task groups")?;

        // 导入标签
        Self::import_tags_from_csv(&transaction, &csv_data.tags_csv)
            .context("Failed to import tags")?;

        // 导入任务
        Self::import_todos_from_csv(&transaction, &csv_data.todos_csv)
            .context("Failed to import todos")?;

        // 导入任务-标签关联
        Self::import_todo_tags_from_csv(&transaction, &csv_data.todo_tags_csv)
            .context("Failed to import todo tags")?;

        // 导入步骤
        Self::import_steps_from_csv(&transaction, &csv_data.steps_csv)
            .context("Failed to import steps")?;

        // 导入附件
        Self::import_attachments_from_csv(&transaction, &csv_data.attachments_csv)
            .context("Failed to import attachments")?;

        // 提交事务
        transaction.commit()?;

        Ok(())
    }

    /// 导入数据（在事务中执行）
    pub fn import_data(conn: &Connection, data: &ExportData) -> Result<()> {
        // 开始事务
        let transaction = conn.unchecked_transaction()?;

        // 导入任务组（跳过已存在的）
        for group in &data.task_groups {
            // 检查是否已存在同名任务组
            let existing = transaction
                .query_row("SELECT id FROM task_groups WHERE name = ?", params![group.name], |row| {
                    row.get::<_, i64>(0)
                });

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
            )?;
        }

        // 导入标签（跳过已存在的）
        for tag in &data.tags {
            // 检查是否已存在同名标签
            let existing = transaction
                .query_row("SELECT id FROM tags WHERE name = ?", params![tag.name], |row| {
                    row.get::<_, i64>(0)
                });

            if existing.is_ok() {
                // 已存在，跳过
                continue;
            }

            // 插入新标签
            transaction.execute(
                "INSERT INTO tags (id, name, color, created_at) VALUES (?1, ?2, ?3, ?4)",
                params![tag.id, tag.name, tag.color, tag.created_at.to_string()],
            )?;
        }

        // 导入任务
        for todo in &data.todos {
            // 检查是否已存在（根据ID）
            let existing = transaction
                .query_row("SELECT id FROM todos WHERE id = ?", params![todo.id], |row| {
                    row.get::<_, i64>(0)
                });

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
                )?;
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
                )?;
            }

            // 导入任务-标签关联
            if let Some(tags) = &todo.tags {
                // 先删除旧的关联
                transaction.execute("DELETE FROM todo_tags WHERE todo_id = ?", params![todo.id])?;

                // 插入新的关联
                for tag in tags {
                    transaction.execute(
                        "INSERT INTO todo_tags (todo_id, tag_id) VALUES (?1, ?2)",
                        params![todo.id, tag.id],
                    )?;
                }
            }
        }

        // 提交事务
        transaction.commit()?;

        Ok(())
    }

    /// 清空所有数据（在事务中执行）
    pub fn clear_all(conn: &Connection) -> Result<()> {
        // 开始事务
        let transaction = conn.unchecked_transaction()?;

        // 删除所有数据（按依赖关系倒序）
        transaction.execute("DELETE FROM todo_tags", [])?;
        transaction.execute("DELETE FROM todo_steps", [])?;
        transaction.execute("DELETE FROM attachments", [])?;
        transaction.execute("DELETE FROM todos", [])?;
        transaction.execute("DELETE FROM tags", [])?;
        transaction.execute("DELETE FROM task_groups", [])?;

        // 提交事务
        transaction.commit()?;

        Ok(())
    }
}
