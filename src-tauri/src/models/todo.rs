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

impl TodoStatus {
    /// 从 i32 值创建 TodoStatus，无效值默认为 Todo
    pub fn from_i32(value: i32) -> Self {
        match value {
            x if x == Self::Todo as i32 => Self::Todo,
            x if x == Self::InProgress as i32 => Self::InProgress,
            x if x == Self::Done as i32 => Self::Done,
            _ => Self::Todo,
        }
    }

    /// 转换为 i32 值
    pub fn as_i32(self) -> i32 {
        self as i32
    }
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

