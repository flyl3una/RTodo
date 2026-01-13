<template>
  <el-dialog
    v-model="visible"
    title="新建任务"
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
      <el-form-item label="标题" prop="title">
        <el-input
          v-model="form.title"
          placeholder="请输入任务标题"
          @keyup.enter="handleSubmit"
        />
      </el-form-item>

      <el-form-item label="描述">
        <el-input
          v-model="form.description"
          type="textarea"
          :rows="3"
          placeholder="请输入任务描述（可选）"
        />
      </el-form-item>

      <el-form-item label="任务组">
        <el-select
          v-model="form.group_id"
          placeholder="选择任务组（可选）"
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

      <el-form-item label="开始时间">
        <el-date-picker
          v-model="form.start_date"
          type="datetime"
          placeholder="选择开始时间（可选）"
          format="YYYY-MM-DD HH:mm"
          value-format="x"
          :clearable="true"
          size="default"
          style="width: 100%"
        />
      </el-form-item>

      <el-form-item label="截止时间">
        <el-date-picker
          v-model="form.due_date"
          type="datetime"
          placeholder="选择截止时间（可选）"
          format="YYYY-MM-DD HH:mm"
          value-format="x"
          :clearable="true"
          size="default"
          style="width: 100%"
        />
      </el-form-item>

      <el-form-item label="优先级">
        <el-radio-group v-model="form.priority">
          <el-radio :label="0">普通</el-radio>
          <el-radio :label="1">重要</el-radio>
          <el-radio :label="2">紧急</el-radio>
        </el-radio-group>
      </el-form-item>

      <el-form-item label="标签">
        <el-select
          v-model="form.tag_ids"
          multiple
          placeholder="选择标签（可选）"
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
      <el-button @click="handleClose">取消</el-button>
      <el-button type="primary" @click="handleSubmit" :loading="loading">
        创建
      </el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { ElMessage } from 'element-plus';
import type { FormInstance, FormRules } from 'element-plus';
import { useTodoStore } from '@/stores';
import { useGroupStore } from '@/stores';
import { useTagStore } from '@/stores';
import type { CreateTodoRequest } from '@/types';

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
    { required: true, message: '请输入任务标题', trigger: 'blur' },
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
      start_date: form.value.start_date ?? undefined,
      due_date: form.value.due_date ?? undefined,
      priority: form.value.priority,
      tag_ids: form.value.tag_ids?.length ? form.value.tag_ids : undefined,
    };

    await todoStore.createTodo(request);
    ElMessage.success('任务创建成功');
    emit('created');
    handleClose();
  } catch (error: any) {
    if (error?.errors) {
      // Validation error, do nothing
      return;
    }
    ElMessage.error(`创建失败: ${error}`);
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
