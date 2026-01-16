/**
 * Linux 平台构建脚本
 * 支持 x86_64 和 arm64 架构
 * 生成 DEB, RPM, AppImage 包
 */

import path from 'path';
import * as logger from './utils/logger.js';
import * as executor from './utils/executor.js';

/**
 * Linux 构建目标
 */
const LINUX_TARGETS = {
  'x86_64': 'x86_64-unknown-linux-gnu',
  'arm64': 'aarch64-unknown-linux-gnu',
};

/**
 * 获取构建的包格式
 * @param {string} arch - 架构
 * @returns {string[]} 包格式列表
 */
function getPackageFormats(arch) {
  const formats = [];

  // DEB 是默认格式
  formats.push('deb');

  // AppImage 需要 linuxdeploy
  if (executor.commandExists('linuxdeploy') || arch === 'x86_64') {
    formats.push('appimage');
  }

  // RPM 需要 rpmbuild
  if (executor.commandExists('rpmbuild') || executor.commandExists('rpm')) {
    formats.push('rpm');
  }

  return formats;
}

/**
 * 检查 Linux 依赖
 */
function checkLinuxDependencies() {
  const deps = [
    { cmd: 'gcc', name: 'GCC compiler' },
    { cmd: 'pkg-config', name: 'pkg-config' },
  ];

  let allFound = true;

  for (const dep of deps) {
    if (executor.commandExists(dep.cmd)) {
      logger.success(`${dep.name}: 已安装`);
    } else {
      logger.warning(`${dep.name}: 未安装`);
      allFound = false;
    }
  }

  // 检查 webkit2gtk
  try {
    executor.exec('pkg-config --exists webkit2gtk-4.1', { silent: true });
    logger.success('webkit2gtk-4.1: 已安装');
  } catch {
    logger.warning('webkit2gtk-4.1: 未安装');
    logger.info('安装方式: sudo apt install libwebkit2gtk-4.1-dev');
    allFound = false;
  }

  return allFound;
}

/**
 * 构建 Linux 应用
 * @param {Object} options - 构建选项
 */
export default async function buildLinux(options) {
  const { arch = 'x86_64', targets, version, projectRoot } = options;

  logger.title('Linux 平台构建');
  logger.newline();

  // 检查架构支持
  const target = LINUX_TARGETS[arch];
  if (!target) {
    logger.error(`不支持的架构: ${arch}`);
    logger.info(`支持的架构: ${Object.keys(LINUX_TARGETS).join(', ')}`);
    process.exit(1);
  }

  logger.info(`目标架构: ${arch}`);
  logger.info(`Rust 目标: ${target}`);
  logger.newline();

  // 检查依赖
  checkLinuxDependencies();
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

  // 为每种包格式构建
  const buildResults = [];

  for (const format of packageFormats) {
    logger.info(`构建 ${format.toUpperCase()} 包...`);

    const buildArgs = ['tauri', 'build', '--target', target];
    buildArgs.push('--bundles', format);

    const buildCommand = `cargo ${buildArgs.join(' ')}`;

    logger.info(`执行: ${buildCommand}`);

    const spinner = logger.createSpinner(`正在构建 ${format.toUpperCase()}...`);
    spinner.start();

    try {
      executor.exec(buildCommand, {
        cwd: projectRoot,
        silent: false,
        timeout: 600000, // 10 分钟
      });
      spinner.succeed(`${format.toUpperCase()} 构建完成`);
      buildResults.push(format);
    } catch (error) {
      spinner.fail(`${format.toUpperCase()} 构建失败`);
      if (format !== 'appimage') {
        // AppImage 可能失败，但不影响其他格式
        throw error;
      }
    }
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
  for (const format of buildResults) {
    const formatDir = path.join(bundleDir, format);
    if (executor.commandExists('ls')) {
      try {
        const files = executor.exec(`ls -lh "${formatDir}"`, { silent: true });
        logger.info(`${format.toUpperCase()} 包:`);
        console.log(files);
      } catch {
        logger.warning(`未找到 ${format.toUpperCase()} 包`);
      }
    }
  }

  logger.newline();
  logger.success(`Linux ${arch} 构建完成!`);

  // 返回构建信息
  return {
    platform: 'linux',
    arch,
    target,
    packageFormats: buildResults,
    bundleDir,
  };
}
