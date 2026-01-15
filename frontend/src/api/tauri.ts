import { invoke } from '@tauri-apps/api/core';
import { safeInvoke } from '@/utils/tauri-helpers';
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
  priority?: number;
  start_date?: number;
  end_date?: number;
}): Promise<Todo[]> {
  console.log('[API] getTodos called with params:', params);

  // 只包含有实际值的参数，避免传递 undefined 导致 Tauri IPC 反序列化问题
  const args: Record<string, unknown> = {};
  if (params?.group_id !== undefined) args.group_id = params.group_id;
  if (params?.tag_id !== undefined) args.tag_id = params.tag_id;
  if (params?.status !== undefined) args.status = params.status;
  if (params?.search !== undefined) args.search = params.search;
  if (params?.priority !== undefined) args.priority = params.priority;
  if (params?.start_date !== undefined) args.start_date = params.start_date;
  if (params?.end_date !== undefined) args.end_date = params.end_date;

  console.log('[API] Sending filtered args to Tauri:', args);
  const result = await safeInvoke<Todo[]>('get_todos', args);
  console.log('[API] getTodos returned', result.length, 'todos');
  return result;
}

/**
 * 获取单个任务详情
 */
export async function getTodo(id: string): Promise<Todo> {
  return safeInvoke<Todo>('get_todo', { id });
}

/**
 * 创建任务
 */
export async function createTodo(request: CreateTodoRequest): Promise<Todo> {
  console.log('[API] createTodo called with:', request);
  return safeInvoke<Todo>('create_todo', request);
}

/**
 * 更新任务
 */
export async function updateTodo(request: UpdateTodoRequest): Promise<Todo> {
  console.log('[API] updateTodo called with:', request);
  console.log('[API] request.start_date:', request.start_date, 'type:', typeof request.start_date);
  console.log('[API] request.due_date:', request.due_date, 'type:', typeof request.due_date);

  // 构建动态 payload - 只包含非 undefined 的字段
  const payload: Record<string, unknown> = {
    id: request.id,
  };

  // 只有非 undefined 的字段才添加到 payload
  if (request.title !== undefined) payload.title = request.title;
  if (request.description !== undefined) payload.description = request.description;
  if (request.status !== undefined) payload.status = request.status;
  if (request.priority !== undefined) payload.priority = request.priority;
  if (request.group_id !== undefined) payload.group_id = request.group_id;
  if (request.assignee !== undefined) payload.assignee = request.assignee;
  if (request.tag_ids !== undefined) payload.tag_ids = request.tag_ids;

  // 日期字段特殊处理
  // - null: 明确传递 null，表示要清除日期
  // - 数字: 传递数字，表示要设置日期
  // - undefined: 不传递该字段，表示不更新日期
  if (request.start_date !== undefined) {
    payload.start_date = request.start_date;
    console.log('[API] Including start_date in payload:', request.start_date);
  } else {
    console.log('[API] Excluding start_date from payload (undefined)');
  }
  if (request.due_date !== undefined) {
    payload.due_date = request.due_date;
    console.log('[API] Including due_date in payload:', request.due_date);
  } else {
    console.log('[API] Excluding due_date from payload (undefined)');
  }

  console.log('[API] Sending payload to Tauri:', { payload });
  console.log('[API] payload.start_date:', payload.start_date, 'type:', typeof payload.start_date);
  console.log('[API] payload.due_date:', payload.due_date, 'type:', typeof payload.due_date);

  const result = await safeInvoke<Todo>('update_todo', { payload });

  console.log('[API] updateTodo returned:', result);
  console.log('[API] result.start_date:', result.start_date, 'result.due_date:', result.due_date);

  // 验证日期字段是否正确返回
  if (request.start_date !== undefined && result.start_date !== request.start_date) {
    console.error('[API] ⚠️ start_date 不匹配！请求:', request.start_date, '返回:', result.start_date);
  }
  if (request.due_date !== undefined && result.due_date !== request.due_date) {
    console.error('[API] ⚠️ due_date 不匹配！请求:', request.due_date, '返回:', result.due_date);
  }

  return result;
}

