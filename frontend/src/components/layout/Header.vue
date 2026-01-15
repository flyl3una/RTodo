<template>
  <div class="header">
    <div class="header-left">
      <el-button
        :icon="collapsed ? Expand : Fold"
        text
        @click="$emit('toggle-sidebar')"
      />
      <el-divider direction="vertical" />
      <el-input
        v-model="searchQuery"
        placeholder="搜索任务..."
        :prefix-icon="Search"
        clearable
        style="width: 300px"
        @input="handleSearch"
      />
      <el-popover
        placement="bottom"
        :width="320"
        trigger="click"
      >
        <template #reference>
          <el-button
            :icon="Filter"
            :type="hasActiveFilters ? 'primary' : ''"
          >
            筛选
            <el-badge
              v-if="activeFilterCount > 0"
              :value="activeFilterCount"
              class="filter-badge"
            />
          </el-button>
        </template>

        <div class="filter-panel">
          <!-- Status Filter -->
          <div class="filter-item">
            <div class="filter-label">状态</div>
            <el-select
              v-model="filterStatus"
              placeholder="全部状态"
              clearable
              style="width: 100%"
              @change="applyFilters"
            >
              <el-option label="待办" :value="TodoStatus.Todo" />
              <el-option label="进行中" :value="TodoStatus.InProgress" />
              <el-option label="已完成" :value="TodoStatus.Done" />
            </el-select>
          </div>

          <!-- Priority Filter -->
          <div class="filter-item">
            <div class="filter-label">优先级</div>
            <el-select
              v-model="filterPriority"
              placeholder="全部优先级"
              clearable
              style="width: 100%"
              @change="applyFilters"
            >
              <el-option label="普通" :value="0" />
              <el-option label="重要" :value="1" />
              <el-option label="紧急" :value="2" />
            </el-select>
          </div>

          <!-- Task Group Filter -->
          <div class="filter-item">
            <div class="filter-label">任务组</div>
            <el-select
              v-model="filterGroupId"
              placeholder="全部任务组"
              clearable
              style="width: 100%"
              @change="applyFilters"
            >
              <el-option
                v-for="group in groups"
                :key="group.id"
                :label="group.name"
                :value="group.id"
              >
                <span>{{ group.icon || '📁' }} {{ group.name }}</span>
              </el-option>
            </el-select>
          </div>

          <!-- Tags Filter -->
          <div class="filter-item">
            <div class="filter-label">标签</div>
            <el-select
              v-model="filterTagIds"
              placeholder="选择标签"
              clearable
              multiple
              collapse-tags
              style="width: 100%"
              @change="applyFilters"
            >
              <el-option
                v-for="tag in tags"
                :key="tag.id"
                :label="tag.name"
                :value="tag.id"
              >
                <span
                  :style="{
                    display: 'inline-block',
                    width: '12px',
                    height: '12px',
                    borderRadius: '50%',
                    backgroundColor: tag.color,
                    marginRight: '8px'
                  }"
                ></span>
                {{ tag.name }}
              </el-option>
            </el-select>
          </div>

          <!-- Date Range Filter -->
          <div class="filter-item">
            <div class="filter-label">日期范围</div>
            <el-date-picker
              v-model="filterDateRange"
              type="daterange"
              range-separator="至"
              start-placeholder="开始日期"
              end-placeholder="结束日期"
              format="YYYY-MM-DD"
              value-format="x"
              clearable
              style="width: 100%"
              @change="applyFilters"
            />
          </div>

          <!-- Actions -->
          <div class="filter-actions">
            <el-button size="small" @click="resetFilters">重置</el-button>
          </div>
        </div>
      </el-popover>
    </div>

    <div class="header-right">
      <el-button-group>
        <el-button
          :type="viewMode === 'list' ? 'primary' : ''"
          :icon="List"
          @click="setViewMode('list')"
        >
          列表
        </el-button>
        <el-button
          :type="viewMode === 'card' ? 'primary' : ''"
          :icon="Grid"
          @click="setViewMode('card')"
        >
          卡片
        </el-button>
      </el-button-group>

      <el-divider direction="vertical" />

      <el-button
        type="primary"
        :icon="Plus"
        @click="$emit('show-create')"
      >
        新建任务
      </el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import {
  Search, Fold, Expand, List, Grid, Plus, Filter,
} from '@element-plus/icons-vue';
import { useUIStore } from '@/stores';
import { useTodoStore } from '@/stores';
import { useGroupStore } from '@/stores';
import { useTagStore } from '@/stores';
import { TodoStatus } from '@/types';

