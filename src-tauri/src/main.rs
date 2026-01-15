// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Manager,
};

mod commands;
mod database;
mod models;
mod utils;
mod logging;

use database::Database;
use logging::{load_config, init_logging};
use commands::app_commands::AppState;

#[tokio::main]
async fn main() {
    // 加载日志配置
    let log_config = load_config();

    // 初始化日志系统并获取 guards
    // _log_guards 必须在程序整个生命周期中保持存活，否则日志后台线程会停止
    let _log_guards = init_logging(&log_config).expect("Failed to initialize logging");

    tracing::info!("RTodo application starting...");

    // 构建应用
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            tracing::info!("Setting up application...");

            // 初始化应用状态
            app.manage(AppState::new());

            // 初始化数据库连接池
            let db = Database::new().expect("Failed to initialize database");
            app.manage(db);

            // 创建托盘图标菜单
            let show_item = MenuItem::with_id(app, "show", "显示", true, None::<&str>).unwrap();
            let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>).unwrap();
            let menu = Menu::with_items(app, &[&show_item, &quit_item]).unwrap();

            // 创建托盘图标
            let _tray = TrayIconBuilder::new()
                .icon_as_template(true)
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_tray_icon_event(|tray, event| {
                    match event {
                        TrayIconEvent::Click {
                            button: MouseButton::Left,
                            ..
                        } => {
                            tracing::info!("Tray left click");
                            if let Some(window) = tray.app_handle().get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        TrayIconEvent::DoubleClick {
                            button: MouseButton::Left,
                            ..
                        } => {
                            tracing::info!("Tray left double click");
                            if let Some(window) = tray.app_handle().get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        TrayIconEvent::Click {
                            button: MouseButton::Right,
                            ..
                        } => {
                            tracing::info!("Tray right click - show menu");
                        }
                        _ => {}
                    }
                })
                .build(app)?;

            // 处理托盘菜单事件
            app.on_menu_event(|app, event| {
                match event.id.0.as_str() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        tracing::info!("Quit requested from tray menu");
                        app.exit(0);
                    }
                    _ => {}
                }
            });

            tracing::info!("Application setup completed successfully");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::todo_commands::get_todos,
            commands::todo_commands::get_todo,
            commands::todo_commands::create_todo,
            commands::todo_commands::update_todo,
            commands::todo_commands::delete_todo,
            commands::todo_commands::update_todo_status,
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
            commands::data_manager_command::export_all_data,
            commands::data_manager_command::import_data,
            commands::data_manager_command::export_data_as_csv,
            commands::data_manager_command::import_data_from_csv,
            commands::data_manager_command::clear_all_data,
            commands::app_commands::set_global_shortcut,
            commands::app_commands::get_global_shortcut,
            commands::app_commands::toggle_window_visibility,
            commands::app_commands::show_window,
            commands::app_commands::hide_window,
        ])
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                // 阻止窗口关闭，改为隐藏
                api.prevent_close();
                let _ = window.hide();
                tracing::info!("Window close requested - hiding instead");
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
