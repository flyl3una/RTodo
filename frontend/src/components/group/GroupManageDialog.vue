<template>
  <el-dialog
    v-model="visible"
    :title="isEdit ? t('group.editGroup') : t('group.createGroup')"
    width="600px"
    @close="handleClose"
  >
    <el-form
      ref="formRef"
      :model="form"
      :rules="rules"
      label-width="80px"
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
</style>
