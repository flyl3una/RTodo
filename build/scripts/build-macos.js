/**
 * macOS 平台构建脚本
 * 支持 x86_64 (Intel), arm64 (Apple Silicon), 和 Universal Binary
 * 生成 APP 和 DMG 包
 */

import path from 'path';
import * as logger from './utils/logger.js';
import * as executor from './utils/executor.js';
import fs from 'fs';

/**
 * macOS 构建目标
 */
const MACOS_TARGETS = {
  'x86_64': 'x86_64-apple-darwin',
  'arm64': 'aarch64-apple-darwin',
  'universal': 'universal-apple-darwin',
};

/**
 * 检查 Xcode
 */
function checkXcode() {
  try {
    const version = executor.exec('xcodebuild -version', { silent: true });
    logger.success('Xcode: 已安装');
    logger.debug(version.trim());
    return true;
  } catch {
    logger.error('Xcode: 未安装');
    logger.info('安装方式: 从 App Store 安装 Xcode');
    return false;
  }
}

/**
 * 检查 Command Line Tools
 */
function checkCommandLineTools() {
  try {
    const path = executor.exec('xcode-select -p', { silent: true });
    logger.success('Command Line Tools: 已安装');
    return true;
  } catch {
    logger.error('Command Line Tools: 未安装');
    logger.info('安装方式: xcode-select --install');
    return false;
  }
}

/**
 * 获取构建的包格式
 * @param {string} arch - 架构
 * @returns {string[]} 包格式列表
 */
function getPackageFormats(arch) {
  // macOS 默认生成 APP 和 DMG
  return ['app', 'dmg'];
}

/**
 * 构建单个架构的 macOS 应用
 * @param {string} target - Rust 目标
 * @param {Object} options - 构建选项
 */
async function buildSingleArch(target, options) {
  const { projectRoot } = options;

  logger.info(`构建目标: ${target}`);

  const buildCommand = `cargo tauri build --target ${target}`;

  const spinner = logger.createSpinner(`正在构建 ${target}...`);
  spinner.start();

  try {
    executor.exec(buildCommand, {
      cwd: projectRoot,
      silent: false,
      timeout: 600000,
    });
    spinner.succeed(`${target} 构建完成`);
  } catch (error) {
    spinner.fail(`${target} 构建失败`);
    throw error;
  }
}

/**
 * 创建 Universal Binary
 * @param {Object} options - 构建选项
 */
async function createUniversalBinary(options) {
  const { projectRoot } = options;

  logger.info('创建 Universal Binary (合并 x86_64 和 arm64)');

  const x86_64Target = 'x86_64-apple-darwin';
  const arm64Target = 'aarch64-apple-darwin';

  // 构建两个架构
  await buildSingleArch(x86_64Target, options);
  await buildSingleArch(arm64Target, options);

  // 使用 lipo 合并二进制文件
  const x86_64Binary = path.join(
    projectRoot,
    `src-tauri/target/${x86_64Target}/release/bundle/macos/RTodo.app/Contents/MacOS/rtodo`
  );

  const arm64Binary = path.join(
    projectRoot,
    `src-tauri/target/${arm64Target}/release/bundle/macos/RTodo.app/Contents/MacOS/rtodo`
  );

  const universalBinary = path.join(
    projectRoot,
    'src-tauri/target/universal-apple-darwin/release/bundle/macos/RTodo.app/Contents/MacOS/rtodo'
  );

  // 创建输出目录
  fs.mkdirSync(path.dirname(universalBinary), { recursive: true });

  // 合并二进制
  logger.info('使用 lipo 合并二进制文件...');
  executor.exec(
    `lipo -create "${x86_64Binary}" "${arm64Binary}" -output "${universalBinary}"`
  );

  // 复制 APP 包的其他文件
  logger.info('复制 APP 包资源...');
  const sourceApp = path.join(
    projectRoot,
    `src-tauri/target/${x86_64Target}/release/bundle/macos/RTodo.app`
  );
  const destApp = path.join(
    projectRoot,
    'src-tauri/target/universal-apple-darwin/release/bundle/macos/RTodo.app'
  );

  // 递归复制除了 MacOS 目录外的所有内容
  const copyDir = (src, dest) => {
    fs.mkdirSync(dest, { recursive: true });
    const entries = fs.readdirSync(src, { withFileTypes: true });

    for (const entry of entries) {
      const srcPath = path.join(src, entry.name);
      const destPath = path.join(dest, entry.name);

      if (entry.isDirectory() && entry.name !== 'MacOS') {
        copyDir(srcPath, destPath);
      } else if (entry.isFile()) {
        fs.copyFileSync(srcPath, destPath);
      }
    }
  };

  copyDir(sourceApp, destApp);

  logger.success('Universal Binary 创建完成');
}

