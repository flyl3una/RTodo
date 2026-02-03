<template>
  <div class="stats-view">
    <div class="stats-header">
      <h1 class="page-title">{{ t('stats.title') }}</h1>
      <div class="header-controls">
        <el-segmented v-model="viewMode" :options="viewModeOptions" style="margin-right: 12px" />
        <el-segmented v-model="dateRange" :options="dateRangeOptions" style="margin-right: 12px" />
        <el-select
          v-model="selectedGroupIds"
          multiple
          collapse-tags
          collapse-tags-tooltip
          clearable
          :placeholder="t('stats.selectGroups')"
          style="width: 160px; margin-right: 12px"
        >
          <el-option
            v-for="group in groups"
            :key="group.id"
            :label="group.name"
            :value="group.id"
          >
            <span class="filter-option">
              <span class="filter-dot" :style="{ backgroundColor: group.color }"></span>
              <span>{{ group.name }}</span>
            </span>
          </el-option>
        </el-select>
        <el-select
          v-model="selectedTagIds"
          multiple
          collapse-tags
          collapse-tags-tooltip
          clearable
          :placeholder="t('stats.selectTags')"
          style="width: 160px; margin-right: 12px"
        >
          <el-option
            v-for="tag in tags"
            :key="tag.id"
            :label="tag.name"
            :value="tag.id"
          >
            <span class="filter-option">
              <span class="filter-dot" :style="{ backgroundColor: tag.color }"></span>
              <span>{{ tag.name }}</span>
            </span>
          </el-option>
        </el-select>
        <div v-if="dateRange === 'custom'" class="custom-date-range">
          <el-date-picker
            v-model="customStartDate"
            type="date"
            :placeholder="t('stats.startDate')"
            value-format="x"
            style="width: 140px; margin-right: 8px"
          />
          <span>{{ t('common.to') }}</span>
          <el-date-picker
            v-model="customEndDate"
            type="date"
            :placeholder="t('stats.endDate')"
            value-format="x"
            style="width: 140px; margin-left: 8px"
          />
        </div>
      </div>
    </div>

    <!-- Dashboard Mode -->
    <template v-if="viewMode === 'dashboard'">
    <!-- Overview Cards -->
    <div class="stats-cards">
      <div class="stat-card total">
        <div class="stat-icon">📊</div>
        <div class="stat-content">
          <div class="stat-label">{{ t('stats.total') }}</div>
          <div class="stat-value">{{ stats.total }}</div>
        </div>
      </div>
      <div class="stat-card todo">
        <div class="stat-icon">📝</div>
        <div class="stat-content">
          <div class="stat-label">{{ t('status.todo') }}</div>
          <div class="stat-value">{{ stats.todo }}</div>
        </div>
      </div>
      <div class="stat-card in-progress">
        <div class="stat-icon">🔄</div>
        <div class="stat-content">
          <div class="stat-label">{{ t('stats.inProgress') }}</div>
          <div class="stat-value">{{ stats.in_progress }}</div>
        </div>
      </div>
      <div class="stat-card done">
        <div class="stat-icon">✅</div>
        <div class="stat-content">
          <div class="stat-label">{{ t('stats.completed') }}</div>
          <div class="stat-value">{{ stats.done }}</div>
        </div>
      </div>
      <div class="stat-card marked">
        <div class="stat-icon">⭐</div>
        <div class="stat-content">
          <div class="stat-label">{{ t('nav.important') }}</div>
          <div class="stat-value">{{ stats.marked }}</div>
        </div>
      </div>
    </div>

    <!-- Task Details by Status -->
    <div class="stats-section">
      <h2 class="section-title">{{ t('stats.taskDetails') }}</h2>

      <!-- Pending Tasks -->
      <el-collapse v-model="activeCollapse" class="task-collapse">
        <el-collapse-item name="todo">
          <template #title>
            <span class="collapse-title">
              <span class="status-dot todo"></span>
              <span>{{ t('stats.todoTasks') }} ({{ statsWithDetails.todos.length }})</span>
            </span>
          </template>
          <div v-if="statsWithDetails.todos.length > 0" class="task-list">
            <div v-for="task in statsWithDetails.todos" :key="task.id" class="task-item">
              <span class="task-title">{{ task.title }}</span>
              <div class="task-meta">
                <span v-if="task.group" class="task-group">{{ task.group.name }}</span>
                <span v-if="task.due_date" class="task-due">{{ formatDate(task.due_date) }}</span>
              </div>
            </div>
          </div>
          <el-empty v-else :description="t('stats.noTodoTasks')" :image-size="40" />
        </el-collapse-item>

        <!-- In Progress Tasks -->
        <el-collapse-item name="in_progress">
          <template #title>
            <span class="collapse-title">
              <span class="status-dot in-progress"></span>
              <span>{{ t('stats.inProgressTasks') }} ({{ statsWithDetails.in_progress_tasks.length }})</span>
            </span>
          </template>
          <div v-if="statsWithDetails.in_progress_tasks.length > 0" class="task-list">
            <div v-for="task in statsWithDetails.in_progress_tasks" :key="task.id" class="task-item">
              <span class="task-title">{{ task.title }}</span>
              <div class="task-meta">
                <span v-if="task.group" class="task-group">{{ task.group.name }}</span>
                <span v-if="task.due_date" class="task-due">{{ formatDate(task.due_date) }}</span>
              </div>
            </div>
          </div>
          <el-empty v-else :description="t('stats.noInProgressTasks')" :image-size="40" />
        </el-collapse-item>

        <!-- Completed Tasks -->
        <el-collapse-item name="done">
          <template #title>
            <span class="collapse-title">
              <span class="status-dot done"></span>
              <span>{{ t('stats.completedTasks') }} ({{ statsWithDetails.done_tasks.length }})</span>
            </span>
          </template>
          <div v-if="statsWithDetails.done_tasks.length > 0" class="task-list">
            <div v-for="task in statsWithDetails.done_tasks" :key="task.id" class="task-item completed">
              <span class="task-title">{{ task.title }}</span>
              <div class="task-meta">
                <span v-if="task.completed_at" class="task-completed">{{ t('stats.completedAt') }} {{ formatDateTime(task.completed_at) }}</span>
              </div>
            </div>
          </div>
          <el-empty v-else :description="t('stats.noCompletedTasks')" :image-size="40" />
        </el-collapse-item>
      </el-collapse>
    </div>

    <!-- Completion Trend -->
    <div class="stats-section">
      <h2 class="section-title">{{ t('stats.completionTrend') }}</h2>
      <div class="chart-container">
        <div v-if="dateStats.length > 0" class="bar-chart">
          <div
            v-for="(item, index) in dateStats"
            :key="index"
            class="bar-item"
          >
            <div class="bar-wrapper">
              <div
                class="bar completed"
                :style="{ height: getBarHeight(item.completed, dateStats) }"
              />
              <div
                class="bar created"
                :style="{ height: getBarHeight(item.created, dateStats) }"
              />
            </div>
            <div class="bar-label">{{ formatBarLabel(item.date) }}</div>
          </div>
        </div>
        <el-empty v-else :description="t('messages.noData')" :image-size="60" />
      </div>
      <div class="chart-legend">
        <span class="legend-item">
          <span class="legend-color completed"></span>
          <span>{{ t('stats.completed') }}</span>
        </span>
        <span class="legend-item">
          <span class="legend-color created"></span>
          <span>{{ t('stats.created') }}</span>
        </span>
      </div>
    </div>

    <!-- Group Distribution -->
    <div v-if="groupStats.length > 0" class="stats-section">
      <h2 class="section-title">{{ t('stats.groupDistribution') }}</h2>
      <div class="group-stats">
        <div
          v-for="group in groupStats"
          :key="group.id"
          class="group-stat-item"
        >
          <div class="group-info">
            <span
              class="group-dot"
              :style="{ backgroundColor: group.color }"
            />
            <span class="group-name">{{ group.name }}</span>
          </div>
          <div class="group-progress">
            <el-progress
              :percentage="getCompletionPercentage(group.done, group.total)"
              :color="group.color"
              :show-text="false"
            />
            <span class="group-count">{{ group.done }}/{{ group.total }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Tag Usage -->
    <div v-if="tagStats.length > 0" class="stats-section">
      <h2 class="section-title">{{ t('stats.tagUsage') }}</h2>
      <div class="tag-stats">
        <span
          v-for="tag in tagStats"
          :key="tag.id"
          class="tag-stat-item"
          :style="{ backgroundColor: tag.color }"
        >
          {{ tag.name }} ({{ tag.count }})
        </span>
      </div>
    </div>
    </template>

    <!-- Report Mode -->
    <div v-if="viewMode === 'report'" class="report-section">
      <div class="report-actions">
        <el-checkbox v-model="showTimeAndTags" style="margin-right: 12px">
          {{ t('stats.showTimeAndTags') }}
        </el-checkbox>
        <el-select
          v-model="reportStatusFilter"
          multiple
          collapse-tags
          collapse-tags-tooltip
          clearable
          :placeholder="t('stats.selectStatuses')"
          style="width: 160px; margin-right: 12px"
        >
          <el-option :value="TodoStatus.Done" :label="t('status.done')">
            <span class="filter-option">
              <span class="status-dot-small done"></span>
              <span>{{ t('status.done') }}</span>
            </span>
          </el-option>
          <el-option :value="TodoStatus.Todo" :label="t('status.todo')">
            <span class="filter-option">
              <span class="status-dot-small todo"></span>
              <span>{{ t('status.todo') }}</span>
            </span>
          </el-option>
          <el-option :value="TodoStatus.InProgress" :label="t('status.inProgress')">
            <span class="filter-option">
              <span class="status-dot-small in-progress"></span>
              <span>{{ t('status.inProgress') }}</span>
            </span>
          </el-option>
        </el-select>
        <el-button type="primary" @click="copyReportText">
          {{ t('common.copy') }}
        </el-button>
      </div>
      <textarea
        v-if="reportText"
        v-model="reportText"
        class="report-textarea"
        readonly
      />
      <el-empty v-else :description="t('stats.noTasks')" :image-size="60" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { ElMessage } from 'element-plus';
import { useTodoStore } from '@/stores';
import { useGroupStore } from '@/stores';
import { useTagStore } from '@/stores';
import type { TodoStats, StatsByDate, TodoStatsWithDetails } from '@/types';
import { TodoStatus } from '@/types';
import * as api from '@/api/tauri';
import { isMarked } from '@/utils/priority-helpers';

const { t } = useI18n();

const isMobile = ref(window.innerWidth <= 768);

const todoStore = useTodoStore();
const groupStore = useGroupStore();
const tagStore = useTagStore();

const groups = computed(() => groupStore.groups);
const tags = computed(() => tagStore.tags);

const viewMode = ref<'dashboard' | 'report'>('dashboard');
const showTimeAndTags = ref(false);

const selectedGroupIds = ref<number[]>([]);
const selectedTagIds = ref<number[]>([]);

// 汇报模式的状态筛选（独立于仪表盘筛选）
const reportStatusFilter = ref<number[]>([TodoStatus.Done]); // 默认为已完成

const dateRange = ref<'day' | 'week' | 'month' | 'custom'>('week');
const customStartDate = ref<number | null>(null);
const customEndDate = ref<number | null>(null);
const statsWithDetails = ref<TodoStatsWithDetails>({
  total: 0,
  todo: 0,
  in_progress: 0,
  done: 0,
  todos: [],
  in_progress_tasks: [],
  done_tasks: [],
});

// 从详细数据计算统计卡片数据
const stats = computed<TodoStats>(() => {
  const allTodos = [
    ...statsWithDetails.value.todos,
    ...statsWithDetails.value.in_progress_tasks,
    ...statsWithDetails.value.done_tasks,
  ];
  
  const now = Date.now();
  
  return {
    total: allTodos.length,
    todo: statsWithDetails.value.todo,
    in_progress: statsWithDetails.value.in_progress,
    done: statsWithDetails.value.done,
    marked: allTodos.filter(t => isMarked(t.priority)).length,
    overdue: allTodos.filter(t => t.due_date && t.due_date < now && t.status !== TodoStatus.Done).length,
  };
});

// 从详细数据计算完成趋势（按日期统计）
const dateStats = computed<StatsByDate[]>(() => {
  const allTodos = [
    ...statsWithDetails.value.todos,
    ...statsWithDetails.value.in_progress_tasks,
    ...statsWithDetails.value.done_tasks,
  ];
  
  // 根据选择的时间范围计算天数
  const rangeValue = dateRange.value;
  const startValue = customStartDate.value;
  const endValue = customEndDate.value;
  
  const now = new Date();
  let startDate: number;
  let days: number;
  
  if (rangeValue === 'day') {
    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate(), 0, 0, 0, 0);
    startDate = today.getTime();
    days = 1;
  } else if (rangeValue === 'week') {
    const weekAgo = new Date(now.getTime() - 6 * 86_400_000);
    const start = new Date(weekAgo.getFullYear(), weekAgo.getMonth(), weekAgo.getDate(), 0, 0, 0, 0);
    startDate = start.getTime();
    days = 7;
  } else if (rangeValue === 'month') {
    const monthAgo = new Date(now.getTime() - 29 * 86_400_000);
    const start = new Date(monthAgo.getFullYear(), monthAgo.getMonth(), monthAgo.getDate(), 0, 0, 0, 0);
    startDate = start.getTime();
    days = 30;
  } else if (rangeValue === 'custom' && startValue && endValue) {
    startDate = Math.floor(startValue / 86_400_000) * 86_400_000;
    const duration = endValue - startValue;
    days = Math.max(1, Math.floor(duration / 86_400_000));
  } else {
    return [];
  }
  
  const stats: StatsByDate[] = [];
  
  // 按天统计
  for (let dayOffset = 0; dayOffset < days; dayOffset++) {
    const dayStart = startDate + (dayOffset * 86_400_000);
    const dayEnd = dayStart + 86_400_000;
    
    // 转换为日期字符串
    const date = new Date(dayStart).toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
    }).replace(/\//g, '-');
    
    // 统计当天完成的任务数
    const completed = allTodos.filter(t => 
      t.completed_at && t.completed_at >= dayStart && t.completed_at < dayEnd
    ).length;
    
    // 统计当天创建的任务数
    const created = allTodos.filter(t => 
      t.created_at >= dayStart && t.created_at < dayEnd
    ).length;
    
    stats.push({ date, completed, created });
  }
  
  return stats;
});
const activeCollapse = ref<string[]>(['todo', 'in_progress', 'done']);

