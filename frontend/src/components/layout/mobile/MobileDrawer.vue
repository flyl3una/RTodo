<template>
  <teleport to="body">
    <transition name="slide-left">
      <div v-if="visible" class="mobile-drawer-overlay" @click="closeDrawer">
        <div class="mobile-drawer" @click.stop @touchstart.stop="handleTouchStart" @touchmove.stop="handleTouchMove" @touchend.stop="handleTouchEnd">
          <!-- Header -->
          <div class="drawer-header">
            <span class="drawer-title">{{ t('nav.quickAccess') }}</span>
          </div>

          <!-- Scrollable Content -->
          <div class="drawer-content">
            <!-- Quick Access -->
            <div class="drawer-section">
              <a
                href="#"
                class="drawer-item"
                :class="{ active: baseView === 'todo' }"
                @click.prevent="setFilter('todo')"
              >
                <el-icon><Tickets /></el-icon>
                <span>{{ t('nav.todo') }}</span>
              </a>
              <a
                href="#"
                class="drawer-item"
                :class="{ active: baseView === 'all' }"
                @click.prevent="setFilter('all')"
              >
                <el-icon><Collection /></el-icon>
                <span>{{ t('nav.allTodos') }}</span>
              </a>
              <a
                href="#"
                class="drawer-item"
                :class="{ active: baseView === 'important' }"
                @click.prevent="setFilter('important')"
              >
                <el-icon><Star /></el-icon>
                <span>{{ t('nav.important') }}</span>
              </a>
              <a
                href="#"
                class="drawer-item"
                :class="{ active: baseView === 'urgent' }"
                @click.prevent="setFilter('urgent')"
              >
                <el-icon><BellFilled /></el-icon>
                <span>{{ t('nav.urgent') }}</span>
              </a>
              <a
                href="#"
                class="drawer-item"
                :class="{ active: baseView === 'completed' }"
                @click.prevent="setFilter('completed')"
              >
                <el-icon><CircleCheck /></el-icon>
                <span>{{ t('nav.completed') }}</span>
              </a>
              <a
                href="#"
                class="drawer-item"
                :class="{ active: baseView === 'overdue' }"
                @click.prevent="setFilter('overdue')"
              >
                <el-icon><Clock /></el-icon>
                <span>{{ t('nav.overdue') }}</span>
              </a>
            </div>

            <!-- Task Groups -->
            <div class="drawer-section">
              <div class="section-header">
                <span class="section-title">{{ t('group.title') }}</span>
                <el-button
                  text
                  :icon="Plus"
                  size="small"
                  @click="showAddGroup"
                >
                  {{ t('common.add') }}
                </el-button>
              </div>
              <div class="drawer-groups">
                <div
                  class="drawer-item"
                  :class="{ active: filterGroupIds.includes(group.id) }"
                  v-for="group in groups"
                  :key="group.id"
                  @click="selectGroup(group.id)"
                  @touchstart.prevent="handleItemTouchStart($event, group, 'group')"
                  @touchend.prevent="handleItemTouchEnd"
                  @contextmenu.prevent="handleItemContextMenu(group, 'group')"
                >
                  <span class="group-icon">{{ group.icon || '📁' }}</span>
                  <span>{{ group.name }}</span>
                </div>
              </div>
            </div>

            <!-- Tags -->
            <div class="drawer-section" v-if="tags.length > 0 || true">
              <div class="section-header">
                <span class="section-title">{{ t('common.tags') }}</span>
                <el-button
                  text
                  :icon="Plus"
                  size="small"
                  @click="showAddTag"
                >
                  {{ t('common.add') }}
                </el-button>
              </div>
              <div class="drawer-tags" v-if="tags.length > 0">
                <span
                  class="drawer-tag"
                  :class="{ active: filterTagIds.includes(tag.id) }"
                  v-for="tag in tags"
                  :key="tag.id"
                  :style="{ backgroundColor: tag.color }"
                  @click="selectTag(tag.id)"
                  @touchstart.prevent="handleItemTouchStart($event, tag, 'tag')"
                  @touchend.prevent="handleItemTouchEnd"
                  @contextmenu.prevent="handleItemContextMenu(tag, 'tag')"
                >
                  {{ tag.name }}
                </span>
              </div>
              <div v-else class="empty-tags">
                <span class="empty-text">{{ t('tag.noTags') }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </transition>

    <!-- Context Menu -->
    <transition name="fade">
      <div v-if="contextMenuVisible" class="context-menu-overlay" @click="hideContextMenu">
        <div class="context-menu" :style="contextMenuStyle" @click.stop>
          <div class="context-menu-item" @click="handleContextMenuAction('edit')">
            <el-icon><Edit /></el-icon>
            <span>{{ t('common.edit') }}</span>
          </div>
          <div class="context-menu-item danger" @click="handleContextMenuAction('delete')">
            <el-icon><Delete /></el-icon>
            <span>{{ t('common.delete') }}</span>
          </div>
        </div>
      </div>
    </transition>

    <!-- Group Edit Dialog -->
    <GroupManageDialog
      v-model="groupDialogVisible"
      :group="editingGroup"
      @updated="handleGroupUpdated"
    />

    <!-- Tag Edit Dialog -->
    <TagCreateDialog
      v-model="tagDialogVisible"
      :tag="editingTag"
      @updated="handleTagUpdated"
    />
  </teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useI18n } from 'vue-i18n';
