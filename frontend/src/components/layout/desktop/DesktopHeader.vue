<template>
  <div class="header" v-if="!isSettingsPage && !isStatsPage">
    <div class="header-left">
      <el-button
        :icon="collapsed ? Expand : Fold"
        text
        @click="$emit('toggle-sidebar')"
      />
      <el-divider direction="vertical" />
      <el-input
        v-model="searchQuery"
        :placeholder="t('placeholder.searchTodo')"
        :prefix-icon="Search"
        clearable
        style="width: 200px"
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
            {{ t('filter.title') }}
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
            <div class="filter-label">{{ t('common.status') }}</div>
            <el-select
              v-model="filterStatus"
              :placeholder="t('filter.allStatus')"
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
            <div class="filter-label">{{ t('common.priority') }}</div>
            <el-select
              v-model="filterPriority"
              :placeholder="t('filter.allPriority')"
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
            <div class="filter-label">{{ t('common.group') }}</div>
            <el-select
              v-model="filterGroupId"
              :placeholder="t('filter.allGroups')"
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
            <div class="filter-label">{{ t('common.tags') }}</div>
            <el-select
              v-model="filterTagIds"
              :placeholder="t('filter.selectTags')"
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

          <!-- Start Date Filter -->
          <div class="filter-item">
            <div class="filter-label">{{ t('filter.startDate') }}</div>
            <el-date-picker
              v-model="filterStartDate"
              type="date"
              :placeholder="t('filter.startDateAfter')"
              format="YYYY-MM-DD"
              value-format="x"
              clearable
              style="width: 100%"
              @change="applyFilters"
            />
          </div>

          <!-- End Date Filter -->
          <div class="filter-item">
            <div class="filter-label">{{ t('filter.endDate') }}</div>
            <el-date-picker
              v-model="filterEndDate"
              type="date"
              :placeholder="t('filter.endDateBefore')"
              format="YYYY-MM-DD"
              value-format="x"
              clearable
              style="width: 100%"
              @change="applyFilters"
            />
          </div>

          <!-- Actions -->
          <div class="filter-actions">
            <el-button size="small" @click="resetFilters">{{ t('filter.reset') }}</el-button>
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
          {{ t('view.list') }}
        </el-button>
        <el-button
          :type="viewMode === 'card' ? 'primary' : ''"
          :icon="Grid"
          @click="setViewMode('card')"
        >
          {{ t('view.card') }}
        </el-button>
      </el-button-group>

      <el-divider direction="vertical" />

      <el-button
        type="primary"
        :icon="Plus"
        @click="$emit('show-create')"
      >
        {{ t('todo.createTodo') }}
      </el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { useRoute } from 'vue-router';
import { useI18n } from 'vue-i18n';
import {
  Search, Fold, Expand, List, Grid, Plus, Filter,
} from '@element-plus/icons-vue';
import { useUIStore } from '@/stores';
import { useTodoStore } from '@/stores';
import { useGroupStore } from '@/stores';
import { useTagStore } from '@/stores';
import { TodoStatus } from '@/types';

const { t } = useI18n();

const props = defineProps<{
  collapsed: boolean;
}>();

defineEmits<{
  'toggle-sidebar': [];
  'show-create': [];
}>();

const route = useRoute();
const uiStore = useUIStore();
const todoStore = useTodoStore();
const groupStore = useGroupStore();
const tagStore = useTagStore();

const searchQuery = ref('');
const viewMode = computed(() => uiStore.viewMode);

const isSettingsPage = computed(() => route.path === '/settings');
const isStatsPage = computed(() => route.path === '/stats');

const filterStatus = ref<TodoStatus | undefined>();
const filterPriority = ref<number | undefined>();
const filterGroupId = ref<string | undefined>();
const filterTagIds = ref<string[]>([]);
const filterStartDate = ref<number | undefined>();
const filterEndDate = ref<number | undefined>();

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
  if (filterStartDate.value !== undefined) count++;
  if (filterEndDate.value !== undefined) count++;
  return count;
});

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
    params.tag_id = filterTagIds.value[0];
  }
  if (filterStartDate.value !== undefined) {
    params.start_date = filterStartDate.value;
  }
  if (filterEndDate.value !== undefined) {
    params.end_date = filterEndDate.value;
  }

  todoStore.setFilter(params);
}

function resetFilters() {
  filterStatus.value = undefined;
  filterPriority.value = undefined;
  filterGroupId.value = undefined;
  filterTagIds.value = [];
  filterStartDate.value = undefined;
  filterEndDate.value = undefined;
  todoStore.setFilter({});
}

function setViewMode(mode: 'list' | 'card') {
  uiStore.setViewMode(mode);
}

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
  background: var(--el-bg-color);
  border-bottom: 1px solid var(--el-border-color-light);
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
  color: var(--el-text-color-regular);
}

.filter-actions {
  display: flex;
  justify-content: flex-end;
  padding-top: 8px;
  border-top: 1px solid var(--el-border-color-light);
}

/* Dark theme */
:global(html.dark) .header {
  background: var(--el-bg-color);
  border-bottom-color: var(--el-border-color);
}

:global(html.dark) .filter-label {
  color: var(--el-text-color-primary);
}

:global(html.dark) .filter-actions {
  border-top-color: var(--el-border-color);
}

/* 紧凑模式 */
[data-density='compact'] .header {
  height: 48px;
  padding: 0 16px;
}

[data-density='compact'] .header-left,
[data-density='compact'] .header-right {
  gap: 8px;
}

[data-density='compact'] .filter-panel {
  gap: 12px;
}

[data-density='compact'] .filter-item {
  gap: 6px;
}
</style>
