/**
 * 设备检测工具
 * 基于 user agent 和屏幕宽度检测移动设备
 */

/**
 * 检测是否为移动设备
 * @returns {boolean} 是否为移动设备
 */
export function isMobile(): boolean {
  if (typeof window === 'undefined') {
    return false;
  }

  // 检查 user agent
  const userAgent = navigator.userAgent;
  const isMobileUA = /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(userAgent);

  // 同时检查屏幕宽度作为备用方案
  const isMobileWidth = window.innerWidth <= 768;

  return isMobileUA || isMobileWidth;
}

/**
 * 获取设备信息（用于调试）
 * @returns {object} 设备信息对象
 */
export function getDeviceInfo(): {
  isMobile: boolean;
  userAgent: string;
  screenWidth: number;
  screenHeight: number;
} {
  return {
    isMobile: isMobile(),
    userAgent: navigator.userAgent,
    screenWidth: window.innerWidth,
    screenHeight: window.innerHeight,
  };
}
