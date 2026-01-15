// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

pub mod data_export;

use chrono::Utc;

/// 获取当前时间戳（毫秒）
pub fn now_timestamp() -> i64 {
    Utc::now().timestamp_millis()
}

/// 生成新的 UUID
pub fn new_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}
