// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

//! 数据导出工具函数
//! 处理 CSV 和 ZIP 文件的读写操作

use crate::models::{TaskGroup, Tag, Todo, TodoStep, Attachment};
use std::io::{Cursor, Read, Seek, Write};
use std::path::Path;
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

#[derive(Debug, Deserialize)]
struct TodoStepCsvRecord {
    id: String,
    todo_id: String,
    title: String,
    is_completed: String,
    sort_order: String,
    created_at: String,
}

#[derive(Debug, Deserialize)]
struct AttachmentCsvRecord {
    id: String,
    todo_id: String,
    name: String,
    file_path: String,
    file_size: String,
    #[serde(default)]
    mime_type: String,
    created_at: String,
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
    pub steps_csv: String,
    pub attachments_csv: String,
}

/// 从 ZIP 压缩包中提取所有 CSV 文件
/// 从 tables/ 目录提取CSV文件
pub fn extract_csv_from_zip(zip_data: Vec<u8>) -> Result<ZipCsvData, String> {
    let reader = Cursor::new(zip_data);
    let mut zip_archive =
        zip::ZipArchive::new(reader).map_err(|e| format!("Failed to open zip: {}", e))?;

    tracing::info!("ZIP file contains {} files", zip_archive.len());

    let mut groups_csv = String::new();
    let mut tags_csv = String::new();
    let mut todos_csv = String::new();
    let mut todo_tags_csv = String::new();
    let mut steps_csv = String::new();
    let mut attachments_csv = String::new();

    // 读取任务组 CSV
    if let Ok(mut file) = zip_archive.by_name("tables/task_groups.csv") {
        file.read_to_string(&mut groups_csv)
            .map_err(|e| format!("Failed to read tables/task_groups.csv: {}", e))?;
        tracing::info!("Successfully read tables/task_groups.csv: {} bytes", groups_csv.len());
    } else {
        tracing::warn!("tables/task_groups.csv not found in ZIP");
    }

    // 读取标签 CSV
    if let Ok(mut file) = zip_archive.by_name("tables/tags.csv") {
        file.read_to_string(&mut tags_csv)
            .map_err(|e| format!("Failed to read tables/tags.csv: {}", e))?;
        tracing::info!("Successfully read tables/tags.csv: {} bytes", tags_csv.len());
    } else {
        tracing::warn!("tables/tags.csv not found in ZIP");
    }

    // 读取任务 CSV
    if let Ok(mut file) = zip_archive.by_name("tables/todos.csv") {
        file.read_to_string(&mut todos_csv)
            .map_err(|e| format!("Failed to read tables/todos.csv: {}", e))?;
        tracing::info!("Successfully read tables/todos.csv: {} bytes", todos_csv.len());
    } else {
        tracing::warn!("tables/todos.csv not found in ZIP");
    }

    // 读取任务-标签关联 CSV
    if let Ok(mut file) = zip_archive.by_name("tables/todo_tags.csv") {
        file.read_to_string(&mut todo_tags_csv)
            .map_err(|e| format!("Failed to read tables/todo_tags.csv: {}", e))?;
        tracing::info!("Successfully read tables/todo_tags.csv: {} bytes", todo_tags_csv.len());
    } else {
        tracing::warn!("tables/todo_tags.csv not found in ZIP");
    }

    // 读取步骤 CSV
    if let Ok(mut file) = zip_archive.by_name("tables/steps.csv") {
        file.read_to_string(&mut steps_csv)
            .map_err(|e| format!("Failed to read tables/steps.csv: {}", e))?;
        tracing::info!("Successfully read tables/steps.csv: {} bytes", steps_csv.len());
    } else {
        tracing::warn!("tables/steps.csv not found in ZIP");
    }

    // 读取附件 CSV
    if let Ok(mut file) = zip_archive.by_name("tables/attachments.csv") {
        file.read_to_string(&mut attachments_csv)
            .map_err(|e| format!("Failed to read tables/attachments.csv: {}", e))?;
        tracing::info!("Successfully read tables/attachments.csv: {} bytes", attachments_csv.len());
    } else {
        tracing::warn!("tables/attachments.csv not found in ZIP");
    }

    // 列出ZIP中的所有文件用于调试
    for i in 0..zip_archive.len() {
        if let Ok(zip_file) = zip_archive.by_index(i) {
            tracing::info!("Found file in ZIP: {}", zip_file.name());
        }
    }

    Ok(ZipCsvData {
        groups_csv,
        tags_csv,
        todos_csv,
        todo_tags_csv,
        steps_csv,
        attachments_csv,
    })
}

