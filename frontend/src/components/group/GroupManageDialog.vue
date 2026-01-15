<template>
  <el-dialog
    v-model="visible"
    :title="isEdit ? '编辑任务组' : '新建任务组'"
    width="600px"
    @close="handleClose"
  >
    <el-form
      ref="formRef"
      :model="form"
      :rules="rules"
      label-width="80px"
    >
      <el-form-item label="名称" prop="name">
        <el-input
          v-model="form.name"
          placeholder="请输入任务组名称"
          @keyup.enter="handleSubmit"
        />
      </el-form-item>

      <el-form-item label="图标">
        <IconPicker v-model="form.icon" :used-icons="usedIcons" />
      </el-form-item>

      <el-form-item label="颜色">
        <ColorPicker v-model="form.color" :used-colors="usedColors" />
      </el-form-item>
    </el-form>

    <template #footer>
      <el-button @click="handleClose">取消</el-button>
      <el-button
        v-if="isEdit"
        type="danger"
        @click="handleDelete"
        :loading="deleteLoading"
      >
        删除
      </el-button>
      <el-button type="primary" @click="handleSubmit" :loading="loading">
        {{ isEdit ? '保存' : '创建' }}
      </el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus';
import { useGroupStore } from '@/stores';
import IconPicker from '@/components/common/IconPicker.vue';
import ColorPicker from '@/components/common/ColorPicker.vue';
import type { TaskGroup } from '@/types';

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
    { required: true, message: '请输入任务组名称', trigger: 'blur' },
    { min: 1, max: 20, message: '长度在 1 到 20 个字符', trigger: 'blur' },
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
      ElMessage.success('任务组更新成功');
    } else {
      await groupStore.createGroup({
        name: form.value.name,
        icon: form.value.icon,
        color: form.value.color,
      });
      ElMessage.success('任务组创建成功');
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
  if (!props.group) return;

  try {
    await ElMessageBox.confirm(
      '删除任务组不会删除其中的任务，确定要删除吗？',
      '删除任务组',
      {
        type: 'warning',
        confirmButtonText: '删除',
        cancelButtonText: '取消',
      }
    );

    deleteLoading.value = true;
    await groupStore.deleteGroup(props.group.id);
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
/* IconPicker and ColorPicker components handle their own styling */
</style>
