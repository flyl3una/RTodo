<template>
  <div class="mobile-home-view">
    <!-- Todo List -->
    <div v-if="viewMode === 'list'" class="todo-list">
      <div
        v-for="todo in filteredTodos"
        :key="todo.id"
        class="todo-item"
        :class="{ completed: todo.status === TodoStatus.Done }"
        @click="selectTodo(todo)"
      >
        <div class="todo-main">
          <div class="todo-left">
            <el-checkbox
              :model-value="todo.status === TodoStatus.Done"
              @change="toggleStatus(todo)"
              @click.stop
            />
            <div class="todo-content">
              <div class="todo-title">
                <el-tag
                  v-if="todo.priority > 0"
                  :type="getPriorityType(todo.priority)"
                  size="small"
                  effect="plain"
                >
                  {{ getPriorityLabel(todo.priority) }}
                </el-tag>
                <span class="title-text">{{ todo.title }}</span>
              </div>
              <div v-if="todo.description" class="todo-description">
                {{ todo.description }}
              </div>
              <div class="todo-meta">
                <span v-if="todo.due_date" class="meta-item" :class="{ 'overdue': isTodoOverdue(todo) }">
                  <el-icon><Calendar /></el-icon>
                  {{ formatSimpleDate(todo.due_date) }}
                </span>
                <span v-if="todo.attachments && todo.attachments.length > 0" class="meta-item attachment-indicator">
                  <el-icon><Paperclip /></el-icon>
                  {{ todo.attachments.length }}
                </span>
                <el-tag
                  v-if="todo.group_id"
                  size="small"
                  effect="plain"
                  class="group-tag"
                >
                  {{ getGroupName(todo.group_id) }}
                </el-tag>
              </div>
            </div>
          </div>
          <div class="todo-actions">
            <el-button
              :icon="todo.priority >= 1 ? StarFilled : Star"
              circle
              text
              @click.stop="toggleMark(todo)"
            />
            <el-button
              :icon="Edit"
              circle
              text
              @click.stop="editTodo(todo)"
            />
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
        @click="selectTodo(todo)"
      >
        <div class="card-header">
          <el-checkbox
            :model-value="todo.status === TodoStatus.Done"
            @change="toggleStatus(todo)"
            @click.stop
          />
          <div class="card-title">
            <el-tag
              v-if="todo.priority >= 1"
              :type="getPriorityType(todo.priority)"
              size="small"
              effect="plain"
            >
              {{ getPriorityLabel(todo.priority) }}
            </el-tag>
            <span class="title-text">{{ todo.title }}</span>
          </div>
        </div>
        <div class="card-content" v-if="todo.description">
          {{ todo.description }}
        </div>
        <div class="card-footer">
          <div class="card-meta">
            <span v-if="todo.due_date" class="meta-item" :class="{ 'overdue': isTodoOverdue(todo) }">
              <el-icon><Calendar /></el-icon>
              {{ formatSimpleDate(todo.due_date) }}
            </span>
            <span v-if="todo.attachments && todo.attachments.length > 0" class="meta-item attachment-indicator">
              <el-icon><Paperclip /></el-icon>
              {{ todo.attachments.length }}
            </span>
            <el-tag
              v-if="todo.group_id"
              size="small"
              effect="plain"
              class="group-tag"
            >
              {{ getGroupName(todo.group_id) }}
            </el-tag>
          </div>
          <div class="card-actions">
            <el-button
              :icon="todo.priority >= 1 ? StarFilled : Star"
              circle
              text
              size="small"
              @click.stop="toggleMark(todo)"
            />
            <el-button
              :icon="Edit"
              circle
              text
              size="small"
              @click.stop="editTodo(todo)"
            />
          </div>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <el-empty
      v-if="filteredTodos.length === 0 && !loading"
      :description="t('todo.noTodosHint')"
    />

    <!-- Loading -->
    <div v-if="loading" class="loading-state">
      <el-skeleton :rows="5" animated />
    </div>

    <!-- Detail Panel (Full screen on mobile) -->
    <el-drawer
      v-model="detailVisible"
      :title="editMode ? t('todo.editTodo') : t('todo.todoDetail')"
      size="100%"
      direction="rtl"
      :class="{ 'mobile-drawer': true }"
      @close="handleDrawerClose"
    >
      <TodoDetailPanel
        v-if="selectedTodo"
        :key="selectedTodo.id + '-' + selectedTodo.updated_at + '-' + editMode"
        :todo="selectedTodo"
        :edit-mode="editMode"
        @updated="handleTodoUpdated"
        @deleted="handleTodoDeleted"
      />
    </el-drawer>
  </div>
</template>