const viewModeOptions = computed(() => [
  { label: t('stats.dashboard'), value: 'dashboard' },
  { label: t('stats.report'), value: 'report' },
]);

const dateRangeOptions = computed(() => [
  { label: t('stats.day'), value: 'day' },
  { label: t('stats.week'), value: 'week' },
  { label: t('stats.month'), value: 'month' },
  { label: t('stats.custom'), value: 'custom' },
]);

// Computed date range for filtering
const dateRangeFilter = computed(() => {
  const now = new Date();
  let startDate: number | undefined;
  let endDate: number | undefined;

  if (dateRange.value === 'day') {
    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate(), 0, 0, 0, 0);
    startDate = today.getTime();
    endDate = startDate + 86_400_000;
  } else if (dateRange.value === 'week') {
    const weekAgo = new Date(now.getTime() - 6 * 86_400_000);
    const start = new Date(weekAgo.getFullYear(), weekAgo.getMonth(), weekAgo.getDate(), 0, 0, 0, 0);
    startDate = start.getTime();
    endDate = startDate + 7 * 86_400_000;
  } else if (dateRange.value === 'month') {
    const monthAgo = new Date(now.getTime() - 29 * 86_400_000);
    const start = new Date(monthAgo.getFullYear(), monthAgo.getMonth(), monthAgo.getDate(), 0, 0, 0, 0);
    startDate = start.getTime();
    endDate = startDate + 30 * 86_400_000;
  } else if (dateRange.value === 'custom' && customStartDate && customEndDate) {
    startDate = customStartDate.value ?? undefined;
    endDate = customEndDate.value ?? undefined;
  }

  return { startDate, endDate };
});


