<template>
  <div class="stats-view">
    <div class="stats-header">
      <h1 class="page-title">任务统计</h1>
      <el-segmented v-model="dateRange" :options="dateRangeOptions" />
    </div>

    <!-- Overview Cards -->
    <div class="stats-cards">
      <div class="stat-card total">
        <div class="stat-icon">📊</div>
        <div class="stat-content">
          <div class="stat-label">总任务数</div>
          <div class="stat-value">{{ stats.total }}</div>
        </div>
      </div>
      <div class="stat-card todo">
        <div class="stat-icon">📝</div>
        <div class="stat-content">
          <div class="stat-label">待办</div>
          <div class="stat-value">{{ stats.todo }}</div>
        </div>
      </div>
      <div class="stat-card in-progress">
        <div class="stat-icon">🔄</div>
        <div class="stat-content">
          <div class="stat-label">进行中</div>
          <div class="stat-value">{{ stats.in_progress }}</div>
        </div>
      </div>
      <div class="stat-card done">
        <div class="stat-icon">✅</div>
        <div class="stat-content">
          <div class="stat-label">已完成</div>
          <div class="stat-value">{{ stats.done }}</div>
        </div>
      </div>
      <div class="stat-card marked">
        <div class="stat-icon">⭐</div>
        <div class="stat-content">
          <div class="stat-label">重要</div>
          <div class="stat-value">{{ stats.marked }}</div>
        </div>
      </div>
    </div>

    <!-- Task Details by Status -->
    <div class="stats-section">
      <h2 class="section-title">任务详情</h2>

      <!-- Pending Tasks -->
      <el-collapse v-model="activeCollapse" class="task-collapse">
        <el-collapse-item title="待办任务" name="todo">
          <template #title>
            <span class="collapse-title">
              <span class="status-dot todo"></span>
              <span>待办任务 ({{ statsWithDetails.todos.length }})</span>
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
          <el-empty v-else description="暂无待办任务" :image-size="40" />
        </el-collapse-item>

        <!-- In Progress Tasks -->
        <el-collapse-item title="进行中" name="in_progress">
          <template #title>
            <span class="collapse-title">
              <span class="status-dot in-progress"></span>
              <span>进行中 ({{ statsWithDetails.in_progress_tasks.length }})</span>
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
          <el-empty v-else description="暂无进行中任务" :image-size="40" />
        </el-collapse-item>

        <!-- Completed Tasks -->
        <el-collapse-item title="已完成" name="done">
          <template #title>
            <span class="collapse-title">
              <span class="status-dot done"></span>
              <span>已完成 ({{ statsWithDetails.done_tasks.length }})</span>
            </span>
          </template>
          <div v-if="statsWithDetails.done_tasks.length > 0" class="task-list">
            <div v-for="task in statsWithDetails.done_tasks" :key="task.id" class="task-item completed">
              <span class="task-title">{{ task.title }}</span>
              <div class="task-meta">
                <span v-if="task.completed_at" class="task-completed">完成于 {{ formatDateTime(task.completed_at) }}</span>
              </div>
            </div>
          </div>
          <el-empty v-else description="暂无已完成任务" :image-size="40" />
        </el-collapse-item>
      </el-collapse>
    </div>

    <!-- Completion Trend -->
    <div class="stats-section">
      <h2 class="section-title">完成趋势</h2>
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
        <el-empty v-else description="暂无数据" :image-size="60" />
      </div>
      <div class="chart-legend">
        <span class="legend-item">
          <span class="legend-color completed"></span>
          <span>已完成</span>
        </span>
        <span class="legend-item">
          <span class="legend-color created"></span>
          <span>已创建</span>
        </span>
      </div>
    </div>

    <!-- Group Distribution -->
    <div v-if="groupStats.length > 0" class="stats-section">
      <h2 class="section-title">任务组分布</h2>
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
      <h2 class="section-title">标签使用</h2>
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
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue';
import { useTodoStore } from '@/stores';
import { useGroupStore } from '@/stores';
import { useTagStore } from '@/stores';
import type { TodoStats, StatsByDate, TodoStatsWithDetails } from '@/types';
import { TodoStatus } from '@/types';
import * as api from '@/api/tauri';

const todoStore = useTodoStore();
const groupStore = useGroupStore();
const tagStore = useTagStore();

const dateRange = ref<'day' | 'week' | 'month'>('week');
const stats = ref<TodoStats>({
  total: 0,
  todo: 0,
  in_progress: 0,
  done: 0,
  overdue: 0,
  marked: 0,
});
const dateStats = ref<StatsByDate[]>([]);
const statsWithDetails = ref<TodoStatsWithDetails>({
  total: 0,
  todo: 0,
  in_progress: 0,
  done: 0,
  todos: [],
  in_progress_tasks: [],
  done_tasks: [],
});
const activeCollapse = ref<string[]>(['todo', 'in_progress', 'done']);

const dateRangeOptions = [
  { label: '按天', value: 'day' },
  { label: '按周', value: 'week' },
  { label: '按月', value: 'month' },
];

