<template>
  <!-- Desktop: Dialog -->
  <el-dialog
    v-if="!isMobile()"
    v-model="visible"
    :title="isEdit ? '编辑标签' : '新建标签'"
    width="450px"
    @close="handleClose"
  >
    <el-form
      ref="formRef"
      :model="form"
      :rules="rules"
      label-width="60px"
      class="desktop-form"
    >
      <el-form-item label="名称" prop="name">
        <el-input
          v-model="form.name"
          placeholder="请输入标签名称"
          @keyup.enter="handleSubmit"
        />
      </el-form-item>

      <el-form-item label="颜色">
        <ColorPicker v-model="form.color" :used-colors="usedColors" />
      </el-form-item>
    </el-form>

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="handleClose">取消</el-button>
        <el-button v-if="isEdit" type="danger" @click="handleDelete" :loading="deleteLoading">
          删除
        </el-button>
        <el-button type="primary" @click="handleSubmit" :loading="loading">
          {{ isEdit ? '保存' : '创建' }}
        </el-button>
      </div>
    </template>
  </el-dialog>

  <!-- Mobile: Full Screen Page -->
  <teleport v-else to="body">
    <transition name="slide-up">
      <div v-if="visible" class="mobile-page">
        <div class="mobile-page-header">
          <el-button class="back-btn" @click="handleClose" text>
            <el-icon><ArrowDown /></el-icon>
          </el-button>
          <span class="mobile-page-title">{{ isEdit ? '编辑标签' : '新建标签' }}</span>
          <div class="spacer"></div>
        </div>

        <div class="mobile-page-content">
          <el-form
            ref="formRef"
            :model="form"
            :rules="rules"
            label-width="auto"
            label-position="top"
            class="mobile-form"
          >
            <el-form-item label="名称" prop="name">
              <el-input
                v-model="form.name"
                placeholder="请输入标签名称"
                @keyup.enter="handleSubmit"
              />
            </el-form-item>

            <el-form-item label="颜色">
              <ColorPicker v-model="form.color" :used-colors="usedColors" />
            </el-form-item>
          </el-form>
        </div>

        <div class="mobile-page-footer">
          <el-button @click="handleClose" class="mobile-btn">取消</el-button>
          <el-button v-if="isEdit" type="danger" @click="handleDelete" :loading="deleteLoading" class="mobile-btn">
            删除
          </el-button>
          <el-button type="primary" @click="handleSubmit" :loading="loading" class="mobile-btn">
            {{ isEdit ? '保存' : '创建' }}
          </el-button>
        </div>
      </div>
    </transition>
  </teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus';
import { ArrowDown } from '@element-plus/icons-vue';
import { useTagStore } from '@/stores';
import ColorPicker from '@/components/common/ColorPicker.vue';
import type { Tag } from '@/types';
import { isMobile } from '@/utils/device';

const props = defineProps<{
  modelValue: boolean;
  tag?: Tag;
}>();

const emit = defineEmits<{
  'update:modelValue': [value: boolean];
  'updated': [];
}>();

const tagStore = useTagStore();

const formRef = ref<FormInstance>();
const loading = ref(false);
const deleteLoading = ref(false);

const form = ref({
  name: '',
  color: '#409EFF',
});

const rules: FormRules = {
  name: [
    { required: true, message: '请输入标签名称', trigger: 'blur' },
    { min: 1, max: 15, message: '长度在 1 到 15 个字符', trigger: 'blur' },
  ],
};

// Get used colors for deduplication (exclude current editing tag)
const usedColors = computed(() => {
  return tagStore.tags
    .filter(t => t.id !== props.tag?.id)
    .map(t => t.color);
});

const visible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value),
});

const isEdit = computed(() => !!props.tag);

// 当对话框打开时，确保加载最新的 tags 数据
watch(visible, async (isOpen) => {
  if (isOpen) {
    try {
      await tagStore.fetchTags();
    } catch (error) {
      console.error('[TagCreateDialog] Failed to fetch tags:', error);
    }
  }
});

