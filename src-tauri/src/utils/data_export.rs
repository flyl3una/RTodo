// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

//! 数据导出工具函数
//! 处理 CSV 和 ZIP 文件的读写操作

use crate::models::{TaskGroup, Tag, Todo};
use std::io::{Cursor, Read, Write};
use anyhow::{Result, Context};
use serde::Deserialize;

/// CSV 解析辅助结构

#[derive(Debug, Deserialize)]
struct TaskGroupCsvRecord {
    id: String,
    name: String,
    #[serde(default)]
    parent_id: String,
    #[serde(default)]
    icon: String,
    #[serde(default)]
    color: String,
    sort_order: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize)]
struct TagCsvRecord {
    id: String,
    name: String,
    color: String,
    created_at: String,
}

#[derive(Debug, Deserialize)]
struct TodoCsvRecord {
    id: String,
    title: String,
    #[serde(default)]
    description: String,
    status: String,
    priority: String,
    #[serde(default)]
    group_id: String,
    #[serde(default)]
    assignee: String,
    #[serde(default)]
    start_date: String,
    #[serde(default)]
    due_date: String,
    #[serde(default)]
    completed_at: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize)]
struct TodoTagCsvRecord {
    todo_id: String,
    tag_id: String,
}

/// 解析任务组 CSV 记录
pub fn parse_task_group_csv(record: &csv::StringRecord) -> anyhow::Result<(i64, String, Option<i64>, Option<String>, Option<String>, i32, i64, i64)> {
    let id: i64 = record
        .get(0)
        .ok_or_else(|| anyhow::anyhow!("Missing id field"))?
        .parse()
        .context("Failed to parse task group id")?;
    let name: String = record
        .get(1)
        .ok_or_else(|| anyhow::anyhow!("Missing name field"))?
        .to_string();
    let parent_id: Option<i64> = record
        .get(2)
        .and_then(|s| if s.is_empty() { None } else { Some(s) })
        .and_then(|s| s.parse().ok());
    let icon: Option<String> = record
        .get(3)
        .and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) });
    let color: Option<String> = record
        .get(4)
        .and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) });
    let sort_order: i32 = record
        .get(5)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let created_at: i64 = record
        .get(6)
        .ok_or_else(|| anyhow::anyhow!("Missing created_at field"))?
        .parse()
        .context("Failed to parse task group created_at")?;
    let updated_at: i64 = record
        .get(7)
        .ok_or_else(|| anyhow::anyhow!("Missing updated_at field"))?
        .parse()
        .context("Failed to parse task group updated_at")?;

    Ok((id, name, parent_id, icon, color, sort_order, created_at, updated_at))
}

/// 解析标签 CSV 记录
pub fn parse_tag_csv(record: &csv::StringRecord) -> anyhow::Result<(i64, String, String, i64)> {
    let id: i64 = record
        .get(0)
        .ok_or_else(|| anyhow::anyhow!("Missing id field"))?
        .parse()
        .context("Failed to parse tag id")?;
    let name: String = record
        .get(1)
        .ok_or_else(|| anyhow::anyhow!("Missing name field"))?
        .to_string();
    let color: String = record
        .get(2)
        .ok_or_else(|| anyhow::anyhow!("Missing color field"))?
        .to_string();
    let created_at: i64 = record
        .get(3)
        .ok_or_else(|| anyhow::anyhow!("Missing created_at field"))?
        .parse()
        .context("Failed to parse tag created_at")?;

    Ok((id, name, color, created_at))
}

/// 解析任务 CSV 记录
pub fn parse_todo_csv(record: &csv::StringRecord) -> anyhow::Result<(i64, String, Option<String>, i32, i32, Option<i64>, Option<String>, Option<i64>, Option<i64>, Option<i64>, i64, i64)> {
    let id: i64 = record
        .get(0)
        .ok_or_else(|| anyhow::anyhow!("Missing id field"))?
        .parse()
        .context("Failed to parse todo id")?;
    let title: String = record
        .get(1)
        .ok_or_else(|| anyhow::anyhow!("Missing title field"))?
        .to_string();
    let description: Option<String> = record
        .get(2)
        .and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) });
    let status: i32 = record
        .get(3)
        .ok_or_else(|| anyhow::anyhow!("Missing status field"))?
        .parse()
        .context("Failed to parse todo status")?;
    let priority: i32 = record
        .get(4)
        .ok_or_else(|| anyhow::anyhow!("Missing priority field"))?
        .parse()
        .context("Failed to parse todo priority")?;
    let group_id: Option<i64> = record
        .get(5)
        .and_then(|s| if s.is_empty() { None } else { Some(s) })
        .and_then(|s| s.parse().ok());
    let assignee: Option<String> = record
        .get(6)
        .and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) });
    let start_date: Option<i64> = record
        .get(7)
        .and_then(|s| s.parse().ok());
    let due_date: Option<i64> = record
        .get(8)
        .and_then(|s| s.parse().ok());
    let completed_at: Option<i64> = record
        .get(9)
        .and_then(|s| s.parse().ok());
    let created_at: i64 = record
        .get(10)
        .ok_or_else(|| anyhow::anyhow!("Missing created_at field"))?
        .parse()
        .context("Failed to parse todo created_at")?;
    let updated_at: i64 = record
        .get(11)
        .ok_or_else(|| anyhow::anyhow!("Missing updated_at field"))?
        .parse()
        .context("Failed to parse todo updated_at")?;

    Ok((id, title, description, status, priority, group_id, assignee, start_date, due_date, completed_at, created_at, updated_at))
}

