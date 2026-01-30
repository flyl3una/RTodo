// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use rusqlite::{Connection, params};
use anyhow::{Result, Context};
use chrono::Utc;
use crate::models::{TodoStats, StatsByDate, TodoStatsWithDetails, Todo, TodoStatus};

/// 统计仓库
pub struct StatsRepository;

impl StatsRepository {
    /// 获取总体统计
    ///
    /// 统计逻辑说明：
    /// - 待办任务 (status = 0): 使用 created_at 过滤（在时间范围内创建的待办任务）
    /// - 进行中任务 (status = 1): 使用 updated_at 过滤（在时间范围内被设为进行中的任务）
    /// - 已完成任务 (status = 2): 使用 completed_at 过滤（在时间范围内被标记为完成的任务）
    /// - 重要任务: 使用 created_at 过滤（在时间范围内创建的重要任务）
    /// - 逾期任务: 使用 created_at 过滤（在时间范围内创建的逾期任务）
    pub fn get_stats(conn: &Connection, start_date: Option<i64>, end_date: Option<i64>) -> Result<TodoStats> {
        // 为不同状态构建不同的时间过滤条件
        let created_filter: String = match (start_date, end_date) {
            (Some(start), Some(end)) => {
                format!(" AND created_at >= {} AND created_at <= {}", start, end)
            }
            (Some(start), None) => {
                format!(" AND created_at >= {}", start)
            }
            (None, Some(end)) => {
                format!(" AND created_at <= {}", end)
            }
            (None, None) => String::new(),
        };

        let updated_filter: String = match (start_date, end_date) {
            (Some(start), Some(end)) => {
                format!(" AND updated_at >= {} AND updated_at <= {}", start, end)
            }
            (Some(start), None) => {
                format!(" AND updated_at >= {}", start)
            }
            (None, Some(end)) => {
                format!(" AND updated_at <= {}", end)
            }
            (None, None) => String::new(),
        };

        let completed_filter: String = match (start_date, end_date) {
            (Some(start), Some(end)) => {
                format!(" AND completed_at >= {} AND completed_at <= {}", start, end)
            }
            (Some(start), None) => {
                format!(" AND completed_at >= {}", start)
            }
            (None, Some(end)) => {
                format!(" AND completed_at <= {}", end)
            }
            (None, None) => String::new(),
        };

        // 获取任务总数（使用创建时间）
        let total: i32 = conn.query_row(
            &format!("SELECT COUNT(*) FROM todos WHERE 1=1{}", created_filter),
            [],
            |row| row.get(0)
        ).context("Failed to get total count")?;

        // 获取待办任务数（使用创建时间）
        let todo: i32 = conn.query_row(
            &format!("SELECT COUNT(*) FROM todos WHERE status = 0{}", created_filter),
            [],
            |row| row.get(0)
        ).context("Failed to get todo count")?;

        // 获取进行中任务数（使用更新时间）
        let in_progress: i32 = conn.query_row(
            &format!("SELECT COUNT(*) FROM todos WHERE status = 1{}", updated_filter),
            [],
            |row| row.get(0)
        ).context("Failed to get in_progress count")?;

        // 获取已完成任务数（使用完成时间）
        let done: i32 = conn.query_row(
            &format!("SELECT COUNT(*) FROM todos WHERE status = 2{}", completed_filter),
            [],
            |row| row.get(0)
        ).context("Failed to get done count")?;

        // 获取重要任务数（使用创建时间）
        let marked: i32 = conn.query_row(
            &format!("SELECT COUNT(*) FROM todos WHERE priority >= 1{}", created_filter),
            [],
            |row| row.get(0)
        ).context("Failed to get marked count")?;

        // 获取逾期任务数（使用创建时间）
        let now = Utc::now().timestamp_millis();
        let overdue: i32 = conn.query_row(
            &format!("SELECT COUNT(*) FROM todos WHERE due_date < {} AND status != 2{}", now, created_filter),
            [],
            |row| row.get(0)
        ).context("Failed to get overdue count")?;

        Ok(TodoStats {
            total,
            todo,
            in_progress,
            done,
            overdue,
            marked,
        })
    }

