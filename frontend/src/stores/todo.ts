import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { Todo, CreateTodoRequest, UpdateTodoRequest, TodoStep } from '@/types';
import { TodoStatus } from '@/types';
import * as api from '@/api/tauri';

export const useTodoStore = defineStore('todo', () => {
  // State
  const todos = ref<Todo[]>([]);
  const currentTodo = ref<Todo | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);

  // Filters
  const filterGroupId = ref<string | undefined>();
  const filterTagId = ref<string | undefined>();
  const filterStatus = ref<TodoStatus | undefined>();
  const filterPriority = ref<number | undefined>();
  const searchQuery = ref<string>('');
  const filterStartDate = ref<number | undefined>();
  const filterEndDate = ref<number | undefined>();

  // Computed
  // filteredTodos 直接返回 todos.value，因为筛选已经在 fetchTodos 中通过 API 完成
  const filteredTodos = computed(() => {
    console.log('[TodoStore] filteredTodos computed, todos.length:', todos.value.length);
    console.log('[TodoStore] filteredTodos data:', todos.value);
    return todos.value;
  });

  const todoStats = computed(() => {
    const total = todos.value.length;
    const todo = todos.value.filter(t => t.status === TodoStatus.Todo).length;
    const inProgress = todos.value.filter(t => t.status === TodoStatus.InProgress).length;
    const done = todos.value.filter(t => t.status === TodoStatus.Done).length;
    const marked = todos.value.filter(t => t.priority >= 1).length;

    return { total, todo, inProgress, done, marked };
  });

  // Actions
  async function fetchTodos() {
    loading.value = true;
    error.value = null;
    try {
      const params = {
        group_id: filterGroupId.value,
        tag_id: filterTagId.value,
        status: filterStatus.value,
        search: searchQuery.value || undefined,
        priority: filterPriority.value,
        start_date: filterStartDate.value,
        end_date: filterEndDate.value,
      };
      console.log('[TodoStore] fetchTodos called with params:', params);
      const result = await api.getTodos(params);
      console.log('[TodoStore] fetchTodos returned', result.length, 'todos');
      todos.value = result;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function fetchTodo(id: string) {
    loading.value = true;
    error.value = null;
    try {
      currentTodo.value = await api.getTodo(id);
      return currentTodo.value;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function createTodo(request: CreateTodoRequest) {
    loading.value = true;
    error.value = null;
    try {
      const newTodo = await api.createTodo(request);
      todos.value.push(newTodo);
      return newTodo;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function updateTodo(request: UpdateTodoRequest) {
    loading.value = true;
    error.value = null;
    try {
      console.log('[Store] updateTodo called with:', request);
      const updated = await api.updateTodo(request);
      console.log('[Store] API returned updated todo:', updated);
      console.log('[Store] updated.start_date:', updated.start_date, 'updated.due_date:', updated.due_date);

      // Update in list using splice to trigger Vue reactivity
      const index = todos.value.findIndex(t => t.id === updated.id);
      console.log('[Store] Found todo at index:', index, 'out of', todos.value.length, 'todos');

      if (index !== -1) {
        console.log('[Store] Old todo at index:', todos.value[index]);
        todos.value.splice(index, 1, updated);
        console.log('[Store] Splice completed, new todo at index:', todos.value[index]);
      } else {
        console.warn('[Store] Todo not found in list, adding it');
        todos.value.push(updated);
      }

      // Update current if selected
      if (currentTodo.value?.id === updated.id) {
        currentTodo.value = updated;
      }

      console.log('[Store] updateTodo completed, returning:', updated);
      return updated;
    } catch (e) {
      console.error('[Store] updateTodo error:', e);
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function deleteTodo(id: string) {
    loading.value = true;
    error.value = null;
    try {
      await api.deleteTodo(id);
      todos.value = todos.value.filter(t => t.id !== id);
      if (currentTodo.value?.id === id) {
        currentTodo.value = null;
      }
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function updateTodoStatus(id: string, status: TodoStatus) {
    loading.value = true;
    error.value = null;
    try {
      console.log('[Store] updateTodoStatus called:', { id, status });
      const updated = await api.updateTodoStatus(id, status);
      console.log('[Store] Updated todo received:', updated);

      const index = todos.value.findIndex(t => t.id === updated.id);
      console.log('[Store] Found todo at index:', index);

      if (index !== -1) {
        console.log('[Store] Old todo status:', todos.value[index].status);
        // Use splice to trigger Vue reactivity
        todos.value.splice(index, 1, updated);
        console.log('[Store] Updated todos[index] status:', todos.value[index].status);
      }
      if (currentTodo.value?.id === updated.id) {
        currentTodo.value = updated;
      }
      return updated;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  function setFilter(params: {
    group_id?: string;
    tag_id?: string;
    status?: TodoStatus;
    search?: string;
    priority?: number;
    start_date?: number;
    end_date?: number;
  }) {
    console.log('[TodoStore] setFilter called with:', params);
    filterGroupId.value = params.group_id;
    filterTagId.value = params.tag_id;
    filterStatus.value = params.status;
    filterPriority.value = params.priority;
    filterStartDate.value = params.start_date;
    filterEndDate.value = params.end_date;
    searchQuery.value = params.search || '';
    console.log('[TodoStore] Filter state updated - group_id:', filterGroupId.value, 'tag_id:', filterTagId.value, 'status:', filterStatus.value);
    fetchTodos();
  }

  function clearError() {
    error.value = null;
  }

  // Step methods
  async function fetchSteps(todoId: string): Promise<TodoStep[]> {
    try {
      return await api.getTodoSteps(todoId);
    } catch (e) {
      error.value = String(e);
      throw e;
    }
  }

  async function createStep(todoId: string, title: string): Promise<TodoStep> {
    try {
      return await api.createStep(todoId, title);
    } catch (e) {
      error.value = String(e);
      throw e;
    }
  }

  async function toggleStep(stepId: string): Promise<TodoStep> {
    try {
      return await api.toggleStep(stepId);
    } catch (e) {
      error.value = String(e);
      throw e;
    }
  }

  async function deleteStep(stepId: string): Promise<void> {
    try {
      await api.deleteStep(stepId);
    } catch (e) {
      error.value = String(e);
      throw e;
    }
  }

  async function getStats() {
    try {
      return await api.getStats();
    } catch (e) {
      error.value = String(e);
      throw e;
    }
  }

  async function getStatsByDate(range: 'day' | 'week' | 'month') {
    try {
      return await api.getStatsByDate(range);
    } catch (e) {
      error.value = String(e);
      throw e;
    }
  }

  return {
    // State
    todos,
    currentTodo,
    loading,
    error,
    // Computed
    filteredTodos,
    todoStats,
    // Actions
    fetchTodos,
    fetchTodo,
    createTodo,
    updateTodo,
    deleteTodo,
    updateTodoStatus,
    setFilter,
    clearError,
    // Step methods
    fetchSteps,
    createStep,
    toggleStep,
    deleteStep,
    // Stats methods
    getStats,
    getStatsByDate,
  };
});
