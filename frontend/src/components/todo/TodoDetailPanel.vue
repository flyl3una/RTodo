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
          <el-button :icon="Plus" size="small" text @click="startAddingStep">
            {{ t('common.add') }}
          </el-button>
        </div>

        <!-- 新增步骤输入框列表 -->
        <div v-if="pendingSteps.length > 0" class="add-step-inputs">
          <div
            v-for="pendingStep in pendingSteps"
            :key="pendingStep.localId"
            class="add-step-input-item"
          >
            <el-input
              v-model="pendingStep.title"
              :placeholder="t('step.stepPlaceholder')"
              @keyup.enter="confirmPendingStep(pendingStep.localId)"
              @blur="handlePendingStepBlur(pendingStep.localId)"
            />
            <el-button
              :icon="Check"
              size="small"
              type="success"
              @click="confirmPendingStep(pendingStep.localId)"
              class="confirm-btn"
            />
            <el-button
              :icon="Close"
              size="small"
              type="danger"
              @click="removePendingStep(pendingStep.localId)"
              class="cancel-btn"
            />
          </div>
        </div>

        <!-- 本地步骤列表 -->
        <div v-if="activeLocalSteps.length > 0" class="edit-steps-list">
          <div
            v-for="step in activeLocalSteps"
            :key="step.localId || step.id"
            class="edit-step-item"
          >
            <el-checkbox
              :model-value="step.is_completed"
              @change="toggleLocalStep(step)"
            />
            <!-- 内联编辑模式 -->
            <el-input
              v-if="editingStepLocalId === (step.localId || `step-${step.id}`)"
              v-model="editingStepTitle"
              size="small"
              @keyup.enter="saveStepEdit(step)"
              @blur="saveStepEdit(step)"
              @keyup.esc="cancelStepEdit"
              class="step-edit-input"
            />
            <span
              v-else
              class="step-title"
              :class="{ completed: step.is_completed }"
              @dblclick="startEditingStep(step)"
            >
              {{ step.title }}
            </span>
            <el-button
              :icon="Edit"
              size="small"
              text
              @click="startEditingStep(step)"
            />
            <el-button
              :icon="Delete"
              size="small"
              text
              type="danger"
              @click="markDeleteStep(step)"
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
          <el-button :icon="Plus" size="small" text :disabled="loading" @click="selectFilesWithTauri">
            {{ t('common.add') }}
          </el-button>
        </div>
        <div v-if="activeLocalAttachments.length > 0" class="edit-attachments-list">
          <div
            v-for="attachment in activeLocalAttachments"
            :key="attachment.localId || attachment.id"
            class="edit-attachment-item"
            :class="{ 'pending': attachment._pending }"
          >
            <el-icon class="attachment-icon"><Document /></el-icon>
            <span class="attachment-name">{{ attachment.name }}</span>
            <span v-if="attachment._pending" class="pending-tag">{{ t('attachment.pendingUpload') }}</span>
            <span class="attachment-size">{{ formatFileSize(attachment.file_size) }}</span>
            <el-button
              :icon="Delete"
              size="small"
              text
              type="danger"
              @click="markDeleteAttachment(attachment)"
            />
          </div>
        </div>
        <div v-else class="empty-text">{{ t('attachment.noAttachments') }}</div>
      </div>

      <el-form-item>
        <el-button
          type="primary"
          @click="handleSave"
          :loading="loading"
          :disabled="!hasChanges"
        >
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
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch, nextTick } from 'vue';
import { Edit, Delete, Star, StarFilled, Plus, Document, Check, Close } from '@element-plus/icons-vue';
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus';
import { useI18n } from 'vue-i18n';
import { useTodoStore } from '@/stores';
import { useTagStore } from '@/stores';
import { useGroupStore } from '@/stores';
import type { Todo, UpdateTodoRequest, TodoStep, Attachment, LocalStep, LocalAttachment } from '@/types';
import { TodoStatus, getStatusLabel, getStatusType } from '@/types';
import { PRIORITY_VALUES, cyclePriority } from '@/utils/priority-helpers';

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

// 基础状态
const isEditing = ref(false);
const loading = ref(false);
const formRef = ref<FormInstance>();
const isTauri = ref(false);

// 原始数据备份（用于比较和恢复）
const originalSteps = ref<TodoStep[]>([]);
const originalAttachments = ref<Attachment[]>([]);

// 本地编辑数据
const localSteps = ref<LocalStep[]>([]);
const localAttachments = ref<LocalAttachment[]>([]);

