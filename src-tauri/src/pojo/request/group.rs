// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use serde::Deserialize;

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