const groupStats = computed(() => {
  const groups = groupStore.groups;
  // Use filtered tasks from statsWithDetails (which is already filtered by date range)
  const allFilteredTasks = [
    ...statsWithDetails.value.todos,
    ...statsWithDetails.value.in_progress_tasks,
    ...statsWithDetails.value.done_tasks,
  ];

  return groups.map(group => {
    const groupTodos = allFilteredTasks.filter(t => t.group_id === group.id);
    return {
      id: group.id,
      name: group.name,
      color: group.color || '#409EFF',
      total: groupTodos.length,
      done: groupTodos.filter(t => t.status === TodoStatus.Done).length,
    };
  }).filter(g => g.total > 0);
});

const tagStats = computed(() => {
  const tags = tagStore.tags;
  // Use filtered tasks from statsWithDetails (which is already filtered by date range)
  const allFilteredTasks = [
    ...statsWithDetails.value.todos,
    ...statsWithDetails.value.in_progress_tasks,
    ...statsWithDetails.value.done_tasks,
  ];

  return tags.map(tagItem => ({
    id: tagItem.id,
    name: tagItem.name,
    color: tagItem.color,
    count: allFilteredTasks.filter(t => t.tags?.some(tg => tg.id === tagItem.id)).length,
  })).filter(t => t.count > 0).sort((a, b) => b.count - a.count);
});
// All tasks sorted for report (from statsWithDetails which is already filtered by date range)
const allTasksSorted = computed(() => {
  // Use filtered tasks from statsWithDetails (which is already filtered by date range)
  const allFilteredTasks = [
    ...statsWithDetails.value.todos,
    ...statsWithDetails.value.in_progress_tasks,
    ...statsWithDetails.value.done_tasks,
  ];

  return allFilteredTasks.sort((a, b) => {
    if (a.status === TodoStatus.Done && b.status !== TodoStatus.Done) return 1;
    if (a.status !== TodoStatus.Done && b.status === TodoStatus.Done) return -1;
    return (b.created_at || 0) - (a.created_at || 0);
  });
});

