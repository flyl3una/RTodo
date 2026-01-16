/**
 * Windows 平台构建脚本
 * 支持 x86_64 和 arm64 架构
 * 生成 MSI 和 NSIS 安装包
 */

import path from 'path';
import * as logger from './utils/logger.js';
import * as executor from './utils/executor.js';

/**
 * Windows 构建目标
 */
const WINDOWS_TARGETS = {
  'x86_64': 'x86_64-pc-windows-msvc',
  'arm64': 'aarch64-pc-windows-msvc',
};

/**
 * 获取构建的包格式
 * @param {string} arch - 架构
 * @returns {string[]} 包格式列表
 */
function getPackageFormats(arch) {
  const formats = [];

  // MSI 是默认格式
  formats.push('msi');

  // NSIS 可选，需要检查是否安装
  if (executor.commandExists('makensis')) {
    formats.push('nsis');
  }

  return formats;
}

/**
 * 检查 WiX Toolset
 */
function checkWiX() {
  if (!executor.commandExists('candle')) {
    logger.warning('WiX Toolset 未安装，无法生成 MSI 安装包');
    logger.info('安装方式: choco install wixtoolset');
    logger.info('或访问: https://wixtoolset.org/releases/');
    return false;
  }
  logger.success('WiX Toolset: 已安装');
  return true;
}

/**
 * 检查 NSIS
 */
function checkNSIS() {
  if (executor.commandExists('makensis')) {
    logger.success('NSIS: 已安装');
    return true;
  }
  logger.warning('NSIS 未安装，跳过 NSIS 安装包');
  logger.info('安装方式: choco install nsis');
  return false;
}

/**
 * 构建 Windows 应用
 * @param {Object} options - 构建选项
 */
export default async function buildWindows(options) {
  const { arch = 'x86_64', targets, version, projectRoot } = options;

  logger.title('Windows 平台构建');
  logger.newline();

  // 检查架构支持
  const target = WINDOWS_TARGETS[arch];
  if (!target) {
    logger.error(`不支持的架构: ${arch}`);
    logger.info(`支持的架构: ${Object.keys(WINDOWS_TARGETS).join(', ')}`);
    process.exit(1);
  }

  logger.info(`目标架构: ${arch}`);
  logger.info(`Rust 目标: ${target}`);
  logger.newline();

  // 检查构建工具
  const hasWiX = checkWiX();
  const hasNSIS = checkNSIS();
  logger.newline();

  // 确定要构建的包格式
  const packageFormats = getPackageFormats(arch);
  logger.info(`将生成的包格式: ${packageFormats.join(', ')}`);
  logger.newline();

  // 检查是否需要添加 Rust 目标
  const installedTargets = executor.exec('rustup target list --installed', {
    silent: true,
  });

  if (!installedTargets.includes(target)) {
    logger.info(`添加 Rust 目标: ${target}`);
    executor.exec(`rustup target add ${target}`);
    logger.success(`Rust 目标已添加`);
    logger.newline();
  }

  // 构建前端
  logger.subtitle('构建前端');
  const frontendDir = path.join(projectRoot, 'frontend');
  logger.info(`安装依赖...`);
  executor.exec('npm install', { cwd: frontendDir });
  logger.info(`构建前端...`);
  executor.exec('npm run build', { cwd: frontendDir });
  logger.success('前端构建完成');
  logger.newline();

  // 构建 Tauri 应用
  logger.subtitle('构建 Tauri 应用');

  // 构建命令
  const buildArgs = ['tauri', 'build', '--target', target];

  // 如果只构建特定格式
  if (packageFormats.length === 1 && packageFormats[0] === 'msi' && !hasNSIS) {
    // 只构建 MSI
    // 不需要额外参数，Tauri 默认会构建 MSI
  }

  const buildCommand = `cargo ${buildArgs.join(' ')}`;

  logger.info(`执行构建: ${buildCommand}`);
  logger.info(`这可能需要几分钟...`);

  const spinner = logger.createSpinner('正在构建...');
  spinner.start();

  try {
    executor.exec(buildCommand, {
      cwd: projectRoot,
      silent: false,
      timeout: 600000, // 10 分钟
    });
    spinner.succeed('构建完成');
  } catch (error) {
    spinner.fail('构建失败');
    throw error;
  }

  logger.newline();

  // 显示构建产物
  logger.subtitle('构建产物');
  const bundleDir = path.join(
    projectRoot,
    'src-tauri/target',
    target,
    'release/bundle'
  );

  logger.info(`构建产物目录: ${bundleDir}`);
  logger.newline();

  // 列出生成的文件
  if (packageFormats.includes('msi')) {
    const msiDir = path.join(bundleDir, 'msi');
    if (executor.commandExists('ls')) {
      try {
        const files = executor.exec(`ls -la "${msiDir}"`, { silent: true });
        logger.info('MSI 安装包:');
        console.log(files);
      } catch {
        logger.warning('未找到 MSI 安装包');
      }
    }
  }

  if (packageFormats.includes('nsis')) {
    const nsisDir = path.join(bundleDir, 'nsis');
    if (executor.commandExists('ls')) {
      try {
        const files = executor.exec(`ls -la "${nsisDir}"`, { silent: true });
        logger.info('NSIS 安装包:');
        console.log(files);
      } catch {
        logger.warning('未找到 NSIS 安装包');
      }
    }
  }

  logger.newline();
  logger.success(`Windows ${arch} 构建完成!`);

  // 返回构建信息
  return {
    platform: 'windows',
    arch,
    target,
    packageFormats,
    bundleDir,
  };
}
