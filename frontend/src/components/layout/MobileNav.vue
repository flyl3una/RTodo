<template>
  <div class="mobile-nav">
    <router-link to="/" class="nav-item" @click="handleNavClick">
      <el-icon><Tickets /></el-icon>
      <span>{{ t('nav.todo') }}</span>
    </router-link>
    <router-link to="/stats" class="nav-item" @click="handleNavClick">
      <el-icon><TrendCharts /></el-icon>
      <span>{{ t('stats.title') }}</span>
    </router-link>
    <router-link to="/settings" class="nav-item" @click="handleNavClick">
      <el-icon><Setting /></el-icon>
      <span>{{ t('settings.title') }}</span>
    </router-link>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Tickets, TrendCharts, Setting } from '@element-plus/icons-vue';

const { t } = useI18n();

function handleNavClick() {
  // 移动端导航点击后可以做一些额外处理
  console.log('[MobileNav] Navigation clicked');
}
</script>

<style scoped>
.mobile-nav {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  height: 60px;
  background: var(--el-bg-color);
  border-top: 1px solid var(--el-border-color-light);
  display: flex;
  align-items: center;
  justify-content: space-around;
  z-index: 1000;
  padding: 0 8px;
  box-shadow: 0 -2px 8px rgba(0, 0, 0, 0.1);
}

.nav-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  flex: 1;
  height: 100%;
  padding: 4px 0;
  color: var(--el-text-color-secondary);
  text-decoration: none;
  transition: all 0.2s ease;
  min-width: 0;
}

.nav-item:hover {
  color: var(--el-color-primary);
}

.nav-item.router-link-active {
  color: var(--el-color-primary);
}

.nav-item .el-icon {
  font-size: 20px;
  margin-bottom: 2px;
}

.nav-item span {
  font-size: 10px;
  line-height: 1.2;
}

/* Dark theme */
:global(html.dark) .mobile-nav {
  background: var(--el-bg-color);
  border-top-color: var(--el-border-color);
  box-shadow: 0 -2px 8px rgba(0, 0, 0, 0.2);
}

/* Safe area for notched devices */
@supports (padding: max(0px)) {
  .mobile-nav {
    padding-bottom: max(0px, env(safe-area-inset-bottom));
  }
}
</style>