const completedTasksSorted = computed(() => {
  return allTasksSorted.value.filter(t => t.status === TodoStatus.Done);
});

const incompleteTasksSorted = computed(() => {
  return allTasksSorted.value.filter(t => t.status !== TodoStatus.Done);
});

function formatFullDateTime(timestamp: number): string {
  const date = new Date(timestamp);
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');
  const hours = String(date.getHours()).padStart(2, '0');
  const minutes = String(date.getMinutes()).padStart(2, '0');
  return year + '/' + month + '/' + day + ' ' + hours + ':' + minutes;
}

const reportText = computed(() => {
  const lines: string[] = [];
  const showDetails = showTimeAndTags.value;
  const statusFilter = reportStatusFilter.value;

  // 如果没有选择任何状态，返回空
  if (statusFilter.length === 0) {
    return '';
  }

  // Helper function to add task metadata
  function addTaskMeta(task: any, targetLines: string[]) {
    if (!showDetails) return;
    const meta: string[] = [];
    if (task.group) {
      meta.push((task.group.icon || '📁') + ' ' + task.group.name);
    }
    if (task.tags && task.tags.length > 0) {
      meta.push(task.tags.map(tagItem => tagItem.name).join(', '));
    }
    if (task.start_date) {
      meta.push(t('todo.start') + ': ' + formatFullDateTime(task.start_date));
    }
    if (task.due_date) {
      meta.push(t('todo.due') + ': ' + formatFullDateTime(task.due_date));
    }
    if (task.completed_at) {
      meta.push(t('stats.completedAt') + ' ' + formatFullDateTime(task.completed_at));
    }
    if (meta.length > 0) {
      targetLines.push('   [' + meta.join(' | ') + ']');
    }
  }

  // Collect all steps with their parent tasks
  type StepWithParent = {
    step: any;
    task: any;
  };

  // 根据选择的状态筛选任务
  const allFilteredTasks = allTasksSorted.value.filter(task => {
    if (statusFilter.length === 0) return true; // 没有筛选时显示所有
    return statusFilter.includes(task.status);
  });

  if (allFilteredTasks.length === 0) {
    return '';
  }

  // 为每个状态生成独立的任务块
  statusFilter.forEach(statusId => {
    const statusTasks = allFilteredTasks.filter(task => task.status === statusId);

    if (statusTasks.length === 0) return;

    let statusTitle = '';
    if (statusId === 2) statusTitle = t('status.done');
    else if (statusId === 0) statusTitle = t('status.todo');
    else if (statusId === 1) statusTitle = t('status.inProgress');

    lines.push('=== ' + statusTitle + ' (' + statusTasks.length + ') ===');

    statusTasks.forEach((task, index) => {
      lines.push(String(index + 1) + '. ' + task.title);
      if (task.description) {
        lines.push('   ' + task.description);
      }
      if (task.steps && task.steps.length > 0) {
        task.steps.forEach(step => {
          const prefix = step.is_completed ? '[√]' : '[ ]';
          lines.push('   ' + prefix + ' ' + step.title);
        });
      }
      addTaskMeta(task, lines);
    });

    // 添加空行分隔不同状态的任务块
    if (statusFilter.indexOf(statusId) < statusFilter.length - 1) {
      lines.push('');
    }
  });

  return lines.join('\n');
});


