<template>
  <el-dialog
    v-model="visible"
    :title="isMobile() ? '' : (isEdit ? t('group.editGroup') : t('group.createGroup'))"
    :width="isMobile() ? '100%' : '500px'"
    :class="{ 'mobile-dialog': isMobile() }"
    @close="handleClose"
  >
    <!-- Mobile Header -->
    <template #header="{ close, titleId, titleClass }">
      <div v-if="isMobile()" class="mobile-header">
        <span class="mobile-header-title">{{ isEdit ? t('group.editGroup') : t('group.createGroup') }}</span>
      </div>
    </template>

    <el-form
      ref="formRef"
      :model="form"
      :rules="rules"
      :label-width="isMobile() ? 'auto' : '80px'"
      :label-position="isMobile() ? 'top' : undefined"
      :class="{ 'mobile-form': isMobile(), 'desktop-form': !isMobile() }"
    >
      <el-form-item :label="t('common.name')" prop="name">
        <el-input
          v-model="form.name"
          :placeholder="t('group.groupNamePlaceholder')"
          @keyup.enter="handleSubmit"
        />
      </el-form-item>

      <el-form-item :label="t('common.icon')">
        <IconPicker v-model="form.icon" :used-icons="usedIcons" />
      </el-form-item>

      <el-form-item :label="t('common.color')">
        <ColorPicker v-model="form.color" :used-colors="usedColors" />
      </el-form-item>
    </el-form>

    <template #footer>
      <div class="dialog-footer" :class="{ 'mobile-footer': isMobile() }">
        <el-button @click="handleClose" :class="{ 'mobile-btn': isMobile() }">{{ t('common.cancel') }}</el-button>
        <el-button
          v-if="isEdit"
          type="danger"
          @click="handleDelete"
          :loading="deleteLoading"
          :class="{ 'mobile-btn': isMobile() }"
        >
          {{ t('common.delete') }}
        </el-button>
        <el-button type="primary" @click="handleSubmit" :loading="loading" :class="{ 'mobile-btn': isMobile() }">
          {{ isEdit ? t('common.save') : t('common.create') }}
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus';
import { useGroupStore } from '@/stores';
import IconPicker from '@/components/common/IconPicker.vue';
import ColorPicker from '@/components/common/ColorPicker.vue';
import type { TaskGroup } from '@/types';
import { isMobile } from '@/utils/device';

const { t } = useI18n();

const props = defineProps<{
  modelValue: boolean;
  group?: TaskGroup;
}>();

const emit = defineEmits<{
  'update:modelValue': [value: boolean];
  'updated': [];
}>();

const groupStore = useGroupStore();

const formRef = ref<FormInstance>();
const loading = ref(false);
const deleteLoading = ref(false);

const form = ref({
  name: '',
  icon: '📁',
  color: '#409EFF',
});

const rules: FormRules = {
  name: [
    { required: true, message: t('group.groupNameRequired'), trigger: 'blur' },
    { min: 1, max: 20, message: t('group.groupNameLengthRule'), trigger: 'blur' },
  ],
};

// Get used icons for deduplication (exclude current editing group)
const usedIcons = computed(() => {
  return groupStore.groups
    .filter(g => g.id !== props.group?.id)
    .map(g => g.icon || '');
});

// Get used colors for deduplication (exclude current editing group)
const usedColors = computed(() => {
  return groupStore.groups
    .filter(g => g.id !== props.group?.id)
    .map(g => g.color || '');
});

const visible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value),
});

const isEdit = computed(() => !!props.group);

// 当对话框打开时，确保加载最新的 groups 数据
watch(visible, async (isOpen) => {
  if (isOpen) {
    try {
      await groupStore.fetchGroups();
    } catch (error) {
      console.error('[GroupManageDialog] Failed to fetch groups:', error);
    }
  }
});

watch(() => props.group, (group) => {
  if (group) {
    form.value = {
      name: group.name,
      icon: group.icon || '📁',
      color: group.color || '#409EFF',
    };
  } else {
    resetForm();
  }
}, { immediate: true });

function resetForm() {
  form.value = {
    name: '',
    icon: '📁',
    color: '#409EFF',
  };
  formRef.value?.clearValidate();
}

async function handleSubmit() {
  if (!formRef.value) return;

  try {
    await formRef.value.validate();
    loading.value = true;

    if (isEdit.value && props.group) {
      await groupStore.updateGroup(props.group.id, {
        name: form.value.name,
        icon: form.value.icon,
        color: form.value.color,
      });
      ElMessage.success(t('group.groupUpdated'));
    } else {
      await groupStore.createGroup({
        name: form.value.name,
        icon: form.value.icon,
        color: form.value.color,
      });
      ElMessage.success(t('group.groupCreated'));
    }

    emit('updated');
    handleClose();
  } catch (error: any) {
    if (error?.errors) {
      return;
    }
    ElMessage.error(`${t('group.operationFailed')}: ${error}`);
  } finally {
    loading.value = false;
  }
}

async function handleDelete() {
  if (!props.group) return;

  try {
    await ElMessageBox.confirm(
      t('group.deleteConfirm'),
      t('group.deleteGroupTitle'),
      {
        type: 'warning',
        confirmButtonText: t('common.delete'),
        cancelButtonText: t('common.cancel'),
      }
    );

    deleteLoading.value = true;
    await groupStore.deleteGroup(props.group.id);
    ElMessage.success(t('group.deleteSuccess'));
    emit('updated');
    handleClose();
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error(t('group.deleteFailed'));
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
/* IconPicker and ColorPicker components handle their own styling */

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
