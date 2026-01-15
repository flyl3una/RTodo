<template>
  <div class="settings-view">
    <h1 class="page-title">{{ t('settings.title') }}</h1>

    <div class="settings-content">
      <!-- Appearance -->
      <section class="settings-section">
        <h2 class="section-title">{{ t('settings.appearance') }}</h2>
        <div class="setting-item">
          <span class="setting-label">{{ t('settings.theme') }}</span>
          <el-radio-group v-model="currentTheme" @change="handleThemeChange">
            <el-radio-button value="light">{{ t('settings.themeLight') }}</el-radio-button>
            <el-radio-button value="dark">{{ t('settings.themeDark') }}</el-radio-button>
            <el-radio-button value="auto">{{ t('settings.themeAuto') }}</el-radio-button>
          </el-radio-group>
        </div>
        <div class="setting-item">
          <span class="setting-label">{{ t('settings.language') }}</span>
          <el-select v-model="currentLanguage" @change="handleLanguageChange" style="width: 200px">
            <el-option :label="t('languages.zh-CN')" value="zh-CN" />
            <el-option :label="t('languages.zh-TW')" value="zh-TW" />
            <el-option :label="t('languages.en-US')" value="en-US" />
            <el-option :label="t('languages.ja-JP')" value="ja-JP" />
          </el-select>
        </div>
        <div class="setting-item">
          <span class="setting-label">{{ t('settings.themeColor') }}</span>
          <el-color-picker
            v-model="currentThemeColor"
            @change="handleThemeColorChange"
            show-alpha
          />
        </div>
        <div class="setting-item">
          <span class="setting-label">{{ t('settings.developerMode') }}</span>
          <el-switch v-model="currentDeveloperMode" @change="handleDeveloperModeChange" />
        </div>
        <div class="setting-item">
          <span class="setting-label">{{ t('settings.densityMode') }}</span>
          <el-radio-group v-model="currentDensityMode" @change="handleDensityModeChange">
            <el-radio-button value="comfortable">{{ t('settings.densityComfortable') }}</el-radio-button>
            <el-radio-button value="compact">{{ t('settings.densityCompact') }}</el-radio-button>
          </el-radio-group>
        </div>
      </section>

      <!-- Shortcuts -->
      <section class="settings-section">
        <h2 class="section-title">{{ t('settings.shortcuts') }}</h2>
        <div class="setting-item">
          <span class="setting-label">{{ t('settings.globalShortcut') }}</span>
          <el-input
            v-model="currentGlobalShortcut"
            :placeholder="t('settings.shortcutPlaceholder')"
            style="width: 200px"
            @blur="handleGlobalShortcutChange"
          />
        </div>
        <div class="shortcut-hint">
          <p>{{ t('settings.shortcutHint') }}</p>
          <ul>
            <li><code>CmdOrCtrl</code> - {{ t('settings.shortcutCmdOrCtrl') }}</li>
            <li><code>Shift</code> - {{ t('settings.shortcutShift') }}</li>
            <li><code>Alt</code> - {{ t('settings.shortcutAlt') }}</li>
            <li><code>A</code> - {{ t('settings.shortcutKey') }}</li>
          </ul>
          <p>{{ t('settings.shortcutExample') }}<code>CmdOrCtrl+Shift+T</code>、<code>CmdOrCtrl+Alt+Space</code></p>
        </div>
      </section>

      <!-- Data Management -->
      <section class="settings-section">
        <h2 class="section-title">{{ t('settings.dataManagement') }}</h2>
        <div class="setting-item">
          <span class="setting-label">{{ t('settings.exportData') }}</span>
          <el-button @click="handleExport" :loading="exportLoading">
            {{ t('settings.exportAllData') }}
          </el-button>
        </div>
        <div class="setting-item">
          <span class="setting-label">{{ t('settings.importData') }}</span>
          <el-button @click="handleImport">{{ t('settings.importData') }}</el-button>
        </div>
        <div class="setting-item danger">
          <span class="setting-label">{{ t('settings.clearData') }}</span>
          <el-button type="danger" @click="handleClear">
            {{ t('settings.clearAllData') }}
          </el-button>
        </div>
      </section>

      <!-- About -->
      <section class="settings-section">
        <h2 class="section-title">{{ t('settings.about') }}</h2>
        <div class="about-content">
          <div class="app-info">
            <div class="app-logo">
              <Logo />
            </div>
            <div class="app-details">
              <h3>{{ t('settings.appName') }}</h3>
              <p class="app-version">{{ t('settings.appVersion') }}</p>
            </div>
          </div>
          <p class="app-description">
            {{ t('settings.appDescription') }}
          </p>
          <div class="tech-stack">
            <span>{{ t('techStack.tauri') }}</span>
            <span>{{ t('techStack.vue') }}</span>
            <span>{{ t('techStack.elementPlus') }}</span>
            <span>{{ t('techStack.sqlite') }}</span>
          </div>
        </div>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { ElMessage, ElMessageBox } from 'element-plus';