async function copyReportText() {
  try {
    await navigator.clipboard.writeText(reportText.value);
    ElMessage.success(t('messages.success'));
  } catch {
    ElMessage.error(t('messages.error'));
  }
}


function getBarHeight(value: number, allData: StatsByDate[]): string {
  const max = Math.max(...allData.map(d => Math.max(d.created, d.completed)));
  if (max === 0) return '0%';
  return `${(value / max) * 100}%`;
}

function formatBarLabel(date: string): string {
  const d = new Date(date);
  if (dateRange.value === 'day') {
    return `${d.getMonth() + 1}/${d.getDate()}`;
  } else if (dateRange.value === 'week') {
    return `${t('stats.week')}${getWeekDay(d.getDay())}`;
  } else {
    return `${d.getMonth() + 1}${t('date.month')}`;
  }
}

function getWeekDay(day: number): string {
  const days = [t('date.sunday'), t('date.monday'), t('date.tuesday'), t('date.wednesday'), t('date.thursday'), t('date.friday'), t('date.saturday')];
  return days[day];
}

function getCompletionPercentage(done: number, total: number): number {
  if (total === 0) return 0;
  return Math.round((done / total) * 100);
}

function formatDate(timestamp: number): string {
  const date = new Date(timestamp);
  return `${date.getMonth() + 1}${t('date.month')}${date.getDate()}${t('date.day')}`;
}

