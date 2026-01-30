// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use crate::database::Database;
use crate::database::repositories::StatsRepository;
use crate::models::{TodoStats, StatsByDate, TodoStatsWithDetails};
use crate::pojo::request::{GetStatsRequest, GetStatsByDateRequest, GetStatsWithDetailsRequest};

/// 获取总体统计
#[tauri::command]
pub async fn get_stats(
    payload: GetStatsRequest,
    db: tauri::State<'_, Database>,
) -> Result<TodoStats, String> {
    tracing::info!("get_stats called: start_date={:?}, end_date={:?}",
        payload.start_date, payload.end_date);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    StatsRepository::get_stats(inner, payload.start_date, payload.end_date)
        .map_err(|e| format!("Failed to get stats: {}", e))
}

/// 按日期获取统计
#[tauri::command]
pub async fn get_stats_by_date(
    payload: GetStatsByDateRequest,
    db: tauri::State<'_, Database>,
) -> Result<Vec<StatsByDate>, String> {
    tracing::info!("get_stats_by_date called: range={}, start_date={:?}, end_date={:?}",
        payload.range, payload.start_date, payload.end_date);

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    StatsRepository::get_stats_by_date(inner, &payload.range, payload.start_date, payload.end_date)
        .map_err(|e| format!("Failed to get stats by date: {}", e))
}

/// 获取带任务详情的统计（支持时间范围、多任务组、多标签、多状态筛选）
#[tauri::command]
pub async fn get_stats_with_details(
    payload: GetStatsWithDetailsRequest,
    db: tauri::State<'_, Database>,
) -> Result<TodoStatsWithDetails, String> {
    tracing::info!("get_stats_with_details called: start_date={:?}, end_date={:?}, group_ids={:?}, tag_ids={:?}, status_ids={:?}",
        payload.start_date, payload.end_date, payload.group_ids, payload.tag_ids, payload.status_ids);
    tracing::info!("group_ids len: {:?}", payload.group_ids.as_ref().map(|v| v.len()));
    tracing::info!("tag_ids len: {:?}", payload.tag_ids.as_ref().map(|v| v.len()));
    tracing::info!("status_ids len: {:?}", payload.status_ids.as_ref().map(|v| v.len()));

    let conn = db.get_connection().await;
    let conn_guard = conn.lock().await;
    let inner = conn_guard.inner();

    StatsRepository::get_stats_with_details(inner, payload.start_date, payload.end_date, payload.group_ids, payload.tag_ids, payload.status_ids)
        .map_err(|e| format!("Failed to get stats with details: {}", e))
}
