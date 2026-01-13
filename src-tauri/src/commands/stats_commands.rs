// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use crate::database::Database;
use crate::database::repositories::StatsRepository;
use crate::models::{TodoStats, StatsByDate, TodoStatsWithDetails};

/// 获取总体统计
#[tauri::command]
pub async fn get_stats(
    db: tauri::State<'_, Database>,
) -> Result<TodoStats, String> {
    tracing::info!("get_stats called");

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    StatsRepository::get_stats(inner)
        .map_err(|e| format!("Failed to get stats: {}", e))
}

/// 按日期获取统计
#[tauri::command]
pub async fn get_stats_by_date(
    range: String,
    db: tauri::State<'_, Database>,
) -> Result<Vec<StatsByDate>, String> {
    tracing::info!("get_stats_by_date called: range={}", range);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    StatsRepository::get_stats_by_date(inner, &range)
        .map_err(|e| format!("Failed to get stats by date: {}", e))
}

/// 获取带任务详情的统计（支持时间范围筛选）
#[tauri::command]
pub async fn get_stats_with_details(
    start_date: Option<i64>,
    end_date: Option<i64>,
    db: tauri::State<'_, Database>,
) -> Result<TodoStatsWithDetails, String> {
    tracing::info!("get_stats_with_details called: start_date={:?}, end_date={:?}", start_date, end_date);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    StatsRepository::get_stats_with_details(inner, start_date, end_date)
        .map_err(|e| format!("Failed to get stats with details: {}", e))
}
