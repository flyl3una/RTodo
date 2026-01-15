<template>
  <div class="todo-detail-panel">
    <el-form
      v-if="isEditing"
      ref="formRef"
      :model="form"
      :rules="rules"
      label-width="80px"
    >
      <el-form-item label="标题" prop="title">
        <el-input v-model="form.title" placeholder="请输入任务标题" />
      </el-form-item>

      <el-form-item label="描述">
        <el-input
          v-model="form.description"
          type="textarea"
          :rows="3"
          placeholder="请输入任务描述"
        />
      </el-form-item>

      <el-form-item label="状态">
        <el-select v-model="form.status" style="width: 100%">
          <el-option label="待办" :value="TodoStatus.Todo" />
          <el-option label="进行中" :value="TodoStatus.InProgress" />
          <el-option label="已完成" :value="TodoStatus.Done" />
        </el-select>
      </el-form-item>

      <el-form-item label="任务组">
        <el-select
          v-model="form.group_id"
          placeholder="选择任务组"
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

      <el-form-item label="优先级">
        <el-radio-group v-model="form.priority">
          <el-radio :label="0">普通</el-radio>
          <el-radio :label="1">重要</el-radio>
          <el-radio :label="3">紧急</el-radio>
        </el-radio-group>
      </el-form-item>

      <el-form-item label="开始时间">
        <el-date-picker
          v-model="form.start_date"
          type="datetime"
          placeholder="选择开始时间"
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
          placeholder="选择截止时间"
          format="YYYY-MM-DD HH:mm"
          value-format="x"
          :clearable="true"
          size="default"
          style="width: 100%"
        />
      </el-form-item>

      <el-form-item label="标签">
        <el-select
          v-model="form.tag_ids"
          multiple
          placeholder="选择标签"
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

      <el-form-item>
        <el-button type="primary" @click="handleSave" :loading="loading">
          保存
        </el-button>
        <el-button @click="cancelEdit">取消</el-button>
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
        <h4 class="section-title">描述</h4>
        <p class="section-content">{{ todo.description }}</p>
      </div>

      <div class="detail-section">
        <h4 class="section-title">时间</h4>
        <div class="time-info">
          <div v-if="displayStartDate">
            <span class="time-label">开始:</span>
            <span>{{ formatDate(displayStartDate) }}</span>
          </div>
          <div v-if="displayDueDate">
            <span class="time-label">截止:</span>
            <span>{{ formatDate(displayDueDate) }}</span>
          </div>
        </div>
      </div>

      <div v-if="todo.tags && todo.tags.length > 0" class="detail-section">
        <h4 class="section-title">标签</h4>
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
          <h4 class="section-title">执行步骤</h4>
          <el-button
            :icon="Plus"
            size="small"
            text
            @click="showAddStep = true"
          >
            添加
          </el-button>
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
            <el-button
              :icon="Delete"
              size="small"
              text
              type="danger"
              @click="deleteStep(step.id)"
            />
          </div>
        </div>
        <el-empty v-else description="暂无执行步骤" :image-size="60" />
      </div>
    </div>

    <!-- Add Step Dialog -->
    <el-dialog
      v-model="showAddStep"
      title="添加执行步骤"
      width="500px"
    >
      <el-input
        v-model="newStepTitle"
        placeholder="请输入步骤标题"
        @keyup.enter="addStep"
      />
      <template #footer>
        <el-button @click="showAddStep = false">取消</el-button>
        <el-button type="primary" @click="addStep" :disabled="!newStepTitle.trim()">
          添加
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { Edit, Delete, Star, StarFilled, Plus } from '@element-plus/icons-vue';
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus';
import { useTodoStore } from '@/stores';
import { useTagStore } from '@/stores';
import { useGroupStore } from '@/stores';
import type { Todo, UpdateTodoRequest, TodoStep } from '@/types';
import { TodoStatus, getStatusLabel, getStatusType } from '@/types';

