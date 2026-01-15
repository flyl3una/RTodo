<template>
  <div class="sidebar" :class="{ collapsed }">
    <!-- Header - Fixed -->
    <div class="sidebar-header">
      <div class="logo" v-if="!collapsed">
        <span class="logo-icon">
          <Logo />
        </span>

        <span class="logo-text">RTodo</span>
      </div>
      <div class="logo-icon-only" v-else>
        <Logo />
      </div>
    </div>

    <!-- Scrollable Content Area -->
    <div class="sidebar-content">
      <!-- Quick Access -->
      <div class="sidebar-section" v-if="!collapsed">
        <div class="section-title">{{ t('nav.quickAccess') }}</div>
        <div class="quick-links">
          <a
            href="#"
            class="quick-link"
            :class="{ active: currentView === 'all' && route.path === '/' }"
            @click.prevent="setFilter('all')"
          >
            <el-icon><List /></el-icon>
            <span>{{ t('nav.allTodos') }}</span>
          </a>
          <!-- <a
            href="#"
            class="quick-link"
            :class="{ active: currentView === 'today' && route.path === '/' }"
            @click.prevent="setFilter('today')"
          >
            <el-icon><Calendar /></el-icon>
            <span>今天</span>
          </a> -->
          <a
            href="#"
            class="quick-link"
            :class="{ active: currentView === 'important' && route.path === '/' }"
            @click.prevent="setFilter('important')"
          >
            <el-icon><Star /></el-icon>
            <span>{{ t('nav.important') }}</span>
          </a>
          <a
            href="#"
            class="quick-link"
            :class="{ active: currentView === 'urgent' && route.path === '/' }"
            @click.prevent="setFilter('urgent')"
          >
            <el-icon><Warning /></el-icon>
            <span>{{ t('nav.urgent') }}</span>
          </a>
          <a
            href="#"
            class="quick-link"
            :class="{ active: currentView === 'completed' && route.path === '/' }"
            @click.prevent="setFilter('completed')"
          >
            <el-icon><CircleCheck /></el-icon>
            <span>{{ t('nav.completed') }}</span>
          </a>
        </div>
      </div>

      <!-- Task Groups -->
      <div class="sidebar-section">
        <div class="section-title" v-if="!collapsed">
          {{ t('group.title') }}
          <el-link type="primary" @click="showAddGroup" style="float: right; font-size: 12px;">
            {{ t('common.add') }}
          </el-link>
        </div>
        <div class="groups">
          <a
            href="#"
            class="group-item"
            :class="{ active: filterGroupId === group.id && route.path === '/' }"
            v-for="group in groups"
            :key="group.id"
            @click.prevent="selectGroup(group.id)"
            @contextmenu.prevent="editGroup(group)"
            data-allow-context-menu
          >
            <span class="group-icon">{{ group.icon || '📁' }}</span>
            <span class="group-name" v-if="!collapsed">{{ group.name }}</span>
          </a>
        </div>
      </div>

      <!-- Tags -->
      <div class="sidebar-section" v-if="!collapsed">
        <div class="section-title">
          {{ t('common.tags') }}
          <el-link type="primary" @click="showTagManage" style="float: right; font-size: 12px;">
            {{ t('common.add') }}
          </el-link>
        </div>
        <div class="tags" v-if="tags.length > 0">
          <span
            class="tag-item"
            v-for="tag in tags"
            :key="tag.id"
            :style="{ backgroundColor: tag.color }"
            @click="selectTag(tag.id)"
            @contextmenu.prevent="editTag(tag)"
            data-allow-context-menu
          >
            {{ tag.name }}
          </span>
        </div>
        <div v-else class="empty-tags">
          <span class="empty-text">{{ t('tag.noTags') }}</span>
          <el-link type="primary" @click="showTagManage">{{ t('placeholder.addTag') }}</el-link>
        </div>
      </div>
    </div>

    <!-- Bottom Actions - Fixed -->
    <div class="sidebar-footer">
      <router-link to="/stats" class="footer-item" :title="t('stats.title')">
        <el-icon><TrendCharts /></el-icon>
        <span v-if="!collapsed">{{ t('stats.title') }}</span>
      </router-link>
      <router-link to="/settings" class="footer-item" :title="t('settings.title')">
        <el-icon><Setting /></el-icon>
        <span v-if="!collapsed">{{ t('settings.title') }}</span>
      </router-link>
    </div>

    <!-- Group Manage Dialog -->
    <GroupManageDialog
      v-model="groupDialogVisible"
      :group="editingGroup"
      @updated="handleGroupUpdated"
    />

    <!-- Tag Manage Dialog -->
    <TagCreateDialog
      v-model="tagDialogVisible"
      :tag="editingTag"
      @updated="handleTagUpdated"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useI18n } from 'vue-i18n';