/// 递归将目录添加到 ZIP 文件中
fn add_dir_to_zip<W>(
    zip_writer: &mut zip::ZipWriter<W>,
    dir_path: &Path,
    zip_prefix: &str,
    options: zip::write::FileOptions<()>,
) -> Result<(), String>
where
    W: Write + Seek,
{
    for entry in std::fs::read_dir(dir_path)
        .map_err(|e| format!("Failed to read directory {}: {}", dir_path.display(), e))?
    {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();
        let file_type = entry.file_type().map_err(|e| format!("Failed to get file type: {}", e))?;

        let relative_path = path
            .strip_prefix(dir_path)
            .map_err(|e| format!("Failed to strip prefix: {}", e))?;

        let zip_name = format!("{}/{}", zip_prefix, relative_path.display());

        if file_type.is_dir() {
            // 对于目录，我们需要以 / 结尾，这样解压时才能正确识别为目录
            let dir_name = format!("{}/", zip_name);
            zip_writer
                .start_file(&dir_name, options)
                .map_err(|e| format!("Failed to create directory {} in ZIP: {}", dir_name, e))?;

            // 递归处理子目录
            add_dir_to_zip(zip_writer, &path, zip_prefix, options)?;
        } else {
            // 对于文件，直接添加到 ZIP
            zip_writer
                .start_file(&zip_name, options)
                .map_err(|e| format!("Failed to create file {} in ZIP: {}", zip_name, e))?;

            let file_content = std::fs::read(&path)
                .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?;

            zip_writer
                .write_all(&file_content)
                .map_err(|e| format!("Failed to write file {} to ZIP: {}", zip_name, e))?;
        }
    }

    Ok(())
}

/// 从 ZIP 文件中提取指定目录到目标位置
fn extract_dir_from_zip(
    zip_archive: &mut zip::ZipArchive<Cursor<Vec<u8>>>,
    zip_dir_prefix: &str,
    target_dir: &Path,
) -> Result<(), String> {
    // 确保目标目录存在
    std::fs::create_dir_all(target_dir)
        .map_err(|e| format!("Failed to create target directory {}: {}", target_dir.display(), e))?;

    // 遍历 ZIP 中的所有文件
    for i in 0..zip_archive.len() {
        let mut zip_file = zip_archive
            .by_index(i)
            .map_err(|e| format!("Failed to get file at index {}: {}", i, e))?;

        let name = zip_file.name();

        // 检查文件是否在指定的目录前缀下
        if name.starts_with(&format!("{}/", zip_dir_prefix)) {
            // 计算相对路径
            let relative_path = name
                .strip_prefix(&format!("{}/", zip_dir_prefix))
                .unwrap_or(name);

            let target_path = target_dir.join(relative_path);

            // 如果是目录（以 / 结尾），创建目录
            if name.ends_with('/') {
                std::fs::create_dir_all(&target_path)
                    .map_err(|e| format!("Failed to create directory {}: {}", target_path.display(), e))?;
            } else {
                // 确保父目录存在
                if let Some(parent) = target_path.parent() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| format!("Failed to create parent directory {}: {}", parent.display(), e))?;
                }

                // 解压文件
                let mut file =
                    std::fs::File::create(&target_path).map_err(|e| {
                        format!("Failed to create file {}: {}", target_path.display(), e)
                    })?;

                std::io::copy(&mut zip_file, &mut file)
                    .map_err(|e| format!("Failed to write file {}: {}", target_path.display(), e))?;
            }
        }
    }

    Ok(())
}

