// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use rusqlite::{Connection, params, OptionalExtension};
use anyhow::{Result, Context};
use chrono::Utc;

use crate::models::{Todo, TodoStatus, Tag, TodoStep, Attachment, TaskGroup};

/// Todo 仓库
pub struct TodoRepository;

impl TodoRepository {
    /// 获取任务列表（支持筛选）
    pub fn list(
        conn: &Connection,
        group_id: Option<&str>,
        tag_id: Option<&str>,
        status: Option<i32>,
        search: Option<&str>,
        is_marked: Option<bool>,
        priority: Option<i32>,
        start_date: Option<i64>,
        end_date: Option<i64>,
    ) -> Result<Vec<Todo>> {
        let mut query = String::from(
            "SELECT t.* FROM todos t
             WHERE 1=1"
        );
        let mut where_clauses = Vec::new();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        // 按任务组筛选
        if let Some(gid) = group_id {
            where_clauses.push("t.group_id = ?");
            params.push(Box::new(gid.to_string()));
        }

        // 按状态筛选
        if let Some(s) = status {
            where_clauses.push("t.status = ?");
            params.push(Box::new(s));
        }

        // 按标签筛选（需要 JOIN）
        if let Some(tid) = tag_id {
            where_clauses.push(
                "EXISTS (
                    SELECT 1 FROM todo_tags tt
                    WHERE tt.todo_id = t.id AND tt.tag_id = ?
                )"
            );
            params.push(Box::new(tid.to_string()));
        }

        // 搜索筛选（标题或描述）
        if let Some(keyword) = search {
            if !keyword.is_empty() {
                where_clauses.push("(t.title LIKE ? OR t.description LIKE ?)");
                let pattern = format!("%{}%", keyword);
                params.push(Box::new(pattern.clone()));
                params.push(Box::new(pattern));
            }
        }

        // 按重要标记筛选
        if let Some(marked) = is_marked {
            where_clauses.push("t.is_marked = ?");
            params.push(Box::new(if marked { 1 } else { 0 }));
        }

        // 按优先级筛选
        if let Some(p) = priority {
            where_clauses.push("t.priority = ?");
            params.push(Box::new(p));
        }

        // 按时间范围筛选（due_date）
        if let Some(start) = start_date {
            where_clauses.push("t.due_date >= ?");
            params.push(Box::new(start));
        }
        if let Some(end) = end_date {
            where_clauses.push("t.due_date < ?");
            params.push(Box::new(end));
        }

        // 添加 WHERE 子句
        if !where_clauses.is_empty() {
            query.push_str(" AND ");
            query.push_str(&where_clauses.join(" AND "));
        }

        // 排序：未完成在前，然后按创建时间倒序
        query.push_str(" ORDER BY
            CASE WHEN t.status = 2 THEN 1 ELSE 0 END,
            t.is_marked DESC,
            t.due_date ASC,
            t.created_at DESC");

        let mut stmt = conn.prepare(&query)
            .context("Failed to prepare get_todos query")?;

        // 将 Box<dyn ToSql> 转换为 &dyn ToSql
        let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        let todo_iter = stmt.query_map(param_refs.as_slice(), |row| {
            let status_int: i32 = row.get("status")?;
            let status = match status_int {
                0 => TodoStatus::Todo,
                1 => TodoStatus::InProgress,
                2 => TodoStatus::Done,
                _ => TodoStatus::Todo, // 默认为待办
            };
            Ok(Todo {
                id: row.get("id")?,
                title: row.get("title")?,
                description: row.get("description")?,
                status,
                priority: row.get("priority")?,
                is_marked: row.get::<_, i32>("is_marked")? == 1,
                group_id: row.get("group_id")?,
                assignee: row.get("assignee")?,
                start_date: row.get("start_date")?,
                due_date: row.get("due_date")?,
                completed_at: row.get("completed_at")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
                tags: None,
                steps: None,
                attachments: None,
                group: None,
            })
        })
        .context("Failed to execute get_todos query")?;

        let mut todos = Vec::new();
        for todo in todo_iter {
            todos.push(todo.context("Failed to parse todo row")?);
        }

        // 为每个 todo 加载关联数据
        for todo in &mut todos {
            Self::load_relations(conn, todo)?;
        }

        Ok(todos)
    }

