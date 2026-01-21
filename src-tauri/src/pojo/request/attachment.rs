// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use serde::Deserialize;

/// 创建附件请求
#[derive(Debug, Deserialize)]
pub struct CreateAttachmentRequest {
    pub todo_id: i64,
    pub file_path: String,
    pub name: String,
}

/// 下载附件请求
#[derive(Debug, Deserialize)]
pub struct DownloadAttachmentRequest {
    pub attachment_id: i64,
    pub target_path: String,
}
