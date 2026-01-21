import { onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useUIStore } from '@/stores';
import { useTodoStore } from '@/stores';
import { useGroupStore } from '@/stores';
import { useTagStore } from '@/stores';
import { getEnvironmentInfo, isTauriAvailable, testIPCConnection } from '@/utils/tauri-helpers';

export function useLayout() {
  const { t } = useI18n();
  const uiStore = useUIStore();
  const todoStore = useTodoStore();
  const groupStore = useGroupStore();
  const tagStore = useTagStore();

  const environmentError = ref(false);

  async function initLayout() {
    // ==================== 环境检测 ====================
    console.log('[useLayout] ==================== 环境检测 ====================');

    const env = getEnvironmentInfo();
    console.log('[useLayout] 当前环境信息:', env);

    if (!isTauriAvailable()) {
      console.error('[useLayout] ❌ Tauri环境检测失败！');
      console.error('[useLayout] 请确保：');
      console.error('[useLayout]   1. 使用 "cargo tauri dev" 命令启动应用');
      console.error('[useLayout]   2. 在Tauri桌面窗口中测试（不是浏览器）');
      console.error('[useLayout] =================================================');

      environmentError.value = true;

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

      return false;
    }

    console.log('[useLayout] ✅ Tauri环境检测成功');

    // 测试IPC连接
    console.log('[useLayout] 测试IPC连接...');
    const ipcConnected = await testIPCConnection();
    if (!ipcConnected) {
      console.error('[useLayout] ❌ IPC连接测试失败');
    } else {
      console.log('[useLayout] ✅ IPC连接正常');
    }
    console.log('[useLayout] =================================================');
    // =================================================

    return true;
  }

  async function loadInitialData() {
    try {
      await todoStore.fetchTodos();
      await groupStore.fetchGroups();
      await tagStore.fetchTags();
    } catch (error) {
      console.error('[useLayout] Failed to load initial data:', error);
    }
  }

  return {
    environmentError,
    initLayout,
    loadInitialData,
  };
}