    /// 按日期获取统计
    pub fn get_stats_by_date(conn: &Connection, range: &str, start_date_param: Option<i64>, end_date_param: Option<i64>) -> Result<Vec<StatsByDate>> {
        let (start_date, days) = if let (Some(start), Some(end)) = (start_date_param, end_date_param) {
            // Use custom date range
            let start = start / 86_400_000 * 86_400_000; // Round down to start of day
            let duration_ms = end - start;
            let days_count = (duration_ms / 86_400_000).max(1) as i32;
            (start, days_count)
        } else {
            // Use predefined range
            let now = Utc::now();
            match range {
                "day" => {
                    // 今天
                    let start = now.date_naive()
                        .and_hms_opt(0, 0, 0)
                        .expect("0:0:0 is always a valid time")
                        .and_utc()
                        .timestamp_millis();
                    (start, 1)
                }
                "week" => {
                    // 最近7天
                    let start = (now - chrono::Duration::days(6))
                        .date_naive()
                        .and_hms_opt(0, 0, 0)
                        .expect("0:0:0 is always a valid time")
                        .and_utc()
                        .timestamp_millis();
                    (start, 7)
                }
                "month" => {
                    // 最近30天
                    let start = (now - chrono::Duration::days(29))
                        .date_naive()
                        .and_hms_opt(0, 0, 0)
                        .expect("0:0:0 is always a valid time")
                        .and_utc()
                        .timestamp_millis();
                    (start, 30)
                }
                _ => return Ok(vec![]),
            }
        };

        let mut stats = Vec::new();

        // 按天统计
        for day_offset in 0..days {
            let day_start = start_date + ((day_offset as i64) * 86_400_000); // 加上天数 * 毫秒数
            let day_end = day_start + 86_400_000; // 加一天

            // 转换为日期字符串
            let date = chrono::DateTime::from_timestamp_millis(day_start)
                .expect("timestamp should be valid for date range")
                .format("%Y-%m-%d")
                .to_string();

            // 统计当天完成的任务数
            let completed: i32 = conn.query_row(
                "SELECT COUNT(*) FROM todos WHERE completed_at >= ? AND completed_at < ?",
                params![day_start, day_end],
                |row| row.get(0)
            ).unwrap_or(0);

            // 统计当天创建的任务数
            let created: i32 = conn.query_row(
                "SELECT COUNT(*) FROM todos WHERE created_at >= ? AND created_at < ?",
                params![day_start, day_end],
                |row| row.get(0)
            ).unwrap_or(0);

            stats.push(StatsByDate {
                date,
                completed,
                created,
            });
        }

        Ok(stats)
    }

    /// 获取带任务详情的统计（支持时间范围、多任务组、多标签、多状态筛选）
    ///
    /// 统计逻辑说明：
    /// - 待办任务 (status = 0): 使用 created_at 过滤（在时间范围内创建的待办任务）
    /// - 进行中任务 (status = 1): 使用 updated_at 过滤（在时间范围内被设为进行中的任务）
    /// - 已完成任务 (status = 2): 使用 completed_at 过滤（在时间范围内被标记为完成的任务）
    pub fn get_stats_with_details(
        conn: &Connection,
        start_date: Option<i64>,
        end_date: Option<i64>,
        group_ids: Option<Vec<i64>>,
        tag_ids: Option<Vec<i64>>,
        status_ids: Option<Vec<i32>>,
    ) -> Result<TodoStatsWithDetails> {
        // 构建时间过滤条件
        let (created_filter, updated_filter, completed_filter) = Self::build_time_filters(start_date, end_date);

        // 构建任务组和标签过滤条件
        let (group_filter, group_params): (String, Vec<i64>) = Self::build_group_filter(&group_ids);
        let (tag_filter, tag_params): (String, Vec<i64>) = Self::build_tag_filter(&tag_ids);

        // 构建状态过滤条件
        let (status_filter, status_params): (String, Vec<i32>) = Self::build_status_filter(&status_ids);

        // 获取待办任务（使用 created_at）
        let todos = Self::query_tasks_by_status(
            conn,
            0,
            &created_filter,
            &group_filter,
            &tag_filter,
            &status_filter,
            &group_params,
            &tag_params,
            &status_params,
        )?;

        // 获取进行中任务（使用 updated_at）
        let in_progress_tasks = Self::query_tasks_by_status(
            conn,
            1,
            &updated_filter,
            &group_filter,
            &tag_filter,
            &status_filter,
            &group_params,
            &tag_params,
            &status_params,
        )?;

        // 获取已完成任务（使用 completed_at）
        let done_tasks = Self::query_tasks_by_status(
            conn,
            2,
            &completed_filter,
            &group_filter,
            &tag_filter,
            &status_filter,
            &group_params,
            &tag_params,
            &status_params,
        )?;

        let total = todos.len() + in_progress_tasks.len() + done_tasks.len();

        Ok(TodoStatsWithDetails {
            total: total as i32,
            todo: todos.len() as i32,
            in_progress: in_progress_tasks.len() as i32,
            done: done_tasks.len() as i32,
            todos,
            in_progress_tasks,
            done_tasks,
        })
    }

