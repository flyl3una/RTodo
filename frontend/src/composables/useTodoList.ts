import { ref, computed, toRef, watch } from 'vue';
import { ElMessageBox, ElMessage } from 'element-plus';
import { useI18n } from 'vue-i18n';
import { useTodoStore } from '@/stores';
import { useUIStore } from '@/stores';
import { useGroupStore } from '@/stores';
import type { Todo, TodoStep } from '@/types';
import { TodoStatus, getPriorityLabel, getPriorityType, isTodoOverdue } from '@/types';
import * as api from '@/api/tauri';
import { isMobile } from '@/utils/device';

export function useTodoList() {
  const { t } = useI18n();

  const todoStore = useTodoStore();
  const uiStore = useUIStore();
  const groupStore = useGroupStore();

  const loading = computed(() => todoStore.loading);
  const filteredTodos = computed(() => todoStore.filteredTodos);
  const viewMode = toRef(uiStore, 'viewMode');

  const selectedTodo = ref<Todo | null>(null);
  const detailVisible = ref(false);
  const expandedTodos = ref<Set<number>>(new Set());
  const editMode = ref(false);

  // Watch for store changes and update selectedTodo if it's the same todo
  watch(() => todoStore.todos, (newTodos) => {
    console.log('[useTodoList] todoStore.todos changed, length:', newTodos.length);
    if (selectedTodo.value) {
      const updatedTodo = newTodos.find(t => t.id === selectedTodo.value!.id);
      console.log('[useTodoList] Looking for todo with id:', selectedTodo.value!.id, 'found:', !!updatedTodo);
      if (updatedTodo) {
        console.log('[useTodoList] Syncing selectedTodo with store data');
        selectedTodo.value = updatedTodo;
      }
    }
  }, { deep: true });

  async function toggleStatus(todo: Todo) {
    try {
      const newStatus = todo.status === TodoStatus.Done ? TodoStatus.Todo : TodoStatus.Done;
      console.log('toggleStatus called:', { todoId: todo.id, oldStatus: todo.status, newStatus });
      await todoStore.updateTodoStatus(todo.id, newStatus);
    } catch (error: any) {
      console.error('Failed to toggle status:', error);
      const errorMsg = error?.toString() || JSON.stringify(error) || 'Unknown error';
      ElMessage.error(`${t('todo.statusUpdateFailed')}: ${errorMsg}`);
    }
  }

  async function toggleMark(todo: Todo) {
    try {
      const newPriority = todo.priority === 0 ? 1 : (todo.priority === 1 ? 3 : 0);
      await todoStore.updateTodo({
        id: todo.id,
        priority: newPriority,
      });
    } catch (error) {
      console.error('Failed to toggle priority:', error);
      ElMessage.error(t('todo.priorityUpdateFailed'));
    }
  }

  function selectTodo(todo: Todo) {
    selectedTodo.value = todo;
    editMode.value = false;
    detailVisible.value = true;
  }

  function toggleExpand(todo: Todo) {
    if (expandedTodos.value.has(todo.id)) {
      expandedTodos.value.delete(todo.id);
    } else {
      expandedTodos.value.add(todo.id);
    }
  }

  async function toggleStep(step: TodoStep) {
    try {
      await api.toggleStep(step.id);
      const updatedTodo = await api.getTodo(step.todo_id);
      const todoIndex = todoStore.todos.findIndex(t => t.id === step.todo_id);
      if (todoIndex !== -1) {
        todoStore.todos[todoIndex] = updatedTodo;
      }
      if (selectedTodo.value?.id === step.todo_id) {
        selectedTodo.value = updatedTodo;
      }
    } catch (error) {
      console.error('Failed to toggle step:', error);
      ElMessage.error(t('step.stepStatusUpdateFailed'));
    }
  }

  async function handleTodoUpdated(todo: Todo) {
    console.log('[useTodoList] handleTodoUpdated called');
    await todoStore.fetchTodos();
    await new Promise(resolve => setTimeout(resolve, 100));
    const updatedInStore = todoStore.todos.find(t => t.id === todo.id);
    if (updatedInStore) {
      console.log('[useTodoList] Syncing selectedTodo from store after update');
      selectedTodo.value = updatedInStore;
    }
    // 退出编辑模式
    editMode.value = false;
  }

  function handleTodoDeleted() {
    detailVisible.value = false;
    selectedTodo.value = null;
    editMode.value = false;
    todoStore.fetchTodos();
  }

  function handleDrawerClose() {
    editMode.value = false;
  }

  function editTodo(todo: Todo) {
    selectedTodo.value = todo;
    editMode.value = true;
    detailVisible.value = true;
  }

  async function handleDeleteTodo(todo: Todo) {
    try {
      await ElMessageBox.confirm(
        t('todo.deleteConfirm', { title: todo.title }),
        t('todo.deleteTodo'),
        {
          type: 'warning',
          confirmButtonText: t('todo.confirmDelete'),
          cancelButtonText: t('common.cancel'),
        }
      );

      await todoStore.deleteTodo(todo.id);
      ElMessage.success(t('todo.deleteSuccess'));

      if (selectedTodo.value?.id === todo.id) {
        detailVisible.value = false;
        selectedTodo.value = null;
      }
    } catch (error) {
      if (error !== 'cancel') {
        console.error('Failed to delete todo:', error);
        ElMessage.error(t('todo.deleteFailed'));
      }
    }
  }

  function formatSimpleDate(timestamp?: number): string {
    if (!timestamp) return '-';
    const date = new Date(timestamp);
    const options: Intl.DateTimeFormatOptions = {
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    };
    return date.toLocaleDateString('zh-CN', options);
  }

  function getGroupName(groupId: number): string {
    const group = groupStore.groups.find(g => g.id === groupId);
    return group ? group.name : '';
  }

  return {
    loading,
    filteredTodos,
    viewMode,
    selectedTodo,
    detailVisible,
    expandedTodos,
    editMode,
    toggleStatus,
    toggleMark,
    selectTodo,
    toggleExpand,
    toggleStep,
    handleTodoUpdated,
    handleTodoDeleted,
    handleDrawerClose,
    editTodo,
    handleDeleteTodo,
    formatSimpleDate,
    getGroupName,
    getPriorityLabel,
    getPriorityType,
    isTodoOverdue,
    isMobile,
    TodoStatus,
    t,
  };
}