import { useUIStore } from '@/stores';
import * as api from '@/api/tauri';
import Logo from '@/components/icon/logo.vue';
import { save, open } from '@tauri-apps/plugin-dialog';

const { t } = useI18n();

const uiStore = useUIStore();

const currentTheme = ref(uiStore.theme);
const currentLanguage = ref(uiStore.language);
const currentThemeColor = ref(uiStore.themeColor);
const currentDeveloperMode = ref(uiStore.developerMode);
const currentDensityMode = ref(uiStore.densityMode);
const currentGlobalShortcut = ref(uiStore.globalShortcut);
const exportLoading = ref(false);

function handleThemeChange(value: 'light' | 'dark' | 'auto') {
  uiStore.setTheme(value);
  ElMessage.success(t('messages.themeChanged'));
}

function handleLanguageChange(value: 'zh-CN' | 'zh-TW' | 'en-US' | 'ja-JP') {
  uiStore.setLanguage(value);
  ElMessage.success(t('messages.languageChanged'));
}

function handleThemeColorChange(color: string) {
  uiStore.setThemeColor(color);
  ElMessage.success(t('messages.themeColorChanged'));
}

function handleDeveloperModeChange(enabled: boolean) {
  console.log('[Settings] handleDeveloperModeChange called with:', enabled);
  console.log('[Settings] uiStore.developerMode before:', uiStore.developerMode);
  uiStore.setDeveloperMode(enabled);
  console.log('[Settings] uiStore.developerMode after:', uiStore.developerMode);
  ElMessage.success(enabled ? t('messages.developerModeEnabled') : t('messages.developerModeDisabled'));
}

function handleDensityModeChange(mode: 'comfortable' | 'compact') {
  uiStore.setDensityMode(mode);
  ElMessage.success(mode === 'compact' ? t('messages.densityCompact') : t('messages.densityComfortable'));
}

async function handleGlobalShortcutChange() {
  try {
    await uiStore.setGlobalShortcut(currentGlobalShortcut.value);
    ElMessage.success(t('messages.globalShortcutSet'));
  } catch (error) {
    console.error('Failed to set global shortcut:', error);
    ElMessage.error(t('messages.globalShortcutFailed'));
    // 恢复原值
    currentGlobalShortcut.value = uiStore.globalShortcut;
  }
}

