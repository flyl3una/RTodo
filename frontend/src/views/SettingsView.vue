<template>
  <div class="settings-view">
    <h1 class="page-title">设置</h1>

    <div class="settings-content">
      <!-- Appearance -->
      <section class="settings-section">
        <h2 class="section-title">外观</h2>
        <div class="setting-item">
          <span class="setting-label">主题</span>
          <el-radio-group v-model="currentTheme" @change="handleThemeChange">
            <el-radio-button value="light">浅色</el-radio-button>
            <el-radio-button value="dark">深色</el-radio-button>
            <el-radio-button value="auto">跟随系统</el-radio-button>
          </el-radio-group>
        </div>
      </section>

      <!-- Data Management -->
      <section class="settings-section">
        <h2 class="section-title">数据管理</h2>
        <div class="setting-item">
          <span class="setting-label">导出数据</span>
          <el-button @click="handleExport" :loading="exportLoading">
            导出所有数据
          </el-button>
        </div>
        <div class="setting-item">
          <span class="setting-label">导入数据</span>
          <el-button @click="handleImport">导入数据</el-button>
        </div>
        <div class="setting-item danger">
          <span class="setting-label">清空数据</span>
          <el-button type="danger" @click="handleClear">
            清空所有数据
          </el-button>
        </div>
      </section>

      <!-- Tag Management -->
      <section class="settings-section">
        <h2 class="section-title">标签管理</h2>
        <TagManageDialog
          :model-value="true"
          :group="undefined"
          @updated="handleTagsUpdated"
          @update:model-value="() => {}"
        />
      </section>

      <!-- About -->
      <section class="settings-section">
        <h2 class="section-title">关于</h2>
        <div class="about-content">
          <div class="app-info">
            <div class="app-logo">📝</div>
            <div class="app-details">
              <h3>RTodo</h3>
              <p class="app-version">版本 0.1.0</p>
            </div>
          </div>
          <p class="app-description">
            基于 Tauri + Vue 3 的跨平台待办事项管理应用
          </p>
          <div class="tech-stack">
            <span>Tauri 2.0</span>
            <span>Vue 3</span>
            <span>Element Plus</span>
            <span>SQLite</span>
          </div>
        </div>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import { useUIStore } from '@/stores';
import * as api from '@/api/tauri';
import TagManageDialog from '@/components/tag/TagManageDialog.vue';

const uiStore = useUIStore();

const currentTheme = ref(uiStore.theme);
const exportLoading = ref(false);

function handleThemeChange(value: 'light' | 'dark' | 'auto') {
  uiStore.setTheme(value);
  ElMessage.success('主题已切换');
}

async function handleExport() {
  try {
    exportLoading.value = true;
    const data = await api.exportAllData();

    // Create JSON file and trigger download
    const json = JSON.stringify(data, null, 2);
    const blob = new Blob([json], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `rtodo-backup-${new Date().toISOString().split('T')[0]}.json`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);

    ElMessage.success('数据导出成功');
  } catch (error) {
    ElMessage.error('导出失败');
  } finally {
    exportLoading.value = false;
  }
}

async function handleImport() {
  try {
    const result = await ElMessageBox.prompt(
      '请选择要导入的 JSON 文件',
      '导入数据',
      {
        confirmButtonText: '选择文件',
        cancelButtonText: '取消',
        inputPlaceholder: '导入将覆盖现有数据',
      }
    );

    if (result.action !== 'confirm') return;

    // Create file input
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.json';

    input.onchange = async (e: Event) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (!file) return;

      try {
        const text = await file.text();
        const data = JSON.parse(text);

        await api.importData(data);
        ElMessage.success('数据导入成功，页面将重新加载');
        setTimeout(() => window.location.reload(), 1500);
      } catch (error) {
        ElMessage.error('导入失败：文件格式错误');
      }
    };

    input.click();
  } catch (error) {
    // User cancelled
  }
}

async function handleClear() {
  try {
    await ElMessageBox.confirm(
      '此操作将清空所有数据且无法恢复，确定要继续吗？',
      '清空所有数据',
      {
        type: 'error',
        confirmButtonText: '确定清空',
        cancelButtonText: '取消',
        distinguishCancelAndClose: true,
      }
    );

    await api.clearAllData();
    ElMessage.success('数据已清空');
    setTimeout(() => window.location.reload(), 1000);
  } catch (error) {
    // User cancelled
  }
}

function handleTagsUpdated() {
  ElMessage.success('标签已更新');
}

onMounted(() => {
  currentTheme.value = uiStore.theme;
});
</script>

<style scoped>
.settings-view {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}

.page-title {
  font-size: 28px;
  font-weight: 600;
  margin-bottom: 24px;
  color: #303133;
}

.settings-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.settings-section {
  background: white;
  border-radius: 8px;
  padding: 20px;
  border: 1px solid #e4e7ed;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  margin: 0 0 16px;
  color: #303133;
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 0;
}

.setting-item:not(:last-child) {
  border-bottom: 1px solid #f0f0f0;
}

.setting-item.danger {
  color: #f56c6c;
}

.setting-label {
  font-size: 14px;
  color: #606266;
}

.about-content {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.app-info {
  display: flex;
  align-items: center;
  gap: 16px;
}

.app-logo {
  font-size: 48px;
}

.app-details h3 {
  margin: 0 0 4px;
  font-size: 20px;
  color: #303133;
}

.app-version {
  margin: 0;
  font-size: 14px;
  color: #909399;
}

.app-description {
  margin: 0;
  font-size: 14px;
  color: #606266;
}

.tech-stack {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.tech-stack span {
  padding: 4px 12px;
  background: #f5f7fa;
  border-radius: 12px;
  font-size: 12px;
  color: #606266;
}

/* Dark theme */
[data-theme='dark'] .page-title {
  color: #e0e0e0;
}

[data-theme='dark'] .settings-section {
  background: #2a2a2a;
  border-color: #3a3a3a;
}

[data-theme='dark'] .section-title {
  color: #e0e0e0;
}

[data-theme='dark'] .setting-item:not(:last-child) {
  border-bottom-color: #3a3a3a;
}

[data-theme='dark'] .setting-label {
  color: #b0b0b0;
}

[data-theme='dark'] .app-details h3 {
  color: #e0e0e0;
}

[data-theme='dark'] .app-description {
  color: #b0b0b0;
}

[data-theme='dark'] .tech-stack span {
  background: #1a1a1a;
  color: #b0b0b0;
}
</style>
