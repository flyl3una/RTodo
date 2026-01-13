<template>
  <div class="tag-manage">
    <div class="tag-header">
      <h3>标签管理</h3>
      <el-button :icon="Plus" size="small" @click="showCreateDialog">
        新建标签
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

    <el-empty v-else description="暂无标签" :image-size="60" />

    <!-- Create/Edit Dialog -->
    <el-dialog
      v-model="dialogVisible"
      :title="isEdit ? '编辑标签' : '新建标签'"
      width="400px"
    >
      <el-form
        ref="formRef"
        :model="form"
        :rules="rules"
        label-width="60px"
      >
        <el-form-item label="名称" prop="name">
          <el-input
            v-model="form.name"
            placeholder="请输入标签名称"
            @keyup.enter="handleSubmit"
          />
        </el-form-item>

        <el-form-item label="颜色">
          <div class="color-selector">
            <div
              v-for="color in colorOptions"
              :key="color"
              class="color-option"
              :class="{ selected: form.color === color }"
              :style="{ backgroundColor: color }"
              @click="form.color = color"
            />
          </div>
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleSubmit" :loading="loading">
          {{ isEdit ? '保存' : '创建' }}
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue';
import { Edit, Delete, Plus } from '@element-plus/icons-vue';
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus';
import { useTagStore } from '@/stores';
import type { Tag } from '@/types';

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
    { required: true, message: '请输入标签名称', trigger: 'blur' },
    { min: 1, max: 15, message: '长度在 1 到 15 个字符', trigger: 'blur' },
  ],
};

const colorOptions = [
  '#409EFF', '#67C23A', '#E6A23C', '#F56C6C',
  '#909399', '#C0392B', '#8E44AD', '#16A085',
  '#D35400', '#27AE60', '#2980B9', '#E74C3C',
];

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
      ElMessage.success('标签更新成功');
    } else {
      await tagStore.createTag(form.value.name, form.value.color);
      ElMessage.success('标签创建成功');
    }

    dialogVisible.value = false;
    emit('updated');
  } catch (error: any) {
    if (error?.errors) {
      return;
    }
    ElMessage.error(`操作失败: ${error}`);
  } finally {
    loading.value = false;
  }
}

async function deleteTag(tag: Tag) {
  try {
    await ElMessageBox.confirm(
      `确定要删除标签 "${tag.name}" 吗？`,
      '删除标签',
      {
        type: 'warning',
        confirmButtonText: '删除',
        cancelButtonText: '取消',
      }
    );

    await tagStore.deleteTag(tag.id);
    ElMessage.success('删除成功');
    emit('updated');
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败');
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
  background: white;
  border: 1px solid #e4e7ed;
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
  color: #303133;
}

.tag-actions {
  display: flex;
  gap: 4px;
}

.color-selector {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.color-option {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  cursor: pointer;
  border: 2px solid transparent;
  transition: all 0.2s ease;
}

.color-option:hover {
  transform: scale(1.1);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.color-option.selected {
  border-color: #303133;
  box-shadow: 0 0 0 2px white, 0 0 0 4px #409eff;
}

/* Dark theme */
[data-theme='dark'] .tag-list-item {
  background: #2a2a2a;
  border-color: #3a3a3a;
}

[data-theme='dark'] .tag-name {
  color: #e0e0e0;
}

[data-theme='dark'] .color-option.selected {
  border-color: #e0e0e0;
  box-shadow: 0 0 0 2px #1a1a1a, 0 0 0 4px #409eff;
}
</style>
