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

export type ViewFilterType = 'all' | 'todo' | 'today' | 'important' | 'urgent' | 'completed' | 'overdue' | 'group' | 'tag';

export function useTodoFilters() {
  const { t } = useI18n();
  const route = useRoute();
  const router = useRouter();
  const todoStore = useTodoStore();
  const groupStore = useGroupStore();
  const tagStore = useTagStore();
  const uiStore = useUIStore();

  const currentView = ref<ViewFilterType>('todo');
  const filterGroupId = ref<string | undefined>();
  const filterTagId = ref<string | undefined>();
  const groupDialogVisible = ref(false);
  const editingGroup = ref<TaskGroup | undefined>();
  const tagDialogVisible = ref(false);
  const editingTag = ref<Tag | undefined>();

  const groups = computed(() => groupStore.groups);
  const tags = computed(() => tagStore.tags);

  // Watch store changes to sync local state
  watch(() => todoStore.filterGroupId, (newVal) => {
    filterGroupId.value = newVal;
    if (newVal) currentView.value = 'group';
  });

  watch(() => todoStore.filterTagId, (newVal) => {
    filterTagId.value = newVal;
    if (newVal) currentView.value = 'tag';
  });

  watch(() => todoStore.isTodoView, (isNewTodoView) => {
    if (isNewTodoView) currentView.value = 'todo';
  });

  watch(() => todoStore.isOverdueView, (isOverdue) => {
    if (isOverdue) currentView.value = 'overdue';
  });

  watch(() => todoStore.filterPriority, (newPriority) => {
    if (newPriority === 1) currentView.value = 'important';
    else if (newPriority === 3) currentView.value = 'urgent';
    else if (!newPriority && !todoStore.filterGroupId && !todoStore.filterTagId && !todoStore.isTodoView && !todoStore.isOverdueView) {
      currentView.value = 'all';
    }
  });

  function setFilter(view: 'all' | 'todo' | 'today' | 'important' | 'urgent' | 'completed' | 'overdue') {
    console.log('[useTodoFilters] setFilter called with view:', view);
    currentView.value = view;
    filterGroupId.value = undefined;
    filterTagId.value = undefined;

    switch (view) {
      case 'all':
        console.log('[useTodoFilters] Applying "all" filter');
        todoStore.setTodoView(false);
        todoStore.setOverdueView(false);
        todoStore.setFilter({});
        break;
      case 'todo':
        console.log('[useTodoFilters] Applying "todo" filter');
        todoStore.setTodoView(true);
        todoStore.setOverdueView(false);
        todoStore.setFilter({});
        break;
      case 'today':
        const today = new Date();
        today.setHours(0, 0, 0, 0);
        const todayStart = today.getTime();
        const tomorrow = new Date(today);
        tomorrow.setDate(tomorrow.getDate() + 1);
        const tomorrowStart = tomorrow.getTime();
        console.log('[useTodoFilters] Applying "today" filter');
        todoStore.setTodoView(false);
        todoStore.setOverdueView(false);
        todoStore.setFilter({ start_date: todayStart, end_date: tomorrowStart });
        break;
      case 'important':
        console.log('[useTodoFilters] Applying "important" filter');
        todoStore.setTodoView(false);
        todoStore.setOverdueView(false);
        todoStore.setFilter({ priority: 1 });
        break;
      case 'urgent':
        console.log('[useTodoFilters] Applying "urgent" filter');
        todoStore.setTodoView(false);
        todoStore.setOverdueView(false);
        todoStore.setFilter({ priority: 3 });
        break;
      case 'completed':
        console.log('[useTodoFilters] Applying "completed" filter');
        todoStore.setTodoView(false);
        todoStore.setOverdueView(false);
        todoStore.setFilter({ status: TodoStatus.Done });
        break;
      case 'overdue':
        console.log('[useTodoFilters] Applying "overdue" filter');
        todoStore.setTodoView(false);
        todoStore.setOverdueView(true);
        todoStore.setFilter({});
        break;
    }

    if (route.path !== '/') {
      router.push('/');
    }
  }

  function selectGroup(groupId: string) {
    console.log('[useTodoFilters] selectGroup called with groupId:', groupId);
    currentView.value = 'group';
    filterGroupId.value = groupId;
    filterTagId.value = undefined;
    todoStore.setFilter({ group_id: groupId });
    if (route.path !== '/') {
      router.push('/');
    }
  }

  function selectTag(tagId: string) {
    console.log('[useTodoFilters] selectTag called with tagId:', tagId);
    currentView.value = 'tag';
    filterGroupId.value = undefined;
    filterTagId.value = tagId;
    todoStore.setFilter({ tag_id: tagId });
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
    currentView.value = 'todo';
    filterGroupId.value = undefined;
    filterTagId.value = undefined;
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
      filterGroupId: filterGroupId.value,
      filterTagId: filterTagId.value,
    };
  }

  return {
    currentView,
    filterGroupId,
    filterTagId,
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
