/**
 * 延迟时间常量（单位：毫秒）
 */
export const DELAYS = {
  UI_SYNC: 100,
  RELOAD_AFTER_MESSAGE: 1000,
  RELOAD_AFTER_IMPORT: 1500,
} as const;

export function delay(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

export const delays = {
  uiSync: () => delay(DELAYS.UI_SYNC),
  reloadAfterMessage: () => delay(DELAYS.RELOAD_AFTER_MESSAGE),
  reloadAfterImport: () => delay(DELAYS.RELOAD_AFTER_IMPORT),
};
