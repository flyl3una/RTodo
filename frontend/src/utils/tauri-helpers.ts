/**
 * Tauri IPC 辅助函数
 * 提供安全的IPC调用和状态检测
 */

import { invoke } from '@tauri-apps/api/core';

/**
 * 检查是否在Tauri环境中运行
 */
export function isTauriAvailable(): boolean {
  if (typeof window === 'undefined') {
    return false;
  }

  // 检查 window.__TAURI__ 对象
  if (!window.__TAURI__) {
    console.warn('[Tauri] window.__TAURI__ 对象不存在');
    return false;
  }

  // 检查 invoke 函数
  if (typeof window.__TAURI__.core?.invoke !== 'function') {
    console.warn('[Tauri] window.__TAURI__.core.invoke 函数不可用');
    return false;
  }

  return true;
}

/**
 * 获取当前环境信息
 */
export function getEnvironmentInfo() {
  return {
    url: window.location.href,
    userAgent: navigator.userAgent,
    hasTauri: typeof window.__TAURI__ !== 'undefined',
    hasInvoke: typeof window.__TAURI__?.core?.invoke === 'function',
    isTauriAvailable: isTauriAvailable(),
  };
}

/**
 * 安全的IPC调用包装器
 * 如果Tauri不可用，会抛出详细的错误信息
 */
export async function safeInvoke<T>(
  command: string,
  args?: Record<string, unknown>
): Promise<T> {
  // 检查环境
  if (!isTauriAvailable()) {
    const env = getEnvironmentInfo();
    throw new Error(
      `Tauri IPC 不可用！\n` +
      `当前环境信息：\n` +
      `- URL: ${env.url}\n` +
      `- User Agent: ${env.userAgent}\n` +
      `- hasTauri: ${env.hasTauri}\n` +
      `- hasInvoke: ${env.hasInvoke}\n\n` +
      `请确保：\n` +
      `1. 使用 'cargo tauri dev' 启动应用\n` +
      `2. 在 Tauri 桌面窗口中测试，而不是浏览器中访问 localhost:1420`
    );
  }

  // 记录调用详情
  console.log(`[IPC] 调用命令: ${command}`, args);

  try {
    const result = await invoke<T>(command, args);
    console.log(`[IPC] 命令 ${command} 成功返回:`, result);
    return result;
  } catch (error) {
    console.error(`[IPC] 命令 ${command} 调用失败:`, error);
    throw error;
  }
}

/**
 * 测试IPC连接
 */
export async function testIPCConnection(): Promise<boolean> {
  try {
    console.log('[IPC] 测试连接...');
    const result = await invoke<any>('get_todos', {payload: {}});
    console.log('[IPC] 测试成功，返回:', result);
    return true;
  } catch (error) {
    console.error('[IPC] 测试失败:', error);
    return false;
  }
}
