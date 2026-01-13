import { defineStore } from 'pinia';
import { ref } from 'vue';

export type ViewMode = 'list' | 'card';
export type Theme = 'light' | 'dark' | 'auto';

export const useUIStore = defineStore('ui', () => {
  // Sidebar
  const sidebarCollapsed = ref(false);

  // View mode
  const viewMode = ref<ViewMode>('list');

  // Theme
  const theme = ref<Theme>('light');

  // Selected todo
  const selectedTodoId = ref<string | null>(null);

  // Dialog states
  const createTodoDialogVisible = ref(false);
  const groupManagerVisible = ref(false);
  const tagManagerVisible = ref(false);

  // Actions
  function toggleSidebar() {
    sidebarCollapsed.value = !sidebarCollapsed.value;
  }

  function setSidebarCollapsed(collapsed: boolean) {
    sidebarCollapsed.value = collapsed;
  }

  function setViewMode(mode: ViewMode) {
    viewMode.value = mode;
  }

  function setTheme(newTheme: Theme) {
    theme.value = newTheme;
    // Apply theme to document
    if (newTheme === 'dark') {
      document.documentElement.setAttribute('data-theme', 'dark');
    } else if (newTheme === 'light') {
      document.documentElement.removeAttribute('data-theme');
    } else {
      // Auto: check system preference
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      if (prefersDark) {
        document.documentElement.setAttribute('data-theme', 'dark');
      } else {
        document.documentElement.removeAttribute('data-theme');
      }
    }
  }

  function selectTodo(id: string | null) {
    selectedTodoId.value = id;
  }

  function showCreateTodoDialog() {
    createTodoDialogVisible.value = true;
  }

  function hideCreateTodoDialog() {
    createTodoDialogVisible.value = false;
  }

  function showGroupManager() {
    groupManagerVisible.value = true;
  }

  function hideGroupManager() {
    groupManagerVisible.value = false;
  }

  function showTagManager() {
    tagManagerVisible.value = true;
  }

  function hideTagManager() {
    tagManagerVisible.value = false;
  }

  return {
    // State
    sidebarCollapsed,
    viewMode,
    theme,
    selectedTodoId,
    createTodoDialogVisible,
    groupManagerVisible,
    tagManagerVisible,
    // Actions
    toggleSidebar,
    setSidebarCollapsed,
    setViewMode,
    setTheme,
    selectTodo,
    showCreateTodoDialog,
    hideCreateTodoDialog,
    showGroupManager,
    hideGroupManager,
    showTagManager,
    hideTagManager,
  };
});
