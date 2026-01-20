<template>
  <div class="mobile-top-bar">
    <!-- Left: View selector -->
    <div class="top-bar-left" @click="openDrawer">
      <el-icon class="view-icon" :size="20">
        <component :is="currentViewIcon" />
      </el-icon>
      <span class="view-name">{{ currentViewName }}</span>
      <el-icon class="menu-icon" :size="16"><Menu /></el-icon>
    </div>

    <!-- Right: Actions -->
    <div class="top-bar-right">
      <!-- View mode toggle -->
      <el-button
        :icon="viewMode === 'list' ? List : Grid"
        text
        @click="toggleViewMode"
      />
      <!-- Add task button -->
      <el-button
        type="primary"
        :icon="Plus"
        @click="showCreateDialog"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { Menu, List, Grid, Plus, Tickets, Star, BellFilled, CircleCheck, Clock } from '@element-plus/icons-vue';
import { useTodoStore } from '@/stores';
import { useGroupStore } from '@/stores';
import { useTagStore } from '@/stores';
import { useUIStore } from '@/stores';

const { t } = useI18n();
const todoStore = useTodoStore();
const groupStore = useGroupStore();
const tagStore = useTagStore();
const uiStore = useUIStore();

const viewMode = computed(() => uiStore.viewMode);

// 当前视图图标和名称
const currentViewIcon = computed(() => {
  // 检查特殊视图
  if (todoStore.isTodoView) {
    return Tickets;
  }
  if (todoStore.isOverdueView) {
    return Clock;
  }

  // 检查优先级筛选
  if (todoStore.filterPriority !== undefined) {
    if (todoStore.filterPriority === 2) {
      return BellFilled;
    }
    if (todoStore.filterPriority === 1) {
      return Star;
    }
  }

  // 检查分组筛选
  if (todoStore.filterGroupId) {
    return Tickets;
  }

  // 检查标签筛选
  if (todoStore.filterTagId) {
    return Tickets;
  }

  // 默认视图 - 所有任务
  return Grid;
});

// 当前视图名称
const currentViewName = computed(() => {
  // 检查特殊视图
  if (todoStore.isTodoView) {
    return t('nav.todo');
  }
  if (todoStore.isOverdueView) {
    return t('nav.overdue');
  }

  // 检查优先级筛选
  if (todoStore.filterPriority !== undefined) {
    if (todoStore.filterPriority === 2) {
      return t('nav.urgent');
    }
    if (todoStore.filterPriority === 1) {
      return t('nav.important');
    }
  }

  // 检查分组筛选
  if (todoStore.filterGroupId) {
    const group = groupStore.groups.find(g => g.id === todoStore.filterGroupId);
    if (group) {
      return group.name;
    }
  }

  // 检查标签筛选
  if (todoStore.filterTagId) {
    const tag = tagStore.tags.find(tg => tg.id === todoStore.filterTagId);
    if (tag) {
      return tag.name;
    }
  }

  // 默认视图
  return t('nav.allTodos');
});

function openDrawer() {
  uiStore.toggleMobileDrawer();
}

function toggleViewMode() {
  uiStore.setViewMode(viewMode.value === 'list' ? 'card' : 'list');
}

function showCreateDialog() {
  uiStore.showCreateTodoDialog();
}
</script>

<style scoped>
.mobile-top-bar {
  height: 52px;
  background: var(--el-bg-color);
  border-bottom: 1px solid var(--el-border-color-light);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 12px;
  user-select: none;
  touch-action: manipulation;
  position: sticky;
  top: 0;
  z-index: 100;
}

.top-bar-left {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
  min-width: 0;
  cursor: pointer;
  padding: 8px 4px;
  margin: -8px -4px;
  border-radius: 8px;
  transition: background 0.2s ease;
}

.top-bar-left:active {
  background: var(--el-fill-color-light);
}

.view-icon {
  color: var(--el-color-primary);
  flex-shrink: 0;
}

.menu-icon {
  color: var(--el-text-color-regular);
  flex-shrink: 0;
}

.view-name {
  font-size: 15px;
  font-weight: 500;
  color: var(--el-text-color-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.top-bar-right {
  display: flex;
  align-items: center;
  gap: 4px;
  flex-shrink: 0;
}

.top-bar-right .el-button {
  padding: 8px;
  width: 36px;
  height: 36px;
}

.top-bar-right .el-button .el-icon {
  font-size: 18px;
}

/* Dark theme */
:global(html.dark) .mobile-top-bar {
  background: var(--el-bg-color);
  border-bottom-color: var(--el-border-color);
}

:global(html.dark) .top-bar-left:active {
  background: var(--el-fill-color-light);
}
</style>