const props = defineProps<{
  todo: Todo;
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
const steps = ref<TodoStep[]>([]);

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
    { required: true, message: '请输入任务标题', trigger: 'blur' },
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
    case 3: return 'danger';
    case 1: return 'warning';
    default: return '';
  }
});

const priorityText = computed(() => {
  switch (props.todo.priority) {
    case 3: return '紧急';
    case 1: return '重要';
    default: return '普通';
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
  console.log('[TodoDetailPanel] props.todo.start_date:', props.todo.start_date);
  console.log('[TodoDetailPanel] props.todo.due_date:', props.todo.due_date);

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

  console.log('[TodoDetailPanel] form.value after init:', {
    start_date: form.value.start_date,
    due_date: form.value.due_date
  });

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

    ElMessage.success('保存成功');

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
    ElMessage.error(`保存失败: ${error}`);
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
    ElMessage.error('状态切换失败');
  }
}

async function handleMarkToggle() {
  try {
    // Cycle through priorities: 0 (Normal) → 1 (Important) → 3 (Urgent) → 0 (Normal)
    const newPriority = props.todo.priority === 0 ? 1 : (props.todo.priority === 1 ? 3 : 0);
    await todoStore.updateTodo({
      id: props.todo.id,
      priority: newPriority,
    });
    emit('updated', { ...props.todo, priority: newPriority });
  } catch (error) {
    ElMessage.error('优先级切换失败');
  }
}

async function handleDelete() {
  try {
    await ElMessageBox.confirm('确定要删除这个任务吗？', '删除任务', {
      type: 'warning',
      confirmButtonText: '删除',
      cancelButtonText: '取消',
    });

    await todoStore.deleteTodo(props.todo.id);
    ElMessage.success('删除成功');
    emit('deleted');
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败');
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
  } catch (error) {
    ElMessage.error('步骤状态更新失败');
  }
}

async function addStep() {
  if (!newStepTitle.value.trim()) return;

  try {
    await todoStore.createStep(props.todo.id, newStepTitle.value);
    ElMessage.success('步骤添加成功');
    newStepTitle.value = '';
    showAddStep.value = false;
    await loadSteps();
  } catch (error) {
    ElMessage.error('添加步骤失败');
  }
}

async function deleteStep(stepId: string) {
  try {
    await ElMessageBox.confirm('确定要删除这个步骤吗？', '删除步骤', {
      type: 'warning',
      confirmButtonText: '删除',
      cancelButtonText: '取消',
    });

    await todoStore.deleteStep(stepId);
    ElMessage.success('删除成功');
    await loadSteps();
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败');
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

onMounted(async () => {
  await groupStore.fetchGroups();
  await tagStore.fetchTags();
  await loadSteps();
});
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
  color: #303133;
}

.detail-meta {
  display: flex;
  gap: 8px;
}

.detail-section {
  padding: 16px 0;
  border-top: 1px solid #e4e7ed;
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
  color: #606266;
}

.section-content {
  margin: 0;
  color: #606266;
  line-height: 1.6;
  white-space: pre-wrap;
}

.time-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
  font-size: 14px;
  color: #606266;
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
  background: #f5f7fa;
  border-radius: 4px;
}

.step-title {
  flex: 1;
  font-size: 14px;
  color: #303133;
}

.step-title.completed {
  text-decoration: line-through;
  color: #909399;
}

/* Dark theme */
[data-theme='dark'] .detail-title {
  color: #e0e0e0;
}

[data-theme='dark'] .section-title {
  color: #a0a0a0;
}

[data-theme='dark'] .section-content {
  color: #b0b0b0;
}

[data-theme='dark'] .detail-section {
  border-top-color: #3a3a3a;
}

[data-theme='dark'] .time-info {
  color: #b0b0b0;
}

[data-theme='dark'] .step-item {
  background: #2a2a2a;
}

[data-theme='dark'] .step-title {
  color: #e0e0e0;
}

[data-theme='dark'] .step-title.completed {
  color: #707070;
}
</style>