/**
 * 删除任务
 */
export async function deleteTodo(id: string): Promise<void> {
  return safeInvoke<void>('delete_todo', { id });
}

/**
 * 更新任务状态
 */
export async function updateTodoStatus(id: string, status: TodoStatus): Promise<Todo> {
  return safeInvoke<Todo>('update_todo_status', { id, status });
}

// ==================== TaskGroup API ====================

/**
 * 获取所有任务组
 */
export async function getTaskGroups(): Promise<TaskGroup[]> {
  return safeInvoke<TaskGroup[]>('get_task_groups');
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
  return safeInvoke<TaskGroup>('create_task_group', params);
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
  return safeInvoke<TaskGroup>('update_task_group', { id, ...params });
}

/**
 * 删除任务组
 */
export async function deleteTaskGroup(id: string): Promise<void> {
  return safeInvoke<void>('delete_task_group', { id });
}

// ==================== Tag API ====================

/**
 * 获取所有标签
 */
export async function getTags(): Promise<Tag[]> {
  return safeInvoke<Tag[]>('get_tags');
}

/**
 * 创建标签
 */
export async function createTag(name: string, color: string): Promise<Tag> {
  return safeInvoke<Tag>('create_tag', { name, color });
}

/**
 * 更新标签
 */
export async function updateTag(id: string, name: string, color: string): Promise<Tag> {
  return safeInvoke<Tag>('update_tag', { id, name, color });
}

/**
 * 删除标签
 */
export async function deleteTag(id: string): Promise<void> {
  return safeInvoke<void>('delete_tag', { id });
}

// ==================== Step API ====================

/**
 * 获取任务的执行步骤
 */
export async function getTodoSteps(todoId: string): Promise<TodoStep[]> {
  return safeInvoke<TodoStep[]>('get_todo_steps', { todoId });
}

/**
 * 创建执行步骤
 */
export async function createStep(todoId: string, title: string): Promise<TodoStep> {
  return safeInvoke<TodoStep>('create_step', { todoId, title });
}

/**
 * 切换步骤完成状态
 */
export async function toggleStep(stepId: string): Promise<TodoStep> {
  return safeInvoke<TodoStep>('toggle_step', { id: stepId });
}

/**
 * 删除步骤
 */
export async function deleteStep(stepId: string): Promise<void> {
  return safeInvoke<void>('delete_step', { id: stepId });
}

// ==================== Attachment API ====================

/**
 * 获取任务的附件列表
 */
export async function getAttachments(todoId: string): Promise<Attachment[]> {
  return safeInvoke<Attachment[]>('get_attachments', { todoId });
}

/**
 * 上传附件
 */
export async function uploadAttachment(todoId: string, filePath: string): Promise<Attachment> {
  return safeInvoke<Attachment>('upload_attachment', { todoId, filePath });
}

/**
 * 删除附件
 */
export async function deleteAttachment(attachmentId: string): Promise<void> {
  return safeInvoke<void>('delete_attachment', { id: attachmentId });
}

// ==================== Stats API ====================

/**
 * 获取统计数据
 */
export async function getStats(): Promise<TodoStats> {
  return safeInvoke<TodoStats>('get_stats');
}

/**
 * 按日期获取统计
 */
export async function getStatsByDate(range: string): Promise<StatsByDate[]> {
  return safeInvoke<StatsByDate[]>('get_stats_by_date', { range });
}

/**
 * 获取带详情的统计
 */
export async function getStatsWithDetails(): Promise<TodoStatsWithDetails> {
  return safeInvoke<TodoStatsWithDetails>('get_stats_with_details');
}

// ==================== Import/Export API ====================

/**
 * 导出所有数据
 */
export async function exportAllData(): Promise<ExportData> {
  return safeInvoke<ExportData>('export_all_data');
}

/**
 * 导入数据
 */
export async function importData(data: ExportData): Promise<void> {
  return safeInvoke<void>('import_data', { data });
}

/**
 * 清空所有数据
 */
export async function clearAllData(): Promise<void> {
  return safeInvoke<void>('clear_all_data');
}
