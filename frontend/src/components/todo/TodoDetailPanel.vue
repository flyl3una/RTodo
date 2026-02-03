<template>
  <div class="todo-detail-panel">
    <el-form
      v-if="isEditing"
      ref="formRef"
      :model="form"
      :rules="rules"
      label-width="80px"
    >
      <el-form-item :label="t('common.title')" prop="title">
        <el-input v-model="form.title" :placeholder="t('todo.titlePlaceholder')" />
      </el-form-item>

      <el-form-item :label="t('common.description')">
        <el-input
          v-model="form.description"
          type="textarea"
          :rows="6"
          :placeholder="t('todo.descriptionPlaceholder')"
        />
      </el-form-item>

      <el-form-item :label="t('common.status')">
        <el-select v-model="form.status" style="width: 100%">
          <el-option :label="t('status.todo')" :value="TodoStatus.Todo" />
          <el-option :label="t('status.inProgress')" :value="TodoStatus.InProgress" />
          <el-option :label="t('status.done')" :value="TodoStatus.Done" />
        </el-select>
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

      <el-form-item :label="t('common.priority')">
        <el-radio-group v-model="form.priority">
          <el-radio :label="PRIORITY_VALUES.NORMAL">{{ t('priority.normal') }}</el-radio>
          <el-radio :label="PRIORITY_VALUES.IMPORTANT">{{ t('priority.important') }}</el-radio>
          <el-radio :label="PRIORITY_VALUES.URGENT">{{ t('priority.urgent') }}</el-radio>
        </el-radio-group>
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

      <el-form-item :label="t('tag.tags')">
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

      <!-- 子步骤编辑区域 -->
      <el-divider />
      <div class="edit-section">
        <div class="section-header">
          <h4>{{ t('step.title') }}</h4>
          <el-button :icon="Plus" size="small" text @click="showAddStep = true">
            {{ t('common.add') }}
          </el-button>
        </div>
        <div v-if="steps.length > 0" class="edit-steps-list">
          <div v-for="step in steps" :key="step.id" class="edit-step-item">
            <el-checkbox :model-value="step.is_completed" @change="toggleStep(step)" />
            <span class="step-title" :class="{ completed: step.is_completed }">
              {{ step.title }}
            </span>
            <el-button
              :icon="Edit"
              size="small"
              text
              @click="editStep(step)"
            />
            <el-button
              :icon="Delete"
              size="small"
              text
              type="danger"
              @click="deleteStep(step.id)"
            />
          </div>
        </div>
        <div v-else class="empty-text">{{ t('step.noSteps') }}</div>
      </div>

      <!-- 附件编辑区域 -->
      <el-divider />
      <div class="edit-section">
        <div class="section-header">
          <h4>{{ t('attachment.attachments') }}</h4>
          <el-button :icon="Plus" size="small" text @click="handleAddAttachment">
            {{ t('common.add') }}
          </el-button>
        </div>
        <div v-if="attachments.length > 0" class="edit-attachments-list">
          <div
            v-for="attachment in attachments"
            :key="attachment.id"
            class="edit-attachment-item"
          >
            <el-icon class="attachment-icon"><Document /></el-icon>
            <span class="attachment-name">{{ attachment.name }}</span>
            <span class="attachment-size">{{ formatFileSize(attachment.file_size) }}</span>
            <el-button
              :icon="Delete"
              size="small"
              text
              type="danger"
              @click="handleDeleteAttachment(attachment)"
            />
          </div>
        </div>
        <div v-else class="empty-text">{{ t('attachment.noAttachments') }}</div>
      </div>

      <el-form-item>
        <el-button type="primary" @click="handleSave" :loading="loading">
          {{ t('common.save') }}
        </el-button>
        <el-button @click="cancelEdit">{{ t('common.cancel') }}</el-button>
      </el-form-item>
    </el-form>

    <div v-else class="detail-view">
      <div class="detail-header">
        <el-checkbox
          :model-value="todo.status === TodoStatus.Done"
          @change="handleStatusToggle"
        />
        <h2 class="detail-title">{{ todo.title }}</h2>
        <el-button
          :icon="todo.priority >= 1 ? StarFilled : Star"
          circle
          text
          @click="handleMarkToggle"
        />
        <el-button :icon="Edit" circle text @click="startEdit" />
        <el-button :icon="Delete" circle text type="danger" @click="handleDelete" />
      </div>

      <div class="detail-meta">
        <el-tag :type="statusType" size="small">
          {{ statusText }}
        </el-tag>
        <el-tag v-if="todo.priority > 0" :type="priorityType" size="small">
          {{ priorityText }}
        </el-tag>
      </div>

      <div v-if="todo.description" class="detail-section">
        <h4 class="section-title">{{ t('common.description') }}</h4>
        <p class="section-content">{{ todo.description }}</p>
      </div>

      <div class="detail-section">
        <h4 class="section-title">{{ t('todo.time') }}</h4>
        <div class="time-info">
          <div v-if="displayStartDate">
            <span class="time-label">{{ t('todo.start') }}:</span>
            <span>{{ formatDate(displayStartDate) }}</span>
          </div>
          <div v-if="displayDueDate">
            <span class="time-label">{{ t('todo.due') }}:</span>
            <span>{{ formatDate(displayDueDate) }}</span>
          </div>
        </div>
      </div>

      <div v-if="todo.tags && todo.tags.length > 0" class="detail-section">
        <h4 class="section-title">{{ t('tag.tags') }}</h4>
        <div class="tags-list">
          <el-tag
            v-for="tag in todo.tags"
            :key="tag.id"
            :style="{ backgroundColor: tag.color }"
            size="small"
          >
            {{ tag.name }}
          </el-tag>
        </div>
      </div>

      <div class="detail-section">
        <div class="section-header">
          <h4 class="section-title">{{ t('step.title') }}</h4>
        </div>
        <div v-if="steps.length > 0" class="steps-list">
          <div
            v-for="step in steps"
            :key="step.id"
            class="step-item"
          >
            <el-checkbox
              :model-value="step.is_completed"
              @change="toggleStep(step)"
            />
            <span
              class="step-title"
              :class="{ completed: step.is_completed }"
            >
              {{ step.title }}
            </span>
          </div>
        </div>
        <el-empty v-else :description="t('step.noSteps')" :image-size="60" />
      </div>

      <!-- 附件区域 -->
      <div class="detail-section">
        <div class="section-header">
          <h4 class="section-title">{{ t('attachment.attachments') }}</h4>
        </div>
        <div v-if="attachments.length > 0" class="attachments-list">
          <div
            v-for="attachment in attachments"
            :key="attachment.id"
            class="attachment-item"
          >
            <el-icon class="attachment-icon"><Document /></el-icon>
            <span
              class="attachment-name"
              @click="handleOpenAttachment(attachment)"
            >
              {{ attachment.name }}
            </span>
            <span class="attachment-size">{{ formatFileSize(attachment.file_size) }}</span>
          </div>
        </div>
        <el-empty v-else :description="t('attachment.noAttachments')" :image-size="60" />
      </div>
    </div>

    <!-- Add/Edit Step Dialog -->
    <el-dialog
      v-model="showAddStep"
      :title="editingStep ? t('step.editStep') : t('step.addStep')"
      width="500px"
    >
      <el-input
        v-model="newStepTitle"
        :placeholder="t('step.stepPlaceholder')"
        @keyup.enter="saveStep"
      />
      <template #footer>
        <el-button @click="showAddStep = false">{{ t('common.cancel') }}</el-button>
        <el-button type="primary" @click="saveStep" :disabled="!newStepTitle.trim()">
          {{ editingStep ? t('common.save') : t('common.add') }}
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { Edit, Delete, Star, StarFilled, Plus, Document, Download } from '@element-plus/icons-vue';
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus';
import { useI18n } from 'vue-i18n';
import { useTodoStore } from '@/stores';
import { useTagStore } from '@/stores';
import { useGroupStore } from '@/stores';
import type { Todo, UpdateTodoRequest, TodoStep, Attachment } from '@/types';
import { TodoStatus, getStatusLabel, getStatusType } from '@/types';
import { PRIORITY_VALUES, cyclePriority, isMarked } from '@/utils/priority-helpers';

