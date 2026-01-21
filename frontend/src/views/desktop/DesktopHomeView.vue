<template>
  <div class="desktop-home-view">
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
                <span v-if="todo.start_date" class="meta-item">
                  <el-icon><Clock /></el-icon>
                  {{ t('todo.start') }}: {{ formatSimpleDate(todo.start_date) }}
                </span>
                <span v-if="todo.due_date" class="meta-item" :class="{ 'overdue': isTodoOverdue(todo) }">
                  <el-icon><Calendar /></el-icon>
                  {{ t('todo.due') }}: {{ formatSimpleDate(todo.due_date) }}
                  <el-tag v-if="isTodoOverdue(todo)" size="small" type="danger" effect="plain">{{ t('nav.overdue') }}</el-tag>
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
                <el-tag
                  v-for="tag in todo.tags"
                  :key="tag.id"
                  size="small"
                  :style="{ backgroundColor: tag.color, color: 'white' }"
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
            <el-button
              :icon="Edit"
              circle
              text
              @click.stop="editTodo(todo)"
            />
            <el-button
              :icon="Delete"
              circle
              text
              type="danger"
              @click.stop="handleDeleteTodo(todo)"
            />
          </div>
        </div>
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
                :type="getPriorityType(todo.priority)"
                size="small"
                effect="plain"
              >
                {{ getPriorityLabel(todo.priority) }}
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
          <el-button
            :icon="Edit"
            circle
            text
            size="small"
            @click.stop="editTodo(todo)"
          />
          <el-button
            :icon="Delete"
            circle
            text
            size="small"
            type="danger"
            @click.stop="handleDeleteTodo(todo)"
          />
        </div>
        <div class="card-content">
          <div v-if="todo.description" class="card-description">
            {{ todo.description }}
          </div>
        </div>
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
          <!-- Date Row -->
          <div v-if="todo.start_date || todo.due_date || (todo.attachments && todo.attachments.length > 0)" class="card-dates">
            <span v-if="todo.start_date" class="meta-item">
              <el-icon><Clock /></el-icon>
              {{ t('todo.start') }}: {{ formatSimpleDate(todo.start_date) }}
            </span>
            <span v-if="todo.due_date" class="meta-item" :class="{ 'overdue': isTodoOverdue(todo) }">
              <el-icon><Calendar /></el-icon>
              {{ t('todo.due') }}: {{ formatSimpleDate(todo.due_date) }}
              <el-tag v-if="isTodoOverdue(todo)" size="small" type="danger" effect="plain">{{ t('nav.overdue') }}</el-tag>
            </span>
            <span v-if="todo.attachments && todo.attachments.length > 0" class="meta-item attachment-indicator">
              <el-icon><Paperclip /></el-icon>
              {{ todo.attachments.length }}
            </span>
          </div>
          <!-- Tags Row -->
          <div v-if="todo.group_id || (todo.tags && todo.tags.length > 0)" class="card-tags">
            <el-tag
              v-if="todo.group_id"
              size="small"
              effect="plain"
              class="group-tag"
            >
              {{ getGroupName(todo.group_id) }}
            </el-tag>
            <el-tag
              v-for="tag in todo.tags?.slice(0, 3)"
              :key="tag.id"
              size="small"
              :style="{ backgroundColor: tag.color, color: 'white' }"
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

    <!-- Empty State -->
    <el-empty
      v-if="filteredTodos.length === 0 && !loading"
      :description="t('todo.noTodosHint')"
    />

    <!-- Loading -->
    <div v-if="loading" class="loading-state">
      <el-skeleton :rows="5" animated />
    </div>

    <!-- Detail Panel -->
    <el-drawer
      v-model="detailVisible"
      :title="editMode ? t('todo.editTodo') : t('todo.todoDetail')"
      size="40%"
      direction="rtl"
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
import { Calendar, Star, StarFilled, ArrowDown, ArrowRight, Delete, Edit, Clock, Paperclip } from '@element-plus/icons-vue';
import TodoDetailPanel from '@/components/todo/TodoDetailPanel.vue';
import { useTodoList } from '@/composables/useTodoList';

