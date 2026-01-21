// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use serde::Deserialize;

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