function formatDateTime(timestamp: number): string {
  const date = new Date(timestamp);
  return `${date.getMonth() + 1}${t('date.month')}${date.getDate()}${t('date.day')} ${date.getHours()}:${String(date.getMinutes()).padStart(2, '0')}`;
}

async function loadStats() {
  try {
    const { startDate, endDate } = dateRangeFilter.value;

    console.log('[DesktopStatsView] loadStats called with:');
    console.log('  startDate:', startDate);
    console.log('  endDate:', endDate);
    console.log('  selectedGroupIds:', selectedGroupIds.value);
    console.log('  selectedTagIds:', selectedTagIds.value);

    // 调用API获取所有详细数据，支持任务组和标签筛选
    const result = await api.getStatsWithDetails(
      startDate,
      endDate,
      selectedGroupIds.value.length > 0 ? selectedGroupIds.value : undefined,
      selectedTagIds.value.length > 0 ? selectedTagIds.value : undefined
    );

    console.log('[DesktopStatsView] loadStats result:', {
      total: result.total,
      todo: result.todo,
      in_progress: result.in_progress,
      done: result.done,
    });

    statsWithDetails.value = result;
  } catch (error) {
    console.error('Failed to load stats:', error);
  }
}

watch([dateRange, customStartDate, customEndDate, selectedGroupIds, selectedTagIds], () => {
  loadStats();
});

onMounted(async () => {
  await Promise.all([
    todoStore.fetchTodos(),
    groupStore.fetchGroups(),
    tagStore.fetchTags(),
  ]);
  await loadStats();
});
</script>

<style scoped>
.stats-view {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

.stats-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 24px;
}

.header-controls {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
}

.filter-option {
  display: flex;
  align-items: center;
  gap: 8px;
}

.filter-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  flex-shrink: 0;
}

.page-title {
  font-size: 28px;
  font-weight: 600;
  margin: 0;
  color: var(--el-text-color-primary);
}

.stats-cards {
  display: grid; 
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 16px;
  margin-bottom: 24px;
}

.stat-card {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 20px;
  background: var(--el-bg-color);
  border-radius: 12px;
  border: 1px solid var(--el-border-color-light);
  transition: all 0.2s ease;
}

