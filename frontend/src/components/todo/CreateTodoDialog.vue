<template>
  <el-dialog
    v-model="visible"
    :title="t('todo.createTodo')"
    width="600px"
    @close="handleClose"
  >
    <el-form
      ref="formRef"
      :model="form"
      :rules="rules"
      label-width="80px"
      @submit.prevent="handleSubmit"
    >
      <el-form-item :label="t('common.title')" prop="title">
        <el-input
          v-model="form.title"
          :placeholder="t('todo.titlePlaceholder')"
          @keyup.enter="handleSubmit"
        />
      </el-form-item>

      <el-form-item :label="t('common.description')">
        <el-input
          v-model="form.description"
          type="textarea"
          :rows="3"
          :placeholder="t('todo.descriptionPlaceholder')"
        />
      </el-form-item>

      <el-form-item :label="t('common.group')">
        <el-select
          v-model="form.group_id"
          :placeholder="t('todo.selectGroup')"
          clearable
          style="width: 100%"
        >
          <el-option
            v-for="group in groups"
            :key="group.id"
            :label="group.name"
            :value="group.id"
          />
        </el-select>
      </el-form-item>

      <el-form-item :label="t('todo.startDate')">
        <el-date-picker
          v-model="form.start_date"
          type="datetime"
          :placeholder="t('todo.selectStartDate')"
          format="YYYY-MM-DD HH:mm"
          value-format="x"
          :clearable="true"
          size="default"
          style="width: 100%"
        />
      </el-form-item>

      <el-form-item :label="t('todo.dueDate')">
        <el-date-picker
          v-model="form.due_date"
          type="datetime"
          :placeholder="t('todo.selectDueDate')"
          format="YYYY-MM-DD HH:mm"
          value-format="x"
          :clearable="true"
          size="default"
          style="width: 100%"
        />
      </el-form-item>

      <el-form-item :label="t('common.priority')">
        <el-radio-group v-model="form.priority">
          <el-radio :label="0">{{ t('priority.normal') }}</el-radio>
          <el-radio :label="1">{{ t('priority.important') }}</el-radio>
          <el-radio :label="2">{{ t('priority.urgent') }}</el-radio>
        </el-radio-group>
      </el-form-item>

      <el-form-item :label="t('common.tags')">
        <el-select
          v-model="form.tag_ids"
          multiple
          :placeholder="t('todo.selectTags')"
          style="width: 100%"
        >
          <el-option
            v-for="tag in tags"
            :key="tag.id"
            :label="tag.name"
            :value="tag.id"
          />
        </el-select>
      </el-form-item>
    </el-form>

    <template #footer>
      <el-button @click="handleClose">{{ t('common.cancel') }}</el-button>
      <el-button type="primary" @click="handleSubmit" :loading="loading">
        {{ t('common.create') }}
      </el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { ElMessage } from 'element-plus';
import type { FormInstance, FormRules } from 'element-plus';
import { useTodoStore } from '@/stores';
import { useGroupStore } from '@/stores';
import { useTagStore } from '@/stores';
import type { CreateTodoRequest } from '@/types';

const { t } = useI18n();

const props = defineProps<{
  modelValue: boolean;
}>();

const emit = defineEmits<{
  'update:modelValue': [value: boolean];
  'created': [];
}>();

const todoStore = useTodoStore();
const groupStore = useGroupStore();
const tagStore = useTagStore();

const formRef = ref<FormInstance>();
const loading = ref(false);

const form = ref<CreateTodoRequest>({
  title: '',
  description: '',
  group_id: undefined,
  start_date: undefined,
  due_date: undefined,
  priority: 0,
  tag_ids: [],
});

const rules: FormRules = {
  title: [
    { required: true, message: t('todo.titleRequired'), trigger: 'blur' },
  ],
};

const visible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value),
});

const groups = computed(() => groupStore.groups);
const tags = computed(() => tagStore.tags);

async function handleSubmit() {
  if (!formRef.value) return;

  try {
    await formRef.value.validate();
    loading.value = true;

    const request: CreateTodoRequest = {
      title: form.value.title,
      description: form.value.description || undefined,
      group_id: form.value.group_id,
      // 鍙湁鍦ㄥ€间笉鏄?undefined 鏃舵墠浼犻€掞紝鍏佽 null 浼犻€掑埌鍚庣
      ...(form.value.start_date !== undefined && { start_date: form.value.start_date }),
      ...(form.value.due_date !== undefined && { due_date: form.value.due_date }),
      priority: form.value.priority,
      tag_ids: form.value.tag_ids?.length ? form.value.tag_ids : undefined,
    };

    await todoStore.createTodo(request);
    ElMessage.success(t('todo.createSuccess'));
    emit('created');
    handleClose();
  } catch (error: any) {
    if (error?.errors) {
      // Validation error, do nothing
      return;
    }
    ElMessage.error(`${t('todo.createFailed')}: ${error}`);
  } finally {
    loading.value = false;
  }
}

function handleClose() {
  formRef.value?.resetFields();
  form.value = {
    title: '',
    description: '',
    group_id: undefined,
    start_date: undefined,
    due_date: undefined,
    priority: 0,
    tag_ids: [],
  };
  visible.value = false;
}

// Load groups and tags when dialog opens
watch(visible, async (isOpen) => {
  if (isOpen) {
    try {
      await groupStore.fetchGroups();
      await tagStore.fetchTags();
    } catch (error) {
      console.error('Failed to load groups/tags:', error);
    }
  }
});
</script>

<style scoped>
:deep(.el-dialog__body) {
  padding-top: 20px;
}
</style>