/// 解析任务-标签关联 CSV 记录
pub fn parse_todo_tag_csv(record: &csv::StringRecord) -> anyhow::Result<(i64, i64)> {
    let todo_id: i64 = record
        .get(0)
        .ok_or_else(|| anyhow::anyhow!("Missing todo_id field"))?
        .parse()
        .context("Failed to parse todo_id")?;
    let tag_id: i64 = record
        .get(1)
        .ok_or_else(|| anyhow::anyhow!("Missing tag_id field"))?
        .parse()
        .context("Failed to parse tag_id")?;

    Ok((todo_id, tag_id))
}

/// 将任务组导出为 CSV 格式
pub fn export_groups_to_csv(groups: &[TaskGroup]) -> Result<Vec<u8>> {
    let mut csv_writer = csv::Writer::from_writer(vec![]);

    // 写入表头
    csv_writer
        .write_record(&[
            "id",
            "name",
            "parent_id",
            "icon",
            "color",
            "sort_order",
            "created_at",
            "updated_at",
        ])
        .context("Failed to write CSV header for groups")?;

    // 写入数据
    for group in groups {
        csv_writer
            .write_record(&[
                &group.id.to_string(),
                &group.name,
                &group.parent_id.map(|v| v.to_string()).unwrap_or("".to_string()),
                group.icon.as_ref().map(|s| s.as_str()).unwrap_or(""),
                group.color.as_ref().map(|s| s.as_str()).unwrap_or(""),
                &group.sort_order.to_string(),
                &group.created_at.to_string(),
                &group.updated_at.to_string(),
            ])
            .context(format!("Failed to write CSV record for group {}", group.name))?;
    }

    csv_writer.into_inner().context("Failed to finalize CSV writer for groups")
}

/// 将标签导出为 CSV 格式
pub fn export_tags_to_csv(tags: &[Tag]) -> Result<Vec<u8>> {
    let mut csv_writer = csv::Writer::from_writer(vec![]);

    // 写入表头
    csv_writer
        .write_record(&["id", "name", "color", "created_at"])
        .context("Failed to write CSV header for tags")?;

    // 写入数据
    for tag in tags {
        csv_writer
            .write_record(&[
                &tag.id.to_string(),
                &tag.name,
                &tag.color,
                &tag.created_at.to_string(),
            ])
            .context(format!("Failed to write CSV record for tag {}", tag.name))?;
    }

    csv_writer.into_inner().context("Failed to finalize CSV writer for tags")
}

/// 将任务导出为 CSV 格式
pub fn export_todos_to_csv(todos: &[Todo]) -> Result<Vec<u8>> {
    let mut csv_writer = csv::Writer::from_writer(vec![]);

    // 写入表头
    csv_writer
        .write_record(&[
            "id",
            "title",
            "description",
            "status",
            "priority",
            "group_id",
            "assignee",
            "start_date",
            "due_date",
            "completed_at",
            "created_at",
            "updated_at",
        ])
        .context("Failed to write CSV header for todos")?;

    // 写入数据
    for todo in todos {
        let status_value = match todo.status {
            crate::models::TodoStatus::Todo => "0",
            crate::models::TodoStatus::InProgress => "1",
            crate::models::TodoStatus::Done => "2",
        };

        csv_writer
            .write_record(&[
                &todo.id.to_string(),
                &todo.title,
                todo.description.as_ref().map(|s| s.as_str()).unwrap_or(""),
                status_value,
                &todo.priority.to_string(),
                &todo.group_id.map(|v| v.to_string()).unwrap_or("".to_string()),
                todo.assignee.as_ref().map(|s| s.as_str()).unwrap_or(""),
                &todo.start_date.map(|d| d.to_string()).unwrap_or("".to_string()),
                &todo.due_date.map(|d| d.to_string()).unwrap_or("".to_string()),
                &todo.completed_at.map(|d| d.to_string()).unwrap_or("".to_string()),
                &todo.created_at.to_string(),
                &todo.updated_at.to_string(),
            ])
            .context(format!("Failed to write CSV record for todo {}", todo.title))?;
    }

    csv_writer.into_inner().context("Failed to finalize CSV writer for todos")
}

