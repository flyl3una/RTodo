<template>
  <!-- Desktop: Dialog -->
  <el-dialog
    v-if="!isMobile()"
    v-model="visible"
    :title="t('todo.createTodo')"
    width="550px"
    :close-on-click-modal="false"
    :close-on-press-escape="false"
    @close="handleClose"
  >
    <el-form
      ref="formRef"
      :model="form"
      :rules="rules"
      label-width="80px"
      class="todo-form desktop-form"
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
        <el-radio-group v-model="form.priority" class="desktop-priority">
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

      <!-- 子步骤 -->
      <el-form-item :label="t('step.steps')">
        <div class="steps-input">
          <div v-if="pendingSteps.length > 0" class="pending-steps">
            <div v-for="(step, index) in pendingSteps" :key="index" class="pending-step">
              <el-input
                v-model="step.title"
                :placeholder="`${t('step.step')} ${index + 1}`"
                size="small"
              />
              <el-button :icon="Delete" size="small" text @click="removeStep(index)" />
            </div>
          </div>
          <el-button :icon="Plus" size="small" @click="addStep">
            {{ t('step.addStep') }}
          </el-button>
        </div>
      </el-form-item>

      <el-form-item :label="t('attachment.attachments')">
        <div class="attachment-upload">
          <el-button :icon="Upload" @click="handleSelectFiles">
            {{ t('attachment.selectFiles') }}
          </el-button>
          <div v-if="pendingFiles.length > 0" class="pending-files">
            <div v-for="(file, index) in pendingFiles" :key="index" class="pending-file">
              <el-icon><Document /></el-icon>
              <span>{{ file.name }}</span>
              <el-button :icon="Close" size="small" text @click="removeFile(index)" />
            </div>
          </div>
        </div>
      </el-form-item>
    </el-form>

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="handleClose">
          {{ t('common.cancel') }}
        </el-button>
        <el-button type="primary" @click="handleSubmit" :loading="loading">
          {{ t('common.create') }}
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
          <span class="mobile-page-title">{{ t('todo.createTodo') }}</span>
          <div class="spacer"></div>
        </div>

        <div class="mobile-page-content">
          <el-form
            ref="formRef"
            :model="form"
            :rules="rules"
            label-width="auto"
            label-position="top"
            class="todo-form mobile-form"
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
              <el-radio-group v-model="form.priority" class="mobile-priority">
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

            <!-- 子步骤 -->
            <el-form-item :label="t('step.steps')">
              <div class="steps-input">
                <div v-if="pendingSteps.length > 0" class="pending-steps">
                  <div v-for="(step, index) in pendingSteps" :key="index" class="pending-step">
                    <el-input
                      v-model="step.title"
                      :placeholder="`${t('step.step')} ${index + 1}`"
                      size="small"
                    />
                    <el-button :icon="Delete" size="small" text @click="removeStep(index)" />
                  </div>
                </div>
                <el-button :icon="Plus" size="small" @click="addStep">
                  {{ t('step.addStep') }}
                </el-button>
              </div>
            </el-form-item>

            <el-form-item :label="t('attachment.attachments')">
              <div class="attachment-upload">
                <el-button :icon="Upload" @click="handleSelectFiles">
                  {{ t('attachment.selectFiles') }}
                </el-button>
                <div v-if="pendingFiles.length > 0" class="pending-files">
                  <div v-for="(file, index) in pendingFiles" :key="index" class="pending-file">
                    <el-icon><Document /></el-icon>
                    <span>{{ file.name }}</span>
                    <el-button :icon="Close" size="small" text @click="removeFile(index)" />
                  </div>
                </div>
              </div>
            </el-form-item>
          </el-form>
        </div>

        <div class="mobile-page-footer">
          <el-button @click="handleClose" class="mobile-btn">
            {{ t('common.cancel') }}
          </el-button>
          <el-button type="primary" @click="handleSubmit" :loading="loading" class="mobile-btn">
            {{ t('common.create') }}
          </el-button>
        </div>
      </div>
    </transition>
  </teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { ElMessage } from 'element-plus';
import { ArrowDown, Upload, Close, Document, Plus, Delete } from '@element-plus/icons-vue';
import type { FormInstance, FormRules } from 'element-plus';
import { useTodoStore } from '@/stores';
import { useGroupStore } from '@/stores';
import { useTagStore } from '@/stores';
import type { CreateTodoRequest } from '@/types';
import { isMobile } from '@/utils/device';

const { t } = useI18n();

