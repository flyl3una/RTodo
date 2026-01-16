/**
 * 清理构建产物
 */

import path from 'path';
import * as logger from './utils/logger.js';
import * as executor from './utils/executor.js';
import fs from 'fs';

/**
 * 删除目录
 * @param {string} dirPath - 目录路径
 * @returns {boolean} 是否成功删除
 */
function removeDirectory(dirPath) {
  if (!fs.existsSync(dirPath)) {
    logger.debug(`目录不存在，跳过: ${dirPath}`);
    return false;
  }

  try {
    const platform = process.platform;
    if (platform === 'win32') {
      executor.exec(`rd /s /q "${dirPath}"`, { silent: true, ignoreError: true });
    } else {
      executor.exec(`rm -rf "${dirPath}"`, { silent: true, ignoreError: true });
    }
    return true;
  } catch (error) {
    logger.warning(`无法删除目录: ${dirPath}`);
    return false;
  }
}

/**
 * 清理构建产物
 */
function clean() {
  logger.title('清理构建产物');
  logger.newline();

  const projectRoot = path.join(process.cwd(), '..');

  // 定义要清理的目录
  const dirsToClean = [
    {
      name: 'Cargo 构建缓存',
      path: path.join(projectRoot, 'src-tauri/target'),
    },
    {
      name: 'Node 模块 (build)',
      path: path.join(process.cwd(), 'node_modules'),
    },
    {
      name: 'Node 模块 (frontend)',
      path: path.join(projectRoot, 'frontend/node_modules'),
    },
    {
      name: '前端构建产物',
      path: path.join(projectRoot, 'frontend/dist'),
    },
    {
      name: '构建输出目录',
      path: path.join(process.cwd(), 'output'),
    },
  ];

  let cleanedCount = 0;

  for (const dir of dirsToClean) {
    logger.info(`清理 ${dir.name}...`);

    if (removeDirectory(dir.path)) {
      logger.success(`${dir.name}: 已清理`);
      cleanedCount++;
    }
  }

  logger.newline();
  logger.success(`清理完成！共清理 ${cleanedCount} 个目录`);
}

/**
 * 主函数
 */
function main() {
  // 解析命令行参数
  const args = process.argv.slice(2);
  const cleanAll = args.includes('--all') || args.includes('-a');

  if (cleanAll) {
    logger.info('执行深度清理...');
    logger.newline();
  }

  clean();
}

// 运行主函数
main();
