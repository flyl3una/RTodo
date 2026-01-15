// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// 标签
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub created_at: i64,
}

/// 创建标签请求
#[derive(Debug, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub color: String,
}

/// 更新标签请求
#[derive(Debug, Deserialize)]
pub struct UpdateTagRequest {
    pub id: i64,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
}