const { t } = useI18n();

const props = defineProps<{
  todo: Todo;
  editMode?: boolean;
}>();

const emit = defineEmits<{
  updated: [todo: Todo];
  deleted: [];
}>();

const todoStore = useTodoStore();
const tagStore = useTagStore();
const groupStore = useGroupStore();

const isEditing = ref(false);
const loading = ref(false);
const formRef = ref<FormInstance>();
const showAddStep = ref(false);
const newStepTitle = ref('');
const editingStep = ref<TodoStep | null>(null);
const steps = ref<TodoStep[]>([]);
const attachments = ref<Attachment[]>([]);

const form = ref<UpdateTodoRequest>({
  title: '',
  description: '',
  status: TodoStatus.Todo,
  priority: 0,
  group_id: undefined,
  start_date: undefined,
  due_date: undefined,
  tag_ids: [],
});

const rules: FormRules = {
  title: [
    { required: true, message: t('todo.titleRequired'), trigger: 'blur' },
  ],
};

const tags = computed(() => tagStore.tags);
const groups = computed(() => groupStore.groups);

// Watch for todo prop changes (when store updates the todo)
watch(() => props.todo, (newTodo, oldTodo) => {
  console.log('[TodoDetailPanel] Todo prop changed');
  console.log('[TodoDetailPanel] New todo start_date:', newTodo.start_date, 'due_date:', newTodo.due_date);
  console.log('[TodoDetailPanel] Old todo start_date:', oldTodo?.start_date, 'due_date:', oldTodo?.due_date);

  // If currently editing, sync the form with new data
  if (isEditing.value) {
    console.log('[TodoDetailPanel] Syncing form with new data');
    form.value = {
      title: newTodo.title,
      description: newTodo.description || '',
      status: newTodo.status,
      priority: newTodo.priority,
      group_id: newTodo.group_id,
      start_date: newTodo.start_date,
      due_date: newTodo.due_date,
      tag_ids: newTodo.tags?.map(t => t.id) || [],
    };
  } else {
    // Force re-render computed properties by accessing them
    console.log('[TodoDetailPanel] Triggering reactivity for display');
    console.log('[TodoDetailPanel] displayStartDate will be:', displayStartDate.value);
    console.log('[TodoDetailPanel] displayDueDate will be:', displayDueDate.value);
  }
}, { deep: true });

