// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use tauri::Manager;

mod commands;
mod database;
mod models;
mod utils;

use database::Database;

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 构建应用
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // 初始化数据库连接池
            let db = Database::new().expect("Failed to initialize database");
            app.manage(db);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::todo_commands::get_todos,
            commands::todo_commands::get_todo,
            commands::todo_commands::create_todo,
            commands::todo_commands::update_todo,
            commands::todo_commands::delete_todo,
            commands::todo_commands::update_todo_status,
            commands::todo_commands::toggle_todo_mark,
            commands::group_commands::get_task_groups,
            commands::group_commands::create_task_group,
            commands::group_commands::update_task_group,
            commands::group_commands::delete_task_group,
            commands::tag_commands::get_tags,
            commands::tag_commands::create_tag,
            commands::tag_commands::update_tag,
            commands::tag_commands::delete_tag,
            commands::step_commands::get_todo_steps,
            commands::step_commands::create_step,
            commands::step_commands::toggle_step,
            commands::step_commands::delete_step,
            commands::attachment_commands::get_attachments,
            commands::attachment_commands::upload_attachment,
            commands::attachment_commands::delete_attachment,
            commands::stats_commands::get_stats,
            commands::stats_commands::get_stats_by_date,
            commands::stats_commands::get_stats_with_details,
            commands::import_export::export_all_data,
            commands::import_export::import_data,
            commands::import_export::clear_all_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
