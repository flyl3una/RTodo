import { ref, computed, watch, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useI18n } from 'vue-i18n';
import { useTodoStore } from '@/stores';
import { useGroupStore } from '@/stores';
import { useTagStore } from '@/stores';
import { useUIStore } from '@/stores';
import { TodoStatus } from '@/types';
import type { TaskGroup } from '@/types';
import type { Tag } from '@/types';
import { ViewFilterTypeEnum, type ViewFilterTypeString } from '@/types/filters';

export type ViewFilterType = ViewFilterTypeString;

export function useTodoFilters() {
  const { t } = useI18n();
  const route = useRoute();
  const router = useRouter();
  const todoStore = useTodoStore();
  const groupStore = useGroupStore();
  const tagStore = useTagStore();
  const uiStore = useUIStore();

  // baseView 用于追踪快速访问选择（todo, important 等），不受任务组/标签选择影响
  const baseView = ref<ViewFilterType>(ViewFilterTypeEnum.Todo);
  const currentView = ref<ViewFilterType>(ViewFilterTypeEnum.Todo);
  const filterGroupIds = ref<number[]>([]);
  const filterTagIds = ref<number[]>([]);
  const groupDialogVisible = ref(false);
  const editingGroup = ref<TaskGroup | undefined>();
  const tagDialogVisible = ref(false);
  const editingTag = ref<Tag | undefined>();

  const groups = computed(() => groupStore.groups);
  const tags = computed(() => tagStore.tags);

  // Watch store changes to sync local state
  watch(() => todoStore.filterGroupIds, (newVal) => {
    filterGroupIds.value = newVal || [];
    if (newVal.length > 0) currentView.value = ViewFilterTypeEnum.Group;
  });

  watch(() => todoStore.filterTagIds, (newVal) => {
    filterTagIds.value = newVal || [];
    if (newVal.length > 0) currentView.value = ViewFilterTypeEnum.Tag;
  });

  watch(() => todoStore.isTodoView, (isNewTodoView) => {
    if (isNewTodoView) currentView.value = ViewFilterTypeEnum.Todo;
  });

  watch(() => todoStore.isOverdueView, (isOverdue) => {
    if (isOverdue) currentView.value = ViewFilterTypeEnum.Overdue;
  });

  watch(() => todoStore.filterPriority, (newPriority) => {
    if (newPriority === 1) currentView.value = ViewFilterTypeEnum.Important;
    else if (newPriority === 3) currentView.value = ViewFilterTypeEnum.Urgent;
    else if (!newPriority && !todoStore.filterGroupIds.length && !todoStore.filterTagIds.length && !todoStore.isTodoView && !todoStore.isOverdueView) {
      currentView.value = ViewFilterTypeEnum.All;
    }
  });

  function setFilter(view: ViewFilterTypeString) {
    console.log('[useTodoFilters] setFilter called with view:', view);
    // 同时更新 currentView 和 baseView
    currentView.value = view;
    baseView.value = view;  // 记录快速访问选择

    // 快速访问切换时，保持组和标签筛选不变，实现组合筛选功能
    // 移除清空逻辑，允许用户同时使用快速访问 + 任务组/标签筛选

    // 构建基础参数
    const baseParams: Record<string, any> = {};
    if (filterGroupIds.value.length > 0) {
      baseParams.group_ids = filterGroupIds.value;
    }
    if (filterTagIds.value.length > 0) {
      baseParams.tag_ids = filterTagIds.value;
    }

    switch (view) {
      case ViewFilterTypeEnum.All:
        console.log('[useTodoFilters] Applying "all" filter');
        todoStore.setTodoView(false);
        todoStore.setOverdueView(false);
        todoStore.setFilter(baseParams);
        break;
      case ViewFilterTypeEnum.Todo:
        console.log('[useTodoFilters] Applying "todo" filter');
        todoStore.setTodoView(true);
        todoStore.setOverdueView(false);
        todoStore.setFilter(baseParams);
        break;
      case ViewFilterTypeEnum.Today:
        const today = new Date();
        today.setHours(0, 0, 0, 0);
        const todayStart = today.getTime();
        const tomorrow = new Date(today);
        tomorrow.setDate(tomorrow.getDate() + 1);
        const tomorrowStart = tomorrow.getTime();
        console.log('[useTodoFilters] Applying "today" filter');
        todoStore.setTodoView(false);
        todoStore.setOverdueView(false);
        todoStore.setFilter({ ...baseParams, start_date: todayStart, end_date: tomorrowStart });
        break;
      case ViewFilterTypeEnum.Important:
        console.log('[useTodoFilters] Applying "important" filter');
        todoStore.setTodoView(false);
        todoStore.setOverdueView(false);
        todoStore.setFilter({ ...baseParams, priority: 1 });
        break;
      case ViewFilterTypeEnum.Urgent:
        console.log('[useTodoFilters] Applying "urgent" filter');
        todoStore.setTodoView(false);
        todoStore.setOverdueView(false);
        todoStore.setFilter({ ...baseParams, priority: 3 });
        break;
      case ViewFilterTypeEnum.Completed:
        console.log('[useTodoFilters] Applying "completed" filter');
        todoStore.setTodoView(false);
        todoStore.setOverdueView(false);
        todoStore.setFilter({ ...baseParams, status: TodoStatus.Done });
        break;
      case ViewFilterTypeEnum.Overdue:
        console.log('[useTodoFilters] Applying "overdue" filter');
        todoStore.setTodoView(false);
        todoStore.setOverdueView(true);
        todoStore.setFilter(baseParams);
        break;
    }

    if (route.path !== '/') {
      router.push('/');
    }
  }

  function selectGroup(groupId: number) {
    console.log('[useTodoFilters] selectGroup called with groupId:', groupId);

    const index = filterGroupIds.value.indexOf(groupId);
    if (index > -1) {
      // 已存在，移除
      filterGroupIds.value.splice(index, 1);
    } else {
      // 不存在，追加
      filterGroupIds.value.push(groupId);
    }

    // 不再修改 baseView，保持快速访问高亮状态
    // 只在没有其他筛选条件时才更新 currentView
    if (filterGroupIds.value.length > 0) {
      currentView.value = ViewFilterTypeEnum.Group;
    } else if (!filterTagIds.value.length) {
      // 恢复到 baseView
      currentView.value = baseView.value;
    }

    // 构建参数
    const params: Record<string, any> = {};
    if (filterGroupIds.value.length > 0) {
      params.group_ids = filterGroupIds.value;
    }
    if (filterTagIds.value.length > 0) {
      params.tag_ids = filterTagIds.value;
    }

    // 根据 baseView 重新应用相应的筛选
    switch (baseView.value) {
      case ViewFilterTypeEnum.Todo:
        todoStore.setTodoView(true);
        break;
      case ViewFilterTypeEnum.Important:
        params.priority = 1;
        break;
      case ViewFilterTypeEnum.Urgent:
        params.priority = 3;
        break;
      case ViewFilterTypeEnum.Completed:
        params.status = TodoStatus.Done;
        break;
      case ViewFilterTypeEnum.Today:
        const today = new Date();
        today.setHours(0, 0, 0, 0);
        const todayStart = today.getTime();
        const tomorrow = new Date(today);
        tomorrow.setDate(tomorrow.getDate() + 1);
        const tomorrowStart = tomorrow.getTime();
        params.start_date = todayStart;
        params.end_date = tomorrowStart;
        break;
    }

    todoStore.setFilter(params);

    if (route.path !== '/') {
      router.push('/');
    }
  }

  function selectTag(tagId: number) {
    console.log('[useTodoFilters] selectTag called with tagId:', tagId);

    const index = filterTagIds.value.indexOf(tagId);
    if (index > -1) {
      // 已存在，移除
      filterTagIds.value.splice(index, 1);
    } else {
      // 不存在，追加
      filterTagIds.value.push(tagId);
    }

    // 不再修改 baseView，保持快速访问高亮状态
    if (filterTagIds.value.length > 0) {
      currentView.value = ViewFilterTypeEnum.Tag;
    } else if (!filterGroupIds.value.length) {
      // 恢复到 baseView
      currentView.value = baseView.value;
    }

    // 构建参数
    const params: Record<string, any> = {};
    if (filterGroupIds.value.length > 0) {
      params.group_ids = filterGroupIds.value;
    }
    if (filterTagIds.value.length > 0) {
      params.tag_ids = filterTagIds.value;
    }

    // 根据 baseView 重新应用相应的筛选
    switch (baseView.value) {
      case ViewFilterTypeEnum.Todo:
        todoStore.setTodoView(true);
        break;
      case ViewFilterTypeEnum.Important:
        params.priority = 1;
        break;
      case ViewFilterTypeEnum.Urgent:
        params.priority = 3;
        break;
      case ViewFilterTypeEnum.Completed:
        params.status = TodoStatus.Done;
        break;
      case ViewFilterTypeEnum.Today:
        const today = new Date();
        today.setHours(0, 0, 0, 0);
        const todayStart = today.getTime();
        const tomorrow = new Date(today);
        tomorrow.setDate(tomorrow.getDate() + 1);
        const tomorrowStart = tomorrow.getTime();
        params.start_date = todayStart;
        params.end_date = tomorrowStart;
        break;
    }

    todoStore.setFilter(params);

    if (route.path !== '/') {
      router.push('/');
    }
  }

  function showAddGroup() {
    editingGroup.value = undefined;
    groupDialogVisible.value = true;
  }

  function editGroup(group: TaskGroup) {
    editingGroup.value = group;
    groupDialogVisible.value = true;
  }

  function handleGroupUpdated() {
    groupDialogVisible.value = false;
    editingGroup.value = undefined;
    groupStore.fetchGroups();
  }

  function showAddTag() {
    editingTag.value = undefined;
    tagDialogVisible.value = true;
  }

  function handleTagUpdated() {
    tagDialogVisible.value = false;
    editingTag.value = undefined;
    tagStore.fetchTags();
  }

  function editTag(tag: Tag) {
    editingTag.value = tag;
    tagDialogVisible.value = true;
  }

  // Reset to 'todo' view
  function resetToTodoView() {
    console.log('[useTodoFilters] resetToTodoView called');
    baseView.value = ViewFilterTypeEnum.Todo;
    currentView.value = ViewFilterTypeEnum.Todo;
    filterGroupIds.value = [];
    filterTagIds.value = [];
    todoStore.setFilter({});
  }

  // Refresh current view without resetting
  async function refreshCurrentView() {
    console.log('[useTodoFilters] refreshCurrentView called');
    await todoStore.fetchTodos();
  }

  // Get current view state
  function getCurrentView() {
    return {
      currentView: currentView.value,
      baseView: baseView.value,
      filterGroupIds: filterGroupIds.value,
      filterTagIds: filterTagIds.value,
    };
  }

  return {
    currentView,
    baseView,
    filterGroupIds,
    filterTagIds,
    groupDialogVisible,
    editingGroup,
    tagDialogVisible,
    editingTag,
    groups,
    tags,
    setFilter,
    selectGroup,
    selectTag,
    showAddGroup,
    editGroup,
    handleGroupUpdated,
    showAddTag,
    handleTagUpdated,
    editTag,
    resetToTodoView,
    refreshCurrentView,
    getCurrentView,
  };
}
