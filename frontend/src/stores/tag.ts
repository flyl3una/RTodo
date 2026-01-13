import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { Tag } from '@/types';
import * as api from '@/api/tauri';

export const useTagStore = defineStore('tag', () => {
  const tags = ref<Tag[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchTags() {
    loading.value = true;
    error.value = null;
    try {
      tags.value = await api.getTags();
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function createTag(name: string, color: string) {
    loading.value = true;
    error.value = null;
    try {
      const newTag = await api.createTag(name, color);
      tags.value.push(newTag);
      return newTag;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function updateTag(id: string, params: {
    name?: string;
    color?: string;
  }) {
    loading.value = true;
    error.value = null;
    try {
      const updated = await api.updateTag(id, params);
      const index = tags.value.findIndex(t => t.id === id);
      if (index !== -1) {
        tags.value[index] = updated;
      }
      return updated;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function deleteTag(id: string) {
    loading.value = true;
    error.value = null;
    try {
      await api.deleteTag(id);
      tags.value = tags.value.filter(t => t.id !== id);
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
    tags,
    loading,
    error,
    fetchTags,
    createTag,
    updateTag,
    deleteTag,
    clearError,
  };
});
