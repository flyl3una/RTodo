<template>
  <div class="sidebar" :class="{ collapsed }">
    <!-- Header -->
    <div class="sidebar-header">
      <div class="logo" v-if="!collapsed">
        <span class="logo-icon">📝</span>
        <span class="logo-text">RTodo</span>
      </div>
      <div class="logo-icon-only" v-else>📝</div>
    </div>

    <!-- Quick Access -->
    <div class="sidebar-section" v-if="!collapsed">
      <div class="section-title">快速访问</div>
      <div class="quick-links">
        <a
          href="#"
          class="quick-link"
          :class="{ active: currentView === 'all' }"
          @click.prevent="setFilter('all')"
        >
          <el-icon><List /></el-icon>
          <span>全部任务</span>
        </a>
        <a
          href="#"
          class="quick-link"
          :class="{ active: currentView === 'today' }"
          @click.prevent="setFilter('today')"
        >
          <el-icon><Calendar /></el-icon>
          <span>今天</span>
        </a>
        <a
          href="#"
          class="quick-link"
          :class="{ active: currentView === 'important' }"
          @click.prevent="setFilter('important')"
        >
          <el-icon><Star /></el-icon>
          <span>重要</span>
        </a>
        <a
          href="#"
          class="quick-link"
          :class="{ active: currentView === 'urgent' }"
          @click.prevent="setFilter('urgent')"
        >
          <el-icon><Warning /></el-icon>
          <span>紧急</span>
        </a>
        <a
          href="#"
          class="quick-link"
          :class="{ active: currentView === 'completed' }"
          @click.prevent="setFilter('completed')"
        >
          <el-icon><CircleCheck /></el-icon>
          <span>已完成</span>
        </a>
      </div>
    </div>

    <!-- Task Groups -->
    <div class="sidebar-section">
      <div class="section-title" v-if="!collapsed">任务组</div>
      <div class="groups">
        <a
          href="#"
          class="group-item"
          :class="{ active: filterGroupId === group.id }"
          v-for="group in groups"
          :key="group.id"
          @click.prevent="selectGroup(group.id)"
          @contextmenu.prevent="editGroup(group)"
        >
          <span class="group-icon">{{ group.icon || '📁' }}</span>
          <span class="group-name" v-if="!collapsed">{{ group.name }}</span>
        </a>
        <a
          href="#"
          class="group-item add-group"
          @click.prevent="showAddGroup"
          v-if="!collapsed"
        >
          <el-icon><Plus /></el-icon>
          <span>添加任务组</span>
        </a>
      </div>
    </div>

    <!-- Tags -->
    <div class="sidebar-section" v-if="!collapsed">
      <div class="section-title">
        标签
        <el-link type="primary" @click="showTagManage" style="float: right; font-size: 12px;">
          新建
        </el-link>
      </div>
      <div class="tags" v-if="tags.length > 0">
        <span
          class="tag-item"
          v-for="tag in tags.slice(0, 8)"
          :key="tag.id"
          :style="{ backgroundColor: tag.color }"
          @click="selectTag(tag.id)"
        >
          {{ tag.name }}
        </span>
      </div>
      <div v-else class="empty-tags">
        <span class="empty-text">暂无标签</span>
        <el-link type="primary" @click="showTagManage">添加标签</el-link>
      </div>
    </div>

    <!-- Bottom Actions -->
    <div class="sidebar-footer">
      <router-link to="/stats" class="footer-item" title="统计">
        <el-icon><TrendCharts /></el-icon>
        <span v-if="!collapsed">统计</span>
      </router-link>
      <router-link to="/settings" class="footer-item" title="设置">
        <el-icon><Setting /></el-icon>
        <span v-if="!collapsed">设置</span>
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
  color: #409eff;
}

.logo-icon-only {
  font-size: 24px;
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
.groups,
.tags {
  display: flex;
  flex-direction: column;
  gap: 4px;
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
  color: #409eff;
}

.quick-link.active,
.group-item.active {
  background: #409eff;
  color: white;
}

.quick-link.add-group {
  color: #67c23a;
}

.quick-link.add-group:hover {
  background: #e1f3d8;
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
  margin-top: auto;
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
  color: #409eff;
}

.footer-item.router-link-active {
  background: #409eff;
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

[data-theme='dark'] .quick-link,
[data-theme='dark'] .group-item,
[data-theme='dark'] .footer-item {
  color: #e0e0e0;
}

[data-theme='dark'] .quick-link:hover,
[data-theme='dark'] .group-item:hover,
[data-theme='dark'] .footer-item:hover {
  background: #2a2a2a;
  color: #409eff;
}

[data-theme='dark'] .quick-link.active,
[data-theme='dark'] .group-item.active,
[data-theme='dark'] .footer-item.router-link-active {
  background: #409eff;
  color: white;
}

[data-theme='dark'] .sidebar-footer {
  border-top-color: #2a2a2a;
}
</style>