const statusType = computed(() => getStatusType(props.todo.status));

const statusText = computed(() => getStatusLabel(props.todo.status));

const priorityType = computed(() => {
  switch (props.todo.priority) {
    case PRIORITY_VALUES.URGENT: return 'danger';
    case PRIORITY_VALUES.IMPORTANT: return 'warning';
    default: return '';
  }
});

const priorityText = computed(() => {
  switch (props.todo.priority) {
    case PRIORITY_VALUES.URGENT: return t('priority.urgent');
    case PRIORITY_VALUES.IMPORTANT: return t('priority.important');
    default: return t('priority.normal');
  }
});

// Computed properties to ensure reactivity for time display
const displayStartDate = computed(() => props.todo.start_date);
const displayDueDate = computed(() => props.todo.due_date);

function formatDate(timestamp?: number): string {
  if (!timestamp) return '-';
  const date = new Date(timestamp);
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  });
}

function startEdit() {
  console.log('[TodoDetailPanel] startEdit called');
  console.log('[TodoDetailPanel] props.todo:', props.todo);

  // 使用现有的 props 数据初始化表单，因为详情面板已经显示了完整的数据
  form.value = {
    title: props.todo.title,
    description: props.todo.description || '',
    status: props.todo.status,
    priority: props.todo.priority,
    group_id: props.todo.group_id,
    start_date: props.todo.start_date,
    due_date: props.todo.due_date,
    tag_ids: props.todo.tags?.map(t => t.id) || [],
  };

  console.log('[TodoDetailPanel] Form initialized:', {
    title: form.value.title,
    group_id: form.value.group_id,
    start_date: form.value.start_date,
    due_date: form.value.due_date,
    tag_ids: form.value.tag_ids,
  });

  // 进入编辑模式
  isEditing.value = true;
}

function cancelEdit() {
  isEditing.value = false;
}

