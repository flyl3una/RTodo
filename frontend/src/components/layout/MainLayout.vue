<template>
  <div class="main-layout" :class="{ 'is-mobile': uiStore.isMobile }">
    <!-- Desktop Sidebar -->
    <Sidebar
      v-if="!uiStore.isMobile"
      ref="sidebarRef"
      :collapsed="uiStore.sidebarCollapsed"
      @toggle="uiStore.toggleSidebar"
    />

    <!-- Main Content -->
    <div class="main-content" :class="{ collapsed: !uiStore.isMobile && uiStore.sidebarCollapsed }">
      <!-- Desktop Header -->
      <Header
        v-if="!uiStore.isMobile"
        :collapsed="uiStore.sidebarCollapsed"
        @toggle-sidebar="uiStore.toggleSidebar"
        @show-create="uiStore.showCreateTodoDialog"
      />

      <!-- Mobile Top Bar -->
      <MobileTopBar v-if="uiStore.isMobile" />

      <!-- Router View -->
      <div class="content-area" :class="{ 'mobile': uiStore.isMobile }">
        <router-view />
      </div>
    </div>

    <!-- Mobile Bottom Navigation -->
    <MobileNav v-if="uiStore.isMobile" />

    <!-- Mobile Drawer -->
    <MobileDrawer v-if="uiStore.isMobile" />

    <!-- Create Todo Dialog -->
    <CreateTodoDialog
      v-model="uiStore.createTodoDialogVisible"
      :current-view="currentViewState.currentView"
      :filter-group-id="currentViewState.filterGroupId"
      :filter-tag-id="currentViewState.filterTagId"
      @created="handleTodoCreated"
    />
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { useUIStore } from '@/stores';
import { useTodoStore } from '@/stores';
import { useGroupStore } from '@/stores';
import { useTagStore } from '@/stores';
import { getEnvironmentInfo, isTauriAvailable, testIPCConnection } from '@/utils/tauri-helpers';
import Sidebar from './Sidebar.vue';
import Header from './Header.vue';
import CreateTodoDialog from '../todo/CreateTodoDialog.vue';
import MobileTopBar from '../mobile/app/MobileTopBar.vue';
import MobileDrawer from '../mobile/app/MobileDrawer.vue';
import MobileNav from './MobileNav.vue';

const { t } = useI18n();

const uiStore = useUIStore();
const todoStore = useTodoStore();
const groupStore = useGroupStore();
const tagStore = useTagStore();

const sidebarRef = ref<InstanceType<typeof Sidebar> | null>(null);

// Get current view state from sidebar for passing to CreateTodoDialog
const currentViewState = computed(() => {
  return sidebarRef.value?.getCurrentView() || {
    currentView: 'todo',
    filterGroupId: undefined,
    filterTagId: undefined,
  };
});

onMounted(async () => {
  // ==================== 环境检测 ====================
  console.log('[MainLayout] ==================== 环境检测 ====================');

  const env = getEnvironmentInfo();
  console.log('[MainLayout] 当前环境信息:', env);

  if (!isTauriAvailable()) {
    console.error('[MainLayout] ❌ Tauri环境检测失败！');
    console.error('[MainLayout] 请确保：');
    console.error('[MainLayout]   1. 使用 "cargo tauri dev" 命令启动应用');
    console.error('[MainLayout]   2. 在Tauri桌面窗口中测试（不是浏览器）');
    console.error('[MainLayout] =================================================');

    // 显示用户友好的错误消息
    const showError = () => {
      import('element-plus').then(({ ElMessageBox }) => {
        ElMessageBox.alert(
          t('messages.tauriEnvironmentError'),
          t('messages.environmentError'),
          { type: 'error' }
        );
      });
    };
    showError();

    // 不继续加载数据
    return;
  }

  console.log('[MainLayout] ✅ Tauri环境检测成功');

  // 测试IPC连接
  console.log('[MainLayout] 测试IPC连接...');
  const ipcConnected = await testIPCConnection();
  if (!ipcConnected) {
    console.error('[MainLayout] ❌ IPC连接测试失败');
  } else {
    console.log('[MainLayout] ✅ IPC连接正常');
  }
  console.log('[MainLayout] =================================================');
  // =================================================

  // Load initial data
  try {
    await todoStore.fetchTodos();
    await groupStore.fetchGroups();
    await tagStore.fetchTags();
  } catch (error) {
    console.error('[MainLayout] Failed to load initial data:', error);
  }
});

async function handleTodoCreated() {
  // Refresh current view instead of resetting to 'all'
  sidebarRef.value?.refreshCurrentView();
}
</script>

<style scoped>
.main-layout {
  display: flex;
  width: 100%;
  height: 100vh;
  overflow: hidden;
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
  overflow: hidden;
}

.content-area {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.content-area::-webkit-scrollbar {
  width: 8px;
}

.content-area::-webkit-scrollbar-track {
  background: var(--el-fill-color);
}

.content-area::-webkit-scrollbar-thumb {
  background: var(--el-border-color);
  border-radius: 4px;
}

.content-area::-webkit-scrollbar-thumb:hover {
  background: var(--el-border-color-dark);
}

/* Dark theme */
:global(html.dark) .content-area::-webkit-scrollbar-track {
  background: var(--el-fill-color-darker);
}

:global(html.dark) .content-area::-webkit-scrollbar-thumb {
  background: var(--el-border-color-dark);
}

:global(html.dark) .content-area::-webkit-scrollbar-thumb:hover {
  background: var(--el-border-color-darker);
}

/* Mobile styles */
.main-layout.is-mobile {
  flex-direction: column;
}

.main-layout.is-mobile .content-area {
  padding: 12px;
  padding-bottom: 72px; /* Space for bottom nav */
}

.main-layout.is-mobile .main-content {
  height: auto;
  flex: 1;
}
</style>