import {
  List, Calendar, Star, Plus, TrendCharts, Setting, Warning, CircleCheck,
} from '@element-plus/icons-vue';
import { useTodoStore } from '@/stores';
import { useGroupStore } from '@/stores';
import { useTagStore } from '@/stores';
import { TodoStatus } from '@/types';
import type { TaskGroup } from '@/types';
import type { Tag } from '@/types';
import GroupManageDialog from '../group/GroupManageDialog.vue';
import TagCreateDialog from '../tag/TagCreateDialog.vue';
import Logo from '@/components/icon/logo.vue';

const { t } = useI18n();

const props = defineProps<{
  collapsed: boolean;
}>();

const emit = defineEmits<{
  toggle: [];
}>();

const route = useRoute();
const router = useRouter();
const todoStore = useTodoStore();
const groupStore = useGroupStore();
const tagStore = useTagStore();

const currentView = ref<'all' | 'today' | 'important' | 'urgent' | 'completed'>('all');
const filterGroupId = ref<string | undefined>();
const groupDialogVisible = ref(false);
const editingGroup = ref<TaskGroup | undefined>();
const tagDialogVisible = ref(false);
const editingTag = ref<Tag | undefined>();

const groups = computed(() => groupStore.groups);
const tags = computed(() => tagStore.tags);

// Reset to 'all' view (can be called from parent)
function resetToAllView() {
  currentView.value = 'all';
  filterGroupId.value = undefined;
  todoStore.setFilter({});
}

function setFilter(view: 'all' | 'today' | 'important' | 'urgent' | 'completed') {
  console.log('[Sidebar] setFilter called with view:', view);
  currentView.value = view;
  filterGroupId.value = undefined;

  // Apply filter
  switch (view) {
    case 'all':
      console.log('[Sidebar] Applying "all" filter (no params)');
      todoStore.setFilter({});
      break;
    case 'today':
      // Get today's date range
      const today = new Date();
      today.setHours(0, 0, 0, 0);
      const todayStart = today.getTime();
      const tomorrow = new Date(today);
      tomorrow.setDate(tomorrow.getDate() + 1);
      const tomorrowStart = tomorrow.getTime();
      console.log('[Sidebar] Applying "today" filter:', { start_date: todayStart, end_date: tomorrowStart });
      // Filter by due_date in range [today, tomorrow)
      todoStore.setFilter({
        start_date: todayStart,
        end_date: tomorrowStart
      });
      break;
    case 'important':
      // Filter by priority = 1 (important)
      console.log('[Sidebar] Applying "important" filter:', { priority: 1 });
      todoStore.setFilter({ priority: 1 });
      break;
    case 'urgent':
      // Filter by priority = 3 (urgent)
      console.log('[Sidebar] Applying "urgent" filter:', { priority: 3 });
      todoStore.setFilter({ priority: 3 });
      break;
    case 'completed':
      // Filter by status = Done (2)
      console.log('[Sidebar] Applying "completed" filter:', { status: TodoStatus.Done });
      todoStore.setFilter({ status: TodoStatus.Done });
      break;
  }

  // Navigate to home
  if (route.path !== '/') {
    router.push('/');
  }
}

function selectGroup(groupId: string) {
  console.log('[Sidebar] selectGroup called with groupId:', groupId);
  currentView.value = 'all';
  filterGroupId.value = groupId; // Keep local state in sync for UI display
  todoStore.setFilter({ group_id: groupId });
  if (route.path !== '/') {
    router.push('/');
  }
}

function selectTag(tagId: string) {
  console.log('[Sidebar] selectTag called with tagId:', tagId);
  currentView.value = 'all';
  filterGroupId.value = undefined; // Tags don't use group_id filter
  todoStore.setFilter({ tag_id: tagId });
  if (route.path !== '/') {
    router.push('/');
  }
}

function showAddGroup() {
  editingGroup.value = undefined;
  groupDialogVisible.value = true;
}

function editGroup(group: TaskGroup) {
  editingGroup.value = group;
  groupDialogVisible.value = true;
}

function handleGroupUpdated() {
  groupDialogVisible.value = false;
  editingGroup.value = undefined;
  groupStore.fetchGroups();
}

function showTagManage() {
  editingTag.value = undefined;
  tagDialogVisible.value = true;
}

function handleTagUpdated() {
  tagDialogVisible.value = false;
  editingTag.value = undefined;
  tagStore.fetchTags();
}

function editTag(tag: Tag) {
  editingTag.value = tag;
  tagDialogVisible.value = true;
}

// Expose methods to parent components
defineExpose({
  resetToAllView,
});
</script>

<style scoped>
.sidebar {
  width: 240px;
  height: 100%;
  background: #f5f7fa;
  border-right: 1px solid #e4e7ed;
  display: flex;
  flex-direction: column;
  transition: width 0.3s ease;
}

.sidebar.collapsed {
  width: 60px;
}