async function handleSave() {
  if (!formRef.value) return;

  try {
    await formRef.value.validate();
    loading.value = true;

    console.log('[TodoDetailPanel] handleSave - form.value:', {
      start_date: form.value.start_date,
      due_date: form.value.due_date,
      start_date_type: typeof form.value.start_date,
      due_date_type: typeof form.value.due_date
    });

    // 构建请求对象
    // 对于日期字段：
    // - undefined: 不传递该字段（不更新）
    // - null: 传递 null（清除日期）
    // - 数字: 传递数字（设置日期）
    const request: UpdateTodoRequest = {
      id: props.todo.id,
      title: form.value.title,
      description: form.value.description || undefined,
      status: form.value.status,
      priority: form.value.priority,
      group_id: form.value.group_id,
      // 保留原始值（包括 null），让 API 层决定如何处理
      start_date: form.value.start_date,
      due_date: form.value.due_date,
      tag_ids: form.value.tag_ids?.length ? form.value.tag_ids : undefined,
    };

    console.log('[TodoDetailPanel] handleSave - request:', {
      start_date: request.start_date,
      start_date_type: typeof request.start_date,
      due_date: request.due_date,
      due_date_type: typeof request.due_date
    });

    console.log('Saving todo with request:', request);

    const updated = await todoStore.updateTodo(request);

    console.log('Updated todo received:', updated);
    console.log('Updated start_date:', updated.start_date, 'due_date:', updated.due_date);

    ElMessage.success(t('todo.updateSuccess'));

    // 先退出编辑模式
    isEditing.value = false;

    // 通知父组件
    emit('updated', updated);

    console.log('[TodoDetailPanel] Save completed, isEditing:', isEditing.value);
  } catch (error: any) {
    console.error('[TodoDetailPanel] Save error:', error);
    if (error?.errors) {
      return;
    }
    ElMessage.error(`${t('todo.createFailed')}: ${error}`);
  } finally {
    loading.value = false;
  }
}

async function handleStatusToggle() {
  try {
    const newStatus = props.todo.status === TodoStatus.Done ? TodoStatus.Todo : TodoStatus.Done;
    await todoStore.updateTodoStatus(props.todo.id, newStatus);
    emit('updated', { ...props.todo, status: newStatus });
  } catch (error) {
    ElMessage.error(t('todo.statusUpdateFailed'));
  }
}

async function handleMarkToggle() {
  try {
    const newPriority = cyclePriority(props.todo.priority);
    await todoStore.updateTodo({
      id: props.todo.id,
      priority: newPriority,
    });
    emit('updated', { ...props.todo, priority: newPriority });
  } catch (error) {
    ElMessage.error(t('todo.priorityUpdateFailed'));
  }
}

async function handleDelete() {
  try {
    await ElMessageBox.confirm(t('todo.deleteConfirm', { title: props.todo.title }), t('todo.deleteTodo'), {
      type: 'warning',
      confirmButtonText: t('common.delete'),
      cancelButtonText: t('common.cancel'),
    });

    await todoStore.deleteTodo(props.todo.id);
    ElMessage.success(t('todo.deleteSuccess'));
    emit('deleted');
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error(t('todo.deleteFailed'));
    }
  }
}

async function toggleStep(step: TodoStep) {
  try {
    const updated = await todoStore.toggleStep(step.id);
    const updatedSteps = steps.value.map(s =>
      s.id === step.id ? updated : s
    );
    steps.value = updatedSteps;
    // 通知父组件刷新数据
    emit('updated', props.todo);
  } catch (error) {
    ElMessage.error(t('step.stepStatusUpdateFailed'));
  }
}

async function saveStep() {
  if (!newStepTitle.value.trim()) return;

  try {
    if (editingStep.value) {
      // 编辑模式 - 使用更新 API
      const updated = await todoStore.updateStep(editingStep.value.id.toString(), newStepTitle.value);
      // 更新本地状态
      const index = steps.value.findIndex(s => s.id === editingStep.value!.id);
      if (index !== -1) {
        steps.value[index] = updated;
      }
      ElMessage.success(t('step.stepUpdated'));
    } else {
      // 新增模式
      await todoStore.createStep(props.todo.id, newStepTitle.value);
      ElMessage.success(t('step.stepCreated'));
      await loadSteps();
    }
    newStepTitle.value = '';
    editingStep.value = null;
    showAddStep.value = false;
    // 通知父组件刷新数据
    emit('updated', props.todo);
  } catch (error) {
    ElMessage.error(editingStep.value ? t('step.updateStepFailed') : t('step.addStepFailed'));
  }
}