/// 创建包含多个 CSV 文件和附件的 ZIP 压缩包
/// attachments_path 是附件目录的路径，如果为 None 则不包含附件文件
///
/// ZIP 结构:
/// tables/
///   ├── task_groups.csv
///   ├── tags.csv
///   ├── todos.csv
///   ├── todo_tags.csv
///   ├── steps.csv
///   └── attachments.csv
/// data/
///   └── attachments/
///       └── (附件文件)
pub fn create_zip_archive_with_attachments(
    groups_csv: Vec<u8>,
    tags_csv: Vec<u8>,
    todos_csv: Vec<u8>,
    todo_tags_csv: Vec<u8>,
    steps_csv: Vec<u8>,
    attachments_csv: Vec<u8>,
    attachments_path: Option<&Path>,
) -> Result<Vec<u8>, String> {
    let zip_buffer = Cursor::new(Vec::new());
    let mut zip_writer = zip::ZipWriter::new(zip_buffer);
    let file_options: zip::write::FileOptions<()> =
        zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

    // 添加 tables 目录下的 CSV 文件
    // 任务组 CSV
    zip_writer
        .start_file("tables/task_groups.csv", file_options)
        .map_err(|e| format!("Failed to create tables/task_groups.csv: {}", e))?;
    zip_writer
        .write_all(&groups_csv)
        .map_err(|e| format!("Failed to write task_groups.csv: {}", e))?;

    // 标签 CSV
    zip_writer
        .start_file("tables/tags.csv", file_options)
        .map_err(|e| format!("Failed to create tables/tags.csv: {}", e))?;
    zip_writer
        .write_all(&tags_csv)
        .map_err(|e| format!("Failed to write tags.csv: {}", e))?;

    // 任务 CSV
    zip_writer
        .start_file("tables/todos.csv", file_options)
        .map_err(|e| format!("Failed to create tables/todos.csv: {}", e))?;
    zip_writer
        .write_all(&todos_csv)
        .map_err(|e| format!("Failed to write todos.csv: {}", e))?;

    // 任务-标签关联 CSV
    zip_writer
        .start_file("tables/todo_tags.csv", file_options)
        .map_err(|e| format!("Failed to create tables/todo_tags.csv: {}", e))?;
    zip_writer
        .write_all(&todo_tags_csv)
        .map_err(|e| format!("Failed to write todo_tags.csv: {}", e))?;

    // 步骤 CSV
    zip_writer
        .start_file("tables/steps.csv", file_options)
        .map_err(|e| format!("Failed to create tables/steps.csv: {}", e))?;
    zip_writer
        .write_all(&steps_csv)
        .map_err(|e| format!("Failed to write steps.csv: {}", e))?;

    // 附件 CSV
    zip_writer
        .start_file("tables/attachments.csv", file_options)
        .map_err(|e| format!("Failed to create tables/attachments.csv: {}", e))?;
    zip_writer
        .write_all(&attachments_csv)
        .map_err(|e| format!("Failed to write attachments.csv: {}", e))?;

    // 如果提供了附件目录，添加附件文件到 data/attachments/
    if let Some(attachments_dir) = attachments_path {
        if attachments_dir.exists() {
            tracing::info!("Adding attachments from: {} to data/attachments/", attachments_dir.display());
            add_dir_to_zip(&mut zip_writer, attachments_dir, "data/attachments", file_options)?;
        } else {
            tracing::info!("Attachments directory does not exist, skipping: {}", attachments_dir.display());
        }
    }

    // 完成 ZIP 文件
    let result = zip_writer
        .finish()
        .map_err(|e| format!("Failed to finish ZIP: {}", e))?;

    Ok(result.into_inner())
}

/// 从 ZIP 压缩包中提取附件到指定目录
/// 从 data/attachments/ 提取附件文件
pub fn extract_attachments_from_zip(
    zip_data: Vec<u8>,
    target_attachments_dir: &Path,
) -> Result<(), String> {
    let reader = Cursor::new(zip_data);
    let mut zip_archive =
        zip::ZipArchive::new(reader).map_err(|e| format!("Failed to open zip: {}", e))?;

    extract_dir_from_zip(&mut zip_archive, "data/attachments", target_attachments_dir)?;

    tracing::info!("Attachments extracted to: {}", target_attachments_dir.display());

    Ok(())
}

/// 导出步骤为 CSV 格式
pub fn export_steps_to_csv(steps: &[TodoStep]) -> Result<Vec<u8>> {
    let mut csv_writer = csv::Writer::from_writer(vec![]);

    // 写入表头
    csv_writer
        .write_record(&[
            "id",
            "todo_id",
            "title",
            "is_completed",
            "sort_order",
            "created_at",
        ])
        .context("Failed to write CSV header for steps")?;

    // 写入数据
    for step in steps {
        csv_writer
            .write_record(&[
                &step.id.to_string(),
                &step.todo_id.to_string(),
                &step.title,
                &if step.is_completed { "1".to_string() } else { "0".to_string() },
                &step.sort_order.to_string(),
                &step.created_at.to_string(),
            ])
            .context(format!("Failed to write CSV record for step {}", step.id))?;
    }

    csv_writer.into_inner().context("Failed to finalize CSV writer for steps")
}

