<template>
  <el-config-provider :locale="elementLocale">
    <MainLayout />
  </el-config-provider>
</template>

<script setup lang="ts">
import { computed, onUnmounted, watch } from 'vue';
import { useUIStore } from '@/stores';
import { useI18n } from 'vue-i18n';
import zhCn from 'element-plus/es/locale/lang/zh-cn';
import en from 'element-plus/es/locale/lang/en';
import ja from 'element-plus/es/locale/lang/ja';
import MainLayout from '@/components/layout/MainLayout.vue';

const uiStore = useUIStore();
const { locale } = useI18n();

// Element Plus locale mapping
const localeMap = {
  'zh-CN': zhCn,
  'zh-TW': zhCn,
  'en-US': en,
  'ja-JP': ja,
};

// 响应式的 Element Plus locale
const elementLocale = computed(() => {
  return localeMap[locale.value as keyof typeof localeMap] || zhCn;
});

// 处理右键菜单限制
function handleContextMenu(event: MouseEvent) {
  if (uiStore.developerMode) return;

  // 检查点击的元素或其父元素是否有 data-allow-context-menu 属性
  const target = event.target as HTMLElement;
  const allowContextMenu = target.closest('[data-allow-context-menu]');
  if (!allowContextMenu) {
    event.preventDefault();
    event.stopPropagation();
  }
}

// 处理快捷键限制
function handleKeyDown(event: KeyboardEvent) {
  if (uiStore.developerMode) return;

  // 阻止常见的开发者快捷键
  const blockedShortcuts = [
    // F12 (开发者工具)
    event.key === 'F12',
    // Ctrl/Cmd + Shift + I (开发者工具)
    (event.ctrlKey || event.metaKey) && event.shiftKey && (event.key === 'I' || event.key === 'i'),
    // Ctrl/Cmd + Shift + J (控制台)
    (event.ctrlKey || event.metaKey) && event.shiftKey && (event.key === 'J' || event.key === 'j'),
    // Ctrl/Cmd + U (查看源代码)
    (event.ctrlKey || event.metaKey) && (event.key === 'u' || event.key === 'U'),
    // Ctrl/Cmd + Shift + C (选择元素)
    (event.ctrlKey || event.metaKey) && event.shiftKey && (event.key === 'C' || event.key === 'c'),
  ];

  if (blockedShortcuts.some(Boolean)) {
    event.preventDefault();
    event.stopPropagation();
  }
}

// 监听开发模式变化并添加/移除事件监听器
watch(
  () => uiStore.developerMode,
  (enabled) => {
    console.log('[App] Developer mode changed:', enabled);
    console.log('[App] Current developerMode value:', uiStore.developerMode);
    if (!enabled) {
      // 开发模式关闭：添加事件监听器
      document.addEventListener('contextmenu', handleContextMenu, true);
      document.addEventListener('keydown', handleKeyDown, true);
      console.log('[App] Event listeners added (dev mode OFF)');
    } else {
      // 开发模式开启：移除事件监听器
      document.removeEventListener('contextmenu', handleContextMenu, true);
      document.removeEventListener('keydown', handleKeyDown, true);
      console.log('[App] Event listeners removed (dev mode ON)');
    }
  },
  { immediate: true }
);

onUnmounted(() => {
  // 清理事件监听器
  document.removeEventListener('contextmenu', handleContextMenu, true);
  document.removeEventListener('keydown', handleKeyDown, true);
});
</script>

<style>
/* 全局样式在 main.css 中定义 */
</style>
