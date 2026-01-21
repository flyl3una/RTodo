<template>
  <el-config-provider :locale="elementLocale">
    <component :is="currentLayout" />
  </el-config-provider>
</template>

<script setup lang="ts">
import { computed, defineAsyncComponent } from 'vue';
import { useUIStore } from '@/stores';
import { useI18n } from 'vue-i18n';
import zhCn from 'element-plus/es/locale/lang/zh-cn';
import en from 'element-plus/es/locale/lang/en';
import ja from 'element-plus/es/locale/lang/ja';

const uiStore = useUIStore();
const { locale } = useI18n();

const DesktopMainLayout = defineAsyncComponent(() =>
  import('@/components/layout/desktop/DesktopMainLayout.vue')
);
const MobileMainLayout = defineAsyncComponent(() =>
  import('@/components/layout/mobile/MobileMainLayout.vue')
);

const currentLayout = computed(() => {
  return uiStore.isMobile ? MobileMainLayout : DesktopMainLayout;
});

// Initialize theme on mount
import { onMounted, onUnmounted, watch } from 'vue';

onMounted(() => {
  // Theme is already initialized in the store, but we ensure it's applied here
  const savedTheme = localStorage.getItem('rtodo-theme') as 'light' | 'dark' | 'auto' | null;
  if (savedTheme) {
    uiStore.setTheme(savedTheme);
  }
});

// Element Plus locale mapping
const localeMap = {
  'zh-CN': zhCn,
  'zh-TW': zhCn,
  'en-US': en,
  'ja-JP': ja,
};

// Responsive Element Plus locale
const elementLocale = computed(() => {
  return localeMap[locale.value as keyof typeof localeMap] || zhCn;
});

// Handle right-click menu restrictions
function handleContextMenu(event: MouseEvent) {
  if (uiStore.developerMode) return;

  const target = event.target as HTMLElement;
  const allowContextMenu = target.closest('[data-allow-context-menu]');
  if (!allowContextMenu) {
    event.preventDefault();
    event.stopPropagation();
  }
}

// Handle keyboard shortcuts restrictions
function handleKeyDown(event: KeyboardEvent) {
  if (uiStore.developerMode) return;

  const blockedShortcuts = [
    event.key === 'F12',
    (event.ctrlKey || event.metaKey) && event.shiftKey && (event.key === 'I' || event.key === 'i'),
    (event.ctrlKey || event.metaKey) && event.shiftKey && (event.key === 'J' || event.key === 'j'),
    (event.ctrlKey || event.metaKey) && (event.key === 'u' || event.key === 'U'),
    (event.ctrlKey || event.metaKey) && event.shiftKey && (event.key === 'C' || event.key === 'c'),
  ];

  if (blockedShortcuts.some(Boolean)) {
    event.preventDefault();
    event.stopPropagation();
  }
}

// Listen for developer mode changes
watch(
  () => uiStore.developerMode,
  (enabled) => {
    console.log('[App] Developer mode changed:', enabled);
    if (!enabled) {
      document.addEventListener('contextmenu', handleContextMenu, true);
      document.addEventListener('keydown', handleKeyDown, true);
      console.log('[App] Event listeners added (dev mode OFF)');
    } else {
      document.removeEventListener('contextmenu', handleContextMenu, true);
      document.removeEventListener('keydown', handleKeyDown, true);
      console.log('[App] Event listeners removed (dev mode ON)');
    }
  },
  { immediate: true }
);

onUnmounted(() => {
  document.removeEventListener('contextmenu', handleContextMenu, true);
  document.removeEventListener('keydown', handleKeyDown, true);
});
</script>

<style>
/* Global styles are defined in main.css */
</style>
