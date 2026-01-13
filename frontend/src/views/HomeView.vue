<template>
  <div class="home-view">
    <!-- Todo List -->
    <div v-if="viewMode === 'list'" class="todo-list">
      <div
        v-for="todo in filteredTodos"
        :key="todo.id"
        class="todo-item"
        :class="{ completed: todo.status === 'done' }"
        @click="selectTodo(todo)"
      >
        <div class="todo-left">
          <el-checkbox
            :model-value="todo.status === 'done'"
            @change="toggleStatus(todo)"
            @click.stop
          />
          <div class="todo-content">
            <div class="todo-title">
              <el-tag
                v-if="todo.is_marked"
                type="warning"
                size="small"
                effect="plain"
              >
                重要
              </el-tag>
              <span class="title-text">{{ todo.title }}</span>
            </div>
            <div class="todo-meta">
              <span class="meta-item">
                <el-icon><Calendar /></el-icon>
                {{ formatDate(todo.due_date, todo.status) }}
              </span>
              <el-tag
                v-for="tag in todo.tags"
                :key="tag.id"
                size="small"
                :style="{ backgroundColor: tag.color }"
              >
                {{ tag.name }}
              </el-tag>
            </div>
          </div>
        </div>
        <div class="todo-actions">
          <el-button
            :icon="todo.is_marked ? StarFilled : Star"
            circle
            text
            @click.stop="toggleMark(todo)"
          />
        </div>
      </div>
    </div>

    <!-- Card View -->
    <div v-else class="todo-cards">
      <div
        v-for="todo in filteredTodos"
        :key="todo.id"
        class="todo-card"
        :class="{ completed: todo.status === 'done' }"
        @click="selectTodo(todo)"
      >
        <div class="card-header">
          <el-checkbox
            :model-value="todo.status === 'done'"
            @change="toggleStatus(todo)"
            @click.stop
          />
          <el-button
            :icon="todo.is_marked ? StarFilled : Star"
            circle
            text
            size="small"
            @click.stop="toggleMark(todo)"
          />
        </div>
        <div class="card-content">
          <div class="card-title">
            <el-tag
              v-if="todo.is_marked"
              type="warning"
              size="small"
              effect="plain"
            >
              重要
            </el-tag>
            <span class="title-text">{{ todo.title }}</span>
          </div>
          <div v-if="todo.description" class="card-description">
            {{ todo.description }}
          </div>
        </div>
        <div class="card-footer">
          <div class="card-meta">
            <span v-if="todo.due_date" class="meta-item">
              <el-icon><Calendar /></el-icon>
              {{ formatDate(todo.due_date, todo.status) }}
            </span>
            <div class="card-tags">
              <el-tag
                v-for="tag in todo.tags?.slice(0, 3)"
                :key="tag.id"
                size="small"
                :style="{ backgroundColor: tag.color }"
              >
                {{ tag.name }}
              </el-tag>
              <span v-if="todo.tags && todo.tags.length > 3" class="more-tags">
                +{{ todo.tags.length - 3 }}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <el-empty
      v-if="filteredTodos.length === 0 && !loading"
      description="暂无任务，点击上方新建任务开始使用"
    />

    <!-- Loading -->
    <div v-if="loading" class="loading-state">
      <el-skeleton :rows="5" animated />
    </div>

    <!-- Detail Panel (when a todo is selected) -->
    <el-drawer
      v-model="detailVisible"
      title="任务详情"
      size="40%"
      direction="rtl"
    >
      <TodoDetailPanel
        v-if="selectedTodo"
        :todo="selectedTodo"
        @updated="handleTodoUpdated"
        @deleted="handleTodoDeleted"
      />
    </el-drawer>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { Calendar, Star, StarFilled } from '@element-plus/icons-vue';
import { ElMessageBox, ElMessage } from 'element-plus';
import { useTodoStore } from '@/stores';
import { useUIStore } from '@/stores';
import type { Todo } from '@/types';
import TodoDetailPanel from '../components/todo/TodoDetailPanel.vue';

const todoStore = useTodoStore();
const uiStore = useUIStore();

const loading = computed(() => todoStore.loading);
const filteredTodos = computed(() => todoStore.filteredTodos);
const viewMode = computed(() => uiStore.viewMode);

const selectedTodo = ref<Todo | null>(null);
const detailVisible = ref(false);

