// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// 任务组
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskGroup {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub sort_order: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 创建任务组请求
#[derive(Debug, Deserialize)]
pub struct CreateGroupRequest {
    pub name: String,
    pub parent_id: Option<i64>,
    pub icon: Option<String>,
    pub color: Option<String>,
}

/// 更新任务组请求
#[derive(Debug, Deserialize)]
pub struct UpdateGroupRequest {
    pub id: i64,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub parent_id: Option<i64>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
}
