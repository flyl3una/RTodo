// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// 执行步骤
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoStep {
    pub id: String,
    pub todo_id: String,
    pub title: String,
    pub is_completed: bool,
    pub sort_order: i32,
    pub created_at: i64,
}

/// 创建步骤请求
#[derive(Debug, Deserialize)]
pub struct CreateStepRequest {
    pub todo_id: String,
    pub title: String,
    #[serde(default)]
    pub sort_order: Option<i32>,
}

/// 更新步骤请求
#[derive(Debug, Deserialize)]
pub struct UpdateStepRequest {
    pub id: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub is_completed: Option<bool>,
    #[serde(default)]
    pub sort_order: Option<i32>,
}
