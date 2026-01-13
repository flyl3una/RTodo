# RTodo 跨平台 Todo 清单应用 - 界面设计方案

## 项目概述

使用 Tauri + Vue 3 + Element Plus + TailwindCSS 构建跨平台 Todo 应用，支持 Web、桌面、移动端。

**技术选择（已确认）：**
- 初始化方式：使用 pnpm/cargo 重新初始化
- UI 组件库：Element Plus
- 数据存储：SQLite
- 同步机制：导入导出

---

## 一、整体界面布局

### 布局结构（三栏式）

```
┌─────────────────────────────────────────────────────────────────┐
│  菜单栏：文件 | 编辑 | 视图 | 帮助 | [设置图标]                  │
├──────────┬──────────────────────────────────────┬───────────────┤
│ 侧边栏   │         主内容区 (任务列表)          │   详情面板     │
│          │                                      │               │
│ 📁 收件箱 │  🔍 搜索任务...                      │  任务详情      │
│ 📁 工作   │  筛选: [全部▼] [今天] [视图切换]     │  📌 标题      │
│  └─项目A │  ┌─────────────────────────────┐    │  📝 描述      │
│  └─项目B │  │ 📌 任务标题 | 📊 进行中|⭐ │    │  执行步骤     │
│ 📁 个人   │  │ 🏷️ 标签1 标签2 | 👤 张三    │    │  文件附件     │
│ 🏷️ 标签   │  └─────────────────────────────┘    │               │
│ #重要    │  ...更多任务...                      │               │
│ #紧急    │                                      │               │
│ 📊 统计   │                                      │               │
│ ⚙️ 设置   │                                      │               │
└──────────┴──────────────────────────────────────┴───────────────┘
│  状态栏: 15 个任务 | 3 个进行中 | 5 个已完成 | 2 个逾期        │
└─────────────────────────────────────────────────────────────────┘
```

### 页面划分

| 页面 | 路由 | 功能 |
|------|------|------|
| 主界面 | `/` | 三栏布局：侧边栏 + 任务列表 + 详情面板 |
| 统计页 | `/stats` | 按天/周/月统计，图表展示 |
| 设置页 | `/settings` | 主题、数据导入导出、快捷键 |
| 关于页 | `/about` | 应用信息 |

### 响应式断点

- `< md`: 详情面板隐藏，点击任务打开全屏抽屉
- `< lg`: 侧边栏可收起为图标导航
- `>= xl`: 三栏完整布局

---

## 二、核心页面设计

### 2.1 侧边栏 (Sidebar.vue)

**功能模块：**
1. 收起/展开按钮
2. 快速访问：收件箱、今天、重要
3. 任务组树形列表（支持嵌套）
4. 标签列表
5. 底部导航：统计、设置

**关键交互：**
- 点击任务组/标签 → 过滤任务列表
- 右键 → 编辑/删除/添加子组
- 拖拽 → 任务组排序

### 2.2 任务列表页 (TodoListView.vue)

**顶部工具栏：**
- 搜索框（支持防抖）
- 状态筛选下拉
- 视图切换（列表/卡片）
- 新建任务按钮

**任务显示模式：**

**列表视图：**
```
☐ 📌 任务标题
   📊 进行中 | ⭐ 重要 | 🏷️ 标签 | 👤 张三 | 📅 今天
```

**卡片视图：**
```
┌────────────────────────────┐
│ ☐  📌 任务标题       ⭐    │
│ 描述预览...                │
│ 📊 进行中 | 🏷️ 工作        │
│ 📅 截止今天 | ✓ 2/3 步骤   │
└────────────────────────────┘
```

### 2.3 任务详情面板 (TodoDetailPanel.vue)

**布局：**
```
┌─────────────────────────────┐
│ ☐ 任务标题            ⭐ ✏️ 🗑️ │
│ 📊 进行中 | 重要 | 🏷️ 工作标签 │
├─────────────────────────────┤
│ 📝 描述                     │
│ 任务详细描述内容...          │
├─────────────────────────────┤
│ 📅 时间                     │
│ 开始: 2025-01-13 09:00      │
│ 截止: 2025-01-15 18:00      │
├─────────────────────────────┤
│ ✅ 执行步骤               [+添加]│
│ ☑ 步骤1                    │
│ ☐ 步骤2                    │
│ ████████░░ 80%             │
├─────────────────────────────┤
│ 📎 附件                  [上传]│
│ 📄 文档.pdf           [查看] [删除]│
│ 🖼️ 截图.png           [查看] [删除]│
└─────────────────────────────┘
```

