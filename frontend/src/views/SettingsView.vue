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
        <div class="setting-item">
          <span class="setting-label">语言</span>
          <el-select v-model="currentLanguage" @change="handleLanguageChange" style="width: 200px">
            <el-option label="简体中文" value="zh-CN" />
            <el-option label="繁體中文" value="zh-TW" />
            <el-option label="English" value="en-US" />
            <el-option label="日本語" value="ja-JP" />
          </el-select>
        </div>
        <div class="setting-item">
          <span class="setting-label">主题色</span>
          <el-color-picker
            v-model="currentThemeColor"
            @change="handleThemeColorChange"
            show-alpha
          />
        </div>
        <div class="setting-item">
          <span class="setting-label">开发模式</span>
          <el-switch v-model="currentDeveloperMode" @change="handleDeveloperModeChange" />
        </div>
        <div class="setting-item">
          <span class="setting-label">展示模式</span>
          <el-radio-group v-model="currentDensityMode" @change="handleDensityModeChange">
            <el-radio-button value="comfortable">宽松</el-radio-button>
            <el-radio-button value="compact">紧凑</el-radio-button>
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

      <!-- About -->
      <section class="settings-section">
        <h2 class="section-title">关于</h2>
        <div class="about-content">
          <div class="app-info">
            <div class="app-logo">
              <Logo />
            </div>
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
import Logo from '@/components/icon/logo.vue';
import { save, open } from '@tauri-apps/plugin-dialog';

const uiStore = useUIStore();

const currentTheme = ref(uiStore.theme);
const currentLanguage = ref(uiStore.language);
const currentThemeColor = ref(uiStore.themeColor);
const currentDeveloperMode = ref(uiStore.developerMode);
const currentDensityMode = ref(uiStore.densityMode);
const exportLoading = ref(false);

function handleThemeChange(value: 'light' | 'dark' | 'auto') {
  uiStore.setTheme(value);
  ElMessage.success('主题已切换');
}

function handleLanguageChange(value: 'zh-CN' | 'zh-TW' | 'en-US' | 'ja-JP') {
  uiStore.setLanguage(value);
  ElMessage.success('语言已切换');
}

function handleThemeColorChange(color: string) {
  uiStore.setThemeColor(color);
  ElMessage.success('主题色已切换');
}

function handleDeveloperModeChange(enabled: boolean) {
  console.log('[Settings] handleDeveloperModeChange called with:', enabled);
  console.log('[Settings] uiStore.developerMode before:', uiStore.developerMode);
  uiStore.setDeveloperMode(enabled);
  console.log('[Settings] uiStore.developerMode after:', uiStore.developerMode);
  ElMessage.success(enabled ? '开发模式已开启' : '开发模式已关闭');
}

function handleDensityModeChange(mode: 'comfortable' | 'compact') {
  uiStore.setDensityMode(mode);
  ElMessage.success(mode === 'compact' ? '已切换到紧凑模式' : '已切换到宽松模式');
}

async function handleExport() {
  try {
    // 先打开文件保存对话框获取用户选择的路径
    const filePath = await save({
      defaultPath: `rtodo-backup-${new Date().toISOString().split('T')[0]}.zip`,
      filters: [{
        name: 'ZIP Archive',
        extensions: ['zip']
      }]
    });

    if (!filePath) {
      // 用户取消了文件选择
      return;
    }

    exportLoading.value = true;

    // 将文件路径传递给后端，后端直接写入文件
    await api.exportDataAsCsv(filePath);
    ElMessage.success('数据导出成功');
  } catch (error) {
    console.error('Export error:', error);
    ElMessage.error('导出失败');
  } finally {
    exportLoading.value = false;
  }
}

async function handleImport() {
  try {
    await ElMessageBox.confirm(
      '导入将覆盖现有数据，确定要继续吗？',
      '导入数据',
      {
        type: 'warning',
        confirmButtonText: '选择文件',
        cancelButtonText: '取消',
      }
    );

    // 使用 Tauri 的 open API 选择文件
    const selectedPath = await open({
      multiple: false,
      filters: [{
        name: 'ZIP Archive',
        extensions: ['zip']
      }]
    });

    if (!selectedPath) {
      // 用户取消了文件选择
      return;
    }

    exportLoading.value = true;

    // 将文件路径传给后端，后端负责读取和解析
    await api.importDataFromCsv(selectedPath);
    ElMessage.success('数据导入成功，页面将重新加载');
    setTimeout(() => window.location.reload(), 1500);
  } catch (error) {
    console.error('Import error:', error);
    ElMessage.error('导入失败：文件格式错误');
  } finally {
    exportLoading.value = false;
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

onMounted(() => {
  currentTheme.value = uiStore.theme;
  currentLanguage.value = uiStore.language;
  currentThemeColor.value = uiStore.themeColor;
  currentDeveloperMode.value = uiStore.developerMode;
  currentDensityMode.value = uiStore.densityMode;
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
  width: 64px;
  height: 64px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.app-logo :deep(svg) {
  width: 100%;
  height: 100%;
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

/* 紧凑模式 */
[data-density='compact'] .settings-view {
  padding: 14px;
}

[data-density='compact'] .page-title {
  margin-bottom: 16px;
}

[data-density='compact'] .settings-content {
  gap: 16px;
}

[data-density='compact'] .settings-section {
  padding: 14px;
}

[data-density='compact'] .section-title {
  margin-bottom: 10px;
}

[data-density='compact'] .setting-item {
  padding: 8px 0;
}

[data-density='compact'] .about-content {
  gap: 10px;
}

[data-density='compact'] .app-info {
  gap: 12px;
}
</style>
