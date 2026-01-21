<template>
  <component :is="currentView" />
</template>

<script setup lang="ts">
import { computed, defineAsyncComponent } from 'vue';
import { useUIStore } from '@/stores';

const uiStore = useUIStore();

const DesktopSettingsView = defineAsyncComponent(() =>
  import('./desktop/DesktopSettingsView.vue')
);
const MobileSettingsView = defineAsyncComponent(() =>
  import('./mobile/MobileSettingsView.vue')
);

const currentView = computed(() => {
  return uiStore.isMobile ? MobileSettingsView : DesktopSettingsView;
});
</script>