async function handleExport() {
  try {
    // 先打开文件保存对话框获取用户选择的路径
    const filePath = await save({
      defaultPath: `rtodo-backup-${new Date().toISOString().split('T')[0]}.zip`,
      filters: [{
        name: t('fileTypes.zipArchive'),
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
    ElMessage.success(t('messages.dataExported'));
  } catch (error) {
    console.error('Export error:', error);
    ElMessage.error(t('messages.exportFailed'));
  } finally {
    exportLoading.value = false;
  }
}

async function handleImport() {
  try {
    await ElMessageBox.confirm(
      t('messages.importConfirm'),
      t('messages.importData'),
      {
        type: 'warning',
        confirmButtonText: t('messages.importSelectFile'),
        cancelButtonText: t('common.cancel'),
      }
    );

    // 使用 Tauri 的 open API 选择文件
    const selectedPath = await open({
      multiple: false,
      filters: [{
        name: t('fileTypes.zipArchive'),
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
    ElMessage.success(t('messages.importSuccess'));
    setTimeout(() => window.location.reload(), 1500);
  } catch (error) {
    console.error('Import error:', error);
    ElMessage.error(t('messages.importFailed'));
  } finally {
    exportLoading.value = false;
  }
}

async function handleClear() {
  try {
    await ElMessageBox.confirm(
      t('messages.clearConfirm'),
      t('messages.clearAllData'),
      {
        type: 'error',
        confirmButtonText: t('messages.clearConfirmButton'),
        cancelButtonText: t('common.cancel'),
        distinguishCancelAndClose: true,
      }
    );

    await api.clearAllData();
    ElMessage.success(t('messages.dataCleared'));
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
  currentGlobalShortcut.value = uiStore.globalShortcut;
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
  color: var(--el-text-color-primary);
}

.settings-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.settings-section {
  background: var(--el-bg-color);
  border-radius: 8px;
  padding: 20px;
  border: 1px solid var(--el-border-color-light);
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  margin: 0 0 16px;
  color: var(--el-text-color-primary);
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 0;
}

.setting-item:not(:last-child) {
  border-bottom: 1px solid var(--el-border-color-lighter);
}

.setting-item.danger {
  color: var(--el-color-danger);
}

.setting-label {
  font-size: 14px;
  color: var(--el-text-color-regular);
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
  color: var(--el-text-color-primary);
}

.app-version {
  margin: 0;
  font-size: 14px;
  color: var(--el-text-color-secondary);
}

.app-description {
  margin: 0;
  font-size: 14px;
  color: var(--el-text-color-regular);
}

.tech-stack {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.tech-stack span {
  padding: 4px 12px;
  background: var(--el-fill-color-light);
  border-radius: 12px;
  font-size: 12px;
  color: var(--el-text-color-regular);
}

.shortcut-hint {
  margin-top: 12px;
  padding: 12px;
  background: var(--el-fill-color-light);
  border-radius: 6px;
  font-size: 13px;
}

.shortcut-hint p {
  margin: 0 0 8px;
  color: var(--el-text-color-regular);
}

.shortcut-hint ul {
  margin: 8px 0;
  padding-left: 20px;
}

.shortcut-hint li {
  margin: 4px 0;
  color: var(--el-text-color-regular);
}

.shortcut-hint code {
  padding: 2px 6px;
  background: var(--el-bg-color);
  border-radius: 4px;
  font-family: 'Courier New', monospace;
  font-size: 12px;
  color: var(--el-color-primary);
}

/* Dark theme */
:global(html.dark) .page-title {
  color: var(--el-text-color-primary);
}

:global(html.dark) .settings-section {
  background: var(--el-fill-color-light);
  border-color: var(--el-border-color);
}

:global(html.dark) .section-title {
  color: var(--el-text-color-primary);
}

:global(html.dark) .setting-item:not(:last-child) {
  border-bottom-color: var(--el-border-color);
}

:global(html.dark) .setting-label {
  color: var(--el-text-color-regular);
}

:global(html.dark) .app-details h3 {
  color: var(--el-text-color-primary);
}

:global(html.dark) .app-description {
  color: var(--el-text-color-regular);
}

:global(html.dark) .tech-stack span {
  background: var(--el-fill-color);
  color: var(--el-text-color-regular);
}

:global(html.dark) .shortcut-hint {
  background: var(--el-fill-color);
}

:global(html.dark) .shortcut-hint p,
:global(html.dark) .shortcut-hint li {
  color: var(--el-text-color-regular);
}

:global(html.dark) .shortcut-hint code {
  background: var(--el-fill-color-light);
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
