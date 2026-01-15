import { defineStore } from 'pinia';
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';

export type ViewMode = 'list' | 'card';
export type Theme = 'light' | 'dark' | 'auto';
export type Language = 'zh-CN' | 'zh-TW' | 'en-US' | 'ja-JP';
export type DensityMode = 'comfortable' | 'compact';

export const useUIStore = defineStore('ui', () => {
  const { locale } = useI18n();

  // Sidebar
  const sidebarCollapsed = ref(false);

  // View mode
  const viewMode = ref<ViewMode>('list');

  // Density mode
  const densityMode = ref<DensityMode>('comfortable');

  // Theme
  const theme = ref<Theme>('light');

  // Language
  const language = ref<Language>('zh-CN');

  // Theme color
  const themeColor = ref<string>('#409EFF');

  // Developer mode
  const developerMode = ref<boolean>(false);

  // Selected todo
  const selectedTodoId = ref<string | null>(null);

  // Dialog states
  const createTodoDialogVisible = ref(false);
  const groupManagerVisible = ref(false);
  const tagManagerVisible = ref(false);

  // Initialize language from localStorage
  const savedLanguage = localStorage.getItem('rtodo-language') as Language;
  if (savedLanguage && ['zh-CN', 'zh-TW', 'en-US', 'ja-JP'].includes(savedLanguage)) {
    language.value = savedLanguage;
    locale.value = savedLanguage;
  }

  // Initialize theme color from localStorage
  const savedThemeColor = localStorage.getItem('rtodo-theme-color');
  if (savedThemeColor) {
    themeColor.value = savedThemeColor;
    document.documentElement.style.setProperty('--el-color-primary', savedThemeColor);
  }

  // Initialize developer mode from localStorage
  const savedDeveloperMode = localStorage.getItem('rtodo-developer-mode');
  if (savedDeveloperMode) {
    developerMode.value = savedDeveloperMode === 'true';
  }

  // Initialize density mode from localStorage
  const savedDensityMode = localStorage.getItem('rtodo-density-mode') as DensityMode;
  if (savedDensityMode && ['comfortable', 'compact'].includes(savedDensityMode)) {
    densityMode.value = savedDensityMode;
    document.documentElement.setAttribute('data-density', savedDensityMode);
  }

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

  function setLanguage(newLanguage: Language) {
    language.value = newLanguage;
    locale.value = newLanguage;
    // Persist to localStorage
    localStorage.setItem('rtodo-language', newLanguage);
  }

  function setThemeColor(color: string) {
    themeColor.value = color;
    document.documentElement.style.setProperty('--el-color-primary', color);
    localStorage.setItem('rtodo-theme-color', color);
  }

  function setDeveloperMode(enabled: boolean) {
    developerMode.value = enabled;
    localStorage.setItem('rtodo-developer-mode', String(enabled));
  }

  function setDensityMode(mode: DensityMode) {
    densityMode.value = mode;
    document.documentElement.setAttribute('data-density', mode);
    localStorage.setItem('rtodo-density-mode', mode);
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
    language,
    themeColor,
    developerMode,
    densityMode,
    selectedTodoId,
    createTodoDialogVisible,
    groupManagerVisible,
    tagManagerVisible,
    // Actions
    toggleSidebar,
    setSidebarCollapsed,
    setViewMode,
    setTheme,
    setLanguage,
    setThemeColor,
    setDeveloperMode,
    setDensityMode,
    selectTodo,
    showCreateTodoDialog,
    hideCreateTodoDialog,
    showGroupManager,
    hideGroupManager,
    showTagManager,
    hideTagManager,
  };
});
