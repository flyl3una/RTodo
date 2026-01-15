// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use crate::models::{Tag, TodoStep, Attachment, TaskGroup};

/// 任务状态 - 使用数字表示
/// 0: 待办, 1: 进行中, 2: 已完成
#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr, PartialEq, Eq)]
#[repr(i32)]
pub enum TodoStatus {
    Todo = 0,
    InProgress = 1,
    Done = 2,
}

/// 任务实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: TodoStatus,
    pub priority: i32,
    pub group_id: Option<i64>,
    pub assignee: Option<String>,
    pub start_date: Option<i64>,
    pub due_date: Option<i64>,
    pub completed_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
    // 关联数据（查询时包含）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<TodoStep>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_info: Option<TaskGroup>,
}

/// 创建任务请求
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
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
    pub group_id: Option<i64>,
    pub tag_id: Option<i64>,
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
