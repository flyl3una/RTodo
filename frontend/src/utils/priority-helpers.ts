import { TodoPriority } from '@/types';

/**
 * 优先级常量
 */
export const PRIORITY_VALUES = {
  NORMAL: TodoPriority.Normal,      // 0
  IMPORTANT: TodoPriority.Important, // 1
  URGENT: TodoPriority.Urgent,       // 3
} as const;

/**
 * 判断任务是否已标记（优先级 >= 重要）
 */
export function isMarked(priority: number): boolean {
  return priority >= PRIORITY_VALUES.IMPORTANT;
}

/**
 * 切换优先级：普通(0) → 重要(1) → 紧急(3) → 普通(0)
 */
export function cyclePriority(currentPriority: number): number {
  if (currentPriority === PRIORITY_VALUES.NORMAL) {
    return PRIORITY_VALUES.IMPORTANT;
  } else if (currentPriority === PRIORITY_VALUES.IMPORTANT) {
    return PRIORITY_VALUES.URGENT;
  } else {
    return PRIORITY_VALUES.NORMAL;
  }
}
