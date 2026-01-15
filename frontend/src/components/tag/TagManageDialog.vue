<template>
  <div class="tag-manage">
    <div class="tag-header">
      <h3>{{ t('tag.manageTags') }}</h3>
      <el-button :icon="Plus" size="small" @click="showCreateDialog">
        {{ t('tag.createTag') }}
      </el-button>
    </div>

    <div v-if="tags.length > 0" class="tag-list">
      <div
        v-for="tag in tags"
        :key="tag.id"
        class="tag-list-item"
        :style="{ borderLeftColor: tag.color }"
      >
        <div class="tag-info">
          <span
            class="tag-color-dot"
            :style="{ backgroundColor: tag.color }"
          />
          <span class="tag-name">{{ tag.name }}</span>
        </div>
        <div class="tag-actions">
          <el-button
            :icon="Edit"
            size="small"
            text
            @click="editTag(tag)"
          />
          <el-button
            :icon="Delete"
            size="small"
            text
            type="danger"
            @click="deleteTag(tag)"
          />
        </div>
      </div>
    </div>

    <el-empty v-else :description="t('tag.noTags')" :image-size="60" />

    <!-- Create/Edit Dialog -->
    <el-dialog
      v-model="dialogVisible"
      :title="isEdit ? t('tag.editTag') : t('tag.createTag')"
      width="400px"
    >
      <el-form
        ref="formRef"
        :model="form"
        :rules="rules"
        label-width="60px"
      >
        <el-form-item :label="t('common.name')" prop="name">
          <el-input
            v-model="form.name"
            :placeholder="t('tag.tagNamePlaceholder')"
            @keyup.enter="handleSubmit"
          />
        </el-form-item>

        <el-form-item :label="t('common.color')">
          <ColorPicker v-model="form.color" :used-colors="usedColors" />
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="dialogVisible = false">{{ t('common.cancel') }}</el-button>
        <el-button type="primary" @click="handleSubmit" :loading="loading">
          {{ isEdit ? t('common.save') : t('common.create') }}
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { Edit, Delete, Plus } from '@element-plus/icons-vue';
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus';
import { useTagStore } from '@/stores';
import ColorPicker from '@/components/common/ColorPicker.vue';
import type { Tag } from '@/types';

const { t } = useI18n();

const emit = defineEmits<{
  updated: [];
}>();

const tagStore = useTagStore();

const formRef = ref<FormInstance>();
const loading = ref(false);
const dialogVisible = ref(false);
const editingTag = ref<Tag | undefined>();

const form = ref({
  name: '',
  color: '#409EFF',
});

const rules: FormRules = {
  name: [
    { required: true, message: t('tag.tagNameRequired'), trigger: 'blur' },
    { min: 1, max: 15, message: t('tag.tagNameLengthRule'), trigger: 'blur' },
  ],
};

// Get used colors for deduplication (exclude current editing tag)
const usedColors = computed(() => {
  return tags.value
    .filter(t => t.id !== editingTag.value?.id)
    .map(t => t.color);
});

const tags = computed(() => tagStore.tags);

const isEdit = computed(() => !!editingTag.value);

function showCreateDialog() {
  editingTag.value = undefined;
  form.value = { name: '', color: '#409EFF' };
  dialogVisible.value = true;
}

function editTag(tag: Tag) {
  editingTag.value = tag;
  form.value = { name: tag.name, color: tag.color };
  dialogVisible.value = true;
}

async function handleSubmit() {
  if (!formRef.value) return;

  try {
    await formRef.value.validate();
    loading.value = true;

    if (isEdit.value && editingTag.value) {
      await tagStore.updateTag(editingTag.value.id, form.value);
      ElMessage.success(t('tag.tagUpdated'));
    } else {
      await tagStore.createTag(form.value.name, form.value.color);
      ElMessage.success(t('tag.tagCreated'));
    }

    dialogVisible.value = false;
    emit('updated');
  } catch (error: any) {
    if (error?.errors) {
      return;
    }
    ElMessage.error(`${t('tag.operationFailed')}: ${error}`);
  } finally {
    loading.value = false;
  }
}

async function deleteTag(tag: Tag) {
  try {
    await ElMessageBox.confirm(
      t('tag.deleteConfirm', { name: tag.name }),
      t('tag.deleteTagTitle'),
      {
        type: 'warning',
        confirmButtonText: t('common.delete'),
        cancelButtonText: t('common.cancel'),
      }
    );

    await tagStore.deleteTag(tag.id);
    ElMessage.success(t('tag.deleteSuccess'));
    emit('updated');
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error(t('tag.deleteFailed'));
    }
  }
}

onMounted(() => {
  tagStore.fetchTags();
});
</script>

<style scoped>
.tag-manage {
  padding: 20px;
}

.tag-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 20px;
}

.tag-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
}

.tag-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.tag-list-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background: var(--el-bg-color);
  border: 1px solid var(--el-border-color-light);
  border-left-width: 4px;
  border-radius: 6px;
  transition: all 0.2s ease;
}

.tag-list-item:hover {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
}

.tag-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.tag-color-dot {
  width: 12px;
  height: 12px;
  border-radius: 50%;
}

.tag-name {
  font-size: 14px;
  color: var(--el-text-color-primary);
}

.tag-actions {
  display: flex;
  gap: 4px;
}

/* Dark theme */
:global(html.dark) .tag-list-item {
  background: var(--el-fill-color-blank) !important;
  border-color: var(--el-border-color) !important;
}

:global(html.dark) .tag-list-item:hover {
  background: var(--el-fill-color-light) !important;
}

:global(html.dark) .tag-name {
  color: var(--el-text-color-primary);
}

:global(html.dark) .tag-header h3 {
  color: var(--el-text-color-primary);
}

/* ColorPicker component handles its own styling */
</style>
