<template>
  <div class="mobile-main-layout">
    <!-- Mobile Top Bar -->
    <MobileTopBar />

    <!-- Main Content -->
    <div class="main-content">
      <!-- Router View -->
      <div class="content-area mobile">
        <router-view />
      </div>
    </div>

    <!-- Mobile Bottom Navigation -->
    <MobileNav />

    <!-- Mobile Drawer -->
    <MobileDrawer />

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
import { computed, onMounted } from 'vue';
import { useUIStore } from '@/stores';
import { useTodoStore } from '@/stores';
import { useGroupStore } from '@/stores';
import { useTagStore } from '@/stores';
import MobileTopBar from './MobileTopBar.vue';
import MobileNav from './MobileNav.vue';
import MobileDrawer from './MobileDrawer.vue';
import CreateTodoDialog from '../../todo/CreateTodoDialog.vue';
import { useLayout } from '@/composables/useLayout';
import { useTodoFilters } from '@/composables/useTodoFilters';

const uiStore = useUIStore();
const todoStore = useTodoStore();
const groupStore = useGroupStore();
const tagStore = useTagStore();

const { initLayout, loadInitialData } = useLayout();
const { getCurrentView, refreshCurrentView } = useTodoFilters();

// Get current view state for passing to CreateTodoDialog
const currentViewState = computed(() => {
  return getCurrentView();
});

onMounted(async () => {
  const envOk = await initLayout();
  if (envOk) {
    await loadInitialData();
  }
});

async function handleTodoCreated() {
  refreshCurrentView();
}
</script>

<style scoped>
.mobile-main-layout {
  display: flex;
  flex-direction: column;
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
  padding: 12px;
  padding-bottom: 72px; /* Space for bottom nav */
}

.content-area::-webkit-scrollbar {
  width: 6px;
}

.content-area::-webkit-scrollbar-track {
  background: transparent;
}

.content-area::-webkit-scrollbar-thumb {
  background: var(--el-border-color);
  border-radius: 3px;
}

.content-area::-webkit-scrollbar-thumb:hover {
  background: var(--el-border-color-dark);
}

/* Dark theme */
:global(html.dark) .content-area::-webkit-scrollbar-thumb {
  background: var(--el-border-color-dark);
}

:global(html.dark) .content-area::-webkit-scrollbar-thumb:hover {
  background: var(--el-border-color-darker);
}
</style>