/// 导出附件为 CSV 格式
pub fn export_attachments_to_csv(attachments: &[Attachment]) -> Result<Vec<u8>> {
    let mut csv_writer = csv::Writer::from_writer(vec![]);

    // 写入表头
    csv_writer
        .write_record(&[
            "id",
            "todo_id",
            "name",
            "file_path",
            "file_size",
            "mime_type",
            "created_at",
        ])
        .context("Failed to write CSV header for attachments")?;

    // 写入数据
    for attachment in attachments {
        csv_writer
            .write_record(&[
                &attachment.id.to_string(),
                &attachment.todo_id.to_string(),
                &attachment.name,
                &attachment.file_path,
                &attachment.file_size.to_string(),
                &attachment.mime_type.as_ref().map(|s| s.as_str()).unwrap_or("").to_string(),
                &attachment.created_at.to_string(),
            ])
            .context(format!("Failed to write CSV record for attachment {}", attachment.id))?;
    }

    csv_writer.into_inner().context("Failed to finalize CSV writer for attachments")
}

/// 解析步骤 CSV 记录
pub fn parse_step_csv(record: &csv::StringRecord) -> anyhow::Result<(i64, i64, String, bool, i32, i64)> {
    let id: i64 = record
        .get(0)
        .ok_or_else(|| anyhow::anyhow!("Missing id field"))?
        .parse()
        .context("Failed to parse step id")?;
    let todo_id: i64 = record
        .get(1)
        .ok_or_else(|| anyhow::anyhow!("Missing todo_id field"))?
        .parse()
        .context("Failed to parse step todo_id")?;
    let title: String = record
        .get(2)
        .ok_or_else(|| anyhow::anyhow!("Missing title field"))?
        .to_string();
    let is_completed_str = record
        .get(3)
        .ok_or_else(|| anyhow::anyhow!("Missing is_completed field"))?;
    let is_completed: bool = match is_completed_str {
        "1" => true,
        "0" | "" => false,
        _ => anyhow::bail!("Invalid is_completed value: {}", is_completed_str),
    };
    let sort_order: i32 = record
        .get(4)
        .ok_or_else(|| anyhow::anyhow!("Missing sort_order field"))?
        .parse()
        .context("Failed to parse step sort_order")?;
    let created_at: i64 = record
        .get(5)
        .ok_or_else(|| anyhow::anyhow!("Missing created_at field"))?
        .parse()
        .context("Failed to parse step created_at")?;

    Ok((id, todo_id, title, is_completed, sort_order, created_at))
}

/// 解析附件 CSV 记录
pub fn parse_attachment_csv(record: &csv::StringRecord) -> anyhow::Result<(i64, i64, String, String, i64, Option<String>, i64)> {
    let id: i64 = record
        .get(0)
        .ok_or_else(|| anyhow::anyhow!("Missing id field"))?
        .parse()
        .context("Failed to parse attachment id")?;
    let todo_id: i64 = record
        .get(1)
        .ok_or_else(|| anyhow::anyhow!("Missing todo_id field"))?
        .parse()
        .context("Failed to parse attachment todo_id")?;
    let name: String = record
        .get(2)
        .ok_or_else(|| anyhow::anyhow!("Missing name field"))?
        .to_string();
    let file_path: String = record
        .get(3)
        .ok_or_else(|| anyhow::anyhow!("Missing file_path field"))?
        .to_string();
    let file_size: i64 = record
        .get(4)
        .ok_or_else(|| anyhow::anyhow!("Missing file_size field"))?
        .parse()
        .context("Failed to parse attachment file_size")?;
    let mime_type: Option<String> = record
        .get(5)
        .and_then(|s| if s.is_empty() { None } else { Some(s.to_string()) });
    let created_at: i64 = record
        .get(6)
        .ok_or_else(|| anyhow::anyhow!("Missing created_at field"))?
        .parse()
        .context("Failed to parse attachment created_at")?;

    Ok((id, todo_id, name, file_path, file_size, mime_type, created_at))
}

