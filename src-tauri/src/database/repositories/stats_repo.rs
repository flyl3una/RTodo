// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use rusqlite::{Connection, params};
use anyhow::{Result, Context};
use chrono::Utc;
use crate::models::{TodoStats, StatsByDate, TodoStatsWithDetails, Todo, TodoStatus};
use crate::database::repositories::TodoRepository;

/// 统计仓库
pub struct StatsRepository;

impl StatsRepository {
    /// 获取总体统计
    pub fn get_stats(conn: &Connection) -> Result<TodoStats> {
        // 获取任务总数
        let total: i32 = conn.query_row(
            "SELECT COUNT(*) FROM todos",
            [],
            |row| row.get(0)
        ).context("Failed to get total count")?;

        // 获取各状态任务数
        let todo: i32 = conn.query_row(
            "SELECT COUNT(*) FROM todos WHERE status = 0",
            [],
            |row| row.get(0)
        ).context("Failed to get todo count")?;

        let in_progress: i32 = conn.query_row(
            "SELECT COUNT(*) FROM todos WHERE status = 1",
            [],
            |row| row.get(0)
        ).context("Failed to get in_progress count")?;

        let done: i32 = conn.query_row(
            "SELECT COUNT(*) FROM todos WHERE status = 2",
            [],
            |row| row.get(0)
        ).context("Failed to get done count")?;

        // 获取重要任务数（优先级 >= 1 的任务）
        let marked: i32 = conn.query_row(
            "SELECT COUNT(*) FROM todos WHERE priority >= 1",
            [],
            |row| row.get(0)
        ).context("Failed to get marked count")?;

        // 获取逾期任务数（截止日期小于当前时间且状态不是已完成）
        let now = Utc::now().timestamp_millis();
        let overdue: i32 = conn.query_row(
            "SELECT COUNT(*) FROM todos WHERE due_date < ? AND status != 2",
            params![now],
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
    pub fn get_stats_by_date(conn: &Connection, range: &str) -> Result<Vec<StatsByDate>> {
        let now = Utc::now();
        let (start_date, days) = match range {
            "day" => {
                // 今天
                let start = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp_millis();
                (start, 1)
            }
            "week" => {
                // 最近7天
                let start = (now - chrono::Duration::days(6))
                    .date_naive()
                    .and_hms_opt(0, 0, 0).unwrap()
                    .and_utc()
                    .timestamp_millis();
                (start, 7)
            }
            "month" => {
                // 最近30天
                let start = (now - chrono::Duration::days(29))
                    .date_naive()
                    .and_hms_opt(0, 0, 0).unwrap()
                    .and_utc()
                    .timestamp_millis();
                (start, 30)
            }
            _ => return Ok(vec![]),
        };

        let mut stats = Vec::new();

        // 按天统计
        for day_offset in 0..days {
            let day_start = start_date + (day_offset * 86_400_000); // 加上天数 * 毫秒数
            let day_end = day_start + 86_400_000; // 加一天

            // 转换为日期字符串
            let date = chrono::DateTime::from_timestamp_millis(day_start)
                .unwrap()
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

    /// 获取带任务详情的统计（支持时间范围筛选）
    pub fn get_stats_with_details(
        conn: &Connection,
        start_date: Option<i64>,
        end_date: Option<i64>,
    ) -> Result<TodoStatsWithDetails> {
        // 获取所有任务（带时间筛选）
        let all_todos = TodoRepository::list(
            conn,
            None, // group_id
            None, // tag_id
            None, // status
            None, // search
            None, // priority
            start_date,
            end_date,
        )?;

        // 按状态分类
        let todos: Vec<Todo> = all_todos.iter().filter(|t| t.status == TodoStatus::Todo).cloned().collect();
        let in_progress_tasks: Vec<Todo> = all_todos.iter().filter(|t| t.status == TodoStatus::InProgress).cloned().collect();
        let done_tasks: Vec<Todo> = all_todos.iter().filter(|t| t.status == TodoStatus::Done).cloned().collect();

        Ok(TodoStatsWithDetails {
            total: all_todos.len() as i32,
            todo: todos.len() as i32,
            in_progress: in_progress_tasks.len() as i32,
            done: done_tasks.len() as i32,
            todos,
            in_progress_tasks,
            done_tasks,
        })
    }
}
