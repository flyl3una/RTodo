// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

//! 应用程序级命令
//! 处理全局快捷键、窗口显示/隐藏、托盘等功能

use tauri::{AppHandle, Emitter, Manager, State, Window};
use tauri_plugin_global_shortcut::GlobalShortcutExt;
use std::sync::Mutex;

/// 关闭行为选项
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloseBehavior {
    /// 直接关闭
    Direct,
    /// 最小化到托盘
    MinimizeToTray,
}

impl CloseBehavior {
    pub fn as_str(&self) -> &'static str {
        match self {
            CloseBehavior::Direct => "direct",
            CloseBehavior::MinimizeToTray => "minimize_to_tray",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "direct" => Some(CloseBehavior::Direct),
            "minimize_to_tray" => Some(CloseBehavior::MinimizeToTray),
            _ => None,
        }
    }
}

/// 应用程序状态
pub struct AppState {
    pub hide_hotkey: Mutex<Option<String>>,
    pub close_behavior: Mutex<CloseBehavior>,
    pub auto_launch: Mutex<bool>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            hide_hotkey: Mutex::new(None),
            close_behavior: Mutex::new(CloseBehavior::Direct), // 默认直接关闭
            auto_launch: Mutex::new(false), // 默认不开启开机启动
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

    // 保存新的快捷键到内存
    *current_hotkey = Some(shortcut.clone());

    // 保存到配置文件
    if let Some(config_state) = app.try_state::<std::sync::Mutex<crate::config::AppConfig>>() {
        if let Ok(mut config) = config_state.lock() {
            let shortcut_to_save = Some(shortcut.clone());
            config.update_global_shortcut(shortcut_to_save, &app)?;
            tracing::info!("Saved global shortcut to config: {}", shortcut);
        }
    }

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
pub async fn toggle_window_visibility(window: tauri::Window) -> Result<(), String> {
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

/// 显示/隐藏窗口 (WebviewWindow 版本)
pub async fn toggle_webview_window_visibility(
    window: tauri::WebviewWindow,
) -> Result<(), String> {
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

/// 设置关闭行为
#[tauri::command]
pub async fn set_close_behavior(
    behavior: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let close_behavior = CloseBehavior::from_str(behavior.as_str())
        .ok_or_else(|| format!("Invalid close behavior: {}", behavior))?;

    let mut current_behavior = state.close_behavior.lock().map_err(|e| format!("Lock error: {}", e))?;
    *current_behavior = close_behavior;

    tracing::info!("Close behavior set to: {:?}", close_behavior);
    Ok(())
}

/// 获取关闭行为
#[tauri::command]
pub async fn get_close_behavior(
    state: State<'_, AppState>,
) -> Result<String, String> {
    let behavior = state.close_behavior.lock().map_err(|e| format!("Lock error: {}", e))?;
    Ok(behavior.as_str().to_string())
}

/// 设置开机启动
#[tauri::command]
pub async fn set_auto_launch(
    enabled: bool,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    use crate::config::AppConfig;

    let mut current = state.auto_launch.lock().map_err(|e| format!("Lock error: {}", e))?;

    // 获取应用名称和路径
    let app_name = app.config().product_name.as_ref()
        .ok_or_else(|| format!("Product name not configured"))?;
    let app_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get exe path: {}", e))?;
    let app_path_str = app_path.to_string_lossy().to_string();

    // Windows: 直接操作注册表设置开机启动
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::*;
        use winreg::RegKey;

        let hklm = RegKey::predef(HKEY_CURRENT_USER);
        let run_key = hklm.open_subkey_with_flags(
            "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
            KEY_WRITE
        ).map_err(|e| format!("无法访问注册表，可能需要管理员权限: {}", e))?;

        if enabled {
            run_key.set_value(app_name, &app_path_str)
                .map_err(|e| format!("设置开机启动失败: {}", e))?;
        } else {
            // 尝试删除值，如果不存在则忽略
            let _ = run_key.delete_value(app_name);
        }
    }

    // 非Windows平台使用auto_launch crate
    #[cfg(not(target_os = "windows"))]
    {
        use auto_launch::AutoLaunchBuilder;

        let auto_launch = AutoLaunchBuilder::new()
            .set_app_name(app_name)
            .set_app_path(&app_path_str)
            .build()
            .map_err(|e| format!("Failed to build auto launch: {}", e))?;

        let result = if enabled {
            auto_launch.enable()
        } else {
            auto_launch.disable()
        };

        result.map_err(|e| format!("Failed to set auto launch: {}", e))?;
    }

    *current = enabled;
    tracing::info!("Auto-launch set to: {}", enabled);

    // 保存到配置文件
    if let Some(config_state) = app.try_state::<std::sync::Mutex<AppConfig>>() {
        if let Ok(mut config) = config_state.lock() {
            let _ = config.update_auto_launch(enabled, &app);
            tracing::info!("Saved auto_launch config to file");
        }
    }

    // 更新托盘菜单状态（在主线程中执行）
    let app_for_menu = app.clone();
    app.run_on_main_thread(move || {
        update_tray_autolaunch_menu(&app_for_menu, enabled);
    }).map_err(|e| format!("Failed to update tray menu: {}", e))?;

    // 发送事件给前端
    let _ = app.emit("autolaunch-changed", enabled);

    Ok(())
}

/// 更新托盘菜单中的开机启动选项状态
fn update_tray_autolaunch_menu(app: &AppHandle, enabled: bool) {
    if let Some(menu) = app.menu() {
        if let Some(item) = menu.get("autolaunch") {
            if let Some(check_item) = item.as_check_menuitem() {
                tracing::info!("Updating tray menu autolaunch to: {}", enabled);
                let _ = check_item.set_checked(enabled);
            }
        }
    }
}

/// 获取开机启动状态
#[tauri::command]
pub async fn get_auto_launch(state: State<'_, AppState>) -> Result<bool, String> {
    let enabled = state.auto_launch.lock().map_err(|e| format!("Lock error: {}", e))?;
    Ok(*enabled)
}

/// 切换开机启动
#[tauri::command]
pub async fn toggle_auto_launch(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    // 读取当前状态并立即释放锁
    let new_state = {
        let current_state = state.auto_launch.lock().map_err(|e| format!("Lock error: {}", e))?;
        !*current_state
    };

    set_auto_launch(new_state, app, state).await?;

    Ok(new_state)
}