const props = defineProps<{
  modelValue: boolean;
  currentView?: 'all' | 'todo' | 'today' | 'important' | 'urgent' | 'completed' | 'overdue' | 'group' | 'tag';
  filterGroupId?: string;
  filterTagId?: string;
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
const pendingFiles = ref<Array<{ path: string; name: string }>>([]);
const pendingSteps = ref<Array<{ title: string }>>([]);

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

async function handleSelectFiles() {
  try {
    const { open } = await import('@tauri-apps/plugin-dialog');
    const selected = await open({
      multiple: true,
      title: t('attachment.selectFiles'),
    });

    if (!selected) return;

    const files = Array.isArray(selected) ? selected : [selected];
    for (const filePath of files) {
      if (typeof filePath === 'string') {
        const fileName = filePath.split(/[/\\]/).pop() || 'unknown';
        // 避免重复添加
        if (!pendingFiles.value.some(f => f.path === filePath)) {
          pendingFiles.value.push({ path: filePath, name: fileName });
        }
      }
    }
  } catch (error) {
    console.error('Failed to select files:', error);
    ElMessage.error(t('messages.folderSelectFailed'));
  }
}

function removeFile(index: number) {
  pendingFiles.value.splice(index, 1);
}

function addStep() {
  pendingSteps.value.push({ title: '' });
}

function removeStep(index: number) {
  pendingSteps.value.splice(index, 1);
}

async function handleSubmit() {
  if (!formRef.value) return;

  try {
    await formRef.value.validate();
    loading.value = true;

    const request: CreateTodoRequest = {
      title: form.value.title,
      description: form.value.description || undefined,
      group_id: form.value.group_id,
      // Only pass when value is not undefined, allow null to be passed to backend
      ...(form.value.start_date !== undefined && { start_date: form.value.start_date }),
      ...(form.value.due_date !== undefined && { due_date: form.value.due_date }),
      priority: form.value.priority,
      tag_ids: form.value.tag_ids?.length ? form.value.tag_ids : undefined,
    };

    const newTodo = await todoStore.createTodo(request);

    // Create steps after creating todo
    if (pendingSteps.value.length > 0) {
      for (const step of pendingSteps.value) {
        if (step.title.trim()) {
          try {
            await todoStore.createStep(newTodo.id.toString(), step.title.trim());
          } catch (stepError) {
            console.error('Failed to create step:', step.title, stepError);
          }
        }
      }
    }

    // Upload attachments after creating todo
    if (pendingFiles.value.length > 0) {
      let uploadFailed = false;
      for (const file of pendingFiles.value) {
        try {
          await todoStore.uploadAttachment(newTodo.id.toString(), file.path, file.name);
        } catch (uploadError) {
          console.error('Failed to upload attachment:', file.name, uploadError);
          uploadFailed = true;
          // Show specific error message (including file size limit)
          const errorMsg = uploadError?.toString() || t('attachment.uploadFailed');
          ElMessage.error(`${file.name}: ${errorMsg}`);
        }
      }
      if (uploadFailed) {
        ElMessage.warning(t('attachment.partialUploadFailed'));
      }
    }

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
  pendingFiles.value = [];
  pendingSteps.value = [];
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

      // Set default values based on current view (convert string to number)
      if (props.currentView === 'group' && props.filterGroupId) {
        const groupId = parseInt(props.filterGroupId, 10);
        if (!isNaN(groupId)) {
          form.value.group_id = groupId;
          console.log('[CreateTodoDialog] Set default group_id:', groupId);
        }
      }
      if (props.currentView === 'tag' && props.filterTagId) {
        const tagId = parseInt(props.filterTagId, 10);
        if (!isNaN(tagId)) {
          form.value.tag_ids = [tagId];
          console.log('[CreateTodoDialog] Set default tag_ids:', [tagId]);
        }
      }
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
  border-radius: 6px;
}

.mobile-priority :deep(.el-radio.is-checked) {
  border-color: var(--el-color-primary);
  background: var(--el-color-primary-light-9);
}

.mobile-priority :deep(.el-radio__label) {
  font-size: 13px;
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
  gap: 12px;
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

/* Attachment upload styles */
.attachment-upload {
  display: flex;
  flex-direction: column;
  gap: 12px;
  width: 100%;
}

.pending-files {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-top: 8px;
}

.pending-file {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px;
  background: var(--el-fill-color-light);
  border-radius: 4px;
  font-size: 14px;
}

.pending-file .el-icon {
  font-size: 16px;
  color: var(--el-color-primary);
}

.pending-file span {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* Steps input styles */
.steps-input {
  display: flex;
  flex-direction: column;
  gap: 12px;
  width: 100%;
}

.pending-steps {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-top: 8px;
}

.pending-step {
  display: flex;
  align-items: center;
  gap: 8px;
}

.pending-step .el-input {
  flex: 1;
}
</style>