// 服务器数据（查看模式使用）
const steps = ref<TodoStep[]>([]);
const attachments = ref<Attachment[]>([]);

// 步骤内联编辑状态
const pendingSteps = ref<Array<{ localId: string; title: string }>>([]);
const editingStepLocalId = ref<string>('');
const editingStepTitle = ref<string>('');

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

// 活跃的本地步骤（排除已删除）
const activeLocalSteps = computed(() =>
  localSteps.value.filter(s => !s._deleted)
);

// 活跃的本地附件（排除已删除）
const activeLocalAttachments = computed(() =>
  localAttachments.value.filter(a => !a._deleted)
);

// 变化检测计算属性
const hasChanges = computed(() => {
  // 检查主任务字段变化
  const mainFieldsChanged =
    form.value.title !== props.todo.title ||
    form.value.description !== (props.todo.description || '') ||
    form.value.status !== props.todo.status ||
    form.value.priority !== props.todo.priority ||
    form.value.group_id !== props.todo.group_id ||
    form.value.start_date !== props.todo.start_date ||
    form.value.due_date !== props.todo.due_date ||
    JSON.stringify(form.value.tag_ids?.sort()) !== JSON.stringify(props.todo.tags?.map(t => t.id).sort() || []);

  // 检查步骤变化
  const activeLocalSteps = localSteps.value.filter(s => !s._deleted);
  const activeOriginalSteps = originalSteps.value;
  const stepsChanged =
    activeLocalSteps.length !== activeOriginalSteps.length ||
    activeLocalSteps.some((localStep, index) => {
      const originalStep = activeOriginalSteps[index];
      if (!originalStep) return true;
      return (
        localStep.title !== originalStep.title ||
        localStep.is_completed !== originalStep.is_completed ||
        localStep._modified
      );
    }) ||
    localSteps.value.some(s => s._deleted || s._modified || !s.id);

  // 检查附件变化
  const activeLocalAttachments = localAttachments.value.filter(a => !a._deleted);
  const activeOriginalAttachments = originalAttachments.value;
  const attachmentsChanged =
    activeLocalAttachments.length !== activeOriginalAttachments.length ||
    localAttachments.value.some(a => a._deleted || a._pending);

  return mainFieldsChanged || stepsChanged || attachmentsChanged;
});

