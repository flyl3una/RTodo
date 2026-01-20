<template>
  <el-dialog
    v-model="visible"
    :title="isMobile() ? '' : (isEdit ? '编辑标签' : '新建标签')"
    :width="isMobile() ? '100%' : '450px'"
    :class="{ 'mobile-dialog': isMobile() }"
    @close="handleClose"
  >
    <!-- Mobile Header -->
    <template #header="{ close, titleId, titleClass }">
      <div v-if="isMobile()" class="mobile-header">
        <span class="mobile-header-title">{{ isEdit ? '编辑标签' : '新建标签' }}</span>
      </div>
    </template>

    <el-form
      ref="formRef"
      :model="form"
      :rules="rules"
      :label-width="isMobile() ? 'auto' : '60px'"
      :label-position="isMobile() ? 'top' : undefined"
      :class="{ 'mobile-form': isMobile(), 'desktop-form': !isMobile() }"
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
      <div class="dialog-footer" :class="{ 'mobile-footer': isMobile() }">
        <el-button @click="handleClose" :class="{ 'mobile-btn': isMobile() }">取消</el-button>
        <el-button v-if="isEdit" type="danger" @click="handleDelete" :loading="deleteLoading" :class="{ 'mobile-btn': isMobile() }">
          删除
        </el-button>
        <el-button type="primary" @click="handleSubmit" :loading="loading" :class="{ 'mobile-btn': isMobile() }">
          {{ isEdit ? '保存' : '创建' }}
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus';
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

/* Mobile dialog styles */
.mobile-dialog :deep(.el-dialog) {
  width: 100% !important;
  max-width: none !important;
  margin: 0 !important;
  height: 100%;
  border-radius: 0;
}

.mobile-dialog :deep(.el-dialog__header) {
  padding: 0;
  margin: 0;
}

.mobile-dialog :deep(.el-dialog__body) {
  padding: 10px 12px;
  padding-bottom: 80px;
  max-height: calc(100vh - 120px);
  overflow-y: auto;
}

.mobile-dialog :deep(.el-dialog__footer) {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  padding: 10px 12px;
  background: var(--el-bg-color);
  border-top: 1px solid var(--el-border-color);
  z-index: 1000;
}

.mobile-header {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 12px;
  border-bottom: 1px solid var(--el-border-color);
  background: var(--el-bg-color);
}

.mobile-header-title {
  font-size: 16px;
  font-weight: 500;
}

.mobile-footer {
  display: flex;
  gap: 8px;
}

.mobile-btn {
  flex: 1;
  height: 38px;
  font-size: 14px;
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

/* Mobile responsive styles for non-mobile dialogs */
@media (max-width: 768px) {
  :deep(.el-dialog:not(.mobile-dialog .el-dialog)) {
    width: 90% !important;
    max-width: 400px !important;
    margin: 5vh auto !important;
  }

  :deep(.el-dialog__header) {
    padding: 16px;
  }

  :deep(.el-dialog__title) {
    font-size: 16px;
  }

  :deep(.el-dialog__body) {
    padding: 16px;
  }

  :deep(.el-form-item) {
    margin-bottom: 16px;
  }

  :deep(.el-form-item__label) {
    font-size: 14px;
    margin-bottom: 8px;
  }

  :deep(.el-dialog__footer) {
    padding: 12px 16px;
  }

  :deep(.el-dialog__footer .el-button) {
    flex: 1;
    margin: 0 4px;
    padding: 12px;
  }
}
</style>
