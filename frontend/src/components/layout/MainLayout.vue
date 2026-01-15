<template>
  <div class="main-layout">
    <!-- Sidebar -->
    <Sidebar
      ref="sidebarRef"
      :collapsed="uiStore.sidebarCollapsed"
      @toggle="uiStore.toggleSidebar"
    />

    <!-- Main Content -->
    <div class="main-content" :class="{ collapsed: uiStore.sidebarCollapsed }">
      <!-- Header -->
      <Header
        :collapsed="uiStore.sidebarCollapsed"
        @toggle-sidebar="uiStore.toggleSidebar"
        @show-create="uiStore.showCreateTodoDialog"
      />

      <!-- Router View -->
      <div class="content-area">
        <router-view />
      </div>
    </div>

    <!-- Create Todo Dialog -->
    <CreateTodoDialog
      v-model="uiStore.createTodoDialogVisible"
      @created="handleTodoCreated"
    />
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useUIStore } from '@/stores';
import { useTodoStore } from '@/stores';
import { useGroupStore } from '@/stores';
import { useTagStore } from '@/stores';
import { getEnvironmentInfo, isTauriAvailable, testIPCConnection } from '@/utils/tauri-helpers';
import Sidebar from './Sidebar.vue';
import Header from './Header.vue';
import CreateTodoDialog from '../todo/CreateTodoDialog.vue';

const { t } = useI18n();

const uiStore = useUIStore();
const todoStore = useTodoStore();
const groupStore = useGroupStore();
const tagStore = useTagStore();

const sidebarRef = ref<InstanceType<typeof Sidebar> | null>(null);

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

function handleTodoCreated() {
  // Reset sidebar to 'all' view so the newly created todo is visible
  sidebarRef.value?.resetToAllView();
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
</style>
