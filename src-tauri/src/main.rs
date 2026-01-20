// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

// Release 模式下隐藏控制台窗口，Debug 模式保留控制台用于调试
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{Menu, MenuItem, CheckMenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Manager,
    Emitter,
    include_image,
};
use tauri_plugin_dialog::DialogExt;

mod commands;
mod database;
mod models;
mod utils;
mod logging;
mod config;

use database::Database;
use logging::{load_config, init_logging};
use commands::app_commands::AppState;
use config::AppConfig;

/// 辅助函数：恢复托盘菜单中开机启动选项的状态
fn restore_autolaunch_menu_item(app: &tauri::AppHandle, checked: bool) {
    if let Some(menu) = app.menu() {
        if let Some(item) = menu.get("autolaunch") {
            if let Some(check_item) = item.as_check_menuitem() {
                let _ = check_item.set_checked(checked);
            }
        }
    }
}

/// 检查系统的开机启动状态
fn check_auto_launch_enabled() -> bool {
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::*;
        use winreg::RegKey;

        let app_path = match std::env::current_exe() {
            Ok(path) => path,
            Err(_) => return false,
        };

        let app_path_str = app_path.to_string_lossy().to_string();

        // 检查当前用户的启动项
        let hklm = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok(run_key) = hklm.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Run", KEY_READ) {
            if let Ok(value) = run_key.get_value::<String, _>("RTodo") {
                return value == app_path_str || value.replace('\\', "/") == app_path_str.replace('\\', "/");
            }
        }
        false
    }

    #[cfg(not(target_os = "windows"))]
    {
        use auto_launch::AutoLaunchBuilder;

        let app_name = "RTodo";
        let app_path = match std::env::current_exe() {
            Ok(path) => path,
            Err(_) => return false,
        };

        if let Ok(auto_launch) = AutoLaunchBuilder::new()
            .set_app_name(app_name)
            .set_app_path(&app_path.to_string_lossy())
            .build()
        {
            auto_launch.is_enabled().unwrap_or(false)
        } else {
            false
        }
    }
}

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

            // 加载应用配置
            let config = AppConfig::load(&app.handle())
                .unwrap_or_else(|e| {
                    tracing::warn!("Failed to load config: {}, using defaults", e);
                    AppConfig::default()
                });

            // 查询系统实际的开机启动状态
            let system_enabled = check_auto_launch_enabled();
            tracing::info!("System auto_launch status: {}", system_enabled);

            // 初始化应用状态（使用配置中的值，但以系统实际状态为准）
            let close_behavior = match config.close_behavior.as_str() {
                "minimize_to_tray" => commands::app_commands::CloseBehavior::MinimizeToTray,
                _ => commands::app_commands::CloseBehavior::Direct,
            };

            let app_state = AppState {
                hide_hotkey: std::sync::Mutex::new(config.global_shortcut.clone()),
                close_behavior: std::sync::Mutex::new(close_behavior),
                auto_launch: std::sync::Mutex::new(system_enabled),
            };
            app.manage(app_state);

            // 更新配置中的开机启动状态（与系统保持同步）
            let mut config_to_save = config;
            config_to_save.auto_launch = system_enabled;

            // 保存配置到 app manage 中，方便后续访问
            app.manage(std::sync::Mutex::new(config_to_save));

            // 初始化数据库连接池
            let db = Database::new().expect("Failed to initialize database");
            app.manage(db);

            // 创建托盘图标菜单（使用系统实际状态作为初始值）
            let show_item = MenuItem::with_id(app, "show", "显示", true, None::<&str>).unwrap();
            let autolaunch_item = CheckMenuItem::with_id(
                app,
                "autolaunch",
                "开机启动",
                true,
                system_enabled,
                None::<&str>
            ).unwrap();
            let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>).unwrap();
            let menu = Menu::with_items(app, &[&show_item, &autolaunch_item, &quit_item]).unwrap();

            // 创建托盘图标
            let tray_icon = include_image!("icons/tray-icon.png");
            let tray = TrayIconBuilder::new()
                .icon(tray_icon)
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
                    "autolaunch" => {
                        // CheckMenuItem 会自动切换复选框状态
                        // 获取复选框的新状态（点击后的状态）
                        let new_state = if let Some(menu) = app.menu() {
                            if let Some(item) = menu.get("autolaunch") {
                                if let Some(check_item) = item.as_check_menuitem() {
                                    check_item.is_checked().unwrap_or(false)
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        };

                        // 获取 AppHandle 和 AppState
                        let app_handle = app.clone();

                        // 使用 async runtime 调用 set_auto_launch 命令
                        tauri::async_runtime::spawn(async move {
                            use crate::commands::app_commands;

                            // 获取 AppState
                            let state = match app_handle.try_state::<commands::app_commands::AppState>() {
                                Some(s) => s,
                                None => {
                                    tracing::error!("Failed to get AppState");
                                    let _ = restore_autolaunch_menu_item(&app_handle, !new_state);
                                    return;
                                }
                            };

                            match app_commands::set_auto_launch(new_state, app_handle.clone(), state).await {
                                Ok(_) => {
                                    tracing::info!("Auto-launch toggled to: {}", new_state);
                                }
                                Err(e) => {
                                    tracing::error!("Failed to toggle auto launch: {}", e);

                                    // 恢复托盘菜单状态
                                    let _ = restore_autolaunch_menu_item(&app_handle, !new_state);

                                    // 显示错误并提供 UAC 提示
                                    let error_msg = if cfg!(target_os = "windows") {
                                        format!("切换开机启动失败: {}\n\nWindows 可能需要管理员权限才能设置开机启动。\n\n建议:\n1. 右键点击应用图标，选择\"以管理员身份运行\"\n2. 或在设置页面中尝试操作", e)
                                    } else if cfg!(target_os = "macos") {
                                        format!("切换开机启动失败: {}\n\nmacOS 需要输入密码才能设置开机启动。\n\n建议在设置页面中尝试操作", e)
                                    } else {
                                        format!("切换开机启动失败: {}\n\n可能需要管理员权限。\n\n建议在设置页面中尝试操作", e)
                                    };

                                    let _ = app_handle.dialog().message(error_msg).show(|_dialog| ());

                                    // 发送事件到前端更新 UI
                                    let _ = app_handle.emit("autolaunch-changed", !new_state);
                                }
                            }
                        });
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
            commands::step_commands::update_step,
            commands::step_commands::delete_step,
            commands::attachment_commands::get_attachments,
            commands::attachment_commands::upload_attachment,
            commands::attachment_commands::delete_attachment,
            commands::attachment_commands::open_attachment,
            commands::attachment_commands::download_attachment,
            commands::data_path_commands::get_data_path,
            commands::data_path_commands::check_directory_empty,
            commands::data_path_commands::set_data_path,
            commands::data_path_commands::reset_data_path,
            commands::data_path_commands::migrate_data,
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
            commands::app_commands::set_close_behavior,
            commands::app_commands::get_close_behavior,
            commands::app_commands::set_auto_launch,
            commands::app_commands::get_auto_launch,
            commands::app_commands::toggle_auto_launch,
        ])
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                // 检查关闭行为设置
                let app_handle = window.app_handle();
                if let Some(state) = app_handle.try_state::<commands::app_commands::AppState>() {
                    let close_behavior = state.close_behavior.lock().unwrap();
                    match *close_behavior {
                        commands::app_commands::CloseBehavior::Direct => {
                            // 直接关闭，不做任何处理
                            tracing::info!("Window close requested - closing directly");
                        }
                        commands::app_commands::CloseBehavior::MinimizeToTray => {
                            // 阻止窗口关闭，改为隐藏到托盘
                            api.prevent_close();
                            let _ = window.hide();
                            tracing::info!("Window close requested - hiding to tray");
                        }
                    }
                } else {
                    // 默认行为：隐藏到托盘（保持向后兼容）
                    api.prevent_close();
                    let _ = window.hide();
                    tracing::info!("Window close requested - hiding to tray (default)");
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
