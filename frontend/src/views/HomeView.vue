<template>
  <component :is="currentView" />
</template>

<script setup lang="ts">
import { computed, defineAsyncComponent } from 'vue';
import { useUIStore } from '@/stores';

const uiStore = useUIStore();

const DesktopHomeView = defineAsyncComponent(() =>
  import('./desktop/DesktopHomeView.vue')
);
const MobileHomeView = defineAsyncComponent(() =>
  import('./mobile/MobileHomeView.vue')
);

const currentView = computed(() => {
  return uiStore.isMobile ? MobileHomeView : DesktopHomeView;
});
</script>
