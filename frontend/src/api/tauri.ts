import { invoke } from '@tauri-apps/api/core';
import type {
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
  ExportData,
} from '@/types';
import { TodoStatus } from '@/types';

// ==================== Todo API ====================

/**
 * 获取任务列表
 */
export async function getTodos(params?: {
  group_id?: string;
  tag_id?: string;
  status?: TodoStatus;
  search?: string;
  is_marked?: boolean;
  priority?: number;
  start_date?: number;
  end_date?: number;
}): Promise<Todo[]> {
  return invoke<Todo[]>('get_todos', {
    group_id: params?.group_id,
    tag_id: params?.tag_id,
    status: params?.status,
    search: params?.search,
    is_marked: params?.is_marked,
    priority: params?.priority,
    start_date: params?.start_date,
    end_date: params?.end_date,
  });
}

/**
 * 获取单个任务详情
 */
export async function getTodo(id: string): Promise<Todo> {
  return invoke<Todo>('get_todo', { id });
}

/**
 * 创建任务
 */
export async function createTodo(request: CreateTodoRequest): Promise<Todo> {
  return invoke<Todo>('create_todo', request);
}

/**
 * 更新任务
 */
export async function updateTodo(request: UpdateTodoRequest): Promise<Todo> {
  return invoke<Todo>('update_todo', request);
}

/**
 * 删除任务
 */
export async function deleteTodo(id: string): Promise<void> {
  return invoke<void>('delete_todo', { id });
}

/**
 * 更新任务状态
 */
export async function updateTodoStatus(id: string, status: TodoStatus): Promise<Todo> {
  return invoke<Todo>('update_todo_status', { id, status });
}

/**
 * 切换任务重要标记
 */
export async function toggleTodoMark(id: string): Promise<Todo> {
  return invoke<Todo>('toggle_todo_mark', { id });
}

// ==================== TaskGroup API ====================

/**
 * 获取所有任务组
 */
export async function getTaskGroups(): Promise<TaskGroup[]> {
  return invoke<TaskGroup[]>('get_task_groups');
}

/**
 * 创建任务组
 */
export async function createTaskGroup(params: {
  name: string;
  parent_id?: string;
  icon?: string;
  color?: string;
}): Promise<TaskGroup> {
  return invoke<TaskGroup>('create_task_group', params);
}

/**
 * 更新任务组
 */
export async function updateTaskGroup(id: string, params: {
  name?: string;
  parent_id?: string;
  icon?: string;
  color?: string;
}): Promise<TaskGroup> {
  return invoke<TaskGroup>('update_task_group', { id, ...params });
}

/**
 * 删除任务组
 */
export async function deleteTaskGroup(id: string): Promise<void> {
  return invoke<void>('delete_task_group', { id });
}

// ==================== Tag API ====================

/**
 * 获取所有标签
 */
export async function getTags(): Promise<Tag[]> {
  return invoke<Tag[]>('get_tags');
}

/**
 * 创建标签
 */
export async function createTag(name: string, color: string): Promise<Tag> {
  return invoke<Tag>('create_tag', { name, color });
}

/**
 * 更新标签
 */
export async function updateTag(id: string, params: {
  name?: string;
  color?: string;
}): Promise<Tag> {
  return invoke<Tag>('update_tag', { id, ...params });
}

/**
 * 删除标签
 */
export async function deleteTag(id: string): Promise<void> {
  return invoke<void>('delete_tag', { id });
}

// ==================== Step API ====================

/**
 * 获取任务的所有步骤
 */
export async function getTodoSteps(todoId: string): Promise<TodoStep[]> {
  return invoke<TodoStep[]>('get_todo_steps', { todoId });
}

/**
 * 创建步骤
 */
export async function createStep(todoId: string, title: string): Promise<TodoStep> {
  return invoke<TodoStep>('create_step', { todoId, title });
}

/**
 * 切换步骤状态
 */
export async function toggleStep(id: string): Promise<TodoStep> {
  return invoke<TodoStep>('toggle_step', { id });
}

/**
 * 删除步骤
 */
export async function deleteStep(id: string): Promise<void> {
  return invoke<void>('delete_step', { id });
}

// ==================== Attachment API ====================

/**
 * 获取任务的所有附件
 */
export async function getAttachments(todoId: string): Promise<Attachment[]> {
  return invoke<Attachment[]>('get_attachments', { todoId });
}

/**
 * 上传附件
 */
export async function uploadAttachment(todoId: string, filePath: string, fileName: string): Promise<Attachment> {
  return invoke<Attachment>('upload_attachment', {
    todoId,
    filePath,
    fileName,
  });
}

/**
 * 删除附件
 */
export async function deleteAttachment(id: string): Promise<void> {
  return invoke<void>('delete_attachment', { id });
}

// ==================== Stats API ====================

/**
 * 获取总体统计
 */
export async function getStats(): Promise<TodoStats> {
  return invoke<TodoStats>('get_stats');
}

/**
 * 按日期获取统计
 */
export async function getStatsByDate(range: 'day' | 'week' | 'month'): Promise<StatsByDate[]> {
  return invoke<StatsByDate[]>('get_stats_by_date', { range });
}

/**
 * 获取带任务详情的统计（支持时间范围筛选）
 */
export async function getStatsWithDetails(startDate?: number, endDate?: number): Promise<TodoStatsWithDetails> {
  return invoke<TodoStatsWithDetails>('get_stats_with_details', {
    start_date: startDate,
    end_date: endDate,
  });
}

// ==================== Import/Export API ====================

/**
 * 导出所有数据
 */
export async function exportAllData(): Promise<ExportData> {
  return invoke<ExportData>('export_all_data');
}

/**
 * 导入数据
 */
export async function importData(data: ExportData): Promise<void> {
  return invoke<void>('import_data', { data });
}

/**
 * 清空所有数据
 */
export async function clearAllData(): Promise<void> {
  return invoke<void>('clear_all_data');
}
