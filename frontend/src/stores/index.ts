import { createPinia } from 'pinia';

export function setupStores(app: any) {
  const pinia = createPinia();
  app.use(pinia);
}

export * from './todo';
export * from './group';
export * from './tag';
export * from './ui';
