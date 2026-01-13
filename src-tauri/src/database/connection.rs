// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use rusqlite::Connection;
use anyhow::{Result, Context};
use std::path::PathBuf;

/// 数据库连接管理
pub struct DbConnection {
    conn: Connection,
}

impl DbConnection {
    /// 创建新的数据库连接
    pub fn new() -> Result<Self> {
        // 获取应用数据目录
        let data_dir = Self::get_data_dir()?;

        // 确保目录存在
        std::fs::create_dir_all(&data_dir)
            .context("Failed to create data directory")?;

        // 数据库文件路径
        let db_path = data_dir.join("rtodo.db");

        tracing::info!("Database path: {:?}", db_path);

        // 打开数据库连接
        let conn = Connection::open(&db_path)
            .context("Failed to open database")?;

        // 设置性能优化参数
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        conn.pragma_update(None, "cache_size", -64_000)?; // 64MB cache
        conn.pragma_update(None, "foreign_keys", "ON")?;
        conn.pragma_update(None, "temp_store", "MEMORY")?;

        Ok(Self { conn })
    }

    /// 获取应用数据目录
    fn get_data_dir() -> Result<PathBuf> {
        let data_dir = if cfg!(target_os = "windows") {
            // Windows: %APPDATA%\rtodo
            dirs::config_dir()
                .map(|p| p.join("rtodo"))
                .ok_or_else(|| anyhow::anyhow!("Failed to get config directory"))?
        } else if cfg!(target_os = "macos") {
            // macOS: ~/Library/Application Support/rtodo
            dirs::config_dir()
                .map(|p| p.join("rtodo"))
                .ok_or_else(|| anyhow::anyhow!("Failed to get config directory"))?
        } else {
            // Linux: ~/.local/share/rtodo
            dirs::data_local_dir()
                .map(|p| p.join("rtodo"))
                .ok_or_else(|| anyhow::anyhow!("Failed to get data directory"))?
        };

        Ok(data_dir)
    }

    /// 获取内部连接（用于直接 SQL 操作）
    pub fn inner(&self) -> &Connection {
        &self.conn
    }
}