.stat-card:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
  transform: translateY(-2px);
}

.stat-icon {
  font-size: 32px;
}

.stat-content {
  flex: 1;
}

.stat-label {
  font-size: 13px;
  color: var(--el-text-color-secondary);
  margin-bottom: 4px;
}

.stat-value {
  font-size: 28px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.stats-section {
  background: var(--el-bg-color);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--el-border-color-light);
  margin-bottom: 16px;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  margin: 0 0 20px;
  color: var(--el-text-color-primary);
}

.chart-container {
  min-height: 200px;
}

.bar-chart {
  display: flex;
  align-items: flex-end;
  justify-content: space-around;
  gap: 8px;
  height: 200px;
  padding: 20px 0;
}

.bar-item {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
}

.bar-wrapper {
  display: flex;
  gap: 4px;
  align-items: flex-end;
  height: 160px;
}

.bar {
  width: 24px;
  border-radius: 4px 4px 0 0;
  transition: height 0.3s ease;
  min-height: 4px;
}

.bar.completed {
  background: var(--el-color-success);
}

.bar.created {
  background: var(--el-color-primary);
}

.bar-label {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.chart-legend {
  display: flex;
  justify-content: center;
  gap: 24px;
  margin-top: 16px;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: var(--el-text-color-regular);
}

.legend-color {
  width: 12px;
  height: 12px;
  border-radius: 2px;
}

.legend-color.completed {
  background: var(--el-color-success);
}

.legend-color.created {
  background: var(--el-color-primary);
}

.group-stats {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.group-stat-item {
  display: flex;
  align-items: center;
  gap: 12px;
}

.group-info {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 120px;
}

.group-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
}

.group-name {
  font-size: 14px;
  color: var(--el-text-color-primary);
}

.group-progress {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 12px;
}

.group-progress :deep(.el-progress) {
  flex: 1;
}

.group-count {
  min-width: 60px;
  text-align: right;
  font-size: 13px;
  color: var(--el-text-color-secondary);
}

.tag-stats {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.tag-stat-item {
  padding: 6px 14px;
  border-radius: 16px;
  font-size: 13px;
  color: white;
}

/* Task Collapse Styles */
.task-collapse {
  border: none;
}

.task-collapse :deep(.el-collapse-item__header) {
  background: transparent;
  border: none;
  padding: 12px 0;
  font-size: 15px;
  font-weight: 500;
}

.collapse-title {
  display: flex;
  align-items: center;
  gap: 8px;
}

.status-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  display: inline-block;
}

.status-dot.todo {
  background: var(--el-text-color-secondary);
}

.status-dot.in-progress {
  background: var(--el-color-primary);
}

.status-dot.done {
  background: var(--el-color-success);
}

.status-dot-small {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  display: inline-block;
}

.status-dot-small.todo {
  background: var(--el-text-color-secondary);
}

.status-dot-small.in-progress {
  background: var(--el-color-primary);
}

.status-dot-small.done {
  background: var(--el-color-success);
}

.task-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 8px 0;
}

.task-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background: var(--el-fill-color-light);
  border-radius: 8px;
  transition: all 0.2s ease;
}

.task-item:hover {
  background: var(--el-fill-color);
}

.task-item.completed {
  opacity: 0.7;
}

.task-item.completed .task-title {
  text-decoration: line-through;
  color: var(--el-text-color-secondary);
}

.task-title {
  font-size: 14px;
  color: var(--el-text-color-primary);
  font-weight: 500;
}

.task-meta {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.task-group {
  display: flex;
  align-items: center;
  gap: 4px;
}

.task-due {
  color: var(--el-color-warning);
}

.task-completed {
  color: var(--el-color-success);
}

/* Dark theme */
:global(html.dark) .page-title {
  color: var(--el-text-color-primary);
}

:global(html.dark) .stat-card {
  background: var(--el-fill-color-light);
  border-color: var(--el-border-color);
}

:global(html.dark) .stat-value {
  color: var(--el-text-color-primary);
}

:global(html.dark) .stats-section {
  background: var(--el-fill-color-light);
  border-color: var(--el-border-color);
}

:global(html.dark) .section-title {
  color: var(--el-text-color-primary);
}

:global(html.dark) .group-name {
  color: var(--el-text-color-primary);
}

:global(html.dark) .task-item {
  background: var(--el-fill-color);
}

:global(html.dark) .task-item:hover {
  background: var(--el-fill-color-light);
}

:global(html.dark) .task-title {
  color: var(--el-text-color-primary);
}

:global(html.dark) .task-item.completed .task-title {
  color: var(--el-text-color-secondary);
}

/* Report Mode Styles */
.report-section {
  background: var(--el-bg-color);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--el-border-color-light);
}

