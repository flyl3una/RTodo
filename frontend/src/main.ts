import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { watch } from 'vue';
import ElementPlus from 'element-plus';
import zhCn from 'element-plus/es/locale/lang/zh-cn';
import en from 'element-plus/es/locale/lang/en';
import ja from 'element-plus/es/locale/lang/ja';
import 'element-plus/dist/index.css';
import './assets/styles/main.css';
import App from './App.vue';
import router from './router';
import i18n from './locales';

const app = createApp(App);
const pinia = createPinia();

// Element Plus locale mapping
const localeMap = {
  'zh-CN': zhCn,
  'zh-TW': zhCn,
  'en-US': en,
  'ja-JP': ja,
};

// Set initial Element Plus locale
app.config.globalProperties.$ELEMENT_locale = localeMap[i18n.global.locale.value];

// Watch for language changes and update Element Plus locale
watch(() => i18n.global.locale.value, (newLocale) => {
  app.config.globalProperties.$ELEMENT_locale = localeMap[newLocale] || zhCn;
});

app.use(pinia);
app.use(router);
app.use(ElementPlus);
app.use(i18n);

app.mount('#app');
