// Copyright 2025 RTodo Team. All rights reserved.
// SPDX-License-Identifier: MIT

import { createI18n } from 'vue-i18n';
import zhCN from './zh-CN';
import zhTW from './zh-TW';
import enUS from './en-US';
import jaJP from './ja-JP';

const messages = {
  'zh-CN': zhCN,
  'zh-TW': zhTW,
  'en-US': enUS,
  'ja-JP': jaJP,
};

// 从 localStorage 读取保存的语言，默认使用 'zh-CN'
const savedLanguage = localStorage.getItem('rtodo-language') as 'zh-CN' | 'zh-TW' | 'en-US' | 'ja-JP' | null;
const defaultLocale = savedLanguage && ['zh-CN', 'zh-TW', 'en-US', 'ja-JP'].includes(savedLanguage)
  ? savedLanguage
  : 'zh-CN';

const i18n = createI18n({
  legacy: false,  // 使用 Composition API 模式
  locale: defaultLocale,  // 从 localStorage 读取的语言
  fallbackLocale: 'zh-CN',  // 回退语言
  messages,
});

export default i18n;
