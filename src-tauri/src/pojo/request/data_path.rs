// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use serde::Deserialize;

/// 迁移数据请求
#[derive(Debug, Deserialize)]
pub struct MigrateDataRequest {
    pub new_path: String,
    pub keep_original: bool,
}
