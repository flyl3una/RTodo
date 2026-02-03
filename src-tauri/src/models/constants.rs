// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

/// 任务状态常量
///
/// 这些值必须与 TodoStatus 枚举的表示值保持一致
/// TodoStatus::Todo = 0, TodoStatus::InProgress = 1, TodoStatus::Done = 2
pub mod status {
    pub const TODO: i32 = 0;
    pub const IN_PROGRESS: i32 = 1;
    pub const DONE: i32 = 2;
}

/// 优先级常量
pub mod priority {
    pub const NORMAL: i32 = 0;
    pub const IMPORTANT: i32 = 1;
    pub const URGENT: i32 = 3;

    /// 判断是否为重要任务的阈值
    pub const MARKED_THRESHOLD: i32 = IMPORTANT;
}
