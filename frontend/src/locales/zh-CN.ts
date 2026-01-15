// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

export default {
  // 通用
  common: {
    confirm: '确定',
    cancel: '取消',
    delete: '删除',
    save: '保存',
    create: '创建',
    edit: '编辑',
    loading: '加载中...',
    search: '搜索',
    searchPlaceholder: '搜索...',
    placeholder: '请输入...',
    add: '添加',
    remove: '移除',
    select: '选择',
    clear: '清除',
    reset: '重置',
    all: '全部',
    none: '无',
    yes: '是',
    no: '否',
    close: '关闭',
    submit: '提交',
    back: '返回',
    more: '更多',
    actions: '操作',
    status: '状态',
    priority: '优先级',
    title: '标题',
    description: '描述',
    name: '名称',
    color: '颜色',
    icon: '图标',
    group: '分组',
    tag: '标签',
    tags: '标签',
    createdAt: '创建时间',
    updatedAt: '更新时间',
  },

  // 导航和布局
  nav: {
    allTodos: '所有任务',
    today: '今天',
    upcoming: '即将到来',
    completed: '已完成',
    trash: '回收站',
    quickAccess: '快速访问',
    important: '重要',
    urgent: '紧急',
  },

  // 任务相关
  todo: {
    title: '任务',
    createTodo: '新建任务',
    editTodo: '编辑任务',
    deleteTodo: '删除任务',
    todoDetail: '任务详情',
    todoList: '任务列表',
    todoTitle: '任务标题',
    todoDescription: '任务描述',
    addTodo: '添加任务',
    updateTodo: '更新任务',
    deleteSuccess: '任务删除成功',
    createSuccess: '任务创建成功',
    updateSuccess: '任务更新成功',
    deleteConfirm: '确定要删除这个任务吗？',
  },

  // 任务状态
  status: {
    todo: '待办',
    inProgress: '进行中',
    done: '已完成',
  },

  // 任务优先级
  priority: {
    normal: '普通',
    important: '重要',
    urgent: '紧急',
  },

  // 标签相关
  tag: {
    title: '标签',
    createTag: '新建标签',
    editTag: '编辑标签',
    deleteTag: '删除标签',
    manageTags: '管理标签',
    tagName: '标签名称',
    selectColor: '选择颜色',
    randomColor: '随机颜色',
    customColor: '自定义颜色',
    tagCreated: '标签创建成功',
    tagUpdated: '标签更新成功',
    tagDeleted: '标签删除成功',
    deleteConfirm: '确定要删除这个标签吗？',
    noTags: '暂无标签',
    tagNamePlaceholder: '请输入标签名称',
    tagNameRequired: '请输入标签名称',
    tagNameTooLong: '标签名称不能超过 15 个字符',
  },

  // 任务组相关
  group: {
    title: '任务组',
    createGroup: '新建任务组',
    editGroup: '编辑任务组',
    deleteGroup: '删除任务组',
    manageGroups: '管理任务组',
    groupName: '任务组名称',
    selectIcon: '选择图标',
    selectColor: '选择颜色',
    randomIcon: '随机图标',
    randomColor: '随机颜色',
    groupCreated: '任务组创建成功',
    groupUpdated: '任务组更新成功',
    groupDeleted: '任务组删除成功',
    deleteConfirm: '确定要删除这个任务组吗？',
    noGroups: '暂无任务组',
    groupNamePlaceholder: '请输入任务组名称',
    groupNameRequired: '请输入任务组名称',
  },

  // 步骤相关
  step: {
    title: '执行步骤',
    addStep: '添加步骤',
    editStep: '编辑步骤',
    deleteStep: '删除步骤',
    stepTitle: '步骤标题',
    stepCreated: '步骤添加成功',
    stepUpdated: '步骤更新成功',
    stepDeleted: '步骤删除成功',
    noSteps: '暂无执行步骤',
    stepPlaceholder: '请输入步骤标题',
  },

  // 设置页面
  settings: {
    title: '设置',
    appearance: '外观',
    theme: '主题',
    themeLight: '浅色',
    themeDark: '深色',
    themeAuto: '跟随系统',
    language: '语言',
    dataManagement: '数据管理',
    exportData: '导出数据',
    importData: '导入数据',
    clearData: '清空数据',
    about: '关于',
  },

  // 语言选项
  languages: {
    'zh-CN': '简体中文',
    'zh-TW': '繁體中文',
    'en-US': 'English',
    'ja-JP': '日本語',
  },

  // 消息提示
  messages: {
    success: '操作成功',
    error: '操作失败',
    confirmDelete: '确定要删除吗？',
    dataExported: '数据导出成功',
    dataImported: '数据导入成功',
    dataCleared: '数据已清空',
    themeChanged: '主题已切换',
    languageChanged: '语言已切换',
    loading: '加载中...',
    noData: '暂无数据',
    networkError: '网络错误，请稍后重试',
    unknownError: '未知错误',
  },

  // 表单验证
  validation: {
    required: '此项为必填项',
    minLength: '长度不能少于 {min} 个字符',
    maxLength: '长度不能超过 {max} 个字符',
    email: '请输入有效的邮箱地址',
    url: '请输入有效的 URL',
  },

  // 日期相关
  date: {
    today: '今天',
    tomorrow: '明天',
    yesterday: '昨天',
    week: '周',
    month: '月',
    year: '年',
    hour: '时',
    minute: '分',
    second: '秒',
  },

  // 统计相关
  stats: {
    title: '统计',
    total: '总计',
    completed: '已完成',
    inProgress: '进行中',
    todo: '待办',
    completionRate: '完成率',
    overdue: '逾期',
    thisWeek: '本周',
    thisMonth: '本月',
  },

  // 附件相关
  attachment: {
    title: '附件',
    upload: '上传附件',
    download: '下载附件',
    delete: '删除附件',
    noAttachments: '暂无附件',
    uploadSuccess: '附件上传成功',
    deleteSuccess: '附件删除成功',
  },

  // 图标分类
  icon: {
    all: '全部',
    files: '文件',
    business: '工作',
    home: '家庭',
    goals: '目标',
    creative: '创意',
    tech: '技术',
    education: '学习',
    sports: '运动',
    time: '时间',
    transport: '交通',
    social: '社交',
  },

  // 颜色分类
  color: {
    reds: '红色',
    oranges: '橙色',
    ambers: '琥珀',
    yellows: '黄色',
    limes: '柠檬',
    greens: '绿色',
    emeralds: '翡翠',
    teals: '青色',
    skys: '天空',
    blues: '蓝色',
    indigos: '靛蓝',
    violets: '紫罗兰',
    purples: '紫色',
    fuchsias: '品红',
    pinks: '粉色',
    roses: '玫瑰',
    slates: '岩灰',
    neutrals: '中性灰',
    stones: '石材灰',
  },
};