    /// 根据 ID 获取单个任务
    pub fn get(conn: &Connection, id: &str) -> Result<Option<Todo>> {
        let mut stmt = conn.prepare(
            "SELECT * FROM todos WHERE id = ?"
        )
        .context("Failed to prepare get_todo query")?;

        let todo_opt = stmt.query_row(params![id], |row| {
            let status_int: i32 = row.get("status")?;
            let status = match status_int {
                0 => TodoStatus::Todo,
                1 => TodoStatus::InProgress,
                2 => TodoStatus::Done,
                _ => TodoStatus::Todo, // 默认为待办
            };
            Ok(Todo {
                id: row.get("id")?,
                title: row.get("title")?,
                description: row.get("description")?,
                status,
                priority: row.get("priority")?,
                is_marked: row.get::<_, i32>("is_marked")? == 1,
                group_id: row.get("group_id")?,
                assignee: row.get("assignee")?,
                start_date: row.get("start_date")?,
                due_date: row.get("due_date")?,
                completed_at: row.get("completed_at")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
                tags: None,
                steps: None,
                attachments: None,
                group: None,
            })
        })
        .optional()
        .context("Failed to execute get_todo query")?;

        if let Some(mut todo) = todo_opt {
            Self::load_relations(conn, &mut todo)?;
            Ok(Some(todo))
        } else {
            Ok(None)
        }
    }

    /// 创建新任务
    pub fn create(
        conn: &Connection,
        title: &str,
        description: Option<&str>,
        group_id: Option<&str>,
        start_date: Option<i64>,
        due_date: Option<i64>,
        priority: i32,
        tag_ids: Option<Vec<String>>,
    ) -> Result<Todo> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now().timestamp_millis();
        let status = TodoStatus::Todo;