function editStep(step: TodoStep) {
  editingStep.value = step;
  newStepTitle.value = step.title;
  showAddStep.value = true;
}

async function deleteStep(stepId: string) {
  try {
    await ElMessageBox.confirm(t('step.deleteConfirm'), t('step.deleteStep'), {
      type: 'warning',
      confirmButtonText: t('common.delete'),
      cancelButtonText: t('common.cancel'),
    });

    await todoStore.deleteStep(stepId);
    ElMessage.success(t('step.deleteSuccess'));
    await loadSteps();
    // 通知父组件刷新数据
    emit('updated', props.todo);
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error(t('step.deleteStepFailed'));
    }
  }
}

async function loadSteps() {
  console.log('loadSteps called, props.todo:', props.todo);
  if (!props.todo?.id) {
    console.warn('No todo id available for loading steps, skipping API call');
    return;
  }
  try {
    console.log('Fetching steps for todo id:', props.todo.id);
    steps.value = await todoStore.fetchSteps(props.todo.id);
    console.log('Steps loaded successfully:', steps.value.length);
  } catch (error) {
    console.error('Failed to load steps:', error);
  }
}

async function loadAttachments() {
  if (!props.todo?.id) {
    console.warn('No todo id available for loading attachments, skipping API call');
    return;
  }
  try {
    attachments.value = await todoStore.fetchAttachments(props.todo.id);
    console.log('Attachments loaded successfully:', attachments.value.length);
  } catch (error) {
    console.error('Failed to load attachments:', error);
  }
}

async function handleAddAttachment() {
  try {
    const { open } = await import('@tauri-apps/plugin-dialog');
    const selected = await open({
      multiple: false,
      title: t('attachment.selectFiles'),
    });

    if (!selected || typeof selected !== 'string') {
      return;
    }

    const fileName = selected.split(/[/\\]/).pop() || 'unknown';
    const newAttachment = await todoStore.uploadAttachment(props.todo.id, selected, fileName);
    attachments.value.push(newAttachment);
    ElMessage.success(t('attachment.uploadSuccess'));
    // 通知父组件刷新数据
    emit('updated', props.todo);
  } catch (error) {
    console.error('Failed to add attachment:', error);
    // 显示具体的错误信息
    const errorMsg = error?.toString() || t('attachment.uploadFailed');
    ElMessage.error(errorMsg);
  }
}

async function handleDeleteAttachment(attachment: Attachment) {
  try {
    await ElMessageBox.confirm(
      t('attachment.deleteConfirm', { name: attachment.name }),
      t('attachment.deleteAttachment'),
      {
        type: 'warning',
        confirmButtonText: t('common.delete'),
        cancelButtonText: t('common.cancel'),
      }
    );

    await todoStore.deleteAttachment(attachment.id);
    attachments.value = attachments.value.filter(a => a.id !== attachment.id);
    ElMessage.success(t('attachment.deleteSuccess'));
    // 通知父组件刷新数据
    emit('updated', props.todo);
  } catch (error) {
    if (error !== 'cancel') {
      console.error('Failed to delete attachment:', error);
      ElMessage.error(t('attachment.deleteFailed'));
    }
  }
}

async function handleOpenAttachment(attachment: Attachment) {
  try {
    await todoStore.openAttachment(attachment.id);
  } catch (error) {
    console.error('Failed to open attachment:', error);
    ElMessage.error(t('attachment.openFailed'));
  }
}

async function handleDownloadAttachment(attachment: Attachment) {
  try {
    const { save } = await import('@tauri-apps/plugin-dialog');
    const filePath = await save({
      defaultPath: attachment.name,
      title: t('attachment.saveAs'),
    });

    if (!filePath || typeof filePath !== 'string') {
      return;
    }

    // 复制文件到用户选择的位置
    await todoStore.downloadAttachment(attachment.id, filePath);
    ElMessage.success(t('attachment.downloadSuccess'));
  } catch (error) {
    console.error('Failed to download attachment:', error);
    ElMessage.error(t('attachment.downloadFailed'));
  }
}

