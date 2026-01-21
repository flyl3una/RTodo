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
  const result = await safeInvoke<Todo[]>('get_todos', { payload: args});
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

  // 构建动态 payload - 只包含非 undefined 的字段
  const payload: Record<string, unknown> = {
    title: request.title,
  };

  // 只有非 undefined 的字段才添加到 payload
  if (request.description !== undefined) payload.description = request.description;
  if (request.group_id !== undefined) payload.group_id = request.group_id;
  if (request.start_date !== undefined) payload.start_date = request.start_date;
  if (request.due_date !== undefined) payload.due_date = request.due_date;
  if (request.priority !== undefined) payload.priority = request.priority;
  if (request.tag_ids !== undefined) payload.tag_ids = request.tag_ids;

  console.log('[API] Sending payload to Tauri:', payload);
  return safeInvoke<Todo>('create_todo', { payload });
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
export async function deleteTodo(id: number): Promise<void> {
  return safeInvoke<void>('delete_todo', { id });
}

/**
 * 更新任务状态
 */
export async function updateTodoStatus(id: number, status: TodoStatus): Promise<Todo> {
  const payload = { id, status };
  return safeInvoke<Todo>('update_todo_status', { payload });
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
  const payload: Record<string, unknown> = { name: params.name };
  if (params.parent_id !== undefined) payload.parent_id = params.parent_id;
  if (params.icon !== undefined) payload.icon = params.icon;
  if (params.color !== undefined) payload.color = params.color;
  return safeInvoke<TaskGroup>('create_task_group', { payload });
}

/**
 * 更新任务组
 */
export async function updateTaskGroup(id: number, params: {
  name?: string;
  parent_id?: string;
  icon?: string;
  color?: string;
}): Promise<TaskGroup> {
  const payload: Record<string, unknown> = { id };
  if (params.name !== undefined) payload.name = params.name;
  if (params.parent_id !== undefined) payload.parent_id = params.parent_id;
  if (params.icon !== undefined) payload.icon = params.icon;
  if (params.color !== undefined) payload.color = params.color;
  return safeInvoke<TaskGroup>('update_task_group', { payload });
}

/**
 * 删除任务组
 */
export async function deleteTaskGroup(id: number): Promise<void> {
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
  const payload = { name, color };
  return safeInvoke<Tag>('create_tag', { payload });
}

/**
 * 更新标签
 */
export async function updateTag(id: number, params: {
  name?: string;
  color?: string;
}): Promise<Tag> {
  const payload: Record<string, unknown> = { id };
  if (params.name !== undefined) payload.name = params.name;
  if (params.color !== undefined) payload.color = params.color;
  return safeInvoke<Tag>('update_tag', { payload });
}

/**
 * 删除标签
 */
export async function deleteTag(id: number): Promise<void> {
  return safeInvoke<void>('delete_tag', { id });
}

// ==================== Step API ====================

/**
 * 获取任务的执行步骤
 */
export async function getTodoSteps(todoId: number): Promise<TodoStep[]> {
  return safeInvoke<TodoStep[]>('get_todo_steps', { todoId });
}

/**
 * 创建执行步骤
 */
export async function createStep(todoId: number, title: string): Promise<TodoStep> {
  const payload = { todo_id: todoId, title };
  return safeInvoke<TodoStep>('create_step', { payload });
}

/**
 * 切换步骤完成状态
 */
export async function toggleStep(stepId: number): Promise<TodoStep> {
  return safeInvoke<TodoStep>('toggle_step', { id: stepId });
}

/**
 * 删除步骤
 */
export async function deleteStep(stepId: number): Promise<void> {
  return safeInvoke<void>('delete_step', { id: stepId });
}

/**
 * 更新步骤标题
 */
export async function updateStep(stepId: number, title: string): Promise<TodoStep> {
  const payload = { id: stepId, title };
  return safeInvoke<TodoStep>('update_step', { payload });
}

// ==================== Attachment API ====================

/**
 * 获取任务的附件列表
 */
export async function getAttachments(todoId: number): Promise<Attachment[]> {
  return safeInvoke<Attachment[]>('get_attachments', { todoId });
}

/**
 * 上传附件
 */
export async function uploadAttachment(todoId: number, filePath: string, fileName: string): Promise<Attachment> {
  return safeInvoke<Attachment>('upload_attachment', { todoId, filePath, fileName });
}

/**
 * 删除附件
 */
export async function deleteAttachment(attachmentId: number): Promise<void> {
  return safeInvoke<void>('delete_attachment', { id: attachmentId });
}

/**
 * 打开附件
 */
export async function openAttachment(attachmentId: number): Promise<void> {
  return safeInvoke<void>('open_attachment', { attachmentId });
}

/**
 * 下载附件到指定位置
 */
export async function downloadAttachment(attachmentId: number, targetPath: string): Promise<void> {
  const payload = { attachment_id: attachmentId, target_path: targetPath };
  return safeInvoke<void>('download_attachment', { payload });
}

