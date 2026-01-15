<template>
  <div class="home-view">
    <!-- Todo List -->
    <div v-if="viewMode === 'list'" class="todo-list">
      <div
        v-for="todo in filteredTodos"
        :key="todo.id"
        class="todo-item"
        :class="{ completed: todo.status === TodoStatus.Done }"
      >
        <div class="todo-main" @click="selectTodo(todo)">
          <div class="todo-left">
            <el-checkbox
              :model-value="todo.status === TodoStatus.Done"
              @change="toggleStatus(todo)"
              @click.stop
            />
            <div class="todo-content">
              <div class="todo-title">
                <el-tag
                  v-if="todo.priority >= 1"
                  :type="todo.priority === 3 ? 'danger' : 'warning'"
                  size="small"
                  effect="plain"
                >
                  {{ todo.priority === 3 ? '紧急' : '重要' }}
                </el-tag>
                <span class="title-text">{{ todo.title }}</span>
              </div>
              <div v-if="todo.description" class="todo-description">
                {{ todo.description }}
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
              v-if="todo.steps && todo.steps.length > 0"
              :icon="expandedTodos.has(todo.id) ? ArrowDown : ArrowRight"
              circle
              text
              @click.stop="toggleExpand(todo)"
            />
            <el-button
              :icon="todo.priority >= 1 ? StarFilled : Star"
              circle
              text
              @click.stop="toggleMark(todo)"
            />
          </div>
        </div>
        <!-- Sub-steps (collapsible) -->
        <div v-if="todo.steps && todo.steps.length > 0 && expandedTodos.has(todo.id)" class="todo-steps">
          <div
            v-for="step in todo.steps"
            :key="step.id"
            class="step-item"
            @click.stop="toggleStep(step)"
          >
            <el-checkbox :model-value="step.is_completed" @click.stop />
            <span class="step-text" :class="{ completed: step.is_completed }">
              {{ step.title }}
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- Card View -->
    <div v-else class="todo-cards">
      <div
        v-for="todo in filteredTodos"
        :key="todo.id"
        class="todo-card"
        :class="{ completed: todo.status === TodoStatus.Done }"
      >
        <div class="card-header">
          <el-checkbox
            :model-value="todo.status === TodoStatus.Done"
            @change="toggleStatus(todo)"
            @click.stop
          />
          <div style="flex: 1" @click="selectTodo(todo)">
            <div class="card-title">
              <el-tag
                v-if="todo.priority >= 1"
                :type="todo.priority === 3 ? 'danger' : 'warning'"
                size="small"
                effect="plain"
              >
                {{ todo.priority === 3 ? '紧急' : '重要' }}
              </el-tag>
              <span class="title-text">{{ todo.title }}</span>
            </div>
          </div>
          <el-button
            v-if="todo.steps && todo.steps.length > 0"
            :icon="expandedTodos.has(todo.id) ? ArrowDown : ArrowRight"
            circle
            text
            size="small"
            @click.stop="toggleExpand(todo)"
          />
          <el-button
            :icon="todo.priority >= 1 ? StarFilled : Star"
            circle
            text
            size="small"
            @click.stop="toggleMark(todo)"
          />
        </div>
        <div class="card-content">
          <div v-if="todo.description" class="card-description">
            {{ todo.description }}
          </div>
        </div>
        <!-- Sub-steps (collapsible) -->
        <div v-if="todo.steps && todo.steps.length > 0 && expandedTodos.has(todo.id)" class="card-steps">
          <div
            v-for="step in todo.steps"
            :key="step.id"
            class="step-item"
            @click.stop="toggleStep(step)"
          >
            <el-checkbox :model-value="step.is_completed" @click.stop />
            <span class="step-text" :class="{ completed: step.is_completed }">
              {{ step.title }}
            </span>
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
        :key="selectedTodo.id + '-' + selectedTodo.updated_at"
        :todo="selectedTodo"
        @updated="handleTodoUpdated"
        @deleted="handleTodoDeleted"
      />
    </el-drawer>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, toRef } from 'vue';
import { Calendar, Star, StarFilled, ArrowDown, ArrowRight } from '@element-plus/icons-vue';
import { ElMessageBox, ElMessage } from 'element-plus';
import { useTodoStore } from '@/stores';
import { useUIStore } from '@/stores';
import type { Todo, TodoStep } from '@/types';
import { TodoStatus } from '@/types';
import TodoDetailPanel from '../components/todo/TodoDetailPanel.vue';
import * as api from '@/api/tauri';

const todoStore = useTodoStore();
const uiStore = useUIStore();

const loading = computed(() => todoStore.loading);
const filteredTodos = computed(() => todoStore.filteredTodos);
const viewMode = toRef(uiStore, 'viewMode');

// Debug logging
console.log('[HomeView] Component setup - viewMode:', viewMode.value);
console.log('[HomeView] Component setup - filteredTodos:', filteredTodos.value);

// Watch for changes
watch(filteredTodos, (newVal) => {
  console.log('[HomeView] filteredTodos changed, length:', newVal?.length);
  console.log('[HomeView] filteredTodos data:', newVal);
}, { immediate: true });

watch(viewMode, (newVal) => {
  console.log('[HomeView] viewMode changed:', newVal);
}, { immediate: true });

const selectedTodo = ref<Todo | null>(null);
const detailVisible = ref(false);
const expandedTodos = ref<Set<number>>(new Set());