    /// 构建时间过滤条件
    fn build_time_filters(start_date: Option<i64>, end_date: Option<i64>) -> (String, String, String) {
        let created_filter: String = match (start_date, end_date) {
            (Some(start), Some(end)) => {
                format!(" AND created_at >= {} AND created_at <= {}", start, end)
            }
            (Some(start), None) => {
                format!(" AND created_at >= {}", start)
            }
            (None, Some(end)) => {
                format!(" AND created_at <= {}", end)
            }
            (None, None) => String::new(),
        };

        let updated_filter: String = match (start_date, end_date) {
            (Some(start), Some(end)) => {
                format!(" AND updated_at >= {} AND updated_at <= {}", start, end)
            }
            (Some(start), None) => {
                format!(" AND updated_at >= {}", start)
            }
            (None, Some(end)) => {
                format!(" AND updated_at <= {}", end)
            }
            (None, None) => String::new(),
        };

        let completed_filter: String = match (start_date, end_date) {
            (Some(start), Some(end)) => {
                format!(" AND completed_at >= {} AND completed_at <= {}", start, end)
            }
            (Some(start), None) => {
                format!(" AND completed_at >= {}", start)
            }
            (None, Some(end)) => {
                format!(" AND completed_at <= {}", end)
            }
            (None, None) => String::new(),
        };

        (created_filter, updated_filter, completed_filter)
    }

    /// 构建任务组过滤条件
    fn build_group_filter(group_ids: &Option<Vec<i64>>) -> (String, Vec<i64>) {
        if let Some(gids) = group_ids {
            if !gids.is_empty() {
                let placeholders: Vec<String> = (0..gids.len()).map(|_| "?".to_string()).collect();
                (format!(" AND group_id IN ({})", placeholders.join(", ")), gids.clone())
            } else {
                (String::new(), vec![])
            }
        } else {
            (String::new(), vec![])
        }
    }

    /// 构建标签过滤条件
    fn build_tag_filter(tag_ids: &Option<Vec<i64>>) -> (String, Vec<i64>) {
        if let Some(tids) = tag_ids {
            if !tids.is_empty() {
                let tag_conditions: Vec<String> = tids.iter().map(|_| {
                    "EXISTS (SELECT 1 FROM todo_tags tt WHERE tt.todo_id = t.id AND tt.tag_id = ?)".to_string()
                }).collect();
                (format!(" AND ({})", tag_conditions.join(" OR ")), tids.clone())
            } else {
                (String::new(), vec![])
            }
        } else {
            (String::new(), vec![])
        }
    }

    /// 构建状态过滤条件
    fn build_status_filter(status_ids: &Option<Vec<i32>>) -> (String, Vec<i32>) {
        if let Some(sids) = status_ids {
            if !sids.is_empty() {
                let placeholders: Vec<String> = (0..sids.len()).map(|_| "?".to_string()).collect();
                (format!(" AND t.status IN ({})", placeholders.join(", ")), sids.clone())
            } else {
                (String::new(), vec![])
            }
        } else {
            (String::new(), vec![])
        }
    }

    /// 按状态和时间字段查询任务
    fn query_tasks_by_status(
        conn: &Connection,
        status: i32,
        time_filter: &str,
        group_filter: &str,
        tag_filter: &str,
        status_filter: &str,
        group_params: &[i64],
        tag_params: &[i64],
        status_params: &[i32],
    ) -> Result<Vec<Todo>> {
        let mut query = format!(
            "SELECT t.* FROM todos t WHERE t.status = {}{}{}{}{}",
            status, time_filter, group_filter, tag_filter, status_filter
        );

        query.push_str(" ORDER BY
            CASE WHEN t.status = 2 THEN 1 ELSE 0 END,
            t.priority DESC,
            t.due_date ASC,
            t.created_at DESC");

        // 构建参数列表（混合类型：i64 和 i32）
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        // 添加 i64 类型参数（group_params 和 tag_params）
        for gid in group_params {
            params.push(Box::new(*gid));
        }
        for tid in tag_params {
            params.push(Box::new(*tid));
        }

        // 添加 i32 类型参数（status_params）
        for sid in status_params {
            params.push(Box::new(*sid));
        }

        // 转换为引用类型
        let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref() as &dyn rusqlite::ToSql).collect();

        let mut stmt = conn.prepare(&query)?;
        let todo_iter = stmt.query_map(param_refs.as_slice(), |row| {
            let status_int: i32 = row.get("status")?;
            let status = match status_int {
                0 => TodoStatus::Todo,
                1 => TodoStatus::InProgress,
                2 => TodoStatus::Done,
                _ => TodoStatus::Todo,
            };
            Ok(Todo {
                id: row.get("id")?,
                title: row.get("title")?,
                description: row.get("description")?,
                status,
                priority: row.get("priority")?,
                group_id: row.get("group_id")?,
                assignee: row.get("assignee")?,
                start_date: row.get("start_date")?,
                due_date: row.get("due_date")?,
                completed_at: row.get("completed_at")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
                tags: None,
                steps: None,
                attachments: None,
                group_info: None,
            })
        })?;

        let mut todos = Vec::new();
        for todo in todo_iter {
            todos.push(todo?);
        }

        Ok(todos)
    }
}