async function toggleStatus(todo: Todo) {
  try {
    const newStatus = todo.status === 'done' ? 'todo' : 'done';
    await todoStore.updateTodoStatus(todo.id, newStatus);
  } catch (error) {
    console.error('Failed to toggle status:', error);
    ElMessage.error('状态更新失败');
  }
}

async function toggleMark(todo: Todo) {
  try {
    await todoStore.toggleTodoMark(todo.id);
  } catch (error) {
    console.error('Failed to toggle mark:', error);
    ElMessage.error('标记更新失败');
  }
}

function selectTodo(todo: Todo) {
  selectedTodo.value = todo;
  detailVisible.value = true;
}

function handleTodoUpdated(todo: Todo) {
  selectedTodo.value = todo;
  // Don't refetch - the store already updated the todos array in updateTodo/updateTodoStatus
  // fetchTodos would overwrite the updated data with stale data from the server
}

function handleTodoDeleted() {
  detailVisible.value = false;
  selectedTodo.value = null;
  todoStore.fetchTodos();
}

function formatDate(timestamp?: number, todoStatus?: string): string {
  if (!timestamp) return '无截止日期';
  const date = new Date(timestamp);
  const now = new Date();
  const isOverdue = date < now && todoStatus !== 'done';

  const options: Intl.DateTimeFormatOptions = {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  };

  const formatted = date.toLocaleDateString('zh-CN', options);
  return isOverdue ? `${formatted} (已逾期)` : formatted;
}
</script>

<style scoped>
.home-view {
  width: 100%;
  height: 100%;
}

.todo-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.todo-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px;
  background: white;
  border-radius: 8px;
  border: 1px solid #e4e7ed;
  cursor: pointer;
  transition: all 0.2s ease;
}

.todo-item:hover {
  border-color: #409eff;
  box-shadow: 0 2px 8px rgba(64, 158, 255, 0.1);
}

.todo-item.completed {
  opacity: 0.6;
}

.todo-item.completed .title-text {
  text-decoration: line-through;
  color: #909399;
}

.todo-left {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  flex: 1;
}

.todo-content {
  flex: 1;
}

.todo-title {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}

.title-text {
  font-size: 16px;
  font-weight: 500;
  color: #303133;
}

.todo-meta {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 13px;
  color: #909399;
}

.meta-item {
  display: flex;
  align-items: center;
  gap: 4px;
}

.todo-cards {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 16px;
}

.todo-card {
  background: white;
  border-radius: 12px;
  border: 1px solid #e4e7ed;
  padding: 16px;
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.todo-card:hover {
  border-color: #409eff;
  box-shadow: 0 4px 12px rgba(64, 158, 255, 0.15);
  transform: translateY(-2px);
}

.todo-card.completed {
  opacity: 0.6;
}

.todo-card.completed .title-text {
  text-decoration: line-through;
  color: #909399;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.card-content {
  flex: 1;
}

.card-title {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}

.card-title .title-text {
  font-size: 16px;
  font-weight: 500;
  color: #303133;
  line-height: 1.4;
}

.card-description {
  font-size: 13px;
  color: #606266;
  line-height: 1.5;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
  text-overflow: ellipsis;
}

.card-footer {
  margin-top: auto;
}

.card-meta {
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 13px;
}

.card-meta .meta-item {
  display: flex;
  align-items: center;
  gap: 4px;
  color: #909399;
}

.card-tags {
  display: flex;
  align-items: center;
  gap: 4px;
}

.more-tags {
  font-size: 12px;
  color: #909399;
  padding: 0 4px;
}

.loading-state {
  padding: 20px;
}

/* Dark theme */
[data-theme='dark'] .todo-item {
  background: #2a2a2a;
  border-color: #3a3a3a;
}

[data-theme='dark'] .todo-item:hover {
  border-color: #409eff;
}

[data-theme='dark'] .title-text {
  color: #e0e0e0;
}

[data-theme='dark'] .todo-meta {
  color: #a0a0a0;
}

[data-theme='dark'] .todo-card {
  background: #2a2a2a;
  border-color: #3a3a3a;
}

[data-theme='dark'] .todo-card:hover {
  border-color: #409eff;
}

[data-theme='dark'] .todo-card .title-text {
  color: #e0e0e0;
}

[data-theme='dark'] .todo-card .card-description {
  color: #b0b0b0;
}

[data-theme='dark'] .todo-card .meta-item {
  color: #a0a0a0;
}

[data-theme='dark'] .todo-card .more-tags {
  color: #a0a0a0;
}
</style>