const {
  loading,
  filteredTodos,
  viewMode,
  selectedTodo,
  detailVisible,
  expandedTodos,
  editMode,
  toggleStatus,
  toggleMark,
  selectTodo,
  toggleExpand,
  toggleStep,
  handleTodoUpdated,
  handleTodoDeleted,
  handleDrawerClose,
  editTodo,
  handleDeleteTodo,
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
.desktop-home-view {
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
  background: var(--el-bg-color);
  border-radius: 8px;
  border: 1px solid var(--el-border-color-light);
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
  color: var(--el-text-color-secondary);
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
  color: var(--el-text-color-regular);
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
  border-top: 1px solid var(--el-border-color-light);
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
  color: var(--el-text-color-regular);
}

.step-text.completed {
  text-decoration: line-through;
  color: var(--el-text-color-secondary);
}

.title-text {
  font-size: 16px;
  font-weight: 500;
  color: var(--el-text-color-primary);
}

.todo-meta {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 13px;
  color: var(--el-text-color-secondary);
}

.meta-item {
  display: flex;
  align-items: center;
  gap: 4px;
}

.meta-item.overdue {
  color: var(--el-color-danger);
  font-weight: 500;
}

.attachment-indicator {
  color: var(--el-color-primary);
}

.group-tag {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  border-color: var(--el-border-color);
}

.todo-cards {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 16px;
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
  color: var(--el-text-color-secondary);
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
  color: var(--el-text-color-primary);
  line-height: 1.4;
}

.card-description {
  font-size: 13px;
  color: var(--el-text-color-regular);
  line-height: 1.5;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
  text-overflow: ellipsis;
}

.card-steps {
  padding: 12px 0 12px 24px;
  border-top: 1px solid var(--el-border-color-light);
}

.card-footer {
  margin-top: auto;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.card-dates {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 13px;
  flex-wrap: wrap;
}

.card-dates .meta-item {
  display: flex;
  align-items: center;
  gap: 4px;
  color: var(--el-text-color-secondary);
}

.card-tags {
  display: flex;
  align-items: center;
  gap: 4px;
  flex-wrap: wrap;
}

.more-tags {
  font-size: 12px;
  color: var(--el-text-color-secondary);
  padding: 0 4px;
}

.loading-state {
  padding: 20px;
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

:global(html.dark) .todo-meta {
  color: var(--el-text-color-secondary);
}

:global(html.dark) .todo-steps {
  border-top-color: var(--el-border-color);
}

:global(html.dark) .step-text {
  color: var(--el-text-color-regular);
}

:global(html.dark) .step-text.completed {
  color: var(--el-text-color-secondary);
}

:global(html.dark) .todo-card {
  background: var(--el-fill-color-light);
  border-color: var(--el-border-color);
}

:global(html.dark) .todo-card:hover {
  border-color: var(--el-color-primary);
}

:global(html.dark) .todo-card .title-text {
  color: var(--el-text-color-primary);
}

:global(html.dark) .todo-card .card-description {
  color: var(--el-text-color-regular);
}

:global(html.dark) .todo-card .meta-item {
  color: var(--el-text-color-secondary);
}

:global(html.dark) .todo-card .more-tags {
  color: var(--el-text-color-secondary);
}

:global(html.dark) .card-steps {
  border-top-color: var(--el-border-color);
}

/* 紧凑模式 */
[data-density='compact'] .todo-list {
  gap: 8px;
}

[data-density='compact'] .todo-item {
  padding: 12px;
}

[data-density='compact'] .todo-left {
  gap: 10px;
}

[data-density='compact'] .todo-title {
  margin-bottom: 6px;
  gap: 6px;
}

[data-density='compact'] .todo-description {
  margin-bottom: 6px;
}

[data-density='compact'] .todo-steps {
  margin-top: 10px;
  padding-top: 10px;
  padding-left: 32px;
}

[data-density='compact'] .step-item {
  padding: 4px 0;
  gap: 6px;
}

[data-density='compact'] .todo-cards {
  gap: 12px;
}

[data-density='compact'] .todo-card {
  padding: 12px;
  gap: 8px;
}

[data-density='compact'] .card-header {
  gap: 6px;
}

[data-density='compact'] .card-title {
  margin-bottom: 6px;
  gap: 6px;
}

[data-density='compact'] .card-steps {
  padding: 10px 0 10px 20px;
}
</style>
