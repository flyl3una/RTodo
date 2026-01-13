# RTodo 开发任务清单

## 当前进度

| 阶段 | 状态 | 说明 |
|------|------|------|
| 项目初始化 | ✅ 已完成 | 创建项目结构，安装依赖 |
| 数据库层 | 🔄 进行中 | Schema 完成，Todo Repository 完成 |
| 后端 API | 🔄 进行中 | Todo Commands 完成 |
| 前端框架 | ⏳ 待开始 | 路由、布局、状态管理 |
| 核心功能 | ⏳ 待开始 | 任务 CRUD、任务组、标签 |
| 高级功能 | ⏳ 待开始 | 步骤、附件、统计 |
| 测试优化 | ⏳ 待开始 | 单元测试、性能优化 |

---

## 详细任务列表

### 阶段 1：项目初始化 ✅

- [x] 创建项目目录结构 (frontend, src-tauri, docs)
- [x] 创建前端 package.json
- [x] 创建 vite.config.ts
- [x] 创建 tailwind.config.js
- [x] 创建 tsconfig.json
- [ ] 安装前端依赖 (pnpm install)
- [x] 创建 Rust 项目结构 (src-tauri)
- [x] 配置 Cargo.toml (workspace)
- [x] 安装 Rust 依赖
- [x] 生成应用图标
- [x] 项目编译成功

### 阶段 2：数据库层 🔄

- [x] 创建 database/schema.rs (SQLite 表结构)
- [x] 创建 database/connection.rs (数据库连接)
- [x] 创建 database/repositories/todo_repo.rs
- [ ] 创建 database/repositories/group_repo.rs
- [ ] 创建 database/repositories/tag_repo.rs
- [ ] 创建 database/repositories/step_repo.rs
- [ ] 创建 database/repositories/attachment_repo.rs
- [x] 实现数据库初始化逻辑
- [ ] 实现数据库迁移机制

### 阶段 3：Rust 数据模型 ✅

- [x] 创建 models/mod.rs
- [x] 创建 models/todo.rs
- [x] 创建 models/group.rs
- [x] 创建 models/tag.rs
- [x] 创建 models/step.rs
- [x] 创建 models/attachment.rs
- [x] 实现 Serialize/Deserialize trait
- [x] 实现 TodoStatus 等枚举

### 阶段 4：Tauri Commands 🔄

- [x] 创建 commands/mod.rs (注册所有命令)
- [x] 实现 commands/todo_commands.rs (Todo CRUD)
  - [x] get_todos
  - [x] get_todo
  - [x] create_todo
  - [x] update_todo
  - [x] delete_todo
  - [x] update_todo_status
  - [x] toggle_todo_mark
- [ ] 实现 commands/group_commands.rs (任务组)
  - [ ] get_task_groups
  - [ ] create_task_group
  - [ ] update_task_group
  - [ ] delete_task_group
- [ ] 实现 commands/tag_commands.rs (标签)
  - [ ] get_tags
  - [ ] create_tag
  - [ ] update_tag
  - [ ] delete_tag
- [ ] 实现 commands/step_commands.rs (步骤)
  - [ ] get_todo_steps
  - [ ] create_step
  - [ ] toggle_step
  - [ ] delete_step
- [ ] 实现 commands/attachment_commands.rs (附件)
  - [ ] get_attachments
  - [ ] upload_attachment
  - [ ] delete_attachment
- [ ] 实现 commands/stats_commands.rs (统计)
  - [ ] get_stats
  - [ ] get_stats_by_date
- [ ] 实现 commands/import_export.rs
  - [ ] export_all_data
  - [ ] import_data
  - [ ] clear_all_data

### 阶段 5：前端基础框架 ✅

- [x] 创建 frontend/src/main.ts
- [x] 创建 frontend/src/App.vue
- [x] 创建 frontend/src/index.html
- [x] 创建 frontend/src/assets/styles/main.css
- [x] 配置 Vue Router (router/index.ts)
- [ ] 配置 Pinia (stores/index.ts)
- [x] 创建 TypeScript 类型定义 (types/index.ts)

### 阶段 6：前端 API 层