import {
  Tickets, Collection, Star, BellFilled, CircleCheck, Clock,
  Plus, Edit, Delete,
} from '@element-plus/icons-vue';
import { useTodoStore } from '@/stores';
import { useGroupStore } from '@/stores';
import { useTagStore } from '@/stores';
import { useUIStore } from '@/stores';
import { TodoStatus } from '@/types';
import type { TaskGroup } from '@/types';
import type { Tag } from '@/types';
import GroupManageDialog from '../../group/GroupManageDialog.vue';
import TagCreateDialog from '../../tag/TagCreateDialog.vue';
import { useTodoFilters } from '@/composables/useTodoFilters';

const { t } = useI18n();
const route = useRoute();
const router = useRouter();
const todoStore = useTodoStore();
const groupStore = useGroupStore();
const tagStore = useTagStore();
const uiStore = useUIStore();

// Touch gesture state
const touchStartX = ref(0);
const touchStartY = ref(0);

// Context menu state
const contextMenuVisible = ref(false);
const contextMenuStyle = ref<{ top: string; left: string }>({ top: '0px', left: '0px' });
const contextMenuItem = ref<{ type: 'group' | 'tag'; data: TaskGroup | Tag } | null>(null);
const longPressTimer = ref<number | null>(null);

const {
  currentView,
  baseView,
  filterGroupIds,
  filterTagIds,
  groupDialogVisible,
  editingGroup,
  tagDialogVisible,
  editingTag,
  groups,
  tags,
  setFilter,
  selectGroup,
  selectTag,
  showAddGroup,
  handleGroupUpdated,
  showAddTag,
  handleTagUpdated,
} = useTodoFilters();

const visible = computed(() => uiStore.mobileDrawerVisible);

function closeDrawer() {
  uiStore.setMobileDrawerVisible(false);
}

// Long press handlers for items
function handleItemTouchStart(e: TouchEvent, item: TaskGroup | Tag, type: 'group' | 'tag') {
  touchStartX.value = e.touches[0].clientX;
  touchStartY.value = e.touches[0].clientY;

  longPressTimer.value = window.setTimeout(() => {
    contextMenuItem.value = { type, data: item };
    showContextMenu(e.touches[0].clientX, e.touches[0].clientY);
    longPressTimer.value = null;
  }, 500);
}

function handleItemTouchEnd() {
  if (longPressTimer.value) {
    clearTimeout(longPressTimer.value);
    longPressTimer.value = null;
  }
}

function handleItemContextMenu(item: TaskGroup | Tag, type: 'group' | 'tag') {
  contextMenuItem.value = { type, data: item };
  const event = window.event as MouseEvent;
  if (event) {
    showContextMenu(event.clientX, event.clientY);
  }
}

function showContextMenu(x: number, y: number) {
  contextMenuStyle.value = {
    top: `${y}px`,
    left: `${x}px`,
  };
  contextMenuVisible.value = true;
}

function hideContextMenu() {
  contextMenuVisible.value = false;
  contextMenuItem.value = null;
}

function handleContextMenuAction(action: 'edit' | 'delete') {
  const item = contextMenuItem.value;
  if (!item) return;

  hideContextMenu();

  if (action === 'edit') {
    if (item.type === 'group') {
      editingGroup.value = item.data as TaskGroup;
      groupDialogVisible.value = true;
    } else {
      editingTag.value = item.data as Tag;
      tagDialogVisible.value = true;
    }
  } else if (action === 'delete') {
    console.log('Delete:', item);
  }
}

// Touch gesture handlers for swipe-to-close
function handleTouchStart(e: TouchEvent) {
  touchStartX.value = e.touches[0].clientX;
  touchStartY.value = e.touches[0].clientY;
}

function handleTouchMove(e: TouchEvent) {
  const deltaX = e.touches[0].clientX - touchStartX.value;
  const deltaY = e.touches[0].clientY - touchStartY.value;

  if (Math.abs(deltaX) > Math.abs(deltaY) && deltaX > 0) {
    e.preventDefault();
  }
}