.sidebar-header {
  flex-shrink: 0;
  padding: 16px;
  border-bottom: 1px solid #e4e7ed;
  display: flex;
  align-items: center;
  justify-content: center;
}

.logo {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 18px;
  font-weight: bold;
  color: var(--el-color-primary);
}

.logo-icon {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.logo-icon :deep(svg) {
  width: 100%;
  height: 100%;
}

.logo-icon-only {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.logo-icon-only :deep(svg) {
  width: 100%;
  height: 100%;
}

.sidebar-content {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
}

.sidebar-content::-webkit-scrollbar {
  width: 6px;
}

.sidebar-content::-webkit-scrollbar-track {
  background: transparent;
}

.sidebar-content::-webkit-scrollbar-thumb {
  background: #dcdfe6;
  border-radius: 3px;
}

.sidebar-content::-webkit-scrollbar-thumb:hover {
  background: #c0c4cc;
}

.sidebar-section {
  padding: 12px 0;
  border-bottom: 1px solid #e4e7ed;
}

.section-title {
  padding: 0 16px 8px;
  font-size: 12px;
  color: #909399;
  text-transform: uppercase;
}

.quick-links,
.groups {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 0 8px;
}

.tags {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  gap: 6px;
  padding: 0 8px;
}

.quick-link,
.group-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 12px;
  border-radius: 6px;
  color: #606266;
  text-decoration: none;
  transition: all 0.2s ease;
}

.quick-link:hover,
.group-item:hover {
  background: #e4e7ed;
  color: var(--el-color-primary);
}

.quick-link.active,
.group-item.active {
  background: var(--el-color-primary);
  color: white;
}

.group-icon {
  font-size: 16px;
}

.tag-item {
  display: inline-block;
  padding: 4px 12px;
  border-radius: 12px;
  font-size: 12px;
  color: white;
  cursor: pointer;
  transition: opacity 0.2s ease;
}

.tag-item:hover {
  opacity: 0.8;
}

.empty-tags {
  padding: 8px 12px;
  text-align: center;
}

.empty-text {
  color: #909399;
  font-size: 12px;
}

.sidebar-footer {
  flex-shrink: 0;
  padding: 12px 8px;
  border-top: 1px solid #e4e7ed;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.footer-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 12px;
  border-radius: 6px;
  color: #606266;
  text-decoration: none;
  transition: all 0.2s ease;
}

.footer-item:hover {
  background: #e4e7ed;
  color: var(--el-color-primary);
}

.footer-item.router-link-active {
  background: var(--el-color-primary);
  color: white;
}

/* Dark theme */
[data-theme='dark'] .sidebar {
  background: #1a1a1a;
  border-right-color: #2a2a2a;
}

[data-theme='dark'] .sidebar-header {
  border-bottom-color: #2a2a2a;
}

[data-theme='dark'] .section-title {
  color: #909399;
}

[data-theme='dark'] .sidebar-content::-webkit-scrollbar-thumb {
  background: #4a4a4a;
}

[data-theme='dark'] .sidebar-content::-webkit-scrollbar-thumb:hover {
  background: #5a5a5a;
}

[data-theme='dark'] .quick-link,
[data-theme='dark'] .group-item,
[data-theme='dark'] .footer-item {
  color: #e0e0e0;
}

[data-theme='dark'] .quick-link:hover,
[data-theme='dark'] .group-item:hover,
[data-theme='dark'] .footer-item:hover {
  background: #2a2a2a;
  color: var(--el-color-primary);
}

[data-theme='dark'] .quick-link.active,
[data-theme='dark'] .group-item.active,
[data-theme='dark'] .footer-item.router-link-active {
  background: var(--el-color-primary);
  color: white;
}

[data-theme='dark'] .sidebar-footer {
  border-top-color: #2a2a2a;
}

/* 紧凑模式 */
[data-density='compact'] .sidebar {
  width: 200px;
}

[data-density='compact'] .sidebar.collapsed {
  width: 50px;
}

[data-density='compact'] .sidebar-header {
  padding: 12px;
}

[data-density='compact'] .sidebar-section {
  padding: 8px 0;
}

[data-density='compact'] .section-title {
  padding: 0 12px 6px;
}

[data-density='compact'] .quick-links,
[data-density='compact'] .groups {
  gap: 3px;
  padding: 0 6px;
}

[data-density='compact'] .tags {
  gap: 4px;
  padding: 0 6px;
}

[data-density='compact'] .quick-link,
[data-density='compact'] .group-item {
  gap: 8px;
  padding: 6px 10px;
}

[data-density='compact'] .tag-item {
  padding: 3px 10px;
}

[data-density='compact'] .empty-tags {
  padding: 6px 10px;
}

[data-density='compact'] .sidebar-footer {
  padding: 8px 6px;
  gap: 3px;
}

[data-density='compact'] .footer-item {
  gap: 8px;
  padding: 6px 10px;
}
</style>