watch(() => props.tag, (tag) => {
  if (tag) {
    form.value = {
      name: tag.name,
      color: tag.color,
    };
  } else {
    resetForm();
  }
}, { immediate: true });

function resetForm() {
  form.value = {
    name: '',
    color: '#409EFF',
  };
  formRef.value?.clearValidate();
}

async function handleSubmit() {
  if (!formRef.value) return;

  try {
    await formRef.value.validate();
    loading.value = true;

    if (isEdit.value && props.tag) {
      await tagStore.updateTag(props.tag.id, form.value);
      ElMessage.success('标签更新成功');
    } else {
      await tagStore.createTag(form.value.name, form.value.color);
      ElMessage.success('标签创建成功');
    }

    emit('updated');
    handleClose();
  } catch (error: any) {
    if (error?.errors) {
      return;
    }
    ElMessage.error(`操作失败: ${error}`);
  } finally {
    loading.value = false;
  }
}

async function handleDelete() {
  if (!props.tag) return;

  try {
    await ElMessageBox.confirm(
      `确定要删除标签 "${props.tag.name}" 吗？`,
      '删除标签',
      {
        type: 'warning',
        confirmButtonText: '删除',
        cancelButtonText: '取消',
      }
    );

    deleteLoading.value = true;
    await tagStore.deleteTag(props.tag.id);
    ElMessage.success('删除成功');
    emit('updated');
    handleClose();
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败');
    }
  } finally {
    deleteLoading.value = false;
  }
}

function handleClose() {
  resetForm();
  visible.value = false;
}
</script>

<style scoped>
/* ColorPicker component handles its own styling */

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

/* Desktop form styles */
.desktop-form :deep(.el-form-item) {
  margin-bottom: 18px;
}

.desktop-form :deep(.el-form-item__label) {
  font-size: 14px;
  font-weight: 500;
  line-height: 1.5;
}

.desktop-form :deep(.el-input__wrapper) {
  font-size: 14px;
}

/* ========== Mobile Full Screen Page ========== */

/* Mobile page container */
.mobile-page {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 9999;
  background: var(--el-bg-color);
  display: flex;
  flex-direction: column;
}

/* Mobile page header */
.mobile-page-header {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid var(--el-border-color);
  background: var(--el-bg-color);
  gap: 12px;
}

.back-btn {
  padding: 8px;
  font-size: 20px;
}

.mobile-page-title {
  font-size: 16px;
  font-weight: 500;
  flex: 1;
  text-align: center;
}

.spacer {
  width: 36px;
}

/* Mobile page content */
.mobile-page-content {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  padding-bottom: 80px;
}

/* Mobile form styles */
.mobile-form :deep(.el-form-item) {
  margin-bottom: 14px;
}

.mobile-form :deep(.el-form-item__label) {
  font-size: 12px;
  font-weight: 500;
  margin-bottom: 4px;
  line-height: 1.4;
}

.mobile-form :deep(.el-input__wrapper) {
  font-size: 14px;
  min-height: 32px;
}

/* Mobile page footer */
.mobile-page-footer {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  padding: 12px 16px;
  background: var(--el-bg-color);
  border-top: 1px solid var(--el-border-color);
  display: flex;
  gap: 8px;
  z-index: 10;
}

.mobile-btn {
  flex: 1;
  height: 38px;
  font-size: 14px;
}

/* Transition for slide up animation */
.slide-up-enter-active,
.slide-up-leave-active {
  transition: transform 0.3s ease;
}

.slide-up-enter-from {
  transform: translateY(100%);
}

.slide-up-leave-to {
  transform: translateY(100%);
}

.slide-up-enter-to,
.slide-up-leave-from {
  transform: translateY(0);
}

/* Mobile responsive styles for non-mobile dialogs */
@media (max-width: 768px) {
  :deep(.el-dialog) {
    width: 90% !important;
    max-width: 400px !important;
    margin: 5vh auto !important;
  }
}
</style>
