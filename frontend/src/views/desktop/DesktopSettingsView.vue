<template>
  <div class="settings-view">
    <el-tabs v-model="activeTab" class="settings-tabs">
      <!-- 应用 -->
      <el-tab-pane :label="t('settings.app')" name="appearance">
        <div class="tab-content">
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
            <el-color-picker v-model="currentThemeColor" @change="handleThemeColorChange" />
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
          <div class="setting-item">
            <span class="setting-label">{{ t('settings.closeBehavior') }}</span>
            <el-radio-group v-model="currentCloseBehavior" @change="handleCloseBehaviorChange">
              <el-radio-button value="direct">{{ t('settings.closeDirect') }}</el-radio-button>
              <el-radio-button value="minimize_to_tray">{{ t('settings.closeMinimizeToTray') }}</el-radio-button>
            </el-radio-group>
          </div>
          <div class="setting-item">
            <span class="setting-label">{{ t('settings.autoLaunch') }}</span>
            <el-switch
              v-model="currentAutoLaunch"
              @change="handleAutoLaunchChange"
              :loading="autoLaunchLoading"
            />
          </div>
        </div>
      </el-tab-pane>

      <!-- 快捷键 -->
      <el-tab-pane :label="t('settings.shortcuts')" name="shortcuts">
        <div class="tab-content">
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
            <p>{{ t('settings.shortcutExample') }}<code>CmdOrCtrl+Shift+T</code><code>CmdOrCtrl+Alt+Space</code></p>
          </div>
        </div>
      </el-tab-pane>

      <!-- 数据管理 -->
      <el-tab-pane :label="t('settings.dataManagement')" name="data">
        <div class="tab-content">
          <div class="setting-item">
            <span class="setting-label">{{ t('settings.dataPath') }}</span>
            <div class="data-path-input-group">
              <el-input :value="currentDataPath" readonly style="flex: 1">
                <template #append>
                  <el-button @click="handleChangeDataPath">{{ t('messages.browse') }}</el-button>
                </template>
              </el-input>
              <el-button @click="handleResetDataPath" type="warning" style="margin-left: 8px">
                {{ t('common.reset') }}
              </el-button>
            </div>
          </div>
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
        </div>
      </el-tab-pane>

      <!-- 日志 -->
      <el-tab-pane :label="t('settings.logging')" name="logging">
        <div class="tab-content">
          <!-- 日志级别选择 -->
          <div class="setting-item">
            <span class="setting-label">{{ t('settings.logLevel') }}</span>
            <el-select v-model="logConfig.level" @change="handleLogLevelChange" style="width: 200px">
              <el-option value="trace">Trace</el-option>
              <el-option value="debug">Debug</el-option>
              <el-option value="info">Info</el-option>
              <el-option value="warn">Warn</el-option>
              <el-option value="error">Error</el-option>
            </el-select>
          </div>

          <!-- 日志目录 -->
          <div class="setting-item">
            <span class="setting-label">{{ t('settings.logDirectory') }}</span>
            <div class="data-path-input-group">
              <el-input :value="logConfig.log_dir || 'Loading...'" readonly style="flex: 1">
                <template #append>
                  <el-button @click="handleChangeLogDirectory">{{ t('messages.browse') }}</el-button>
                </template>
              </el-input>
            </div>
          </div>

          <!-- 回滚策略 -->
          <div class="setting-item">
            <span class="setting-label">{{ t('settings.logRolling') }}</span>
            <el-select v-model="logConfig.rolling" @change="handleLogRollingChange" style="width: 200px">
              <el-option value="daily">{{ t('settings.rollingDaily') }}</el-option>
              <el-option value="hourly">{{ t('settings.rollingHourly') }}</el-option>
              <el-option value="minutely">{{ t('settings.rollingMinutely') }}</el-option>
              <el-option value="never">{{ t('settings.rollingNever') }}</el-option>
            </el-select>
          </div>

          <!-- 自动压缩开关 -->
          <div class="setting-item">
            <span class="setting-label">{{ t('settings.logCompress') }}</span>
            <el-switch v-model="logConfig.compress" @change="handleLogCompressChange" />
          </div>

          <!-- 保留天数 -->
          <div class="setting-item">
            <span class="setting-label">{{ t('settings.logRetention') }}</span>
            <el-input-number v-model="logConfig.retention_days" :min="1" :max="365" @change="handleLogRetentionDaysChange" style="width: 150px" />
            <span style="margin-left: 8px; color: var(--el-text-color-secondary);">{{ t('settings.days') }}</span>
          </div>

          <!-- 日志文件列表 -->
          <div class="setting-item">
            <span class="setting-label">{{ t('settings.logFiles') }}</span>
            <el-button @click="handleRefreshLogFiles" :loading="logFilesLoading">
              {{ t('settings.refreshLogFiles') }}
            </el-button>
          </div>

          <!-- 显示日志文件列表 -->
          <div v-if="logFiles.length > 0" class="log-files-list">
            <div v-for="file in logFiles" :key="file" class="log-file-item">
              <span>{{ file }}</span>
            </div>
          </div>

          <!-- 手动压缩 -->
          <div class="setting-item">
            <span class="setting-label">{{ t('settings.manualCompress') }}</span>
            <el-button @click="handleManualCompress" :loading="compressLoading" type="primary">
              {{ t('settings.compressNow') }}
            </el-button>
          </div>
        </div>
      </el-tab-pane>

      <!-- 关于 -->
      <el-tab-pane :label="t('settings.about')" name="about">
        <div class="tab-content about-content">
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
      </el-tab-pane>
    </el-tabs>

    <!-- Data Path Change Dialog -->
    <el-dialog v-model="showDataPathDialog" :title="t('settings.changeDataPath')" width="500px">
      <el-form :model="dataPathForm" label-width="100px">
        <el-form-item :label="t('settings.newPath')">
          <el-input v-model="dataPathForm.newPath" readonly>
            <template #append>
              <el-button @click="selectDataPathFolder">{{ t('messages.browse') }}</el-button>
            </template>
          </el-input>
        </el-form-item>
        <el-form-item>
          <el-checkbox v-model="dataPathForm.migrateData">
            {{ t('settings.migrateDataConfirm') }}
          </el-checkbox>
        </el-form-item>
        <el-form-item v-if="dataPathForm.migrateData">
          <el-radio-group v-model="dataPathForm.keepOriginal">
            <el-radio :label="false">{{ t('settings.deleteOriginalData') }}</el-radio>
            <el-radio :label="true">{{ t('settings.keepOriginalData') }}</el-radio>
          </el-radio-group>
        </el-form-item>
      </el-form>

      <!-- Migration progress -->
      <div v-if="migrating" class="migrate-progress">
        <el-progress :percentage="migrateProgress" :status="migrateStatus" />
        <p class="progress-text">{{ migrateMessage }}</p>
      </div>

      <template #footer>
        <el-button @click="showDataPathDialog = false">{{ t('common.cancel') }}</el-button>
        <el-button type="primary" @click="confirmDataPathChange" :loading="migrating" :disabled="!dataPathForm.newPath">
          {{ t('common.confirm') }}
        </el-button>
      </template>
    </el-dialog>

    <!-- Migration Progress Dialog (Full Screen Modal) -->
    <el-dialog
      v-model="showMigrateDialog"
      :title="t('settings.migratingData')"
      :close-on-click-modal="false"
      :close-on-press-escape="false"
      :show-close="false"
      width="500px"
      center
    >
      <div class="migrate-dialog-content">
        <div class="migrate-icon">
          <el-icon :size="60" :class="{ 'is-rotating': migrating }">
            <Refresh />
          </el-icon>
        </div>
        <div class="migrate-info">
          <h3 class="migrate-title">{{ migrateMessage }}</h3>
          <el-progress
            :percentage="migrateProgress"
            :status="migrateStatus"
            :stroke-width="12"
            :duration="{ duration: 0.3, easing: 'linear' }"
          />
        </div>
      </div>

      <template #footer v-if="migrateStatus === 'success' || migrateStatus === 'exception'">
        <el-button type="primary" @click="closeMigrateDialog">
          {{ migrateStatus === 'success' ? t('common.confirm') : t('common.close') }}
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { ElMessage, ElMessageBox } from 'element-plus';
import { useUIStore } from '@/stores';
import type { CloseBehavior } from '@/stores';
import * as api from '@/api/tauri';
import Logo from '@/components/icon/logo.vue';
import { save, open } from '@tauri-apps/plugin-dialog';
import { listen } from '@tauri-apps/api/event';
import { Refresh } from '@element-plus/icons-vue';
import { delays } from '@/constants/delays';