/// 导出任务-标签关联为 CSV 格式
pub fn export_todo_tags_to_csv(todos: &[Todo]) -> Result<Vec<u8>> {
    let mut csv_writer = csv::Writer::from_writer(vec![]);

    // 写入表头
    csv_writer
        .write_record(&["todo_id", "tag_id"])
        .context("Failed to write CSV header for todo_tags")?;

    // 写入数据
    for todo in todos {
        if let Some(tags) = &todo.tags {
            for tag in tags {
                csv_writer
                    .write_record(&[&todo.id.to_string(), &tag.id.to_string()])
                    .context(format!("Failed to write CSV record for todo-tag relation {}-{}", todo.id, tag.id))?;
            }
        }
    }

    csv_writer.into_inner().context("Failed to finalize CSV writer for todo_tags")
}

/// 创建包含多个 CSV 文件的 ZIP 压缩包
pub fn create_zip_archive(
    groups_csv: Vec<u8>,
    tags_csv: Vec<u8>,
    todos_csv: Vec<u8>,
    todo_tags_csv: Vec<u8>,
) -> Result<Vec<u8>, String> {
    let zip_buffer = Cursor::new(Vec::new());
    let mut zip_writer = zip::ZipWriter::new(zip_buffer);
    let file_options: zip::write::FileOptions<()> =
        zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

    // 添加任务组 CSV
    zip_writer
        .start_file("task_groups.csv", file_options)
        .map_err(|e| format!("Failed to create task_groups.csv: {}", e))?;
    zip_writer
        .write_all(&groups_csv)
        .map_err(|e| format!("Failed to write task_groups.csv: {}", e))?;

    // 添加标签 CSV
    zip_writer
        .start_file("tags.csv", file_options)
        .map_err(|e| format!("Failed to create tags.csv: {}", e))?;
    zip_writer
        .write_all(&tags_csv)
        .map_err(|e| format!("Failed to write tags.csv: {}", e))?;

    // 添加任务 CSV
    zip_writer
        .start_file("todos.csv", file_options)
        .map_err(|e| format!("Failed to create todos.csv: {}", e))?;
    zip_writer
        .write_all(&todos_csv)
        .map_err(|e| format!("Failed to write todos.csv: {}", e))?;

    // 添加任务-标签关联 CSV
    zip_writer
        .start_file("todo_tags.csv", file_options)
        .map_err(|e| format!("Failed to create todo_tags.csv: {}", e))?;
    zip_writer
        .write_all(&todo_tags_csv)
        .map_err(|e| format!("Failed to write todo_tags.csv: {}", e))?;

    // 完成 ZIP 文件
    let result = zip_writer
        .finish()
        .map_err(|e| format!("Failed to finish ZIP: {}", e))?;

    Ok(result.into_inner())
}

/// 从 ZIP 压缩包中读取 CSV 文件内容
pub struct ZipCsvData {
    pub groups_csv: String,
    pub tags_csv: String,
    pub todos_csv: String,
    pub todo_tags_csv: String,
}

/// 从 ZIP 压缩包中提取所有 CSV 文件
pub fn extract_csv_from_zip(zip_data: Vec<u8>) -> Result<ZipCsvData, String> {
    let reader = Cursor::new(zip_data);
    let mut zip_archive =
        zip::ZipArchive::new(reader).map_err(|e| format!("Failed to open zip: {}", e))?;

    let mut groups_csv = String::new();
    let mut tags_csv = String::new();
    let mut todos_csv = String::new();
    let mut todo_tags_csv = String::new();

    // 读取任务组 CSV
    if let Ok(mut file) = zip_archive.by_name("task_groups.csv") {
        file.read_to_string(&mut groups_csv)
            .map_err(|e| format!("Failed to read task_groups.csv: {}", e))?;
    }

    // 读取标签 CSV
    if let Ok(mut file) = zip_archive.by_name("tags.csv") {
        file.read_to_string(&mut tags_csv)
            .map_err(|e| format!("Failed to read tags.csv: {}", e))?;
    }

    // 读取任务 CSV
    if let Ok(mut file) = zip_archive.by_name("todos.csv") {
        file.read_to_string(&mut todos_csv)
            .map_err(|e| format!("Failed to read todos.csv: {}", e))?;
    }

    // 读取任务-标签关联 CSV
    if let Ok(mut file) = zip_archive.by_name("todo_tags.csv") {
        file.read_to_string(&mut todo_tags_csv)
            .map_err(|e| format!("Failed to read todo_tags.csv: {}", e))?;
    }

    Ok(ZipCsvData {
        groups_csv,
        tags_csv,
        todos_csv,
        todo_tags_csv,
    })
}