const groupStats = computed(() => {
  const groups = groupStore.groups;
  const todos = todoStore.todos;

  return groups.map(group => {
    const groupTodos = todos.filter(t => t.group_id === group.id);
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
  const todos = todoStore.todos;

  return tags.map(tag => ({
    id: tag.id,
    name: tag.name,
    color: tag.color,
    count: todos.filter(t => t.tags?.some(tg => tg.id === tag.id)).length,
  })).filter(t => t.count > 0).sort((a, b) => b.count - a.count);
});

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
    return `周${getWeekDay(d.getDay())}`;
  } else {
    return `${d.getMonth() + 1}月`;
  }
}

function getWeekDay(day: number): string {
  const days = ['日', '一', '二', '三', '四', '五', '六'];
  return days[day];
}

function getCompletionPercentage(done: number, total: number): number {
  if (total === 0) return 0;
  return Math.round((done / total) * 100);
}

function formatDate(timestamp: number): string {
  const date = new Date(timestamp);
  return `${date.getMonth() + 1}月${date.getDate()}日`;
}

function formatDateTime(timestamp: number): string {
  const date = new Date(timestamp);
  return `${date.getMonth() + 1}月${date.getDate()}日 ${date.getHours()}:${String(date.getMinutes()).padStart(2, '0')}`;
}

async function loadStats() {
  try {
    // Calculate date range based on selection
    const now = new Date();
    let startDate: number | undefined;
    let endDate: number | undefined;

    if (dateRange.value === 'day') {
      // Today: from 00:00 to 23:59
      const today = new Date(now.getFullYear(), now.getMonth(), now.getDate(), 0, 0, 0, 0);
      startDate = today.getTime();
      endDate = startDate + 86_400_000; // Add one day
    } else if (dateRange.value === 'week') {
      // Last 7 days
      const weekAgo = new Date(now.getTime() - 6 * 86_400_000);
      const start = new Date(weekAgo.getFullYear(), weekAgo.getMonth(), weekAgo.getDate(), 0, 0, 0, 0);
      startDate = start.getTime();
      endDate = startDate + 7 * 86_400_000; // Add 7 days
    } else if (dateRange.value === 'month') {
      // Last 30 days
      const monthAgo = new Date(now.getTime() - 29 * 86_400_000);
      const start = new Date(monthAgo.getFullYear(), monthAgo.getMonth(), monthAgo.getDate(), 0, 0, 0, 0);
      startDate = start.getTime();
      endDate = startDate + 30 * 86_400_000; // Add 30 days
    }

    const [statsData, dateStatsData, statsWithDetailsData] = await Promise.all([
      todoStore.getStats(),
      todoStore.getStatsByDate(dateRange.value),
      api.getStatsWithDetails(startDate, endDate),
    ]);
    stats.value = statsData;
    dateStats.value = dateStatsData;
    statsWithDetails.value = statsWithDetailsData;
  } catch (error) {
    console.error('Failed to load stats:', error);
  }
}

watch(dateRange, loadStats);

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

.page-title {
  font-size: 28px;
  font-weight: 600;
  margin: 0;
  color: #303133;
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
  background: white;
  border-radius: 12px;
  border: 1px solid #e4e7ed;
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
  color: #909399;
  margin-bottom: 4px;
}

.stat-value {
  font-size: 28px;
  font-weight: 600;
  color: #303133;
}

.stats-section {
  background: white;
  border-radius: 12px;
  padding: 20px;
  border: 1px solid #e4e7ed;
  margin-bottom: 16px;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  margin: 0 0 20px;
  color: #303133;
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
  background: #67c23a;
}

.bar.created {
  background: #409eff;
}

.bar-label {
  font-size: 12px;
  color: #909399;
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
  color: #606266;
}

.legend-color {
  width: 12px;
  height: 12px;
  border-radius: 2px;
}

.legend-color.completed {
  background: #67c23a;
}

.legend-color.created {
  background: #409eff;
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
  color: #303133;
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
  color: #909399;
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
  background: #909399;
}

.status-dot.in-progress {
  background: #409eff;
}

.status-dot.done {
  background: #67c23a;
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
  background: #f5f7fa;
  border-radius: 8px;
  transition: all 0.2s ease;
}

.task-item:hover {
  background: #e4e7ed;
}

.task-item.completed {
  opacity: 0.7;
}

.task-item.completed .task-title {
  text-decoration: line-through;
  color: #909399;
}

.task-title {
  font-size: 14px;
  color: #303133;
  font-weight: 500;
}

.task-meta {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 12px;
  color: #909399;
}

.task-group {
  display: flex;
  align-items: center;
  gap: 4px;
}

.task-due {
  color: #e6a23c;
}

.task-completed {
  color: #67c23a;
}

/* Dark theme */
[data-theme='dark'] .page-title {
  color: #e0e0e0;
}

[data-theme='dark'] .stat-card {
  background: #2a2a2a;
  border-color: #3a3a3a;
}

[data-theme='dark'] .stat-value {
  color: #e0e0e0;
}

[data-theme='dark'] .stats-section {
  background: #2a2a2a;
  border-color: #3a3a3a;
}

[data-theme='dark'] .section-title {
  color: #e0e0e0;
}

[data-theme='dark'] .group-name {
  color: #e0e0e0;
}

[data-theme='dark'] .task-item {
  background: #3a3a3a;
}

[data-theme='dark'] .task-item:hover {
  background: #4a4a4a;
}

[data-theme='dark'] .task-title {
  color: #e0e0e0;
}

[data-theme='dark'] .task-item.completed .task-title {
  color: #909399;
}
</style>
