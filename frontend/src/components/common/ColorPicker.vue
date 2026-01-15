<template>
  <div class="color-picker">
    <!-- 顶部控制行：颜色选择器 + 随机按钮 -->
    <div class="control-row">
      <el-color-picker
        v-model="currentColor"
        size="default"
        show-alpha
        :predefine="predefineColors"
        @change="handleChange"
      />
      <el-button
        size="small"
        :icon="Refresh"
        @click="handleRandom"
      >
        {{ t('tag.randomColor') }}
      </el-button>
    </div>

    <!-- 预设颜色网格 - 左右布局 -->
    <div class="preset-colors">
      <div
        v-for="category in categories"
        :key="category.key"
        class="color-row"
      >
        <div class="category-label">{{ t(category.label) }}</div>
        <div class="color-grid">
          <div
            v-for="color in category.colors"
            :key="color"
            class="color-option"
            :class="{ selected: modelValue === color }"
            :style="{ backgroundColor: color }"
            :title="color"
            @click="selectColor(color)"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { Refresh } from '@element-plus/icons-vue';
import { COLOR_CATEGORIES, ALL_COLORS, getRandomColor, DEFAULT_COLOR } from '@/config/colors';

const props = defineProps<{
  modelValue: string;
  usedColors?: string[];  // 已使用的颜色，用于随机去重
}>();

const emit = defineEmits<{
  'update:modelValue': [value: string];
}>();

const { t } = useI18n();
const currentColor = ref(props.modelValue);

// 监听 modelValue 变化
watch(() => props.modelValue, (newVal) => {
  currentColor.value = newVal;
});

const categories = computed(() => COLOR_CATEGORIES);

// 预定义颜色（用于 Element Plus ColorPicker）
const predefineColors = computed(() =>
  ALL_COLORS.slice(0, 24)  // 取前 24 个作为预定义
);

function handleChange(value: string) {
  if (value) {
    emit('update:modelValue', value);
  }
}

function selectColor(color: string) {
  currentColor.value = color;
  emit('update:modelValue', color);
}

function handleRandom() {
  const newColor = getRandomColor(props.usedColors || []);
  currentColor.value = newColor;
  emit('update:modelValue', newColor);
}
</script>

<style scoped>
.color-picker {
  display: flex;
  flex-direction: column;
  gap: 12px;
  width: 100%;
}

.control-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 8px;
  background: #f5f7fa;
  border-radius: 6px;
}

.preset-colors {
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-height: 200px;
  overflow-y: auto;
  padding: 4px;
}

.color-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.category-label {
  min-width: 60px;
  font-size: 12px;
  color: #909399;
  font-weight: 500;
  flex-shrink: 0;
}

.color-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.color-option {
  width: 24px;
  height: 24px;
  border-radius: 4px;
  cursor: pointer;
  border: 2px solid transparent;
  transition: all 0.2s ease;
}

.color-option:hover {
  transform: scale(1.1);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.15);
}

.color-option.selected {
  border-color: #409eff;
  box-shadow: 0 0 0 2px white, 0 0 0 4px #409eff;
}

/* Dark theme */
[data-theme='dark'] .control-row {
  background: #2a2a2a;
}

[data-theme='dark'] .color-option.selected {
  border-color: #409eff;
  box-shadow: 0 0 0 2px #1a1a1a, 0 0 0 4px #409eff;
}

[data-theme='dark'] .category-label {
  color: #a0a0a0;
}
</style>