// Watch for store changes and update selectedTodo if it's the same todo
watch(() => todoStore.todos, (newTodos) => {
  console.log('[HomeView] todoStore.todos changed, length:', newTodos.length);
  if (selectedTodo.value) {
    const updatedTodo = newTodos.find(t => t.id === selectedTodo.value!.id);
    console.log('[HomeView] Looking for todo with id:', selectedTodo.value!.id, 'found:', !!updatedTodo);
    if (updatedTodo) {
      console.log('[HomeView] Syncing selectedTodo with store data');
      console.log('[HomeView] Old start_date:', selectedTodo.value.start_date, 'New start_date:', updatedTodo.start_date);
      console.log('[HomeView] Old due_date:', selectedTodo.value.due_date, 'New due_date:', updatedTodo.due_date);
      console.log('[HomeView] Old updated_at:', selectedTodo.value.updated_at, 'New updated_at:', updatedTodo.updated_at);
      selectedTodo.value = updatedTodo;
      console.log('[HomeView] selectedTodo synced, new key would be:', updatedTodo.id + '-' + updatedTodo.updated_at);
    }
  }
}, { deep: true });

async function toggleStatus(todo: Todo) {
  try {
    const newStatus = todo.status === TodoStatus.Done ? TodoStatus.Todo : TodoStatus.Done;
    console.log('toggleStatus called:', { todoId: todo.id, oldStatus: todo.status, newStatus });

    const updated = await todoStore.updateTodoStatus(todo.id, newStatus);

    console.log('toggleStatus result:', updated);
    console.log('Current filteredTodos:', filteredTodos.map(t => ({ id: t.id, status: t.status })));
  } catch (error: any) {
    console.error('Failed to toggle status:', error);
    const errorMsg = error?.toString() || JSON.stringify(error) || 'Unknown error';
    ElMessage.error(`状态更新失败: ${errorMsg}`);
  }
}

async function toggleMark(todo: Todo) {
  try {
    // Cycle through priorities: 0 (Normal) → 1 (Important) → 3 (Urgent) → 0 (Normal)
    const newPriority = todo.priority === 0 ? 1 : (todo.priority === 1 ? 3 : 0);
    await todoStore.updateTodo({
      id: todo.id,
      priority: newPriority,
    });
  } catch (error) {
    console.error('Failed to toggle priority:', error);
    ElMessage.error('优先级更新失败');
  }
}

function selectTodo(todo: Todo) {
  selectedTodo.value = todo;
  detailVisible.value = true;
}

function toggleExpand(todo: Todo) {
  if (expandedTodos.value.has(todo.id)) {
    expandedTodos.value.delete(todo.id);
  } else {
    expandedTodos.value.add(todo.id);
  }
}

async function toggleStep(step: TodoStep) {
  try {
    await api.toggleStep(step.id);
    // Refresh the todo to get updated steps
    const updatedTodo = await api.getTodo(step.todo_id);
    const todoIndex = todoStore.todos.findIndex(t => t.id === step.todo_id);
    if (todoIndex !== -1) {
      todoStore.todos[todoIndex] = updatedTodo;
    }
    if (selectedTodo.value?.id === step.todo_id) {
      selectedTodo.value = updatedTodo;
    }
  } catch (error) {
    console.error('Failed to toggle step:', error);
    ElMessage.error('步骤状态更新失败');
  }
}

async function handleTodoUpdated(todo: Todo) {
  console.log('[HomeView] handleTodoUpdated called');
  // Wait for store to update, then sync selectedTodo
  // Give the store time to update first
  await new Promise(resolve => setTimeout(resolve, 100));

  // Then update selectedTodo from the store
  const updatedInStore = todoStore.todos.find(t => t.id === todo.id);
  if (updatedInStore) {
    console.log('[HomeView] Syncing selectedTodo from store after update');
    selectedTodo.value = updatedInStore;
  }
}

function handleTodoDeleted() {
  detailVisible.value = false;
  selectedTodo.value = null;
  todoStore.fetchTodos();
}

function formatDate(timestamp?: number, todoStatus?: TodoStatus): string {
  if (!timestamp) return '无截止日期';
  const date = new Date(timestamp);
  const now = new Date();
  const isOverdue = date < now && todoStatus !== TodoStatus.Done;

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
  flex-direction: column;
  padding: 16px;
  background: white;
  border-radius: 8px;
  border: 1px solid #e4e7ed;
  cursor: pointer;
  transition: all 0.2s ease;
}

.todo-item:hover {
  border-color: var(--el-color-primary);
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

.todo-main {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
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

.todo-description {
  font-size: 13px;
  color: #606266;
  line-height: 1.5;
  margin-bottom: 8px;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  text-overflow: ellipsis;
}

.todo-steps {
  margin-top: 12px;
  padding-top: 12px;
  padding-left: 40px;
  border-top: 1px solid #e4e7ed;
}

.step-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 0;
  cursor: pointer;
}

.step-text {
  font-size: 13px;
  color: #606266;
}

.step-text.completed {
  text-decoration: line-through;
  color: #909399;
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
  border-color: var(--el-color-primary);
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
  gap: 8px;
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

.card-steps {
  padding: 12px 0 12px 24px;
  border-top: 1px solid #e4e7ed;
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

/* Note: hover colors are already using var(--el-color-primary) */

[data-theme='dark'] .title-text {
  color: #e0e0e0;
}

[data-theme='dark'] .todo-description {
  color: #b0b0b0;
}

[data-theme='dark'] .todo-meta {
  color: #a0a0a0;
}

[data-theme='dark'] .todo-steps {
  border-top-color: #3a3a3a;
}

[data-theme='dark'] .step-text {
  color: #b0b0b0;
}

[data-theme='dark'] .step-text.completed {
  color: #909399;
}

[data-theme='dark'] .todo-card {
  background: #2a2a2a;
  border-color: #3a3a3a;
}

[data-theme='dark'] .todo-card:hover {
  border-color: var(--el-color-primary);
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

[data-theme='dark'] .card-steps {
  border-top-color: #3a3a3a;
}
</style>