function handleTouchEnd(e: TouchEvent) {
  const touchEndX = e.changedTouches[0].clientX;
  const touchEndY = e.changedTouches[0].clientY;
  const deltaX = touchEndX - touchStartX.value;
  const deltaY = touchEndY - touchStartY.value;

  if (Math.abs(deltaX) > Math.abs(deltaY) && Math.abs(deltaX) > 50) {
    if (deltaX > 0) {
      closeDrawer();
    }
  }
}
</script>

<style scoped>
.mobile-drawer-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  z-index: 2000;
}

.mobile-drawer {
  position: absolute;
  top: 0;
  left: 0;
  bottom: 0;
  width: 280px;
  max-width: 80vw;
  background: var(--el-bg-color-page);
  display: flex;
  flex-direction: column;
  box-shadow: 2px 0 8px rgba(0, 0, 0, 0.15);
}

.slide-left-enter-active,
.slide-left-leave-active {
  transition: transform 0.3s ease;
}

.slide-left-enter-from,
.slide-left-leave-to {
  transform: translateX(-100%);
}

.drawer-header {
  flex-shrink: 0;
  height: 56px;
  padding: 0 20px;
  border-bottom: 1px solid var(--el-border-color-light);
  display: flex;
  align-items: center;
}

.drawer-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.drawer-content {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
}

.drawer-content::-webkit-scrollbar {
  width: 6px;
}

.drawer-content::-webkit-scrollbar-track {
  background: transparent;
}

.drawer-content::-webkit-scrollbar-thumb {
  background: var(--el-border-color);
  border-radius: 3px;
}

.drawer-section {
  padding: 12px 0;
  border-bottom: 1px solid var(--el-border-color-lighter);
}

.section-header {
  padding: 0 16px 8px;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.section-title {
  font-size: 12px;
  color: var(--el-text-color-secondary);
  text-transform: uppercase;
}

.drawer-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  color: var(--el-text-color-regular);
  text-decoration: none;
  transition: all 0.2s ease;
  cursor: pointer;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.drawer-item:active {
  background: var(--el-fill-color-light);
}

.drawer-item.active {
  background: var(--el-color-primary);
  color: white;
}

.drawer-item .el-icon {
  font-size: 18px;
}

.group-icon {
  font-size: 16px;
}

.drawer-groups {
  display: flex;
  flex-direction: column;
}

.drawer-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  padding: 0 16px;
}

.drawer-tag {
  display: inline-block;
  padding: 6px 14px;
  border-radius: 16px;
  font-size: 13px;
  color: white;
  cursor: pointer;
  transition: opacity 0.2s ease, transform 0.2s ease;
  -webkit-tap-highlight-color: transparent;
}

.drawer-tag:active {
  opacity: 0.8;
  transform: scale(0.95);
}

.drawer-tag.active {
  box-shadow: 0 0 0 2px white, 0 0 0 4px var(--el-color-primary);
}

.empty-tags {
  padding: 16px;
  text-align: center;
}

.empty-text {
  color: var(--el-text-color-secondary);
  font-size: 13px;
}

/* Context Menu */
.context-menu-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 2100;
}

.context-menu {
  position: absolute;
  background: var(--el-bg-color);
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  border: 1px solid var(--el-border-color-light);
  min-width: 140px;
  overflow: hidden;
  transform: translate(-50%, -100%);
  margin-top: -8px;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.context-menu-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 16px;
  font-size: 14px;
  color: var(--el-text-color-regular);
  cursor: pointer;
  transition: background 0.2s ease;
}

.context-menu-item:active {
  background: var(--el-fill-color-light);
}

.context-menu-item.danger {
  color: var(--el-color-danger);
}

/* Dark theme */
:global(html.dark) .mobile-drawer {
  background: var(--el-bg-color);
  box-shadow: 2px 0 8px rgba(0, 0, 0, 0.3);
}

:global(html.dark) .drawer-header {
  border-bottom-color: var(--el-border-color);
}

:global(html.dark) .drawer-section {
  border-bottom-color: var(--el-border-color-lighter);
}

:global(html.dark) .drawer-item {
  color: var(--el-text-color-primary);
}

:global(html.dark) .drawer-item:active {
  background: var(--el-fill-color-light);
}

:global(html.dark) .context-menu {
  background: var(--el-bg-color);
  border-color: var(--el-border-color);
}

/* Safe area for notched devices */
@supports (padding: max(0px)) {
  .mobile-drawer {
    padding-top: max(0px, env(safe-area-inset-top));
    padding-bottom: max(0px, env(safe-area-inset-bottom));
  }

  .drawer-header {
    padding-top: calc(12px + max(0px, env(safe-area-inset-top)));
  }
}
</style>
