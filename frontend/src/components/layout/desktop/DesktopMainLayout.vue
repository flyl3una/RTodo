<template>
  <div class="desktop-main-layout">
    <!-- Desktop Sidebar -->
    <DesktopSidebar
      ref="sidebarRef"
      :collapsed="uiStore.sidebarCollapsed"
      @toggle="uiStore.toggleSidebar"
    />

    <!-- Main Content -->
    <div class="main-content" :class="{ collapsed: uiStore.sidebarCollapsed }">
      <!-- Desktop Header -->
      <DesktopHeader
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
      :current-view="currentViewState.currentView"
      :filter-group-id="currentViewState.filterGroupId"
      :filter-tag-id="currentViewState.filterTagId"
      @created="handleTodoCreated"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useUIStore } from '@/stores';
import { useTodoStore } from '@/stores';
import { useGroupStore } from '@/stores';
import { useTagStore } from '@/stores';
import DesktopSidebar from './DesktopSidebar.vue';
import DesktopHeader from './DesktopHeader.vue';
import CreateTodoDialog from '../../todo/CreateTodoDialog.vue';
import { useLayout } from '@/composables/useLayout';

const uiStore = useUIStore();
const todoStore = useTodoStore();
const groupStore = useGroupStore();
const tagStore = useTagStore();

const { initLayout, loadInitialData } = useLayout();

const sidebarRef = ref<InstanceType<typeof DesktopSidebar> | null>(null);

// Get current view state from sidebar for passing to CreateTodoDialog
const currentViewState = computed(() => {
  return sidebarRef.value?.getCurrentView() || {
    currentView: 'todo',
    filterGroupId: undefined,
    filterTagId: undefined,
  };
});

onMounted(async () => {
  const envOk = await initLayout();
  if (envOk) {
    await loadInitialData();
  }
});

async function handleTodoCreated() {
  sidebarRef.value?.refreshCurrentView();
}
</script>

<style scoped>
.desktop-main-layout {
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