<script setup lang="ts">
import { Calendar, Star, StarFilled, Edit, Paperclip } from '@element-plus/icons-vue';
import TodoDetailPanel from '@/components/todo/TodoDetailPanel.vue';
import { useTodoList } from '@/composables/useTodoList';

const {
  loading,
  filteredTodos,
  viewMode,
  selectedTodo,
  detailVisible,
  editMode,
  toggleStatus,
  toggleMark,
  selectTodo,
  handleTodoUpdated,
  handleTodoDeleted,
  handleDrawerClose,
  editTodo,
  formatSimpleDate,
  getGroupName,
  getPriorityLabel,
  getPriorityType,
  isTodoOverdue,
  TodoStatus,
  t,
} = useTodoList();
</script>

<style scoped>
.mobile-home-view {
  width: 100%;
  min-height: 100%;
}

.todo-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.todo-item {
  padding: 16px;
  background: var(--el-bg-color);
  border-radius: 12px;
  border: 1px solid var(--el-border-color-light);
  cursor: pointer;
  transition: all 0.2s ease;
  touch-action: manipulation;
}

.todo-item:active {
  background: var(--el-fill-color-light);
}

.todo-item.completed {
  opacity: 0.6;
}

.todo-item.completed .title-text {
  text-decoration: line-through;
  color: var(--el-text-color-secondary);
}

.todo-main {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
}

.todo-left {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  flex: 1;
  min-width: 0;
}

.todo-content {
  flex: 1;
  min-width: 0;
}

.todo-title {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 6px;
}

.todo-title .title-text {
  font-size: 16px;
  font-weight: 500;
  color: var(--el-text-color-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.todo-description {
  font-size: 13px;
  color: var(--el-text-color-regular);
  line-height: 1.5;
  margin-bottom: 6px;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  text-overflow: ellipsis;
}

.todo-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.meta-item {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.meta-item.overdue {
  color: var(--el-color-danger);
  font-weight: 500;
}
.attachment-indicator {
  color: var(--el-color-primary);
}

.group-tag {
  border-color: var(--el-border-color);
}

.todo-actions {
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex-shrink: 0;
}

.todo-cards {
  display: grid;
  grid-template-columns: 1fr;
  gap: 12px;
}

.todo-card {
  background: var(--el-bg-color);
  border-radius: 12px;
  border: 1px solid var(--el-border-color-light);
  padding: 16px;
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  flex-direction: column;
  gap: 12px;
  touch-action: manipulation;
}

.todo-card:active {
  background: var(--el-fill-color-light);
}

.todo-card.completed {
  opacity: 0.6;
}

.todo-card.completed .title-text {
  text-decoration: line-through;
  color: var(--el-text-color-secondary);
}

.card-header {
  display: flex;
  align-items: center;
  gap: 12px;
}

.card-title {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
  min-width: 0;
}

.card-title .title-text {
  font-size: 16px;
  font-weight: 500;
  color: var(--el-text-color-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.card-content {
  font-size: 13px;
  color: var(--el-text-color-regular);
  line-height: 1.5;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
  text-overflow: ellipsis;
}

.card-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-top: 8px;
  border-top: 1px solid var(--el-border-color-lighter);
}
.card-footer .card-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}


.card-footer .meta-item {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.card-actions {
  display: flex;
  gap: 4px;
}

.loading-state {
  padding: 20px;
}

/* Mobile drawer full screen */
.mobile-drawer :deep(.el-drawer) {
  width: 100% !important;
}

.mobile-drawer :deep(.el-drawer__header) {
  padding: 16px;
  margin-bottom: 0;
  border-bottom: 1px solid var(--el-border-color);
}

.mobile-drawer :deep(.el-drawer__body) {
  padding: 0;
}

.mobile-drawer :deep(.el-drawer__title) {
  font-size: 18px;
  font-weight: 500;
}

/* Dark theme */
:global(html.dark) .todo-item {
  background: var(--el-fill-color-light);
  border-color: var(--el-border-color);
}

:global(html.dark) .title-text {
  color: var(--el-text-color-primary);
}

:global(html.dark) .todo-description {
  color: var(--el-text-color-regular);
}

:global(html.dark) .todo-card {
  background: var(--el-fill-color-light);
  border-color: var(--el-border-color);
}

:global(html.dark) .card-content {
  color: var(--el-text-color-regular);
}

:global(html.dark) .card-footer {
  border-top-color: var(--el-border-color);
}

/* Touch-friendly tap targets */
.todo-item,
.todo-card {
  -webkit-tap-highlight-color: transparent;
}

/* Safe area for notched devices */
@supports (padding: max(0px)) {
  .mobile-home-view {
    padding-bottom: max(0px, env(safe-area-inset-bottom));
  }
}
</style>
