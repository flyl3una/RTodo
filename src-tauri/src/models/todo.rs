// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use crate::models::{Tag, TodoStep, Attachment, TaskGroup};

/// 任务状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TodoStatus {
    Todo,
    InProgress,
    Done,
}

impl From<String> for TodoStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "in_progress" => TodoStatus::InProgress,
            "done" => TodoStatus::Done,
            _ => TodoStatus::Todo,
        }
    }
}

impl From<TodoStatus> for String {
    fn from(status: TodoStatus) -> Self {
        match status {
            TodoStatus::Todo => "todo".to_string(),
            TodoStatus::InProgress => "in_progress".to_string(),
            TodoStatus::Done => "done".to_string(),
        }
    }
}

/// 任务实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: TodoStatus,
    pub priority: i32,
    pub is_marked: bool,
    pub group_id: Option<String>,
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
    pub group: Option<TaskGroup>,
}

/// 创建任务请求
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CreateTodoRequest {
    pub title: String,
    pub description: Option<String>,
    pub group_id: Option<String>,
    pub start_date: Option<i64>,
    pub due_date: Option<i64>,
    pub priority: Option<i32>,
    pub tag_ids: Option<Vec<String>>,
}

/// 更新任务请求
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct UpdateTodoRequest {
    pub id: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub status: Option<TodoStatus>,
    #[serde(default)]
    pub priority: Option<i32>,
    #[serde(default)]
    pub is_marked: Option<bool>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub assignee: Option<String>,
    #[serde(default)]
    pub start_date: Option<i64>,
    #[serde(default)]
    pub due_date: Option<i64>,
    #[serde(default)]
    pub tag_ids: Option<Vec<String>>,
}
