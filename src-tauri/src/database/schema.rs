// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use rusqlite::Connection;
use anyhow::Result;

/// 初始化数据库表结构
pub fn init_database(conn: &Connection) -> Result<()> {
    // 任务组表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS task_groups (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            parent_id TEXT,
            icon TEXT,
            color TEXT,
            sort_order INTEGER DEFAULT 0,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (parent_id) REFERENCES task_groups(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // 创建索引
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_task_groups_parent ON task_groups(parent_id)",
        [],
    )?;

    // 标签表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            color TEXT NOT NULL DEFAULT '#409EFF',
            created_at INTEGER NOT NULL
        )",
        [],
    )?;

    // 任务表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'todo',
            priority INTEGER DEFAULT 0,
            is_marked INTEGER DEFAULT 0,
            group_id TEXT,
            assignee TEXT,
            start_date INTEGER,
            due_date INTEGER,
            completed_at INTEGER,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (group_id) REFERENCES task_groups(id) ON DELETE SET NULL
        )",
        [],
    )?;

    // 创建索引
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_todos_group ON todos(group_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_todos_status ON todos(status)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_todos_due_date ON todos(due_date)",
        [],
    )?;

    // 任务-标签关联表（多对多）
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo_tags (
            todo_id TEXT NOT NULL,
            tag_id TEXT NOT NULL,
            PRIMARY KEY (todo_id, tag_id),
            FOREIGN KEY (todo_id) REFERENCES todos(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // 创建索引
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_todo_tags_todo ON todo_tags(todo_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_todo_tags_tag ON todo_tags(tag_id)",
        [],
    )?;

    // 执行步骤表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo_steps (
            id TEXT PRIMARY KEY,
            todo_id TEXT NOT NULL,
            title TEXT NOT NULL,
            is_completed INTEGER DEFAULT 0,
            sort_order INTEGER DEFAULT 0,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (todo_id) REFERENCES todos(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // 创建索引
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_todo_steps_todo ON todo_steps(todo_id)",
        [],
    )?;

    // 附件表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS attachments (
            id TEXT PRIMARY KEY,
            todo_id TEXT NOT NULL,
            name TEXT NOT NULL,
            file_path TEXT NOT NULL,
            file_size INTEGER,
            mime_type TEXT,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (todo_id) REFERENCES todos(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // 创建索引
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_attachments_todo ON attachments(todo_id)",
        [],
    )?;

    // 导出历史表（用于导入导出功能）
    conn.execute(
        "CREATE TABLE IF NOT EXISTS export_history (
            id TEXT PRIMARY KEY,
            version TEXT NOT NULL,
            exported_at INTEGER NOT NULL,
            file_path TEXT NOT NULL
        )",
        [],
    )?;

    tracing::info!("Database initialized successfully");
    Ok(())
}
