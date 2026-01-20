// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

//! 应用配置管理
//! 负责加载和保存应用配置到文件系统

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;
use tauri::AppHandle;

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 开机启动是否启用
    #[serde(default = "default_auto_launch")]
    pub auto_launch: bool,
    /// 关闭行为
    #[serde(default = "default_close_behavior")]
    pub close_behavior: String,
    /// 全局快捷键
    #[serde(default)]
    pub global_shortcut: Option<String>,
    /// 自定义数据路径（可选，为空时使用默认路径）
    #[serde(default)]
    pub data_path: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            auto_launch: false,
            close_behavior: "direct".to_string(),
            global_shortcut: None,
            data_path: None,
        }
    }
}

fn default_auto_launch() -> bool {
    false
}

fn default_close_behavior() -> String {
    "direct".to_string()
}

impl AppConfig {
    /// 获取配置文件路径
    pub fn config_path(app: &AppHandle) -> Result<PathBuf, String> {
        let config_dir = app.path().app_config_dir()
            .map_err(|e| format!("Failed to get config dir: {}", e))?;

        // 确保配置目录存在
        fs::create_dir_all(&config_dir)
            .map_err(|e| format!("Failed to create config dir: {}", e))?;

        Ok(config_dir.join("config.json"))
    }

    /// 加载配置
    pub fn load(app: &AppHandle) -> Result<Self, String> {
        let config_path = Self::config_path(app)?;

        if !config_path.exists() {
            tracing::info!("Config file not found, using defaults");
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        let config: AppConfig = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse config file: {}", e))?;

        tracing::info!("Loaded config from file: {:?}", config);
        Ok(config)
    }

    /// 保存配置
    pub fn save(&self, app: &AppHandle) -> Result<(), String> {
        let config_path = Self::config_path(app)?;

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(&config_path, content)
            .map_err(|e| format!("Failed to write config file: {}", e))?;

        tracing::info!("Saved config to file: {:?}", self);
        Ok(())
    }

    /// 更新开机启动设置并保存
    pub fn update_auto_launch(&mut self, enabled: bool, app: &AppHandle) -> Result<(), String> {
        self.auto_launch = enabled;
        self.save(app)?;
        Ok(())
    }

    /// 更新关闭行为并保存
    pub fn update_close_behavior(&mut self, behavior: &str, app: &AppHandle) -> Result<(), String> {
        self.close_behavior = behavior.to_string();
        self.save(app)?;
        Ok(())
    }

    /// 更新全局快捷键并保存
    pub fn update_global_shortcut(&mut self, shortcut: Option<String>, app: &AppHandle) -> Result<(), String> {
        self.global_shortcut = shortcut;
        self.save(app)?;
        Ok(())
    }
}
