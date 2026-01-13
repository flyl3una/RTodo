// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

pub mod schema;
pub mod connection;
pub mod repositories;
pub mod migrations;

use anyhow::Result;
use connection::DbConnection;
use std::sync::Arc;
use tokio::sync::Mutex;

/// 数据库管理器
pub struct Database {
    conn: Arc<Mutex<DbConnection>>,
}

impl Database {
    /// 创建新的数据库实例
    pub fn new() -> Result<Self> {
        let conn = DbConnection::new()?;

        // 初始化数据库表结构
        schema::init_database(conn.inner())?;

        // 运行数据库迁移
        migrations::run_migrations(conn.inner())?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// 获取数据库连接（用于命令中）
    pub async fn get_connection(&self) -> Arc<Mutex<DbConnection>> {
        self.conn.clone()
    }
}

// 实现 Clone
impl Clone for Database {
    fn clone(&self) -> Self {
        Self {
            conn: self.conn.clone(),
        }
    }
}
