<template>
  <el-dialog
    v-model="visible"
    :title="isMobile() ? '' : t('todo.createTodo')"
    :width="isMobile() ? '100%' : '550px'"
    :class="{ 'mobile-dialog': isMobile() }"
    :close-on-click-modal="false"
    :close-on-press-escape="false"
    @close="handleClose"
  >
    <!-- Mobile Header -->
    <template #header="{ close, titleId, titleClass }">
      <div v-if="isMobile()" class="mobile-header">
        <span class="mobile-header-title">{{ t('todo.createTodo') }}</span>
      </div>
    </template>

    <el-form
      ref="formRef"
      :model="form"
      :rules="rules"
      :label-width="isMobile() ? 'auto' : '80px'"
      :label-position="isMobile() ? 'top' : undefined"
      class="todo-form"
      :class="{ 'mobile-form': isMobile(), 'desktop-form': !isMobile() }"
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
        <el-radio-group v-model="form.priority" :class="{ 'mobile-priority': isMobile(), 'desktop-priority': !isMobile() }">
          <el-radio :label="0">{{ t('priority.normal') }}</el-radio>
          <el-radio :label="1">{{ t('priority.important') }}</el-radio>
          <el-radio :label="3">{{ t('priority.urgent') }}</el-radio>
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
      <div class="dialog-footer" :class="{ 'mobile-footer': isMobile() }">
        <el-button @click="handleClose" :class="{ 'mobile-btn': isMobile() }">
          {{ t('common.cancel') }}
        </el-button>
        <el-button type="primary" @click="handleSubmit" :loading="loading" :class="{ 'mobile-btn': isMobile() }">
          {{ t('common.create') }}
        </el-button>
      </div>
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
import { isMobile } from '@/utils/device';

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

.todo-form {
  max-height: 60vh;
  overflow-y: auto;
}

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
  gap: 12px;
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

.desktop-form :deep(.el-input__wrapper),
.desktop-form :deep(.el-textarea__inner) {
  font-size: 14px;
}

.desktop-form :deep(.el-textarea__inner) {
  padding: 8px 12px;
}

.desktop-form :deep(.el-select) {
  width: 100%;
}

.desktop-form :deep(.el-date-editor) {
  width: 100%;
}

/* Desktop priority buttons */
.desktop-priority {
  display: flex;
  gap: 12px;
}

.desktop-priority :deep(.el-radio) {
  margin-right: 0;
}

.mobile-form {
  /* padding: 0 8px; */
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

.mobile-form :deep(.el-input__wrapper),
.mobile-form :deep(.el-textarea__inner) {
  font-size: 14px;
  min-height: 32px;
}

.mobile-form :deep(.el-textarea__inner) {
  min-height: 60px;
  padding: 6px 8px;
}

.mobile-form :deep(.el-select) {
  width: 100%;
}

.mobile-form :deep(.el-select .el-input__wrapper) {
  min-height: 32px;
}

.mobile-form :deep(.el-date-editor) {
  width: 100%;
}

.mobile-form :deep(.el-date-editor .el-input__wrapper) {
  min-height: 32px;
}

/* Mobile priority buttons */
.mobile-priority {
  display: flex;
  flex-direction: row;
  gap: 8px;
}

.mobile-priority :deep(.el-radio) {
  margin-right: 0;
  flex: 1;
  height: 36px;
  padding: 0 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  /* border: 1px solid var(--el-border-color); */
  border-radius: 6px;
  /* transition: all 0.2s; */
}

.mobile-priority :deep(.el-radio.is-checked) {
  border-color: var(--el-color-primary);
  /* background: var(--el-color-primary-light-9); */
}

.mobile-priority :deep(.el-radio__label) {
  font-size: 13px;
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