const { t } = useI18n();

const uiStore = useUIStore();

const activeTab = ref('appearance');
const currentTheme = ref(uiStore.theme);
const currentLanguage = ref(uiStore.language);
const currentThemeColor = ref(uiStore.themeColor);
const currentDeveloperMode = ref(uiStore.developerMode);
const currentDensityMode = ref(uiStore.densityMode);
const currentGlobalShortcut = ref(uiStore.globalShortcut);
const currentCloseBehavior = ref<CloseBehavior>(uiStore.closeBehavior);
const exportLoading = ref(false);
const currentAutoLaunch = ref(false);
const autoLaunchLoading = ref(false);

// Data path management
const currentDataPath = ref('');
const showDataPathDialog = ref(false);
const showMigrateDialog = ref(false);
const dataPathForm = ref({
  newPath: '',
  migrateData: true,
  keepOriginal: false,
});
const migrating = ref(false);
const migrateProgress = ref(0);
const migrateStatus = ref<'success' | 'exception' | ''>('');
const migrateMessage = ref('');
let unlistenMigrate: (() => void) | null = null;

// 日志配置状态
const logConfig = ref<api.LogConfig>({
  level: 'info',
  console: false,
  file: true,
  log_dir: null,
  format: 'full',
  rolling: 'daily',
  max_file_size_mb: 100,
  compress: true,
  retention_days: 30,
});
const logFiles = ref<string[]>([]);
const logFilesLoading = ref(false);
const compressLoading = ref(false);

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

