/**
 * 任务状态枚举
 */
export enum TodoStatus {
  Todo = 'todo',
  InProgress = 'in_progress',
  Done = 'done',
}

/**
 * 任务优先级
 */
export enum TodoPriority {
  Normal = 0,
  Important = 1,
  Urgent = 2,
}

/**
 * 任务实体
 */
export interface Todo {
  id: string;
  title: string;
  description?: string;
  status: 'todo' | 'in_progress' | 'done';
  priority: number;
  is_marked: boolean;
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
  group?: TaskGroup;
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
  status?: 'todo' | 'in_progress' | 'done';
  priority?: number;
  is_marked?: boolean;
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
  is_marked?: boolean;
  search_keyword?: string;
  date_range?: {
    start: number;
    end: number;
  };
}

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
