// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

/**
 * 预设图标分类列表
 */
export const PRESET_ICONS = {
  // 文件文件夹
  files: ['📁', '📂', '📄', '📃', '📑', '🗂️', '🗃️', '📋', '📰', '🗞️', '📓', '📔', '📒', '📕', '📗'],

  // 工作商务
  business: ['💼', '🏢', '🏦', '💹', '📊', '📈', '📉', '💱', '💴', '💵', '💶', '💷', '🪙', '💰', '💳'],

  // 家庭生活
  home: ['🏠', '🏡', '🏘️', '🏚️', '🏗️', '🏭', '🏰', '🏯', '🏩', '⛪', '🕌', '🛕', '🕍', '⛩️', '🕋'],

  // 目标成就
  goals: ['🎯', '🏆', '🥇', '🥈', '🥉', '🏅', '🎖️', '🏵️', '🎗️', '🎫', '🎟️', '🎪'],

  // 创意设计
  creative: ['🎨', '🖌️', '🖍️', '✏️', '✒️', '🖊️', '🖋️', '📝', '📜', '📰', '📔', '📕'],

  // 技术开发
  tech: ['💡', '🔧', '⚙️', '🔨', '🛠️', '⛏️', '🔩', '🔫', '💻', '🖥️', '🖨️', '⌨️'],

  // 学习教育
  education: ['📚', '📖', '📓', '📒', '📕', '📗', '📘', '📙', '🏫', '🎓', '🎓', '📐'],

  // 运动健康
  sports: ['⚽', '🏀', '🏈', '⚾', '🥎', '🎾', '🏐', '🏉', '🥏', '🎱'],

  // 时间计划
  time: ['📅', '📆', '🗓️', '⏰', '⏱️', '⌛', '⏳', '⌚', '🕐', '🕑', '🕒', '🕓'],

  // 交通出行
  transport: ['🚗', '🚕', '🚙', '🚌', '🚎', '🏍️', '🚲', '✈️', '🚂', '🚅'],

  // 社交沟通
  social: ['📧', '📨', '📩', '📤', '📥', '📞', '📱', '📟', '💬', '👥'],
};

/**
 * 所有图标列表
 */
export const ALL_ICONS = Object.values(PRESET_ICONS).flat();

/**
 * 图标分类信息（label 为 i18n key）
 */
export const ICON_CATEGORIES = [
  { key: 'all', label: 'icon.all', icons: ALL_ICONS },
  { key: 'files', label: 'icon.files', icons: PRESET_ICONS.files },
  { key: 'business', label: 'icon.business', icons: PRESET_ICONS.business },
  { key: 'home', label: 'icon.home', icons: PRESET_ICONS.home },
  { key: 'goals', label: 'icon.goals', icons: PRESET_ICONS.goals },
  { key: 'creative', label: 'icon.creative', icons: PRESET_ICONS.creative },
  { key: 'tech', label: 'icon.tech', icons: PRESET_ICONS.tech },
  { key: 'education', label: 'icon.education', icons: PRESET_ICONS.education },
  { key: 'sports', label: 'icon.sports', icons: PRESET_ICONS.sports },
  { key: 'time', label: 'icon.time', icons: PRESET_ICONS.time },
  { key: 'transport', label: 'icon.transport', icons: PRESET_ICONS.transport },
  { key: 'social', label: 'icon.social', icons: PRESET_ICONS.social },
];

/**
 * 默认图标
 */
export const DEFAULT_ICON = '📁';

/**
 * 图标分类键的类型
 */
export type IconCategoryKey = 'all' | keyof typeof PRESET_ICONS;

/**
 * 获取随机图标（从所有图标中）
 * @param excludeIcons 要排除的图标列表
 */
export function getRandomIcon(excludeIcons: string[] = []): string {
  const availableIcons = ALL_ICONS.filter(icon => !excludeIcons.includes(icon));

  if (availableIcons.length === 0) {
    return DEFAULT_ICON;
  }

  const randomIndex = Math.floor(Math.random() * availableIcons.length);
  return availableIcons[randomIndex];
}

/**
 * 从指定分类获取随机图标
 * @param category 图标分类键
 * @param excludeIcons 要排除的图标列表
 */
export function getRandomIconFromCategory(
  category: IconCategoryKey,
  excludeIcons: string[] = []
): string {
  // 如果是 "all" 分类，使用全部图标
  if (category === 'all') {
    return getRandomIcon(excludeIcons);
  }

  const categoryIcons = PRESET_ICONS[category];
  const availableIcons = categoryIcons.filter(icon => !excludeIcons.includes(icon));

  if (availableIcons.length === 0) {
    return DEFAULT_ICON;
  }

  const randomIndex = Math.floor(Math.random() * availableIcons.length);
  return availableIcons[randomIndex];
}