        conn.execute(
            "INSERT INTO todos (
                id, title, description, status, priority,
                is_marked, group_id, start_date, due_date,
                created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                id,
                title,
                description,
                status as i32,
                priority,
                0, // is_marked
                group_id,
                start_date,
                due_date,
                now,
                now,
            ],
        )
        .context("Failed to insert todo")?;

        // 插入标签关联
        if let Some(tags) = tag_ids {
            for tag_id in tags {
                conn.execute(
                    "INSERT INTO todo_tags (todo_id, tag_id) VALUES (?1, ?2)",
                    params![&id, &tag_id],
                )
                .context("Failed to insert todo_tags")?;
            }
        }

        // 返回完整的 todo 对象
        Self::get(conn, &id)?.context("Created todo not found")
    }

    /// 更新任务
    pub fn update(
        conn: &Connection,
        id: &str,
        title: Option<&str>,
        description: Option<Option<String>>,
        status: Option<i32>,
        priority: Option<i32>,
        is_marked: Option<bool>,
        group_id: Option<Option<String>>,
        assignee: Option<Option<String>>,
        start_date: Option<Option<i64>>,
        due_date: Option<Option<i64>>,
        tag_ids: Option<Vec<String>>,
    ) -> Result<Todo> {
        let now = Utc::now().timestamp_millis();

        // 构建 SET 子句
        let mut sets = Vec::new();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(t) = title {
            sets.push("title = ?");
            params.push(Box::new(t.to_string()));
        }
        if let Some(d) = description {
            sets.push("description = ?");
            match d {
                Some(val) => params.push(Box::new(val)),
                None => params.push(Box::new(None::<String>)),
            }
        }
        if let Some(s) = status {
            sets.push("status = ?");
            params.push(Box::new(s));

            // 如果状态变为完成 (2)，设置完成时间
            if s == 2 {
                sets.push("completed_at = ?");
                params.push(Box::new(now));
            } else {
                sets.push("completed_at = ?");
                params.push(Box::new(None::<i64>));
            }
        }
        if let Some(p) = priority {
            sets.push("priority = ?");
            params.push(Box::new(p));
        }
        if let Some(m) = is_marked {
            sets.push("is_marked = ?");
            params.push(Box::new(if m { 1 } else { 0 }));
        }
        if let Some(g) = group_id {
            sets.push("group_id = ?");
            match g {
                Some(val) => params.push(Box::new(val)),
                None => params.push(Box::new(None::<String>)),
            }
        }
        if let Some(a) = assignee {
            sets.push("assignee = ?");
            match a {
                Some(val) => params.push(Box::new(val)),
                None => params.push(Box::new(None::<String>)),
            }
        }
        if let Some(sd) = start_date {
            sets.push("start_date = ?");
            match sd {
                Some(val) => params.push(Box::new(val)),
                None => params.push(Box::new(None::<i64>)),
            }
        }
        if let Some(dd) = due_date {
            sets.push("due_date = ?");
            match dd {
                Some(val) => params.push(Box::new(val)),
                None => params.push(Box::new(None::<i64>)),
            }
        }

        sets.push("updated_at = ?");
        params.push(Box::new(now));

        if sets.is_empty() && tag_ids.is_none() {
            // 没有要更新的字段，直接返回现有数据
            return Self::get(conn, id)?.context("Todo not found");
        }

        // 执行更新（如果有没有字段需要更新）
        if !sets.is_empty() {
            let query = format!("UPDATE todos SET {} WHERE id = ?", sets.join(", "));

            // 添加 id 参数（需要借用）
            let mut param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
            param_refs.push(&id);

            conn.execute(&query, param_refs.as_slice())
                .context("Failed to update todo")?;
        }

        // 更新标签关联（如果提供）
        if let Some(tags) = tag_ids {
            // 先删除旧关联
            conn.execute(
                "DELETE FROM todo_tags WHERE todo_id = ?",
                params![id],
            )
            .context("Failed to delete old todo_tags")?;

            // 插入新关联
            for tag_id in tags {
                conn.execute(
                    "INSERT INTO todo_tags (todo_id, tag_id) VALUES (?1, ?2)",
                    params![id, &tag_id],
                )
                .context("Failed to insert todo_tags")?;
            }
        }

        Self::get(conn, id)?.context("Updated todo not found")
    }

    /// 删除任务
    pub fn delete(conn: &Connection, id: &str) -> Result<()> {
        let rows_affected = conn.execute(
            "DELETE FROM todos WHERE id = ?",
            params![id],
        )
        .context("Failed to delete todo")?;

        if rows_affected == 0 {
            anyhow::bail!("Todo not found");
        }

        Ok(())
    }

    /// 更新任务状态
    pub fn update_status(conn: &Connection, id: &str, status: i32) -> Result<Todo> {
        let now = Utc::now().timestamp_millis();
        let completed_at = if status == 2 { Some(now) } else { None };

        conn.execute(
            "UPDATE todos SET status = ?1, completed_at = ?2, updated_at = ?3 WHERE id = ?4",
            params![status, completed_at, now, id],
        )
        .context("Failed to update todo status")?;

        Self::get(conn, id)?.context("Updated todo not found")
    }

    /// 切换任务重要标记
    pub fn toggle_mark(conn: &Connection, id: &str) -> Result<Todo> {
        // 先获取当前状态
        let current: i32 = conn.query_row(
            "SELECT is_marked FROM todos WHERE id = ?",
            params![id],
            |row| row.get(0),
        )
        .context("Failed to get current mark status")?;

        let new_mark = if current == 0 { 1 } else { 0 };
        let now = Utc::now().timestamp_millis();

        conn.execute(
            "UPDATE todos SET is_marked = ?1, updated_at = ?2 WHERE id = ?3",
            params![new_mark, now, id],
        )
        .context("Failed to toggle todo mark")?;

        Self::get(conn, id)?.context("Updated todo not found")
    }

    // ========== 辅助方法 ==========

    /// 加载关联数据（标签、步骤、附件、任务组）
    fn load_relations(conn: &Connection, todo: &mut Todo) -> Result<()> {
        // 加载标签
        let mut tag_stmt = conn.prepare(
            "SELECT t.id, t.name, t.color, t.created_at
             FROM tags t
             JOIN todo_tags tt ON t.id = tt.tag_id
             WHERE tt.todo_id = ?"
        )
        .context("Failed to prepare tags query")?;

        let tags = tag_stmt.query_map(params![&todo.id], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
                created_at: row.get(3)?,
            })
        })
        .context("Failed to execute tags query")?
        .collect::<std::result::Result<Vec<_>, _>>()
        .context("Failed to parse tags")?;

        if !tags.is_empty() {
            todo.tags = Some(tags);
        }

        // 加载步骤
        let mut step_stmt = conn.prepare(
            "SELECT id, todo_id, title, is_completed, sort_order, created_at
             FROM todo_steps
             WHERE todo_id = ?
             ORDER BY sort_order ASC"
        )
        .context("Failed to prepare steps query")?;

        let steps = step_stmt.query_map(params![&todo.id], |row| {
            Ok(TodoStep {
                id: row.get(0)?,
                todo_id: row.get(1)?,
                title: row.get(2)?,
                is_completed: row.get::<_, i32>(3)? == 1,
                sort_order: row.get(4)?,
                created_at: row.get(5)?,
            })
        })
        .context("Failed to execute steps query")?
        .collect::<std::result::Result<Vec<_>, _>>()
        .context("Failed to parse steps")?;

        if !steps.is_empty() {
            todo.steps = Some(steps);
        }

        // 加载附件
        let mut attachment_stmt = conn.prepare(
            "SELECT id, todo_id, name, file_path, file_size, mime_type, created_at
             FROM attachments
             WHERE todo_id = ?"
        )
        .context("Failed to prepare attachments query")?;

        let attachments = attachment_stmt.query_map(params![&todo.id], |row| {
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
        .context("Failed to execute attachments query")?
        .collect::<std::result::Result<Vec<_>, _>>()
        .context("Failed to parse attachments")?;

        if !attachments.is_empty() {
            todo.attachments = Some(attachments);
        }

        // 加载任务组
        if let Some(gid) = &todo.group_id {
            let group_opt = conn.query_row(
                "SELECT id, name, parent_id, icon, color, sort_order, created_at, updated_at
                 FROM task_groups
                 WHERE id = ?",
                params![gid],
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
                }
            )
            .optional()
            .context("Failed to execute group query")?;

            if let Some(group) = group_opt {
                todo.group = Some(group);
            }
        }

        Ok(())
    }
}