### 2.4 归纳统计页 (StatsView.vue)

**布局：**

```
┌─────────────────────────────────┐
│ 任务统计          [按天|按周|按月]│
├─────────────────────────────────┤
│ ┌────┐ ┌────┐ ┌────┐ ┌────┐    │
│ │ 总数│ │进行│ │完成│ │逾期│    │
│ │ 15 │ │ 3  │ │ 5  │ │ 2  │    │
│ └────┘ └────┘ └────┘ └────┘    │
├─────────────────────────────────┤
│ 完成趋势                         │
│ 📈 折线图 (创建 vs 完成)        │
├─────────────────────────────────┤
│ 任务组分布                       │
│ 工作 ████████░░ 80% (8/10)      │
│ 个人 ██████░░░░ 60% (3/5)       │
├─────────────────────────────────┤
│ 标签使用                         │
│ [重要(8)] [紧急(3)] [工作(12)]  │
└─────────────────────────────────┘
```

### 2.5 设置页 (SettingsView.vue)

```
┌─────────────────────────────────┐
│ 设置                             │
├─────────────────────────────────┤
│ 外观                             │
│   主题: ○浅色 ○深色 ○跟随系统   │
│   字体: ────●──── (14px)        │
├─────────────────────────────────┤
│ 数据管理                         │
│   [导出所有数据]                 │
│   [导入数据]                     │
│   [清空所有数据] (红色按钮)      │
├─────────────────────────────────┤
│ 快捷键                           │
│   创建新任务    Ctrl+N           │
│   搜索        Ctrl+F             │
│   切换侧边栏    Ctrl+B            │
├─────────────────────────────────┤
│ 关于                             │
│   RTodo v0.1.0                   │
│   基于 Tauri + Vue 3             │
└─────────────────────────────────┘
```

---

## 三、关键组件设计

### 3.1 TodoCard.vue - 任务卡片

**Props:** `todo: Todo`
**Events:** `click`, `toggle-status`, `toggle-mark`

**特性：**
- 复选框快速切换状态
- 标题、描述预览（100字）
- 标签、优先级、截止日期显示
- 子任务进度条
- 悬浮效果和点击反馈

### 3.2 GroupManager.vue - 任务组管理

**功能：**
- 树形展示任务组（支持拖拽排序）
- 添加/编辑/删除任务组
- 颜色选择器（8种预设颜色）
- 图标自动生成（首字母）
- 支持嵌套分组

### 3.3 TagSelector.vue - 标签选择器

**功能：**
- 多选标签
- 快速创建新标签
- 颜色关联显示
- 搜索过滤

### 3.4 DateTimePicker.vue - 时间选择器

**功能：**
- 开始时间/截止时间选择
- 快捷选项（今天、明天、下周）
- 逾期警告标识

### 3.5 StepEditor.vue - 步骤编辑器

**功能：**
- 添加/删除/编辑步骤
- 拖拽排序
- 批量操作
- 进度计算

---

## 四、数据模型

### 4.1 SQLite 表结构