function handleCloseBehaviorChange(behavior: CloseBehavior) {
  uiStore.setCloseBehavior(behavior);
  ElMessage.success(behavior === 'minimize_to_tray' ? t('messages.closeBehaviorMinimizeToTray') : t('messages.closeBehaviorDirect'));
}

async function handleAutoLaunchChange(enabled: boolean) {
  try {
    autoLaunchLoading.value = true;
    await api.setAutoLaunch(enabled);
    ElMessage.success(enabled ? t('messages.autoLaunchEnabled') : t('messages.autoLaunchDisabled'));
  } catch (error) {
    console.error('Failed to set auto launch:', error);
    ElMessage.error(t('messages.autoLaunchFailed'));
    currentAutoLaunch.value = !enabled;
  } finally {
    autoLaunchLoading.value = false;
  }
}

async function handleGlobalShortcutChange() {
  try {
    await uiStore.setGlobalShortcut(currentGlobalShortcut.value);
    // Reset to original value on failure
  } catch (error) {
    console.error('Failed to set global shortcut:', error);
    ElMessage.error(t('messages.globalShortcutFailed'));
    // Restore original value if setting failed
    currentGlobalShortcut.value = uiStore.globalShortcut;
  }
}

async function handleExport() {
  try {
    // Use Tauri save API to let user choose the save path
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

    // Send the file path to backend for CSV export
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

    // Use Tauri open API to select file
    const selectedPath = await open({
      multiple: false,
      title: t('messages.importSelectFile'),
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

    // After sending file path to backend, parse and load data
    await api.importDataFromCsv(selectedPath);
    ElMessage.success(t('messages.importSuccess'));
    await delays.reloadAfterImport();
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
    await delays.reloadAfterMessage();
  } catch (error) {
    // User cancelled
  }
}

// Data path management functions
async function handleChangeDataPath() {
  dataPathForm.value.newPath = '';
  dataPathForm.value.migrateData = true;
  dataPathForm.value.keepOriginal = false;
  showDataPathDialog.value = true;
}

async function selectDataPathFolder() {
  try {
    const selectedPath = await open({
      multiple: false,
      directory: true,
      title: t('messages.browse'),
    });

    if (selectedPath && typeof selectedPath === 'string') {
      dataPathForm.value.newPath = selectedPath;
    }
  } catch (error) {
    console.error('Failed to select folder:', error);
    ElMessage.error(t('messages.folderSelectFailed'));
  }
}

async function confirmDataPathChange() {
  if (!dataPathForm.value.newPath) return;

  console.log('[SettingsView] confirmDataPathChange called, newPath:', dataPathForm.value.newPath);
  console.log('[SettingsView] migrateData:', dataPathForm.value.migrateData, 'keepOriginal:', dataPathForm.value.keepOriginal);

  try {
    // Warn about migration if selected
    if (dataPathForm.value.migrateData) {
      console.log('[SettingsView] Showing migrateDataWarning dialog');
      await ElMessageBox.confirm(
        t('messages.migrateDataWarning'),
        t('settings.changeDataPath'),
        {
          type: 'warning',
          confirmButtonText: t('common.confirm'),
          cancelButtonText: t('common.cancel'),
        }
      );
      console.log('[SettingsView] User confirmed migrateDataWarning');
    }

    // Check if target directory is empty
    console.log('[SettingsView] Calling checkDirectoryEmpty for path:', dataPathForm.value.newPath);
    const isTargetEmpty = await api.checkDirectoryEmpty(dataPathForm.value.newPath);
    console.log('[SettingsView] checkDirectoryEmpty returned:', isTargetEmpty, 'type:', typeof isTargetEmpty);

    // If target directory is not empty, ask for confirmation
    if (!isTargetEmpty) {
      console.log('[SettingsView] Directory is NOT empty, showing confirmation dialog');
      await ElMessageBox.confirm(
        t('messages.targetDirectoryNotEmptyWarning'),
        t('settings.changeDataPath'),
        {
          type: 'warning',
          confirmButtonText: t('common.confirm'),
          cancelButtonText: t('common.cancel'),
        }
      );
      console.log('[SettingsView] User confirmed to continue with non-empty directory');
    } else {
      console.log('[SettingsView] Directory is empty, proceeding without confirmation');
    }

    migrating.value = true;
    migrateProgress.value = 0;
    migrateStatus.value = '';
    migrateMessage.value = t('messages.migrationStarting');

    // Close data path dialog and show migrate dialog
    showDataPathDialog.value = false;
    showMigrateDialog.value = true;

    if (dataPathForm.value.migrateData) {
      // Perform migration
      await api.migrateData(dataPathForm.value.newPath, dataPathForm.value.keepOriginal);

      // 迁移完成后立即刷新路径显示
      console.log('[SettingsView] Migration completed, reloading path');
      await loadDataPath();
    } else {
      // Just set the new path
      await api.setDataPath(dataPathForm.value.newPath);
      // For simple path change without migration, complete immediately
      migrateProgress.value = 100;
      migrateStatus.value = 'success';
      migrateMessage.value = t('messages.dataPathChanged');

      // 立即刷新路径显示
      await loadDataPath();
    }

    // Ask about restart (only show if migration is complete)
    if (migrateStatus.value === 'success') {
      try {
        await ElMessageBox.confirm(
          t('messages.restartRequired'),
          t('settings.changeDataPath'),
          {
            type: 'info',
            confirmButtonText: t('messages.restartNow'),
            cancelButtonText: t('messages.restartLater'),
          }
        );
        // Restart the app
        window.location.reload();
      } catch {
        // User chose to restart later
      }
    }

    showDataPathDialog.value = false;
    await loadDataPath();
  } catch (error) {
    console.error('[SettingsView] Failed to change data path:', error);
    console.error('[SettingsView] Error type:', typeof error);
    console.error('[SettingsView] Error message:', String(error));
    ElMessage.error(`${t('messages.operationFailed')}: ${error}`);
    migrateStatus.value = 'exception';
    migrateMessage.value = t('messages.migrationFailed');
    showMigrateDialog.value = true; // Keep dialog open to show error
  } finally {
    migrating.value = false;
  }
}

async function handleResetDataPath() {
  try {
    await ElMessageBox.confirm(
      t('messages.resetDataPathWarning'),
      t('settings.resetDataPath'),
      {
        type: 'warning',
        confirmButtonText: t('common.confirm'),
        cancelButtonText: t('common.cancel'),
      }
    );

    await api.resetDataPath();

    // Ask about restart
    try {
      await ElMessageBox.confirm(
        t('messages.restartRequired'),
        t('settings.resetDataPath'),
        {
          type: 'info',
          confirmButtonText: t('messages.restartNow'),
          cancelButtonText: t('messages.restartLater'),
        }
      );
      window.location.reload();
    } catch {
      // User chose to restart later
    }

    ElMessage.success(t('messages.dataPathReset'));
    await loadDataPath();
  } catch (error) {
    if (error !== 'cancel') {
      console.error('Failed to reset data path:', error);
      ElMessage.error(`${t('messages.operationFailed')}: ${error}`);
    }
  }
}

async function loadDataPath() {
  try {
    currentDataPath.value = await api.getDataPath();
  } catch (error) {
    console.error('Failed to load data path:', error);
  }
}

function closeMigrateDialog() {
  showMigrateDialog.value = false;
  migrating.value = false;
  migrateProgress.value = 0;
  migrateStatus.value = '';
  migrateMessage.value = '';
}

// 日志配置相关函数
async function loadLogConfig() {
  try {
    logConfig.value = await api.getLogConfig();
  } catch (error) {
    console.error('Failed to load log config:', error);
  }
}

async function handleLogLevelChange(level: string) {
  try {
    await api.setLogLevel(level);
    ElMessage.success(t('messages.logLevelChanged') + ' ' + t('messages.logLevelRestartRequired'));
  } catch (error) {
    console.error('Failed to set log level:', error);
    ElMessage.error(t('messages.logLevelChangeFailed'));
    await loadLogConfig();
  }
}

async function handleChangeLogDirectory() {
  try {
    const selectedPath = await open({
      multiple: false,
      directory: true,
      title: t('settings.selectLogDirectory'),
    });

    if (selectedPath && typeof selectedPath === 'string') {
      await ElMessageBox.confirm(
        t('messages.logDirectoryChangeWarning'),
        t('settings.changeLogDirectory'),
        { type: 'warning', confirmButtonText: t('common.confirm'), cancelButtonText: t('common.cancel') }
      );

      await api.setLogDirectory(selectedPath);
      logConfig.value.log_dir = selectedPath;
      ElMessage.success(t('messages.logDirectoryChanged') + ' ' + t('messages.restartRequired'));
    }
  } catch (error) {
    if (error !== 'cancel') {
      console.error('Failed to change log directory:', error);
      ElMessage.error(t('messages.operationFailed'));
    }
  }
}

async function handleLogRollingChange(rolling: string) {
  try {
    await api.setLogRolling(rolling);
    ElMessage.success(t('messages.logRollingChanged') + ' ' + t('messages.restartRequired'));
  } catch (error) {
    console.error('Failed to set log rolling:', error);
    ElMessage.error(t('messages.operationFailed'));
    await loadLogConfig();
  }
}

async function handleLogCompressChange(compress: boolean) {
  try {
    await api.setLogCompress(compress);
    ElMessage.success((compress ? t('messages.logCompressEnabled') : t('messages.logCompressDisabled')) + ' ' + t('messages.restartRequired'));
  } catch (error) {
    console.error('Failed to set log compress:', error);
    ElMessage.error(t('messages.operationFailed'));
    await loadLogConfig();
  }
}

async function handleLogRetentionDaysChange(days: number) {
  try {
    await api.setLogRetentionDays(days);
    ElMessage.success(t('messages.logRetentionDaysChanged') + ' ' + t('messages.restartRequired'));
  } catch (error) {
    console.error('Failed to set log retention days:', error);
    ElMessage.error(t('messages.operationFailed'));
    await loadLogConfig();
  }
}

async function handleRefreshLogFiles() {
  try {
    logFilesLoading.value = true;
    logFiles.value = await api.getLogFiles();
  } catch (error) {
    console.error('Failed to get log files:', error);
    ElMessage.error(t('messages.operationFailed'));
  } finally {
    logFilesLoading.value = false;
  }
}

async function handleManualCompress() {
  try {
    compressLoading.value = true;
    await api.compressLogs();
    ElMessage.success(t('messages.logCompressed'));
    await handleRefreshLogFiles();
  } catch (error) {
    console.error('Failed to compress logs:', error);
    ElMessage.error(t('messages.logCompressFailed'));
  } finally {
    compressLoading.value = false;
  }
}

onMounted(async () => {
  currentTheme.value = uiStore.theme;
  currentLanguage.value = uiStore.language;
  currentThemeColor.value = uiStore.themeColor;
  currentDeveloperMode.value = uiStore.developerMode;
  currentDensityMode.value = uiStore.densityMode;
  currentGlobalShortcut.value = uiStore.globalShortcut;
  currentCloseBehavior.value = uiStore.closeBehavior;

  // Load auto-launch status
  try {
    currentAutoLaunch.value = await api.getAutoLaunch();
  } catch (error) {
    console.error('Failed to get auto launch status:', error);
  }

  // Load data path
  await loadDataPath();

  // Load log configuration
  await loadLogConfig();
  await handleRefreshLogFiles();

  // Listen for auto-launch state changes from tray menu
  const unlisten = await listen<boolean>('autolaunch-changed', (event) => {
    currentAutoLaunch.value = event.payload;
  });

  // Listen for migration progress events
  unlistenMigrate = await await listen<{
    status: string;
    message: string;
    progress?: number;
  }>('migrate-progress', (event) => {
    const payload = event.payload;
    migrateMessage.value = payload.message;

    switch (payload.status) {
      case 'started':
        migrateProgress.value = 5;
        break;
      case 'copying_db':
        migrateProgress.value = 30;
        break;
      case 'copying_attachments':
        migrateProgress.value = 60;
        break;
      case 'validating':
        migrateProgress.value = 80;
        break;
      case 'finalizing':
        migrateProgress.value = 95;
        break;
      case 'cleaning':
        migrateProgress.value = 98;
        break;
      case 'completed':
        migrateProgress.value = 100;
        migrateStatus.value = 'success';
        migrating.value = false; // Stop rotation animation
        break;
      case 'error':
        migrateStatus.value = 'exception';
        migrating.value = false; // Stop rotation animation
        break;
    }
  });

  // Cleanup listeners
  onUnmounted(() => {
    unlisten();
    unlistenMigrate?.();
  });
});
</script>

<style scoped>
.settings-view {
  max-width: 900px;
  margin: 0 auto;
  padding: 20px;
}

.page-title {
  font-size: 24px;
  font-weight: 600;
  margin-bottom: 20px;
  text-align: center;
  color: var(--el-text-color-primary);
}

.settings-tabs {
  background: var(--el-bg-color);
  border-radius: 8px;
  padding: 20px;
  border: 1px solid var(--el-border-color-light);
}

.tab-content {
  padding-top: 20px;
  padding-right: 1px;
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 0;
  min-height: 40px;
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
  font-weight: 500;
}

.data-path-input-group {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
  max-width: 550px;
}

.data-path-input-group .el-input {
  flex: 1;
}

.about-content {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 20px 0;
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
  margin-top: 16px;
  padding: 16px;
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
  margin-right: 8px;
}

.shortcut-hint p:last-child code {
  margin-right: 16px;
}

/* Element Plus Tabs 样式覆盖 */
:deep(.el-tabs__header) {
  margin: 0 0 20px 0;
  border-bottom: 2px solid var(--el-border-color-light);
}

:deep(.el-tabs__item) {
  font-size: 15px;
  padding: 0 20px;
  height: 40px;
  line-height: 40px;
  font-weight: 500;
}

:deep(.el-tabs__item.is-active) {
  color: var(--el-color-primary);
  font-weight: 600;
}

:deep(.el-tabs__active-bar) {
  height: 3px;
}

/* Dark theme */
:global(html.dark) .page-title {
  color: var(--el-text-color-primary);
}

:global(html.dark) .settings-tabs {
  background: var(--el-fill-color-light);
  border-color: var(--el-border-color);
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
  font-size: 20px;
}

[data-density='compact'] .settings-tabs {
  padding: 14px;
}

[data-density='compact'] .tab-content {
  padding-top: 14px;
  padding-right: 1px;
}

[data-density='compact'] .setting-item {
  padding: 10px 0;
  min-height: 36px;
}

[data-density='compact'] :deep(.el-tabs__item) {
  padding: 0 16px;
  height: 36px;
  line-height: 36px;
  font-size: 14px;
}

[data-density='compact'] .about-content {
  gap: 10px;
  padding: 14px 0;
}

[data-density='compact'] .app-info {
  gap: 12px;
}

/* Migration progress styles */
.migrate-progress {
  margin-top: 20px;
  padding: 16px;
  background: var(--el-fill-color-light);
  border-radius: 8px;
}

.progress-text {
  margin: 12px 0 0;
  font-size: 14px;
  color: var(--el-text-color-regular);
  text-align: center;
}

/* Migration Dialog Styles */
.migrate-dialog-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 20px;
  gap: 24px;
}

.migrate-icon {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 80px;
  height: 80px;
  border-radius: 50%;
  background: var(--el-fill-color-light);
}

.migrate-icon .el-icon {
  color: var(--el-color-primary);
  transition: transform 0.3s ease;
}

.migrate-icon .is-rotating {
  animation: rotate 2s linear infinite;
}

@keyframes rotate {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.migrate-info {
  width: 100%;
  text-align: center;
}

.migrate-title {
  margin: 0 0 16px;
  font-size: 16px;
  font-weight: 500;
  color: var(--el-text-color-primary);
}

/* Mobile responsive */
@media (max-width: 768px) {
  .migrate-dialog-content {
    padding: 16px;
    gap: 16px;
  }

  .migrate-icon {
    width: 60px;
    height: 60px;
  }

  .migrate-icon .el-icon {
    font-size: 48px;
  }

  .migrate-title {
    font-size: 14px;
  }
}

/* 日志文件列表样式 */
.log-files-list {
  margin: 12px 0;
  padding: 12px;
  background: var(--el-fill-color-light);
  border-radius: 6px;
  max-height: 200px;
  overflow-y: auto;
}

.log-file-item {
  padding: 6px 12px;
  margin: 4px 0;
  background: var(--el-bg-color);
  border-radius: 4px;
  font-size: 13px;
  color: var(--el-text-color-regular);
  word-break: break-all;
}

.log-file-item:hover {
  background: var(--el-fill-color);
}
</style>
