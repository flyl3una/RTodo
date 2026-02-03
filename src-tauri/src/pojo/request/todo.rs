// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use serde::Deserialize;
use crate::models::TodoStatus;

/// 创建任务请求
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct CreateTodoRequest {
    pub title: String,
    pub description: Option<String>,
    pub group_id: Option<i64>,
    pub start_date: Option<i64>,
    pub due_date: Option<i64>,
    pub priority: Option<i32>,
    pub tag_ids: Option<Vec<i64>>,
}

/// 获取任务列表请求
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct GetTodosRequest {
    // 保留原有字段（向后兼容）
    pub group_id: Option<i64>,
    pub tag_id: Option<i64>,

    // 新增多选字段
    pub group_ids: Option<Vec<i64>>,
    pub tag_ids: Option<Vec<i64>>,

    pub status: Option<i32>,
    pub search: Option<String>,
    pub priority: Option<i32>,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
}

/// 更新任务请求
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct UpdateTodoRequest {
    pub id: i64,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub status: Option<TodoStatus>,
    #[serde(default)]
    pub priority: Option<i32>,
    #[serde(default)]
    pub group_id: Option<i64>,
    #[serde(default)]
    pub assignee: Option<String>,
    #[serde(default)]
    pub start_date: Option<i64>,
    #[serde(default)]
    pub due_date: Option<i64>,
    #[serde(default)]
    pub tag_ids: Option<Vec<i64>>,
}

/// 更新任务状态请求
#[derive(Debug, Deserialize)]
pub struct UpdateTodoStatusRequest {
    pub id: i64,
    pub status: i32,
}