```sql
-- 任务组表
CREATE TABLE task_groups (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    parent_id TEXT,
    icon TEXT,
    color TEXT,
    sort_order INTEGER DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- 标签表
CREATE TABLE tags (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    color TEXT NOT NULL DEFAULT '#409EFF',
    created_at INTEGER NOT NULL
);

-- 任务表
CREATE TABLE todos (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'todo',
    priority INTEGER DEFAULT 0,
    is_marked INTEGER DEFAULT 0,
    group_id TEXT,
    assignee TEXT,
    start_date INTEGER,
    due_date INTEGER,
    completed_at INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (group_id) REFERENCES task_groups(id)
);

-- 任务-标签关联
CREATE TABLE todo_tags (
    todo_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    PRIMARY KEY (todo_id, tag_id)
);

-- 执行步骤
CREATE TABLE todo_steps (
    id TEXT PRIMARY KEY,
    todo_id TEXT NOT NULL,
    title TEXT NOT NULL,
    is_completed INTEGER DEFAULT 0,
    sort_order INTEGER DEFAULT 0,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (todo_id) REFERENCES todos(id)
);

-- 附件
CREATE TABLE attachments (
    id TEXT PRIMARY KEY,
    todo_id TEXT NOT NULL,
    name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_size INTEGER,
    mime_type TEXT,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (todo_id) REFERENCES todos(id)
);
```

### 4.2 TypeScript 核心类型

```typescript
enum TodoStatus { Todo = 'todo', InProgress = 'in_progress', Done = 'done' }

interface Todo {
  id: string;
  title: string;
  description?: string;
  status: TodoStatus;
  priority: number;        // 0: 普通, 1: 重要, 2: 紧急
  is_marked: boolean;      // 重要标记
  group_id?: string;
  assignee?: string;       // 分配人
  start_date?: number;
  due_date?: number;
  completed_at?: number;
  tags?: Tag[];
  steps?: TodoStep[];
  attachments?: Attachment[];
}

interface TaskGroup {
  id: string;
  name: string;
  parent_id?: string;
  icon?: string;
  color?: string;
  sort_order: number;
}

interface Tag {
  id: string;
  name: string;
  color: string;
}

interface TodoStep {
  id: string;
  todo_id: string;
  title: string;
  is_completed: boolean;
  sort_order: number;
}

interface Attachment {
  id: string;
  todo_id: string;
  name: string;
  file_path: string;
  file_size: number;
  mime_type?: string;
}
```

---

## 五、Tauri Command API

### 后端命令（Rust）

| 命令 | 功能 |
|------|------|
| `get_todos` | 获取任务列表（支持筛选） |
| `get_todo` | 获取单个任务详情 |
| `create_todo` | 创建任务 |
| `update_todo` | 更新任务 |
| `delete_todo` | 删除任务 |
| `update_todo_status` | 更新任务状态 |
| `toggle_todo_mark` | 切换重要标记 |
| `get_task_groups` | 获取所有任务组 |
| `create_task_group` | 创建任务组 |
| `update_task_group` | 更新任务组 |
| `delete_task_group` | 删除任务组 |
| `get_tags` | 获取所有标签 |
| `create_tag` | 创建标签 |
| `update_tag` | 更新标签 |
| `delete_tag` | 删除标签 |
| `get_todo_steps` | 获取任务步骤 |
| `create_step` | 创建步骤 |
| `toggle_step` | 切换步骤状态 |
| `delete_step` | 删除步骤 |
| `get_attachments` | 获取附件列表 |
| `upload_attachment` | 上传附件 |
| `delete_attachment` | 删除附件 |
| `get_stats` | 获取总体统计 |
| `get_stats_by_date` | 按日期统计（天/周/月） |
| `export_all_data` | 导出所有数据 |
| `import_data` | 导入数据 |
| `clear_all_data` | 清空所有数据 |

### 前端调用示例

```typescript
import { invoke } from '@tauri-apps/api/core';

// 获取任务列表
const todos = await invoke<Todo[]>('get_todos', {
  group_id: 'xxx',
  status: 'todo'
});

// 创建任务
const newTodo = await invoke<Todo>('create_todo', {
  title: 'New Task',
  description: 'Details...',
  tag_ids: ['tag1', 'tag2']
});

// 更新状态
await invoke('update_todo_status', {
  id: todo.id,
  status: 'done'
});
```

---

## 六、编码规范

1. **Rust 不使用 unwrap**：正式代码中使用 `?` 或 `expect`，明确错误处理
2. **UTF-8 编码**：所有代码文件使用 UTF-8 无 BOM
3. **单元测试**：核心业务逻辑需要测试覆盖
4. **安全**：用户输入验证，SQL 使用参数化查询防止注入
5. **异步规范**：高 CPU 操作使用独立线程，避免阻塞
