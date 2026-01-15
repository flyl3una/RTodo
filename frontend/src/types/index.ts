/**
 * 任务状态枚举 - 使用数字表示
 * 0: 待办, 1: 进行中, 2: 已完成
 */
export enum TodoStatus {
  Todo = 0,
  InProgress = 1,
  Done = 2,
}

/**
 * 获取状态标签文本
 */
export function getStatusLabel(status: TodoStatus): string {
  switch (status) {
    case TodoStatus.Todo:
      return '待办';
    case TodoStatus.InProgress:
      return '进行中';
    case TodoStatus.Done:
      return '已完成';
    default:
      return '未知';
  }
}

/**
 * 获取状态对应的 Element Plus 标签类型
 */
export function getStatusType(status: TodoStatus): 'success' | 'warning' | 'info' {
  switch (status) {
    case TodoStatus.Done:
      return 'success';
    case TodoStatus.InProgress:
      return 'warning';
    default:
      return 'info';
  }
}

/**
 * 任务优先级
 * 0: 普通, 1: 重要, 3: 紧急
 */
export enum TodoPriority {
  Normal = 0,
  Important = 1,
  Urgent = 3,
}

/**
 * 获取优先级标签文本
 */
export function getPriorityLabel(priority: number): string {
  switch (priority) {
    case 0:
      return '普通';
    case 1:
      return '重要';
    case 3:
      return '紧急';
    default:
      return '未知';
  }
}

/**
 * 获取优先级对应的 Element Plus 标签类型
 */
export function getPriorityType(priority: number): 'info' | 'warning' | 'danger' {
  switch (priority) {
    case 3:
      return 'danger';
    case 1:
      return 'warning';
    default:
      return 'info';
  }
}

/**
 * 任务实体
 */
export interface Todo {
  id: string;
  title: string;
  description?: string;
  status: TodoStatus;
  priority: number;
  group_id?: string;
  assignee?: string;
  start_date?: number;
  due_date?: number;
  completed_at?: number;
  created_at: number;
  updated_at: number;
  tags?: Tag[];
  steps?: TodoStep[];
  attachments?: Attachment[];
  group_info?: TaskGroup;
}

/**
 * 任务组
 */
export interface TaskGroup {
  id: string;
  name: string;
  parent_id?: string;
  icon?: string;
  color?: string;
  sort_order: number;
  created_at: number;
  updated_at: number;
  children?: TaskGroup[];
}

/**
 * 标签
 */
export interface Tag {
  id: string;
  name: string;
  color: string;
  created_at: number;
}

/**
 * 执行步骤
 */
export interface TodoStep {
  id: string;
  todo_id: string;
  title: string;
  is_completed: boolean;
  sort_order: number;
  created_at: number;
}

/**
 * 附件
 */
export interface Attachment {
  id: string;
  todo_id: string;
  name: string;
  file_path: string;
  file_size: number;
  mime_type?: string;
  created_at: number;
}

/**
 * 创建任务请求
 */
export interface CreateTodoRequest {
  title: string;
  description?: string;
  group_id?: string;
  start_date?: number;
  due_date?: number;
  priority?: number;
  tag_ids?: string[];
}

/**
 * 更新任务请求
 */
export interface UpdateTodoRequest {
  id: string;
  title?: string;
  description?: string;
  status?: TodoStatus;
  priority?: number;
  group_id?: string;
  assignee?: string;
  start_date?: number;
  due_date?: number;
  tag_ids?: string[];
}

/**
 * 任务统计
 */
export interface TodoStats {
  total: number;
  todo: number;
  in_progress: number;
  done: number;
  overdue: number;
  marked: number;
}

/**
 * 按日期统计
 */
export interface StatsByDate {
  date: string;
  completed: number;
  created: number;
}

/**
 * 带任务详情的统计
 */
export interface TodoStatsWithDetails {
  total: number;
  todo: number;
  in_progress: number;
  done: number;
  todos: Todo[];
  in_progress_tasks: Todo[];
  done_tasks: Todo[];
}

/**
 * 筛选条件
 */
export interface TodoFilter {
  group_id?: string;
  tag_id?: string;
  status?: TodoStatus;
  priority?: number;
  search_keyword?: string;
  date_range?: {
    start: number;
    end: number;
  };
}

// 导出所有类型
export type {
  Todo,
  TaskGroup,
  Tag,
  TodoStep,
  Attachment,
  CreateTodoRequest,
  UpdateTodoRequest,
  TodoStats,
  StatsByDate,
  TodoStatsWithDetails,
  TodoFilter,
  GroupStats,
  TagStats,
  ExportData,
};

/**
 * 任务组统计
 */
export interface GroupStats {
  id: string;
  name: string;
  color: string;
  total: number;
  done: number;
}

/**
 * 标签统计
 */
export interface TagStats {
  id: string;
  name: string;
  color: string;
  count: number;
}

/**
 * 导出数据结构
 */
export interface ExportData {
  version: string;
  exported_at: number;
  task_groups: TaskGroup[];
  tags: Tag[];
  todos: Todo[];
}
