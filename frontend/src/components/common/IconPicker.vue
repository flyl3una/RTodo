<template>
  <div class="icon-picker">
    <!-- 分类标签 + 随机按钮 -->
    <div class="category-tabs">
      <el-button
        v-for="category in categories"
        :key="category.key"
        size="small"
        :type="activeCategory === category.key ? 'primary' : ''"
        @click="activeCategory = category.key"
      >
        {{ t(category.label) }}
      </el-button>
      <div style="flex: 1"></div>
      <el-button
        size="small"
        :icon="Refresh"
        @click="handleRandom"
      >
        随机图标
      </el-button>
    </div>

    <!-- 图标网格 -->
    <div class="icon-grid">
      <div
        v-for="icon in filteredIcons"
        :key="icon"
        class="icon-option"
        :class="{ selected: modelValue === icon }"
        @click="$emit('update:modelValue', icon)"
      >
        {{ icon }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { Refresh } from '@element-plus/icons-vue';
import { ICON_CATEGORIES, PRESET_ICONS, DEFAULT_ICON, getRandomIconFromCategory, type IconCategoryKey, ALL_ICONS } from '@/config/icons';

const props = defineProps<{
  modelValue: string;
  usedIcons?: string[];  // 已使用的图标
}>();

const emit = defineEmits<{
  'update:modelValue': [value: string];
}>();

const { t } = useI18n();
const activeCategory = ref<IconCategoryKey>('all');

const categories = computed(() => ICON_CATEGORIES);

const filteredIcons = computed(() => {
  if (activeCategory.value === 'all') {
    return ALL_ICONS;
  }
  return PRESET_ICONS[activeCategory.value] || [];
});

function handleRandom() {
  // 从当前分类中随机选择
  const newIcon = getRandomIconFromCategory(activeCategory.value, props.usedIcons || []);
  emit('update:modelValue', newIcon);
}
</script>

<style scoped>
.icon-picker {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.category-tabs {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
  align-items: center;
}

.icon-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(40px, 1fr));
  gap: 8px;
  max-height: 200px;
  overflow-y: auto;
  padding: 4px;
}

.icon-option {
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  border: 2px solid #e4e7ed;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.icon-option:hover {
  border-color: var(--el-color-primary);
  background: var(--el-fill-color-light);
  transform: scale(1.1);
}

.icon-option.selected {
  border-color: var(--el-color-primary);
  background: var(--el-fill-color-lighter);
  box-shadow: 0 0 0 2px var(--el-color-primary);
}

/* Dark theme */
:global(html.dark) .category-tabs {
  background: transparent;
}

:global(html.dark) .icon-option {
  border-color: var(--el-border-color);
  background: transparent !important;
}

:global(html.dark) .icon-option:hover {
  background: #1f1f1f !important;
}

:global(html.dark) .icon-option.selected {
  background: #1f1f1f !important;
  box-shadow: 0 0 0 2px var(--el-color-primary);
}

:global(html.dark) .icon-grid {
  background: transparent;
}
</style>