// Watch for todo prop changes
watch(() => props.todo, (newTodo) => {
  console.log('[TodoDetailPanel] Todo prop changed');

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

// 生成临时ID
function generateLocalId(): string {
  return `local-${Date.now()}-${Math.random().toString(36).substring(2, 11)}`;
}

// 进入编辑模式
function startEdit() {
  console.log('[TodoDetailPanel] startEdit called');

  // 初始化主任务表单
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

  // 深拷贝原始数据
  originalSteps.value = JSON.parse(JSON.stringify(steps.value));
  originalAttachments.value = JSON.parse(JSON.stringify(attachments.value));

  // 初始化本地数据
  localSteps.value = steps.value.map(s => ({
    ...s,
    _modified: false,
    _deleted: false,
  }));

  localAttachments.value = attachments.value.map(a => ({
    ...a,
    _deleted: false,
    _pending: false,
  }));

  isEditing.value = true;
}

// 取消编辑
function cancelEdit() {
  // 恢复原始数据
  localSteps.value = originalSteps.value.map(s => ({ ...s }));
  localAttachments.value = originalAttachments.value.map(a => ({ ...a }));

  // 重置编辑状态
  pendingSteps.value = [];
  editingStepLocalId.value = '';
  editingStepTitle.value = '';

  isEditing.value = false;
}

// 保存所有更改
async function handleSave() {
  if (!formRef.value) return;

  try {
    await formRef.value.validate();
    loading.value = true;

    // 处理待定步骤：将所有有待定内容的步骤移入 localSteps
    for (const pendingStep of pendingSteps.value) {
      if (pendingStep.title && pendingStep.title.trim()) {
        localSteps.value.unshift({
          localId: pendingStep.localId,
          title: pendingStep.title.trim(),
          is_completed: false,
          _modified: true,
        });
      }
    }
    pendingSteps.value = [];

    // 1. 保存主任务
    const request: UpdateTodoRequest = {
      id: props.todo.id,
      title: form.value.title,
      description: form.value.description || undefined,
      status: form.value.status,
      priority: form.value.priority,
      group_id: form.value.group_id,
      start_date: form.value.start_date,
      due_date: form.value.due_date,
      tag_ids: form.value.tag_ids?.length ? form.value.tag_ids : undefined,
    };

    const updatedTodo = await todoStore.updateTodo(request);

    // 2. 批量处理步骤变更
    for (const step of localSteps.value) {
      if (step._deleted) {
        // 删除已有步骤
        if (step.id) {
          await todoStore.deleteStep(step.id.toString());
        }
        // 新增步骤标记删除的直接跳过
        continue;
      }

      if (step.id) {
        // 已有步骤 - 检查是否需要更新
        const originalStep = originalSteps.value.find(s => s.id === step.id);
        if (originalStep) {
          if (originalStep.is_completed !== step.is_completed) {
            await todoStore.toggleStep(step.id);
          }
          if (originalStep.title !== step.title) {
            await todoStore.updateStep(step.id.toString(), step.title);
          }
        }
      } else if (step.title && step.title.trim()) {
        // 新增步骤
        await todoStore.createStep(updatedTodo.id, step.title.trim());
      }
    }

    // 3. 批量处理附件变更
    for (const attachment of localAttachments.value) {
      if (attachment._deleted) {
        // 删除已有附件
        if (attachment.id) {
          await todoStore.deleteAttachment(attachment.id);
        }
        // 新增附件标记删除的直接跳过
        continue;
      }

      if (attachment._pending && attachment.file_path) {
        // 上传新附件
        await todoStore.uploadAttachment(updatedTodo.id, attachment.file_path, attachment.name);
      }
    }

    // 4. 重新加载数据
    await loadSteps();
    await loadAttachments();

    ElMessage.success(t('todo.updateSuccess'));

    // 退出编辑模式
    isEditing.value = false;

    // 通知父组件
    emit('updated', updatedTodo);
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

// ============ 步骤操作（编辑模式）============

// 开始添加步骤
function startAddingStep() {
  const newLocalId = generateLocalId();
  pendingSteps.value.push({
    localId: newLocalId,
    title: '',
  });
  nextTick(() => {
    const inputElements = document.querySelectorAll('.add-step-input-item:last-child input');
    (inputElements[0] as HTMLInputElement)?.focus();
  });
}

// 确认添加待定步骤
function confirmPendingStep(localId: string) {
  const index = pendingSteps.value.findIndex(p => p.localId === localId);
  if (index === -1) return;

  const { title } = pendingSteps.value[index];

  if (!title.trim()) {
    removePendingStep(localId);
    return;
  }

  localSteps.value.unshift({
    localId: localId,
    title: title.trim(),
    is_completed: false,
    _modified: true,
  });

  pendingSteps.value.splice(index, 1);
}

// 处理待定步骤失去焦点
function handlePendingStepBlur(localId: string) {
  setTimeout(() => {
    const index = pendingSteps.value.findIndex(p => p.localId === localId);
    if (index === -1) return;

    const { title } = pendingSteps.value[index];
    if (!title.trim()) {
      removePendingStep(localId);
    }
  }, 200);
}

// 移除待定步骤
function removePendingStep(localId: string) {
  const index = pendingSteps.value.findIndex(p => p.localId === localId);
  if (index !== -1) {
    pendingSteps.value.splice(index, 1);
  }
}

// 开始编辑步骤
function startEditingStep(step: LocalStep) {
  const stepLocalId = step.localId || `step-${step.id}`;
  editingStepLocalId.value = stepLocalId;
  editingStepTitle.value = step.title;
}

// 取消编辑步骤
function cancelStepEdit() {
  editingStepLocalId.value = '';
  editingStepTitle.value = '';
}

// 保存步骤编辑
function saveStepEdit(step: LocalStep) {
  if (!editingStepTitle.value.trim()) {
    cancelStepEdit();
    return;
  }

  const index = localSteps.value.findIndex(
    s => (s.localId || `step-${s.id}`) === editingStepLocalId.value
  );

  if (index !== -1) {
    localSteps.value[index].title = editingStepTitle.value.trim();
    localSteps.value[index]._modified = true;
  }

  cancelStepEdit();
}

// 切换本地步骤状态
function toggleLocalStep(step: LocalStep) {
  const index = localSteps.value.findIndex(
    s => (s.localId || `step-${s.id}`) === (step.localId || `step-${step.id}`)
  );

  if (index !== -1) {
    localSteps.value[index].is_completed = !localSteps.value[index].is_completed;
    localSteps.value[index]._modified = true;
  }
}

// 标记删除步骤
function markDeleteStep(step: LocalStep) {
  const index = localSteps.value.findIndex(
    s => (s.localId || `step-${s.id}`) === (step.localId || `step-${step.id}`)
  );

  if (index !== -1) {
    if (step.id) {
      // 已有步骤，标记为删除
      localSteps.value[index]._deleted = true;
    } else {
      // 新步骤，直接移除
      localSteps.value.splice(index, 1);
    }
  }
}

// ============ 附件操作（编辑模式）============

// 使用 Tauri Dialog API 选择文件
async function selectFilesWithTauri() {
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
        await addFileFromPath(filePath);
      }
    }
  } catch (error) {
    console.error('Failed to select files:', error);
    ElMessage.error(t('attachment.selectFilesFailed'));
  }
}

// 从文件路径添加附件
async function addFileFromPath(filePath: string) {
  try {
    // 获取文件信息
    const { stat } = await import('@tauri-apps/plugin-fs');
    const fileStats = await stat(filePath);
    const fileName = filePath.split(/[/\\]/).pop() || 'unknown';

    // 检查文件大小
    const MAX_FILE_SIZE = 50 * 1024 * 1024; // 50MB
    if (fileStats.size > MAX_FILE_SIZE) {
      const sizeMB = (fileStats.size / 1024 / 1024).toFixed(2);
      ElMessage.error(
        t('attachment.fileTooLargeDetailed', { size: sizeMB, maxSize: '50' })
      );
      return;
    }

    // 检查重复文件
    if (localAttachments.value.some(a => a.file_path === filePath && !a._deleted)) {
      ElMessage.warning(t('attachment.fileAlreadyAdded', { name: fileName }));
      return;
    }

    // 创建 LocalAttachment 对象
    const newAttachment = {
      localId: generateLocalId(),
      name: fileName,
      file_path: filePath,
      file_size: fileStats.size,
      _pending: true,
      _deleted: false,
    };

    localAttachments.value = [...localAttachments.value, newAttachment];
    ElMessage.success(t('attachment.fileSelected'));
  } catch (error) {
    console.error('Failed to add file from path:', filePath, error);
    ElMessage.error(t('attachment.addFileFailed'));
  }
}

// 标记删除附件
function markDeleteAttachment(attachment: LocalAttachment) {
  const index = localAttachments.value.findIndex(
    a => (a.localId || a.id) === (attachment.localId || attachment.id)
  );

  if (index !== -1) {
    if (attachment.id) {
      // 已有附件，标记为删除
      localAttachments.value[index]._deleted = true;
    } else {
      // 新附件，直接移除
      localAttachments.value.splice(index, 1);
    }
  }
}

// ============ 查看模式操作 ============

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
    emit('updated', props.todo);
  } catch (error) {
    ElMessage.error(t('step.stepStatusUpdateFailed'));
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

function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

async function loadSteps() {
  if (!props.todo?.id) return;
  try {
    steps.value = await todoStore.fetchSteps(props.todo.id);
  } catch (error) {
    console.error('Failed to load steps:', error);
  }
}

async function loadAttachments() {
  if (!props.todo?.id) return;
  try {
    attachments.value = await todoStore.fetchAttachments(props.todo.id);
  } catch (error) {
    console.error('Failed to load attachments:', error);
  }
}

onMounted(async () => {
  isTauri.value = window.__TAURI__ !== undefined;
  await groupStore.fetchGroups();
  await tagStore.fetchTags();
  await loadSteps();
  await loadAttachments();
});

// Watch for editMode prop changes
watch(() => props.editMode, (newEditMode) => {
  if (newEditMode && !isEditing.value) {
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
  cursor: text;
}

.step-title.completed {
  text-decoration: line-through;
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

.add-step-input {
  margin-bottom: 12px;
}

.add-step-inputs {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 12px;
}

.add-step-input-item {
  display: flex;
  align-items: center;
  gap: 8px;
}

.add-step-input-item .el-input {
  flex: 1;
}

.confirm-btn,
.cancel-btn {
  flex-shrink: 0;
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

.edit-attachment-item.pending {
  border-left: 3px solid var(--el-color-warning);
  background: var(--el-fill-color-lighter);
}

.step-edit-input {
  flex: 1;
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

.pending-tag {
  font-size: 12px;
  padding: 2px 6px;
  background: var(--el-color-warning);
  color: white;
  border-radius: 4px;
}

.attachment-icon {
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

.empty-text {
  text-align: center;
  color: var(--el-text-color-secondary);
  font-size: 14px;
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
</style>
