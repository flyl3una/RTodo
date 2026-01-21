// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

//! 请求对象模块

pub mod todo;
pub mod group;
pub mod tag;
pub mod step;
pub mod attachment;
pub mod stats;
pub mod data_path;

// 重新导出常用类型
pub use todo::{CreateTodoRequest, UpdateTodoRequest, UpdateTodoStatusRequest, GetTodosRequest};
pub use group::{CreateGroupRequest, UpdateGroupRequest};
pub use tag::{CreateTagRequest, UpdateTagRequest};
pub use step::{CreateStepRequest, UpdateStepRequest};
pub use attachment::{CreateAttachmentRequest, DownloadAttachmentRequest};
pub use stats::{GetStatsRequest, GetStatsByDateRequest, GetStatsWithDetailsRequest};
pub use data_path::MigrateDataRequest;
