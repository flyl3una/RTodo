# RTodo - 跨平台待办事项应用

基于 Tauri + Vue 3 + Element Plus + TailwindCSS 构建的跨平台 Todo 清单应用。

## 技术栈

### 后端
- **Rust** - 系统编程语言
- **Tauri 2.0** - 跨平台桌面应用框架
- **SQLite** - 嵌入式数据库
- **Tokio** - 异步运行时

### 前端
- **Vue 3** - 渐进式 JavaScript 框架
- **TypeScript** - 类型安全的 JavaScript
- **Vite** - 前端构建工具
- **Element Plus** - Vue 3 UI 组件库
- **TailwindCSS** - CSS 框架
- **Pinia** - Vue 状态管理

## 功能特性

### 核心功能
- ✅ 任务 CRUD（创建、读取、更新、删除）
- ✅ 任务状态管理（未开始、进行中、已完成）
- ✅ 任务组管理（支持嵌套分组）
- ✅ 标签系统（多标签支持）
- ✅ 优先级设置（普通、重要、紧急）
- ✅ 重要标记
- ✅ 分配人管理
- ✅ 时间范围（开始时间、截止时间）
- ✅ 执行步骤（子任务）
- ✅ 文件附件
- ✅ 搜索和筛选
- ✅ 归纳统计（按天/周/月）

### 待实现
- ⏳ Todo CRUD 完整实现
- ⏳ 任务组管理界面
- ⏳ 标签管理界面
- ⏳ 统计图表可视化
- ⏳ 导入导出功能

## 项目结构

```
rtodo/
├── docs/                    # 设计文档和任务清单
│   ├── README.md           # 文档索引
│   ├── design.md           # 界面设计方案
│   └── tasks.md            # 开发任务清单
├── frontend/               # Vue 前端
│   ├── src/
│   │   ├── api/           # Tauri API 封装
│   │   ├── assets/        # 静态资源
│   │   ├── components/    # Vue 组件
│   │   ├── router/        # 路由配置
│   │   ├── stores/        # Pinia stores
│   │   ├── types/         # TypeScript 类型
│   │   ├── views/         # 页面组件
│   │   ├── App.vue        # 根组件
│   │   └── main.ts        # 入口
│   ├── package.json
│   └── vite.config.ts
└── src-tauri/              # Rust 后端
    ├── src/
    │   ├── commands/      # Tauri 命令
    │   ├── database/      # 数据库层
    │   ├── models/        # 数据模型
    │   ├── utils/         # 工具函数
    │   └── main.rs        # 入口
    ├── icons/             # 应用图标
    ├── capabilities/      # Tauri 权限配置
    ├── Cargo.toml
    └── tauri.conf.json
```

## 开发指南

### 环境要求

- **Rust**: >= 1.70
- **Node.js**: >= 18
- **pnpm**: >= 8

### 安装依赖

```bash
# 安装前端依赖
cd frontend
pnpm install

# Rust 依赖会在首次构建时自动安装
```

### 开发模式

```bash
# 启动开发服务器（前端 + Tauri）
cargo tauri dev
```

这将启动：
1. Vite 开发服务器（http://localhost:1420）
2. Tauri 开发窗口

### 构建

```bash
# 构建生产版本
cargo tauri build

# 制作安装包
python -m build/build 
或者
python build-cli -p windows/macos -a x86_64
```

### 运行测试

```bash
# Rust 单元测试
cd src-tauri
cargo test

# Rust 代码检查
cargo clippy

# 前端测试（待配置）
cd frontend
pnpm test
```

## 数据库

SQLite 数据库文件位置：
- **Windows**: `%APPDATA%\rtodo\rtodo.db`
- **macOS**: `~/Library/Application Support/rtodo/rtodo.db`
- **Linux**: `~/.local/share/rtodo/rtodo.db`

### 数据表结构

- `task_groups` - 任务组（支持嵌套）
- `tags` - 标签
- `todos` - 任务
- `todo_tags` - 任务-标签关联（多对多）
- `todo_steps` - 执行步骤
- `attachments` - 附件
- `export_history` - 导出历史

## 设计文档

详细的设计文档请查看 [docs/](./docs/) 目录：

- **[design.md](./docs/design.md)** - 完整的界面设计方案
- **[tasks.md](./docs/tasks.md)** - 开发任务清单和里程碑


## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！

## 当前状态

- [x] 项目结构搭建
- [x] 数据库 schema 设计
- [x] Rust 数据模型定义
- [x] Tauri Commands 框架
- [x] 前端基础框架
- [x] Todo CRUD 实现
- [x] 前端状态管理
- [x] UI 组件开发
- [x] 统计功能
- [x] 导入导出