// ==================== Data Path API ====================

/**
 * 获取当前数据路径
 */
export async function getDataPath(): Promise<string> {
  return safeInvoke<string>('get_data_path');
}

/**
 * 检查目录是否为空
 */
export async function checkDirectoryEmpty(path: string): Promise<boolean> {
  return safeInvoke<boolean>('check_directory_empty', { path });
}

/**
 * 设置数据路径
 */
export async function setDataPath(newPath: string): Promise<void> {
  return safeInvoke<void>('set_data_path', { new_path: newPath });
}

/**
 * 重置数据路径为默认值
 */
export async function resetDataPath(): Promise<void> {
  return safeInvoke<void>('reset_data_path');
}

/**
 * 迁移数据到新路径
 */
export async function migrateData(newPath: string, keepOriginal: boolean): Promise<void> {
  console.log('[API] migrateData called with:', { newPath, keepOriginal });
  const payload = { new_path: newPath, keep_original: keepOriginal };
  const result = await safeInvoke<void>('migrate_data', { payload });
  console.log('[API] migrateData result:', result);
  return result;
}

// ==================== Stats API ====================

/**
 * 获取统计数据
 */
export async function getStats(startDate?: number, endDate?: number): Promise<TodoStats> {
  const payload: Record<string, unknown> = {};
  if (startDate !== undefined) payload.start_date = startDate;
  if (endDate !== undefined) payload.end_date = endDate;
  return safeInvoke<TodoStats>('get_stats', { payload });
}

/**
 * 按日期获取统计
 */
export async function getStatsByDate(range: string, startDate?: number, endDate?: number): Promise<StatsByDate[]> {
  const payload: Record<string, unknown> = { range };
  if (startDate !== undefined) payload.start_date = startDate;
  if (endDate !== undefined) payload.end_date = endDate;
  return safeInvoke<StatsByDate[]>('get_stats_by_date', { payload });
}

/**
 * 获取带详情的统计
 */
export async function getStatsWithDetails(
  startDate?: number,
  endDate?: number,
  groupIds?: number[],
  tagIds?: number[]
): Promise<TodoStatsWithDetails> {
  const payload: Record<string, unknown> = {};
  if (startDate !== undefined) payload.start_date = startDate;
  if (endDate !== undefined) payload.end_date = endDate;
  if (groupIds !== undefined && groupIds.length > 0) payload.group_ids = groupIds;
  if (tagIds !== undefined && tagIds.length > 0) payload.tag_ids = tagIds;
  return safeInvoke<TodoStatsWithDetails>('get_stats_with_details', { payload });
}

// ==================== Import/Export API ====================

/**
 * 导出所有数据
 */
export async function exportAllData(): Promise<ExportData> {
  return safeInvoke<ExportData>('export_all_data');
}

/**
 * 导出所有数据为CSV格式并打包到指定路径
 */
export async function exportDataAsCsv(filePath: string): Promise<void> {
  return safeInvoke<void>('export_data_as_csv', { filePath });
}

/**
 * 从CSV压缩包导入数据
 */
export async function importDataFromCsv(filePath: string): Promise<void> {
  return safeInvoke<void>('import_data_from_csv', { filePath });
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

// ==================== App API ====================

/**
 * 设置全局快捷键
 */
export async function setGlobalShortcut(shortcut: string): Promise<void> {
  return safeInvoke<void>('set_global_shortcut', { shortcut });
}

/**
 * 获取当前注册的全局快捷键
 */
export async function getGlobalShortcut(): Promise<string | null> {
  return safeInvoke<string | null>('get_global_shortcut');
}

/**
 * 切换窗口显示/隐藏
 */
export async function toggleWindowVisibility(): Promise<void> {
  return safeInvoke<void>('toggle_window_visibility');
}

/**
 * 显示窗口
 */
export async function showWindow(): Promise<void> {
  return safeInvoke<void>('show_window');
}

/**
 * 隐藏窗口
 */
export async function hideWindow(): Promise<void> {
  return safeInvoke<void>('hide_window');
}

/**
 * 设置关闭行为
 */
export async function setCloseBehavior(behavior: 'direct' | 'minimize_to_tray'): Promise<void> {
  return safeInvoke<void>('set_close_behavior', { behavior });
}

/**
 * 获取关闭行为
 */
export async function getCloseBehavior(): Promise<string> {
  return safeInvoke<string>('get_close_behavior');
}


/**
 * 设置开机启动
 */
export async function setAutoLaunch(enabled: boolean): Promise<void> {
  return safeInvoke<void>('set_auto_launch', { enabled });
}

/**
 * 获取开机启动状态
 */
export async function getAutoLaunch(): Promise<boolean> {
  return safeInvoke<boolean>('get_auto_launch');
}

/**
 * 切换开机启动状态
 */
export async function toggleAutoLaunch(): Promise<boolean> {
  return safeInvoke<boolean>('toggle_auto_launch');
}