/**
 * 构建 macOS 应用
 * @param {Object} options - 构建选项
 */
export default async function buildMacOS(options) {
  const { arch = 'universal', targets, version, projectRoot } = options;

  logger.title('macOS 平台构建');
  logger.newline();

  // 检查架构支持
  const target = MACOS_TARGETS[arch];
  if (!target) {
    logger.error(`不支持的架构: ${arch}`);
    logger.info(`支持的架构: ${Object.keys(MACOS_TARGETS).join(', ')}`);
    process.exit(1);
  }

  logger.info(`目标架构: ${arch}`);
  logger.info(`Rust 目标: ${target}`);
  logger.newline();

  // 检查开发工具
  checkXcode();
  checkCommandLineTools();
  logger.newline();

  // 确定要构建的包格式
  const packageFormats = getPackageFormats(arch);
  logger.info(`将生成的包格式: ${packageFormats.join(', ')}`);
  logger.newline();

  // 检查并添加 Rust 目标
  const targetsToAdd = [];
  if (arch === 'universal' || arch === 'x86_64') {
    targetsToAdd.push('x86_64-apple-darwin');
  }
  if (arch === 'universal' || arch === 'arm64') {
    targetsToAdd.push('aarch64-apple-darwin');
  }

  const installedTargets = executor.exec('rustup target list --installed', {
    silent: true,
  });

  for (const t of targetsToAdd) {
    if (!installedTargets.includes(t)) {
      logger.info(`添加 Rust 目标: ${t}`);
      executor.exec(`rustup target add ${t}`);
    }
  }

  logger.newline();

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

  if (arch === 'universal') {
    // 构建通用二进制
    await createUniversalBinary(options);
  } else {
    // 构建单个架构
    await buildSingleArch(target, options);
  }

  logger.newline();

  // 显示构建产物
  logger.subtitle('构建产物');

  const actualTarget = arch === 'universal' ? 'universal-apple-darwin' : target;
  const bundleDir = path.join(
    projectRoot,
    'src-tauri/target',
    actualTarget,
    'release/bundle'
  );

  logger.info(`构建产物目录: ${bundleDir}`);
  logger.newline();

  // 列出生成的文件
  if (executor.commandExists('ls')) {
    try {
      const files = executor.exec(`ls -lh "${bundleDir}/macos"`, {
        silent: true,
      });
      logger.info('APP 包:');
      console.log(files);
      logger.newline();
    } catch {
      logger.warning('未找到 APP 包');
    }

    try {
      const files = executor.exec(`ls -lh "${bundleDir}/dmg"`, {
        silent: true,
      });
      logger.info('DMG 镜像:');
      console.log(files);
    } catch {
      logger.warning('未找到 DMG 镜像');
    }
  }

  logger.newline();
  logger.success(`macOS ${arch} 构建完成!`);

  // 返回构建信息
  return {
    platform: 'macos',
    arch,
    target: actualTarget,
    packageFormats,
    bundleDir,
  };
}
