<template>
  <!-- Desktop: Dialog -->
  <el-dialog
    v-if="!isMobile()"
    v-model="visible"
    :title="isEdit ? t('group.editGroup') : t('group.createGroup')"
    width="500px"
    @close="handleClose"
  >
    <el-form
      ref="formRef"
      :model="form"
      :rules="rules"
      label-width="80px"
      class="desktop-form"
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
      <div class="dialog-footer">
        <el-button @click="handleClose">{{ t('common.cancel') }}</el-button>
        <el-button
          v-if="isEdit"
          type="danger"
          @click="handleDelete"
          :loading="deleteLoading"
        >
          {{ t('common.delete') }}
        </el-button>
        <el-button type="primary" @click="handleSubmit" :loading="loading">
          {{ isEdit ? t('common.save') : t('common.create') }}
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
          <span class="mobile-page-title">{{ isEdit ? t('group.editGroup') : t('group.createGroup') }}</span>
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
        </div>

        <div class="mobile-page-footer">
          <el-button @click="handleClose" class="mobile-btn">{{ t('common.cancel') }}</el-button>
          <el-button
            v-if="isEdit"
            type="danger"
            @click="handleDelete"
            :loading="deleteLoading"
            class="mobile-btn"
          >
            {{ t('common.delete') }}
          </el-button>
          <el-button type="primary" @click="handleSubmit" :loading="loading" class="mobile-btn">
            {{ isEdit ? t('common.save') : t('common.create') }}
          </el-button>
        </div>
      </div>
    </transition>
  </teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus';
import { ArrowDown } from '@element-plus/icons-vue';
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
