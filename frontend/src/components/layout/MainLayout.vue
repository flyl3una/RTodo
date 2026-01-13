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
import { useUIStore } from '@/stores';
import { useTodoStore } from '@/stores';
import { useGroupStore } from '@/stores';
import { useTagStore } from '@/stores';
import Sidebar from './Sidebar.vue';
import Header from './Header.vue';
import CreateTodoDialog from '../todo/CreateTodoDialog.vue';

const uiStore = useUIStore();
const todoStore = useTodoStore();
const groupStore = useGroupStore();
const tagStore = useTagStore();

const sidebarRef = ref<InstanceType<typeof Sidebar> | null>(null);

onMounted(async () => {
  // Load initial data
  try {
    await todoStore.fetchTodos();
    await groupStore.fetchGroups();
    await tagStore.fetchTags();
  } catch (error) {
    console.error('Failed to load initial data:', error);
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
  background: #f1f1f1;
}

.content-area::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 4px;
}

.content-area::-webkit-scrollbar-thumb:hover {
  background: #a8a8a8;
}

/* Dark theme */
[data-theme='dark'] .content-area::-webkit-scrollbar-track {
  background: #2a2a2a;
}

[data-theme='dark'] .content-area::-webkit-scrollbar-thumb {
  background: #4a4a4a;
}

[data-theme='dark'] .content-area::-webkit-scrollbar-thumb:hover {
  background: #5a5a5a;
}
</style>
