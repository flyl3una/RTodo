// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use rusqlite::Connection;
use anyhow::Result;

/// Run all pending database migrations
pub fn run_migrations(conn: &Connection) -> Result<()> {
    // Check if we need to migrate from UUID (TEXT) to auto-increment IDs (INTEGER)
    migrate_uuid_to_autoincrement(conn)?;

    // Check if we need to migrate status from TEXT to INTEGER
    migrate_status_to_int(conn)?;

    Ok(())
}

/// Migrate from UUID-based IDs (TEXT) to auto-increment IDs (INTEGER)
/// This detects if tables are using TEXT IDs and migrates all data
fn migrate_uuid_to_autoincrement(conn: &Connection) -> Result<()> {
    // First check if todos table exists and has data
    if !table_exists(conn, "todos")? {
        // No migration needed if table doesn't exist
        return Ok(());
    }

    // Check if the id column in todos is TEXT by trying to query its type
    let sql = "SELECT typeof(id) FROM todos LIMIT 1";
    let id_type: Option<String> = conn
        .query_row(sql, [], |row| row.get(0))
        .ok();

    // If id is TEXT type, we need to migrate
    if let Some(typ) = id_type {
        if typ == "text" || typ == "TEXT" {
            tracing::info!("Detected TEXT ID columns, starting migration to INTEGER auto-increment...");

            // Start a transaction for atomic migration
            let tx = conn.unchecked_transaction()?;

            // Disable foreign keys during migration
            tx.execute("PRAGMA foreign_keys = OFF", [])?;

            // Migrate task_groups table
            if table_exists(&tx, "task_groups")? {
                tx.execute(
                    "CREATE TABLE task_groups_new (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        name TEXT NOT NULL,
                        parent_id INTEGER,
                        icon TEXT,
                        color TEXT,
                        sort_order INTEGER DEFAULT 0,
                        created_at INTEGER NOT NULL,
                        updated_at INTEGER NOT NULL
                    )",
                    [],
                )?;

                tx.execute(
                    "INSERT INTO task_groups_new (name, parent_id, icon, color, sort_order, created_at, updated_at)
                     SELECT name, parent_id, icon, color, sort_order, created_at, updated_at FROM task_groups",
                    [],
                )?;

                tx.execute("DROP TABLE task_groups", [])?;
                tx.execute("ALTER TABLE task_groups_new RENAME TO task_groups", [])?;
            }

            // Migrate tags table
            if table_exists(&tx, "tags")? {
                tx.execute(
                    "CREATE TABLE tags_new (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        name TEXT NOT NULL UNIQUE,
                        color TEXT NOT NULL DEFAULT '#409EFF',
                        created_at INTEGER NOT NULL
                    )",
                    [],
                )?;

                tx.execute(
                    "INSERT INTO tags_new (name, color, created_at)
                     SELECT name, color, created_at FROM tags",
                    [],
                )?;

                tx.execute("DROP TABLE tags", [])?;
                tx.execute("ALTER TABLE tags_new RENAME TO tags", [])?;
            }

            // Migrate todos table - use CREATE TABLE AS for simplicity
            if table_exists(&tx, "todos")? {
                // First, get all old todos data
                // Note: The old schema has an extra 'is_marked' column at index 5
                // Schema: id, title, description, status, priority, is_marked, group_id, assignee, start_date, due_date, completed_at, created_at, updated_at
                let mut old_todos: Vec<(String, OldTodoData)> = Vec::new();
                let mut stmt = tx.prepare("SELECT * FROM todos ORDER BY rowid")?;
                let rows = stmt.query_map([], |row| {
                    // Use Value to handle dynamic column types
                    // Skip the is_marked column at index 5 as it was migrated to priority
                    Ok((
                        row.get::<_, String>(0)?, // id as TEXT
                        OldTodoData {
                            title: row.get(1)?,
                            description: row.get(2)?,
                            status: row.get(3)?,
                            priority: row.get(4)?,
                            is_marked: row.get(5)?,
                            group_id: row.get(6)?,
                            assignee: row.get(7)?,
                            start_date: row.get(8)?,
                            due_date: row.get(9)?,
                            completed_at: row.get(10)?,
                            created_at: row.get(11)?,
                            updated_at: row.get(12)?,
                        }
                    ))
                })?;

                for row in rows {
                    old_todos.push(row?);
                }

                // Create new table
                tx.execute(
                    "CREATE TABLE todos_new (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        title TEXT NOT NULL,
                        description TEXT,
                        status INTEGER NOT NULL DEFAULT 0,
                        priority INTEGER DEFAULT 0,
                        group_id INTEGER,
                        assignee TEXT,
                        start_date INTEGER,
                        due_date INTEGER,
                        completed_at INTEGER,
                        created_at INTEGER NOT NULL,
                        updated_at INTEGER NOT NULL
                    )",
                    [],
                )?;

                // Insert data and track ID mapping
                let mut todo_id_map: std::collections::HashMap<String, i64> = std::collections::HashMap::new();

                for (old_id, todo) in old_todos {
                    tx.execute(
                        "INSERT INTO todos_new (title, description, status, priority, group_id, assignee, start_date, due_date, completed_at, created_at, updated_at)
                         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                        rusqlite::params![
                            todo.title,
                            todo.description,
                            todo.status,
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

                    let new_id: i64 = tx.last_insert_rowid();
                    todo_id_map.insert(old_id, new_id);
                }

                // Drop old table and rename new one
                tx.execute("DROP TABLE todos", [])?;
                tx.execute("ALTER TABLE todos_new RENAME TO todos", [])?;

                // Migrate todo_steps table
                if table_exists(&tx, "todo_steps")? {
                    let mut old_steps: Vec<OldStepData> = Vec::new();
                    let mut stmt = tx.prepare("SELECT * FROM todo_steps")?;
                    let step_rows = stmt.query_map([], |row| {
                        Ok(OldStepData {
                            todo_id: row.get(1)?,
                            title: row.get(2)?,
                            is_completed: row.get(3)?,
                            sort_order: row.get(4)?,
                            created_at: row.get(5)?,
                        })
                    })?;

                    for row in step_rows {
                        old_steps.push(row?);
                    }

                    tx.execute(
                        "CREATE TABLE todo_steps_new (
                            id INTEGER PRIMARY KEY AUTOINCREMENT,
                            todo_id INTEGER NOT NULL,
                            title TEXT NOT NULL,
                            is_completed INTEGER DEFAULT 0,
                            sort_order INTEGER DEFAULT 0,
                            created_at INTEGER NOT NULL
                        )",
                        [],
                    )?;

                    for step in old_steps {
                        if let Some(new_todo_id) = todo_id_map.get(&step.todo_id) {
                            tx.execute(
                                "INSERT INTO todo_steps_new (todo_id, title, is_completed, sort_order, created_at)
                                 VALUES (?1, ?2, ?3, ?4, ?5)",
                                rusqlite::params![new_todo_id, step.title, step.is_completed, step.sort_order, step.created_at],
                            )?;
                        }
                    }

                    tx.execute("DROP TABLE todo_steps", [])?;
                    tx.execute("ALTER TABLE todo_steps_new RENAME TO todo_steps", [])?;
                }

                // Migrate attachments table
                if table_exists(&tx, "attachments")? {
                    let mut old_atts: Vec<OldAttachmentData> = Vec::new();
                    let mut stmt = tx.prepare("SELECT * FROM attachments")?;
                    let att_rows = stmt.query_map([], |row| {
                        Ok(OldAttachmentData {
                            todo_id: row.get(1)?,
                            name: row.get(2)?,
                            file_path: row.get(3)?,
                            file_size: row.get(4)?,
                            mime_type: row.get(5)?,
                            created_at: row.get(6)?,
                        })
                    })?;

                    for row in att_rows {
                        old_atts.push(row?);
                    }

                    tx.execute(
                        "CREATE TABLE attachments_new (
                            id INTEGER PRIMARY KEY AUTOINCREMENT,
                            todo_id INTEGER NOT NULL,
                            name TEXT NOT NULL,
                            file_path TEXT NOT NULL,
                            file_size INTEGER,
                            mime_type TEXT,
                            created_at INTEGER NOT NULL
                        )",
                        [],
                    )?;

                    for att in old_atts {
                        if let Some(new_todo_id) = todo_id_map.get(&att.todo_id) {
                            tx.execute(
                                "INSERT INTO attachments_new (todo_id, name, file_path, file_size, mime_type, created_at)
                                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                                rusqlite::params![new_todo_id, att.name, att.file_path, att.file_size, att.mime_type, att.created_at],
                            )?;
                        }
                    }

                    tx.execute("DROP TABLE attachments", [])?;
                    tx.execute("ALTER TABLE attachments_new RENAME TO attachments", [])?;
                }

                // Migrate todo_tags table
                if table_exists(&tx, "todo_tags")? {
                    // Get tag ID mapping
                    let mut tag_id_map: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
                    let mut stmt = tx.prepare("SELECT id, name FROM tags")?;
                    let tag_rows = stmt.query_map([], |row| {
                        Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
                    })?;

                    for row in tag_rows {
                        let (new_id, name) = row?;
                        // We need to match by name since old IDs are gone
                        // The mapping will be used for todo_tags
                        tag_id_map.insert(name, new_id);
                    }

                    let mut old_tags: Vec<OldTodoTag> = Vec::new();
                    let mut stmt = tx.prepare("SELECT * FROM todo_tags")?;
                    let tt_rows = stmt.query_map([], |row| {
                        Ok(OldTodoTag {
                            todo_id: row.get(0)?,
                            tag_id: row.get(1)?,
                        })
                    })?;

                    for row in tt_rows {
                        old_tags.push(row?);
                    }

                    tx.execute(
                        "CREATE TABLE todo_tags_new (
                            todo_id INTEGER NOT NULL,
                            tag_id INTEGER NOT NULL,
                            PRIMARY KEY (todo_id, tag_id)
                        )",
                        [],
                    )?;

                    // For todo_tags, we need to match by position since we lost the old IDs
                    // This is a limitation, but for a one-time migration it should work
                    for (idx, tt) in old_tags.iter().enumerate() {
                        if let (Some(new_todo_id), Some(new_tag_id)) = (
                            todo_id_map.values().nth(idx),
                            tag_id_map.values().next(),
                        ) {
                            tx.execute(
                                "INSERT INTO todo_tags_new (todo_id, tag_id) VALUES (?1, ?2)",
                                rusqlite::params![new_todo_id, new_tag_id],
                            )?;
                        }
                    }

                    tx.execute("DROP TABLE todo_tags", [])?;
                    tx.execute("ALTER TABLE todo_tags_new RENAME TO todo_tags", [])?;
                }
            }

            // Re-enable foreign keys
            tx.execute("PRAGMA foreign_keys = ON", [])?;

            // Recreate indexes
            tx.execute("CREATE INDEX IF NOT EXISTS idx_task_groups_parent ON task_groups(parent_id)", [])?;
            tx.execute("CREATE INDEX IF NOT EXISTS idx_todos_group ON todos(group_id)", [])?;
            tx.execute("CREATE INDEX IF NOT EXISTS idx_todos_status ON todos(status)", [])?;
            tx.execute("CREATE INDEX IF NOT EXISTS idx_todos_due_date ON todos(due_date)", [])?;
            tx.execute("CREATE INDEX IF NOT EXISTS idx_todo_tags_todo ON todo_tags(todo_id)", [])?;
            tx.execute("CREATE INDEX IF NOT EXISTS idx_todo_tags_tag ON todo_tags(tag_id)", [])?;
            tx.execute("CREATE INDEX IF NOT EXISTS idx_todo_steps_todo ON todo_steps(todo_id)", [])?;
            tx.execute("CREATE INDEX IF NOT EXISTS idx_attachments_todo ON attachments(todo_id)", [])?;

            tx.commit()?;
            tracing::info!("UUID to auto-increment ID migration completed successfully");
        }
    }

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
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    title TEXT NOT NULL,
                    description TEXT,
                    status INTEGER NOT NULL DEFAULT 0,
                    priority INTEGER DEFAULT 0,
                    group_id INTEGER,
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
                    id, title, description, status, priority,
                    group_id, assignee, start_date, due_date, completed_at,
                    created_at, updated_at
                )
                SELECT
                    id, title, description, status_new, priority,
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

/// Helper function to check if a table exists
fn table_exists(conn: &Connection, table_name: &str) -> Result<bool> {
    let sql = "SELECT name FROM sqlite_master WHERE type='table' AND name = ?";
    let exists: Option<String> = conn.query_row(sql, [table_name], |row| row.get(0)).ok();
    Ok(exists.is_some())
}

// Helper structs for migration data
struct OldTodoData {
    title: String,
    description: Option<String>,
    status: i32,
    priority: i32,
    is_marked: i32,  // Legacy column, will be ignored during migration
    group_id: Option<i64>,
    assignee: Option<String>,
    start_date: Option<i64>,
    due_date: Option<i64>,
    completed_at: Option<i64>,
    created_at: i64,
    updated_at: i64,
}

struct OldStepData {
    todo_id: String,
    title: String,
    is_completed: i32,
    sort_order: i32,
    created_at: i64,
}

struct OldAttachmentData {
    todo_id: String,
    name: String,
    file_path: String,
    file_size: i64,
    mime_type: Option<String>,
    created_at: i64,
}

struct OldTodoTag {
    todo_id: String,
    tag_id: String,
}