.report-actions {
  margin-bottom: 16px;
}

.report-textarea {
  width: 100%;
  min-height: 400px;
  padding: 16px;
  font-family: 'Microsoft YaHei', 'SimSun', 'SimHei', 'PingFang SC', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  font-size: 13px;
  line-height: 1.6;
  border: 1px solid var(--el-border-color);
  border-radius: 8px;
  background: var(--el-fill-color-light);
  color: var(--el-text-color-primary);
  resize: vertical;
}

.report-textarea:focus {
  outline: none;
  border-color: var(--el-color-primary);
}

:global(html.dark) .report-section {
  background: var(--el-fill-color-light);
  border-color: var(--el-border-color);
}

:global(html.dark) .report-textarea {
  background: var(--el-fill-color);
  border-color: var(--el-border-color);
}

.custom-date-range {
  display: flex;
  align-items: center;
  margin-left: 12px;
  gap: 4px;
}

/* Mobile responsive styles */
@media (max-width: 768px) {
  .stats-view {
    padding: 12px;
  }

  .stats-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
    margin-bottom: 16px;
  }

  .page-title {
    display: none; /* 隐藏标题 */
  }

  .header-controls {
    display: flex; /* 保持控制器显示 */
    flex-wrap: wrap;
    width: 100%;
    gap: 8px;
  }

  .header-controls :deep(.el-segmented) {
    flex: 1;
    min-width: 0;
  }

  /* 缩小移动端按钮组 */
  .header-controls :deep(.el-segmented__item) {
    padding: 4px 8px;
    font-size: 12px;
    height: 28px;
    line-height: 20px;
  }

  .header-controls :deep(.el-segmented) {
    height: 28px;
  }

  .header-controls :deep(.el-select) {
    width: 100% !important;
    margin: 4px 0;
  }

  .custom-date-range {
    flex-direction: column;
    width: 100%;
    margin-left: 0;
    margin-top: 8px;
    align-items: stretch;
  }

  .custom-date-range :deep(.el-date-picker) {
    width: 100% !important;
    margin: 4px 0;
  }

  .stats-cards {
    grid-template-columns: repeat(2, 1fr);
    gap: 8px;
    margin-bottom: 16px;
  }

  .stat-card {
    padding: 12px;
    gap: 8px;
  }

  .stat-icon {
    display: none; /* 移动端隐藏图标 */
  }

  .stat-label {
    font-size: 11px;
  }

  .stat-value {
    font-size: 20px;
  }

  .stats-section {
    padding: 12px;
    margin-bottom: 12px;
  }

  .section-title {
    font-size: 14px;
    margin-bottom: 12px;
  }

  .bar-chart {
    height: 160px;
    padding: 10px 0;
    gap: 4px;
  }

  .bar-wrapper {
    height: 120px;
  }

  .bar {
    width: 16px;
  }

  .bar-label {
    font-size: 10px;
  }

  .chart-legend {
    gap: 16px;
    margin-top: 12px;
  }

  .legend-item {
    font-size: 12px;
  }

  .group-stat-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }

  .group-info {
    min-width: auto;
  }

  .group-progress {
    width: 100%;
  }

  .group-count {
    min-width: auto;
    text-align: left;
  }

  .task-item {
    padding: 10px 12px;
  }

  .task-title {
    font-size: 13px;
  }

  .task-meta {
    font-size: 11px;
  }

  .tag-stat-item {
    font-size: 12px;
    padding: 4px 10px;
  }

  .report-textarea {
    min-height: 300px;
    font-size: 12px;
  }

  .report-actions {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .report-actions :deep(.el-button) {
    width: 100%;
  }

  .report-actions :deep(.el-checkbox) {
    margin: 0;
  }
}
</style>
