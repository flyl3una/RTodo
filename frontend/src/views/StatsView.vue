<template>
  <component :is="currentView" />
</template>

<script setup lang="ts">
import { computed, defineAsyncComponent } from 'vue';
import { useUIStore } from '@/stores';

const uiStore = useUIStore();

const DesktopStatsView = defineAsyncComponent(() =>
  import('./desktop/DesktopStatsView.vue')
);
const MobileStatsView = defineAsyncComponent(() =>
  import('./mobile/MobileStatsView.vue')
);

const currentView = computed(() => {
  return uiStore.isMobile ? MobileStatsView : DesktopStatsView;
});
</script>
