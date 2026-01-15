// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

//! 应用程序级命令
//! 处理全局快捷键、窗口显示/隐藏、托盘等功能

use tauri::{AppHandle, State, Window};
use tauri_plugin_global_shortcut::GlobalShortcutExt;
use std::sync::Mutex;

/// 应用程序状态
pub struct AppState {
    pub hide_hotkey: Mutex<Option<String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            hide_hotkey: Mutex::new(None),
        }
    }
}

/// 设置全局快捷键
#[tauri::command]
pub async fn set_global_shortcut(
    shortcut: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    tracing::info!("set_global_shortcut called with: {}", shortcut);

    // 获取全局快捷键管理器
    let shortcut_manager = app.global_shortcut();

    // 如果之前有注册的快捷键，先取消注册
    let mut current_hotkey = state.hide_hotkey.lock().map_err(|e| format!("Lock error: {}", e))?;
    if let Some(old_shortcut) = current_hotkey.take() {
        // 使用字符串检查和取消注册
        if shortcut_manager.is_registered(old_shortcut.as_str()) {
            shortcut_manager
                .unregister(old_shortcut.as_str())
                .map_err(|e| format!("Failed to unregister old shortcut: {}", e))?;
            tracing::info!("Unregistered old shortcut: {}", old_shortcut);
        }
    }

    // 注册新的快捷键 - 使用字符串切片
    shortcut_manager
        .register(shortcut.as_str())
        .map_err(|e| format!("Failed to register shortcut: {}", e))?;

    // 保存新的快捷键
    *current_hotkey = Some(shortcut.clone());

    tracing::info!("Registered new shortcut: {}", shortcut);

    Ok(())
}

/// 获取当前注册的全局快捷键
#[tauri::command]
pub async fn get_global_shortcut(
    state: State<'_, AppState>,
) -> Result<Option<String>, String> {
    let hotkey = state.hide_hotkey.lock().map_err(|e| format!("Lock error: {}", e))?;
    Ok(hotkey.clone())
}

/// 显示/隐藏窗口
#[tauri::command]
pub async fn toggle_window_visibility(window: Window) -> Result<(), String> {
    if window.is_visible().map_err(|e| format!("Failed to check visibility: {}", e))? {
        window.hide().map_err(|e| format!("Failed to hide window: {}", e))?;
        tracing::info!("Window hidden");
    } else {
        window.show().map_err(|e| format!("Failed to show window: {}", e))?;
        window.set_focus().map_err(|e| format!("Failed to focus window: {}", e))?;
        tracing::info!("Window shown and focused");
    }
    Ok(())
}

/// 显示窗口
#[tauri::command]
pub async fn show_window(window: Window) -> Result<(), String> {
    window.show().map_err(|e| format!("Failed to show window: {}", e))?;
    window.set_focus().map_err(|e| format!("Failed to focus window: {}", e))?;
    tracing::info!("Window shown and focused");
    Ok(())
}

/// 隐藏窗口
#[tauri::command]
pub async fn hide_window(window: Window) -> Result<(), String> {
    window.hide().map_err(|e| format!("Failed to hide window: {}", e))?;
    tracing::info!("Window hidden");
    Ok(())
}
