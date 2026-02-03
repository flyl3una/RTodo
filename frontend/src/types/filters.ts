/**
 * 视图过滤器类型枚举
 */
export enum ViewFilterTypeEnum {
  All = 'all',
  Todo = 'todo',
  Today = 'today',
  Important = 'important',
  Urgent = 'urgent',
  Completed = 'completed',
  Overdue = 'overdue',
  Group = 'group',
  Tag = 'tag',
}

export type ViewFilterTypeString =
  | 'all' | 'todo' | 'today' | 'important' | 'urgent'
  | 'completed' | 'overdue' | 'group' | 'tag';
