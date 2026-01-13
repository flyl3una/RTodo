import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { TaskGroup } from '@/types';
import * as api from '@/api/tauri';

export const useGroupStore = defineStore('group', () => {
  const groups = ref<TaskGroup[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchGroups() {
    loading.value = true;
    error.value = null;
    try {
      groups.value = await api.getTaskGroups();
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function createGroup(params: {
    name: string;
    parent_id?: string;
    icon?: string;
    color?: string;
  }) {
    loading.value = true;
    error.value = null;
    try {
      const newGroup = await api.createTaskGroup(params);
      groups.value.push(newGroup);
      return newGroup;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function updateGroup(id: string, params: {
    name?: string;
    parent_id?: string;
    icon?: string;
    color?: string;
  }) {
    loading.value = true;
    error.value = null;
    try {
      const updated = await api.updateTaskGroup(id, params);
      const index = groups.value.findIndex(g => g.id === id);
      if (index !== -1) {
        groups.value[index] = updated;
      }
      return updated;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function deleteGroup(id: string) {
    loading.value = true;
    error.value = null;
    try {
      await api.deleteTaskGroup(id);
      groups.value = groups.value.filter(g => g.id !== id);
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  function clearError() {
    error.value = null;
  }

  return {
    groups,
    loading,
    error,
    fetchGroups,
    createGroup,
    updateGroup,
    deleteGroup,
    clearError,
  };
});
