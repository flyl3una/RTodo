<template>
  <el-dialog
    v-model="visible"
    :title="isEdit ? '编辑标签' : '新建标签'"
    width="400px"
    @close="handleClose"
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
      <el-button v-if="isEdit" type="danger" @click="handleDelete" :loading="deleteLoading">
        删除
      </el-button>
      <div style="flex: 1"></div>
      <el-button @click="handleClose">取消</el-button>
      <el-button type="primary" @click="handleSubmit" :loading="loading">
        {{ isEdit ? '保存' : '创建' }}
      </el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus';
import { useTagStore } from '@/stores';
import type { Tag } from '@/types';

const props = defineProps<{
  modelValue: boolean;
  tag?: Tag;
}>();

const emit = defineEmits<{
  'update:modelValue': [value: boolean];
  'updated': [];
}>();

const tagStore = useTagStore();

const formRef = ref<FormInstance>();
const loading = ref(false);
const deleteLoading = ref(false);

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

const visible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value),
});

const isEdit = computed(() => !!props.tag);

watch(() => props.tag, (tag) => {
  if (tag) {
    form.value = {
      name: tag.name,
      color: tag.color,
    };
  } else {
    resetForm();
  }
}, { immediate: true });

function resetForm() {
  form.value = {
    name: '',
    color: '#409EFF',
  };
  formRef.value?.clearValidate();
}

async function handleSubmit() {
  if (!formRef.value) return;

  try {
    await formRef.value.validate();
    loading.value = true;

    if (isEdit.value && props.tag) {
      await tagStore.updateTag(props.tag.id, form.value);
      ElMessage.success('标签更新成功');
    } else {
      await tagStore.createTag(form.value.name, form.value.color);
      ElMessage.success('标签创建成功');
    }

    emit('updated');
    handleClose();
  } catch (error: any) {
    if (error?.errors) {
      return;
    }
    ElMessage.error(`操作失败: ${error}`);
  } finally {
    loading.value = false;
  }
}

async function handleDelete() {
  if (!props.tag) return;

  try {
    await ElMessageBox.confirm(
      `确定要删除标签 "${props.tag.name}" 吗？`,
      '删除标签',
      {
        type: 'warning',
        confirmButtonText: '删除',
        cancelButtonText: '取消',
      }
    );

    deleteLoading.value = true;
    await tagStore.deleteTag(props.tag.id);
    ElMessage.success('删除成功');
    emit('updated');
    handleClose();
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败');
    }
  } finally {
    deleteLoading.value = false;
  }
}

function handleClose() {
  resetForm();
  visible.value = false;
}
</script>

<style scoped>
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
[data-theme='dark'] .color-option.selected {
  border-color: #e0e0e0;
  box-shadow: 0 0 0 2px #1a1a1a, 0 0 0 4px #409eff;
}
</style>