const props = defineProps<{
  collapsed: boolean;
}>();

defineEmits<{
  'toggle-sidebar': [];
  'show-create': [];
}>();

const uiStore = useUIStore();
const todoStore = useTodoStore();
const groupStore = useGroupStore();
const tagStore = useTagStore();

const searchQuery = ref('');
const viewMode = uiStore.viewMode;

// Filter states
const filterStatus = ref<TodoStatus | undefined>();
const filterPriority = ref<number | undefined>();
const filterGroupId = ref<string | undefined>();
const filterTagIds = ref<string[]>([]);
const filterDateRange = ref<[number, number] | null>(null);

// Computed
const groups = computed(() => groupStore.groups);
const tags = computed(() => tagStore.tags);

const hasActiveFilters = computed(() => {
  return activeFilterCount.value > 0;
});

const activeFilterCount = computed(() => {
  let count = 0;
  if (filterStatus.value !== undefined) count++;
  if (filterPriority.value !== undefined) count++;
  if (filterGroupId.value !== undefined) count++;
  if (filterTagIds.value.length > 0) count++;
  if (filterDateRange.value !== null) count++;
  return count;
});

// Methods
function handleSearch(value: string) {
  todoStore.setFilter({ search: value || undefined });
}

function applyFilters() {
  const params: {
    status?: TodoStatus;
    priority?: number;
    group_id?: string;
    tag_id?: string;
    start_date?: number;
    end_date?: number;
  } = {};

  if (filterStatus.value !== undefined) {
    params.status = filterStatus.value;
  }
  if (filterPriority.value !== undefined) {
    params.priority = filterPriority.value;
  }
  if (filterGroupId.value !== undefined) {
    params.group_id = filterGroupId.value;
  }
  if (filterTagIds.value.length > 0) {
    // Use first tag for now (API only supports single tag)
    params.tag_id = filterTagIds.value[0];
  }
  if (filterDateRange.value !== null) {
    params.start_date = filterDateRange.value[0];
    params.end_date = filterDateRange.value[1];
  }

  todoStore.setFilter(params);
}

function resetFilters() {
  filterStatus.value = undefined;
  filterPriority.value = undefined;
  filterGroupId.value = undefined;
  filterTagIds.value = [];
  filterDateRange.value = null;
  todoStore.setFilter({});
}

function setViewMode(mode: 'list' | 'card') {
  uiStore.setViewMode(mode);
}

// Load groups and tags on mount
(async () => {
  try {
    await Promise.all([
      groupStore.fetchGroups(),
      tagStore.fetchTags(),
    ]);
  } catch (error) {
    console.error('Failed to load groups/tags:', error);
  }
})();
</script>

<style scoped>
.header {
  height: 56px;
  background: white;
  border-bottom: 1px solid #e4e7ed;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 20px;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 12px;
}

.filter-badge {
  margin-left: 4px;
}

.filter-badge :deep(.el-badge__content) {
  background-color: var(--el-color-primary);
}

.filter-panel {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.filter-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.filter-label {
  font-size: 13px;
  font-weight: 500;
  color: #606266;
}

.filter-actions {
  display: flex;
  justify-content: flex-end;
  padding-top: 8px;
  border-top: 1px solid #e4e7ed;
}

/* Dark theme */
[data-theme='dark'] .header {
  background: #1a1a1a;
  border-bottom-color: #2a2a2a;
}

[data-theme='dark'] .filter-label {
  color: #e0e0e0;
}

[data-theme='dark'] .filter-actions {
  border-top-color: #2a2a2a;
}
</style>
