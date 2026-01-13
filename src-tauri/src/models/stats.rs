// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use crate::models::{Todo, TaskGroup, Tag};

/// 任务统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoStats {
    pub total: i32,
    pub todo: i32,
    pub in_progress: i32,
    pub done: i32,
    pub overdue: i32,
    pub marked: i32,
}

/// 按日期统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsByDate {
    pub date: String,  // YYYY-MM-DD
    pub completed: i32,
    pub created: i32,
}

/// 任务组统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupStats {
    pub id: String,
    pub name: String,
    pub color: String,
    pub total: i32,
    pub done: i32,
}

/// 标签统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagStats {
    pub id: String,
    pub name: String,
    pub color: String,
    pub count: i32,
}

/// 导出数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportData {
    pub version: String,
    pub exported_at: i64,
    pub task_groups: Vec<TaskGroup>,
    pub tags: Vec<Tag>,
    pub todos: Vec<Todo>,
}

/// 带任务详情的统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoStatsWithDetails {
    pub total: i32,
    pub todo: i32,
    pub in_progress: i32,
    pub done: i32,
    pub todos: Vec<Todo>,
    pub in_progress_tasks: Vec<Todo>,
    pub done_tasks: Vec<Todo>,
}
