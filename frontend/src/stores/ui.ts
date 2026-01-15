import { defineStore } from 'pinia';
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';

export type ViewMode = 'list' | 'card';
export type Theme = 'light' | 'dark' | 'auto';
export type Language = 'zh-CN' | 'zh-TW' | 'en-US' | 'ja-JP';
export type DensityMode = 'comfortable' | 'compact';
export type CloseBehavior = 'direct' | 'minimize_to_tray';

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

  // Helper function to apply theme (defined before initialization)
  function applyTheme(newTheme: Theme) {
    if (newTheme === 'dark') {
      document.documentElement.setAttribute('data-theme', 'dark');
      // Add Element Plus dark theme class
      document.documentElement.classList.add('dark');
    } else if (newTheme === 'light') {
      document.documentElement.removeAttribute('data-theme');
      document.documentElement.classList.remove('dark');
    } else {
      // Auto: check system preference
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      if (prefersDark) {
        document.documentElement.setAttribute('data-theme', 'dark');
        document.documentElement.classList.add('dark');
      } else {
        document.documentElement.removeAttribute('data-theme');
        document.documentElement.classList.remove('dark');
      }
    }
  }

  // Initialize theme from localStorage
  const savedTheme = localStorage.getItem('rtodo-theme') as Theme;
  if (savedTheme && ['light', 'dark', 'auto'].includes(savedTheme)) {
    theme.value = savedTheme;
    // Apply saved theme immediately
    applyTheme(savedTheme);
  } else {
    // Default to light theme
    applyTheme('light');
  }

  // Language
  const language = ref<Language>('zh-CN');

  // Theme color
  const themeColor = ref<string>('#409EFF');

  // Developer mode
  const developerMode = ref<boolean>(false);

  // Global shortcut
  const globalShortcut = ref<string>('CmdOrCtrl+Shift+T');

  // Close behavior
  const closeBehavior = ref<CloseBehavior>('direct');

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

  // Initialize global shortcut from localStorage
  const savedGlobalShortcut = localStorage.getItem('rtodo-global-shortcut');
  if (savedGlobalShortcut) {
    globalShortcut.value = savedGlobalShortcut;
  }

  // Initialize close behavior from localStorage and sync with backend
  const savedCloseBehavior = localStorage.getItem('rtodo-close-behavior') as CloseBehavior;
  if (savedCloseBehavior && ['direct', 'minimize_to_tray'].includes(savedCloseBehavior)) {
    closeBehavior.value = savedCloseBehavior;
    // 异步同步到后端（不使用 await，因为是顶层初始化）
    import('@/api/tauri').then(({ setCloseBehavior: setBehavior }) => {
      setBehavior(savedCloseBehavior).catch((error) => {
        console.error('Failed to sync close behavior to backend:', error);
      });
    });
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
    applyTheme(newTheme);
    // Persist to localStorage
    localStorage.setItem('rtodo-theme', newTheme);
  }

  // Listen for system theme changes when in auto mode
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
    if (theme.value === 'auto') {
      applyTheme('auto');
    }
  });

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

  async function setGlobalShortcut(shortcut: string) {
    globalShortcut.value = shortcut;
    localStorage.setItem('rtodo-global-shortcut', shortcut);
    // 调用后端 API 注册快捷键
    try {
      const { setGlobalShortcut: setShortcut } = await import('@/api/tauri');
      await setShortcut(shortcut);
    } catch (error) {
      console.error('Failed to register global shortcut:', error);
      throw error;
    }
  }

  async function setCloseBehavior(behavior: CloseBehavior) {
    closeBehavior.value = behavior;
    localStorage.setItem('rtodo-close-behavior', behavior);
    // 调用后端 API 设置关闭行为
    try {
      const { setCloseBehavior: setBehavior } = await import('@/api/tauri');
      await setBehavior(behavior);
    } catch (error) {
      console.error('Failed to set close behavior:', error);
      throw error;
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
    language,
    themeColor,
    developerMode,
    densityMode,
    globalShortcut,
    closeBehavior,
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
    setGlobalShortcut,
    setCloseBehavior,
    selectTodo,
    showCreateTodoDialog,
    hideCreateTodoDialog,
    showGroupManager,
    hideGroupManager,
    showTagManager,
    hideTagManager,
  };
});
