// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use serde::Deserialize;

/// 获取统计请求
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct GetStatsRequest {
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
}

/// 按日期获取统计请求
#[derive(Debug, Deserialize)]
pub struct GetStatsByDateRequest {
    pub range: String,
    #[serde(default)]
    pub start_date: Option<i64>,
    #[serde(default)]
    pub end_date: Option<i64>,
}

/// 获取带详情统计请求
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct GetStatsWithDetailsRequest {
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
    pub group_ids: Option<Vec<i64>>,
    pub tag_ids: Option<Vec<i64>>,
    pub status_ids: Option<Vec<i32>>,
}
