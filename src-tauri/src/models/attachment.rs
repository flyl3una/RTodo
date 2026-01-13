// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// 附件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub id: String,
    pub todo_id: String,
    pub name: String,
    pub file_path: String,
    pub file_size: i64,
    pub mime_type: Option<String>,
    pub created_at: i64,
}

/// 创建附件请求
#[derive(Debug, Deserialize)]
pub struct CreateAttachmentRequest {
    pub todo_id: String,
    pub file_path: String,
    pub name: String,
}
