<template>
  <el-dialog
    v-model="visible"
    :title="isEdit ? '编辑任务组' : '新建任务组'"
    width="500px"
    @close="handleClose"
  >
    <el-form
      ref="formRef"
      :model="form"
      :rules="rules"
      label-width="80px"
    >
      <el-form-item label="名称" prop="name">
        <el-input
          v-model="form.name"
          placeholder="请输入任务组名称"
          @keyup.enter="handleSubmit"
        />
      </el-form-item>

      <el-form-item label="图标">
        <div class="icon-selector">
          <div
            v-for="icon in iconOptions"
            :key="icon"
            class="icon-option"
            :class="{ selected: form.icon === icon }"
            @click="form.icon = icon"
          >
            {{ icon }}
          </div>
        </div>
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
      <el-button @click="handleClose">取消</el-button>
      <el-button
        v-if="isEdit"
        type="danger"
        @click="handleDelete"
        :loading="deleteLoading"
      >
        删除
      </el-button>
      <el-button type="primary" @click="handleSubmit" :loading="loading">
        {{ isEdit ? '保存' : '创建' }}
      </el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus';
import { useGroupStore } from '@/stores';
import type { TaskGroup } from '@/types';

const props = defineProps<{
  modelValue: boolean;
  group?: TaskGroup;
}>();

const emit = defineEmits<{
  'update:modelValue': [value: boolean];
  'updated': [];
}>();

const groupStore = useGroupStore();

const formRef = ref<FormInstance>();
const loading = ref(false);
const deleteLoading = ref(false);

const form = ref({
  name: '',
  icon: '📁',
  color: '#409EFF',
});

const rules: FormRules = {
  name: [
    { required: true, message: '请输入任务组名称', trigger: 'blur' },
    { min: 1, max: 20, message: '长度在 1 到 20 个字符', trigger: 'blur' },
  ],
};

const iconOptions = ['📁', '📂', '💼', '🏠', '🎯', '💡', '🔧', '📊', '🎨', '🚀'];

const colorOptions = [
  '#409EFF', // 蓝色
  '#67C23A', // 绿色
  '#E6A23C', // 橙色
  '#F56C6C', // 红色
  '#909399', // 灰色
  '#C0392B', // 深红
  '#8E44AD', // 紫色
  '#16A085', // 青色
];

const visible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value),
});

const isEdit = computed(() => !!props.group);

watch(() => props.group, (group) => {
  if (group) {
    form.value = {
      name: group.name,
      icon: group.icon || '📁',
      color: group.color || '#409EFF',
    };
  } else {
    resetForm();
  }
}, { immediate: true });

function resetForm() {
  form.value = {
    name: '',
    icon: '📁',
    color: '#409EFF',
  };
  formRef.value?.clearValidate();
}

async function handleSubmit() {
  if (!formRef.value) return;

  try {
    await formRef.value.validate();
    loading.value = true;

    if (isEdit.value && props.group) {
      await groupStore.updateGroup(props.group.id, {
        name: form.value.name,
        icon: form.value.icon,
        color: form.value.color,
      });
      ElMessage.success('任务组更新成功');
    } else {
      await groupStore.createGroup({
        name: form.value.name,
        icon: form.value.icon,
        color: form.value.color,
      });
      ElMessage.success('任务组创建成功');
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
  if (!props.group) return;

  try {
    await ElMessageBox.confirm(
      '删除任务组不会删除其中的任务，确定要删除吗？',
      '删除任务组',
      {
        type: 'warning',
        confirmButtonText: '删除',
        cancelButtonText: '取消',
      }
    );

    deleteLoading.value = true;
    await groupStore.deleteGroup(props.group.id);
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
.icon-selector {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.icon-option {
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  border: 2px solid #e4e7ed;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.icon-option:hover {
  border-color: #409eff;
  background: #f0f7ff;
}

.icon-option.selected {
  border-color: #409eff;
  background: #e6f4ff;
}

.color-selector {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.color-option {
  width: 32px;
  height: 32px;
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
[data-theme='dark'] .icon-option {
  border-color: #3a3a3a;
}

[data-theme='dark'] .icon-option:hover {
  background: #2a2a2a;
}

[data-theme='dark'] .icon-option.selected {
  background: #1a1a1a;
}

[data-theme='dark'] .color-option.selected {
  border-color: #e0e0e0;
  box-shadow: 0 0 0 2px #1a1a1a, 0 0 0 4px #409eff;
}
</style>
