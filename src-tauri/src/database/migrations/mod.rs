// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use rusqlite::Connection;
use anyhow::Result;

/// Run all pending database migrations
pub fn run_migrations(conn: &Connection) -> Result<()> {
    // Check if we need to migrate status from TEXT to INTEGER
    migrate_status_to_int(conn)?;

    Ok(())
}

/// Migrate status column from TEXT to INTEGER
/// This checks if the status column is TEXT type and migrates existing data
fn migrate_status_to_int(conn: &Connection) -> Result<()> {
    // Check if the status column is TEXT by trying to query its type
    let sql = "SELECT typeof(status) FROM todos LIMIT 1";
    let status_type: Option<String> = conn
        .query_row(sql, [], |row| row.get(0))
        .ok();

    // If status is TEXT type, we need to migrate
    if let Some(typ) = status_type {
        if typ == "text" || typ == "TEXT" {
            tracing::info!("Detected TEXT status column, starting migration to INTEGER...");

            // Start a transaction for atomic migration
            let tx = conn.unchecked_transaction()?;

            // 1. Add new INTEGER column
            tx.execute(
                "ALTER TABLE todos ADD COLUMN status_new INTEGER NOT NULL DEFAULT 0",
                [],
            )?;

            // 2. Migrate data from TEXT to INTEGER
            tx.execute(
                "UPDATE todos SET status_new = CASE
                    WHEN LOWER(status) = 'todo' THEN 0
                    WHEN LOWER(status) = 'in_progress' THEN 1
                    WHEN LOWER(status) = 'done' THEN 2
                    ELSE 0
                END",
                [],
            )?;

            // 3. Copy data and drop old column
            // SQLite doesn't support DROP COLUMN directly, so we need to recreate the table
            tx.execute(
                "CREATE TABLE todos_new (
                    id TEXT PRIMARY KEY,
                    title TEXT NOT NULL,
                    description TEXT,
                    status INTEGER NOT NULL DEFAULT 0,
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

            // 4. Copy data from old table to new table (using status_new)
            tx.execute(
                "INSERT INTO todos_new (
                    id, title, description, status, priority, is_marked,
                    group_id, assignee, start_date, due_date, completed_at,
                    created_at, updated_at
                )
                SELECT
                    id, title, description, status_new, priority, is_marked,
                    group_id, assignee, start_date, due_date, completed_at,
                    created_at, updated_at
                FROM todos",
                [],
            )?;

            // 5. Drop old table and rename new table
            tx.execute("DROP TABLE todos", [])?;
            tx.execute("ALTER TABLE todos_new RENAME TO todos", [])?;

            // 6. Recreate indexes
            tx.execute(
                "CREATE INDEX IF NOT EXISTS idx_todos_group ON todos(group_id)",
                [],
            )?;
            tx.execute(
                "CREATE INDEX IF NOT EXISTS idx_todos_status ON todos(status)",
                [],
            )?;
            tx.execute(
                "CREATE INDEX IF NOT EXISTS idx_todos_due_date ON todos(due_date)",
                [],
            )?;

            tx.commit()?;
            tracing::info!("Status migration completed successfully");
        }
    }

    Ok(())
}
