// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// 执行步骤
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoStep {
    pub id: i64,
    pub todo_id: i64,
    pub title: String,
    pub is_completed: bool,
    pub sort_order: i32,
    pub created_at: i64,
}