- [ ] 创建 api/tauri.ts (Tauri API 封装)
- [ ] 创建 api/todo.ts
- [ ] 创建 api/group.ts
- [ ] 创建 api/tag.ts
- [ ] 创建 api/step.ts
- [ ] 创建 api/attachment.ts
- [ ] 创建 api/stats.ts
- [ ] 创建 api/index.ts

### 阶段 7：前端状态管理

- [ ] 创建 stores/todo.ts (Todo Store)
- [ ] 创建 stores/group.ts (Group Store)
- [ ] 创建 stores/tag.ts (Tag Store)
- [ ] 创建 stores/ui.ts (UI Store - 侧边栏、主题等)

### 阶段 8：前端布局组件

- [ ] 创建 components/layout/Sidebar.vue
  - [ ] 快速访问区域
  - [ ] 任务组树形列表
  - [ ] 标签列表
  - [ ] 收起/展开功能
- [ ] 创建 components/layout/Header.vue (菜单栏)
- [ ] 创建 components/layout/Footer.vue (状态栏)
- [ ] 创建 components/layout/MainLayout.vue (主布局)

### 阶段 9：前端任务组件

- [ ] 创建 components/todo/TodoList.vue
- [ ] 创建 components/todo/TodoItem.vue (列表视图)
- [ ] 创建 components/todo/TodoCard.vue (卡片视图)
- [ ] 创建 components/todo/TodoDetailPanel.vue
- [ ] 创建 components/todo/CreateTodoDialog.vue
- [ ] 创建 components/todo/QuickAddDialog.vue

### 阶段 10：前端任务组组件

- [ ] 创建 components/group/GroupItem.vue
- [ ] 创建 components/group/GroupManager.vue
- [ ] 创建 components/group/GroupTree.vue

### 阶段 11：前端标签组件

- [ ] 创建 components/tag/TagSelector.vue
- [ ] 创建 components/tag/TagManager.vue
- [ ] 创建 components/tag/TagList.vue

### 阶段 12：前端其他组件

- [ ] 创建 components/StepEditor.vue
- [ ] 创建 components/DateTimePicker.vue
- [ ] 创建 components/AttachmentUploader.vue

### 阶段 13：前端页面

- [ ] 创建 views/HomeView.vue (主页面)
- [ ] 创建 views/StatsView.vue (统计页面)
  - [ ] 概览卡片
  - [ ] 完成趋势图表
  - [ ] 任务组分布
  - [ ] 标签使用统计
- [ ] 创建 views/SettingsView.vue (设置页面)
  - [ ] 外观设置
  - [ ] 数据管理
  - [ ] 快捷键
  - [ ] 关于
- [ ] 创建 views/AboutView.vue

### 阶段 14：Tauri 配置

- [ ] 创建 tauri.conf.json
- [ ] 配置应用图标
- [ ] 配置窗口大小和行为
- [ ] 配置系统托盘（可选）
- [ ] 配置自动更新（可选）

### 阶段 15：测试

- [ ] Rust 单元测试 (database, models)
- [ ] Tauri 命令集成测试
- [ ] 前端组件测试
- [ ] 端到端测试
- [ ] 性能测试

### 阶段 16：优化与完善

- [ ] 虚拟滚动优化（大列表）
- [ ] 搜索防抖优化
- [ ] 错误处理完善
- [ ] 加载状态优化
- [ ] 动画效果优化
- [ ] 主题切换实现
- [ ] 响应式布局适配
- [ ] 快捷键支持

---

## 里程碑

| 里程碑 | 目标 | 预计完成 |
|--------|------|----------|
| M1 | 项目可运行 | 阶段 1-5 完成 |
| M2 | 基础功能可用 | 阶段 6-11 完成 |
| M3 | 核心功能完整 | 阶段 12-14 完成 |
| M4 | 生产就绪 | 阶段 15-16 完成 |

---

## 备注

- 遵循编码规范：不使用 unwrap、UTF-8 编码、完善错误处理
- 所有关键文件需添加单元测试
- 提交前确保代码通过 cargo clippy 和前端 lint 检查