function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

onMounted(async () => {
  await groupStore.fetchGroups();
  await tagStore.fetchTags();
  await loadSteps();
  await loadAttachments();
});

// Watch for editMode prop changes
watch(() => props.editMode, (newEditMode) => {
  if (newEditMode && !isEditing.value) {
    // 确保todo数据存在后再初始化表单
    if (props.todo?.id) {
      startEdit();
    }
  } else if (!newEditMode && isEditing.value) {
    isEditing.value = false;
  }
}, { immediate: true });
</script>

<style scoped>
.todo-detail-panel {
  padding: 0;
}

.detail-view {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.detail-header {
  display: flex;
  align-items: center;
  gap: 12px;
}

.detail-title {
  flex: 1;
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.detail-meta {
  display: flex;
  gap: 8px;
}

.detail-section {
  padding: 16px 0;
  border-top: 1px solid var(--el-border-color-light);
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.section-title {
  margin: 0 0 12px;
  font-size: 14px;
  font-weight: 600;
  color: var(--el-text-color-regular);
}

.section-content {
  margin: 0;
  color: var(--el-text-color-regular);
  line-height: 1.6;
  white-space: pre-wrap;
}

.time-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
  font-size: 14px;
  color: var(--el-text-color-regular);
}

.time-label {
  font-weight: 500;
}

.tags-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.steps-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.step-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px;
  background: var(--el-fill-color-light);
  border-radius: 4px;
}

.step-title {
  flex: 1;
  font-size: 14px;
  color: var(--el-text-color-primary);
}

.step-title.completed {
  text-decoration: line-through;
  color: var(--el-text-color-secondary);
}

/* Dark theme */
:global(html.dark) .detail-title {
  color: var(--el-text-color-primary);
}

:global(html.dark) .section-title {
  color: var(--el-text-color-secondary);
}

:global(html.dark) .section-content {
  color: var(--el-text-color-regular);
}

:global(html.dark) .detail-section {
  border-top-color: var(--el-border-color);
}

:global(html.dark) .time-info {
  color: var(--el-text-color-regular);
}

:global(html.dark) .step-item {
  background: var(--el-fill-color-light);
}

:global(html.dark) .step-title {
  color: var(--el-text-color-primary);
}

:global(html.dark) .step-title.completed {
  color: var(--el-text-color-secondary);
}

.attachments-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.attachment-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px;
  background: var(--el-fill-color-light);
  border-radius: 4px;
  transition: background-color 0.2s;
}

.attachment-item:hover {
  background: var(--el-fill-color);
}

.attachment-icon {
  font-size: 18px;
  color: var(--el-color-primary);
}

.attachment-name {
  flex: 1;
  font-size: 14px;
  color: var(--el-text-color-primary);
  cursor: pointer;
  text-decoration: underline;
  text-decoration-style: dotted;
}

.attachment-name:hover {
  color: var(--el-color-primary);
}

.attachment-size {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

/* 编辑模式样式 */
.edit-section {
  margin-bottom: 16px;
}

.edit-section .section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.edit-section h4 {
  margin: 0;
  font-size: 14px;
  font-weight: 500;
  color: var(--el-text-color-primary);
}

.edit-steps-list,
.edit-attachments-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.edit-step-item,
.edit-attachment-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px;
  background: var(--el-fill-color-light);
  border-radius: 4px;
}

.edit-step-item .step-title {
  flex: 1;
  font-size: 14px;
  color: var(--el-text-color-primary);
}

.edit-step-item .step-title.completed {
  text-decoration: line-through;
  color: var(--el-text-color-secondary);
}

.edit-attachment-item .attachment-icon {
  font-size: 16px;
  color: var(--el-color-primary);
}

.edit-attachment-item .attachment-name {
  flex: 1;
  font-size: 14px;
  color: var(--el-text-color-primary);
}

.edit-attachment-item .attachment-size {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.empty-text {
  text-align: center;
  color: var(--el-text-color-secondary);
  font-size: 14px;
}
</style>
