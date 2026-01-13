import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { Todo, CreateTodoRequest, UpdateTodoRequest, TodoStep } from '@/types';
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
  const filterStatus = ref<string | undefined>();
  const filterIsMarked = ref<boolean | undefined>();
  const filterPriority = ref<number | undefined>();
  const searchQuery = ref<string>('');
  const filterStartDate = ref<number | undefined>();
  const filterEndDate = ref<number | undefined>();

  // Computed
  const filteredTodos = computed(() => {
    let result = todos.value;

    if (filterGroupId.value) {
      result = result.filter(t => t.group_id === filterGroupId.value);
    }
    if (filterTagId.value) {
      result = result.filter(t => t.tags?.some(tag => tag.id === filterTagId.value));
    }
    if (filterStatus.value) {
      result = result.filter(t => t.status === filterStatus.value);
    }
    if (filterIsMarked.value !== undefined) {
      result = result.filter(t => t.is_marked === filterIsMarked.value);
    }
    if (filterPriority.value !== undefined) {
      result = result.filter(t => t.priority === filterPriority.value);
    }
    if (filterStartDate.value !== undefined) {
      result = result.filter(t => {
        // Filter by due_date or start_date falling within the range
        const date = t.due_date || t.start_date;
        return date && date >= filterStartDate.value!;
      });
    }
    if (filterEndDate.value !== undefined) {
      result = result.filter(t => {
        // Filter by due_date or start_date falling within the range
        const date = t.due_date || t.start_date;
        return date && date < filterEndDate.value!;
      });
    }
    if (searchQuery.value) {
      const query = searchQuery.value.toLowerCase();
      result = result.filter(t =>
        t.title.toLowerCase().includes(query) ||
        t.description?.toLowerCase().includes(query)
      );
    }

    return result;
  });

  const todoStats = computed(() => {
    const total = todos.value.length;
    const todo = todos.value.filter(t => t.status === 'todo').length;
    const inProgress = todos.value.filter(t => t.status === 'in_progress').length;
    const done = todos.value.filter(t => t.status === 'done').length;
    const marked = todos.value.filter(t => t.is_marked).length;

    return { total, todo, inProgress, done, marked };
  });

  // Actions
  async function fetchTodos() {
    loading.value = true;
    error.value = null;
    try {
      todos.value = await api.getTodos({
        group_id: filterGroupId.value,
        tag_id: filterTagId.value,
        status: filterStatus.value,
        search: searchQuery.value || undefined,
        is_marked: filterIsMarked.value,
        priority: filterPriority.value,
        start_date: filterStartDate.value,
        end_date: filterEndDate.value,
      });
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
      const updated = await api.updateTodo(request);
      // Update in list
      const index = todos.value.findIndex(t => t.id === updated.id);
      if (index !== -1) {
        todos.value[index] = updated;
      }
      // Update current if selected
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

  async function updateTodoStatus(id: string, status: string) {
    loading.value = true;
    error.value = null;
    try {
      const updated = await api.updateTodoStatus(id, status);
      const index = todos.value.findIndex(t => t.id === updated.id);
      if (index !== -1) {
        todos.value[index] = updated;
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

  async function toggleTodoMark(id: string) {
    loading.value = true;
    error.value = null;
    try {
      const updated = await api.toggleTodoMark(id);
      const index = todos.value.findIndex(t => t.id === updated.id);
      if (index !== -1) {
        todos.value[index] = updated;
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
    status?: string;
    search?: string;
    is_marked?: boolean;
    priority?: number;
    start_date?: number;
    end_date?: number;
  }) {
    filterGroupId.value = params.group_id;
    filterTagId.value = params.tag_id;
    filterStatus.value = params.status;
    filterIsMarked.value = params.is_marked;
    filterPriority.value = params.priority;
    filterStartDate.value = params.start_date;
    filterEndDate.value = params.end_date;
    searchQuery.value = params.search || '';
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
    toggleTodoMark,
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
