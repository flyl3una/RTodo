/**
 * 环境验证脚本
 * 检查构建环境是否正确配置
 */

import * as logger from './utils/logger.js';
import * as executor from './utils/executor.js';
import fs from 'fs';
import path from 'path';

/**
 * 检查 Node.js
 */
function checkNodeJS() {
  const nodeVersion = process.version;
  const majorVersion = parseInt(nodeVersion.slice(1).split('.')[0]);

  if (majorVersion >= 16) {
    logger.success(`Node.js: ${nodeVersion} (符合要求 >= 16.0.0)`);
    return true;
  } else {
    logger.warning(`Node.js: ${nodeVersion} (建议 >= 16.0.0)`);
    return false;
  }
}

/**
 * 检查 npm
 */
function checkNpm() {
  try {
    const version = executor.exec('npm --version', { silent: true }).trim();
    const majorVersion = parseInt(version.split('.')[0]);

    if (majorVersion >= 8) {
      logger.success(`npm: ${version} (符合要求 >= 8.0.0)`);
      return true;
    } else {
      logger.warning(`npm: ${version} (建议 >= 8.0.0)`);
      return false;
    }
  } catch {
    logger.error('npm: 未安装');
    return false;
  }
}

/**
 * 检查 Rust 工具链
 */
function checkRust() {
  try {
    const rustcVersion = executor.exec('rustc --version', { silent: true }).trim();
    const cargoVersion = executor.exec('cargo --version', { silent: true }).trim();

    logger.success(`Rust: ${rustcVersion}`);
    logger.success(`Cargo: ${cargoVersion}`);
    return true;
  } catch {
    logger.error('Rust 工具链: 未安装');
    logger.info('安装方式: 访问 https://rustup.rs/');
    return false;
  }
}

/**
 * 检查 Tauri CLI
 */
function checkTauriCLI() {
  // 检查 cargo tauri
  try {
    const version = executor.exec('cargo tauri --version', { silent: true }).trim();
    logger.success(`Tauri CLI: ${version}`);
    return true;
  } catch {
    logger.error('Tauri CLI: 未安装');
    logger.info('安装方式: cargo install tauri-cli --version "^2.0.0"');
    return false;
  }
}

/**
 * 检查平台特定工具
 */
function checkPlatformTools() {
  const platform = process.platform;
  let allGood = true;

  logger.newline();
  logger.subtitle('平台特定工具');

  if (platform === 'win32') {
    // Windows 特定工具
    allGood &= checkWindowsTools();
  } else if (platform === 'darwin') {
    // macOS 特定工具
    allGood &= checkMacOSTools();
  } else if (platform === 'linux') {
    // Linux 特定工具
    allGood &= checkLinuxTools();
  }

  return allGood;
}

/**
 * 检查 Windows 工具
 */
function checkWindowsTools() {
  let allGood = true;

  // 检查 MSVC
  if (executor.commandExists('cl')) {
    logger.success('MSVC (Visual Studio): 已安装');
  } else {
    logger.warning('MSVC (Visual Studio): 未找到');
    logger.info('安装 Build Tools for Visual Studio 2022');
    allGood = false;
  }

  // 检查 WiX Toolset
  if (executor.commandExists('candle')) {
    logger.success('WiX Toolset: 已安装');
  } else {
    logger.warning('WiX Toolset: 未安装 (用于生成 MSI 安装包)');
    logger.info('安装方式: choco install wixtoolset');
    // 不计入错误，因为 WiX 是可选的
  }

  // 检查 NSIS
  if (executor.commandExists('makensis')) {
    logger.success('NSIS: 已安装');
  } else {
    logger.info('NSIS: 未安装 (用于生成 NSIS 安装包)');
    logger.info('安装方式: choco install nsis');
  }

  return allGood;
}

/**
 * 检查 macOS 工具
 */
function checkMacOSTools() {
  let allGood = true;

  // 检查 Xcode
  try {
    const version = executor.exec('xcodebuild -version', { silent: true }).trim();
    logger.success(`Xcode: ${version.split('\n')[0]}`);
  } catch {
    logger.error('Xcode: 未安装');
    logger.info('从 App Store 安装 Xcode');
    allGood = false;
  }

  // 检查 Command Line Tools
  try {
    executor.exec('xcode-select -p', { silent: true });
    logger.success('Command Line Tools: 已安装');
  } catch {
    logger.error('Command Line Tools: 未安装');
    logger.info('运行: xcode-select --install');
    allGood = false;
  }

  return allGood;
}

/**
 * 检查 Linux 工具
 */
function checkLinuxTools() {
  let allGood = true;

  // 检查 GCC
  if (executor.commandExists('gcc')) {
    const version = executor.exec('gcc --version', { silent: true }).trim();
    logger.success(`GCC: ${version.split('\n')[0]}`);
  } else {
    logger.error('GCC: 未安装');
    logger.info('安装方式: sudo apt install build-essential');
    allGood = false;
  }

  // 检查 webkit2gtk
  try {
    executor.exec('pkg-config --exists webkit2gtk-4.1', { silent: true });
    logger.success('webkit2gtk-4.1: 已安装');
  } catch {
    logger.error('webkit2gtk-4.1: 未安装');
    logger.info('安装方式: sudo apt install libwebkit2gtk-4.1-dev');
    allGood = false;
  }

  // 检查其他可选工具
  if (executor.commandExists('linuxdeploy')) {
    logger.success('linuxdeploy: 已安装 (用于生成 AppImage)');
  } else {
    logger.info('linuxdeploy: 未安装 (用于生成 AppImage)');
  }

  if (executor.commandExists('rpmbuild')) {
    logger.success('rpmbuild: 已安装 (用于生成 RPM 包)');
  } else {
    logger.info('rpmbuild: 未安装 (用于生成 RPM 包)');
  }

  return allGood;
}

/**
 * 检查项目配置
 */
function checkProjectConfig() {
  logger.newline();
  logger.subtitle('项目配置');

  const projectRoot = path.join(process.cwd(), '..');
  let allGood = true;

  // 检查 Tauri 配置
  const tauriConfigPath = path.join(projectRoot, 'src-tauri/tauri.conf.json');
  if (fs.existsSync(tauriConfigPath)) {
    try {
      const config = JSON.parse(fs.readFileSync(tauriConfigPath, 'utf-8'));

      if (config.bundle?.active) {
        logger.success('Tauri 配置: 打包功能已启用');
      } else {
        logger.warning('Tauri 配置: 打包功能未启用');
        logger.info('在 tauri.conf.json 中设置 "bundle.active": true');
        allGood = false;
      }

      if (config.bundle?.icon?.length > 0) {
        logger.success('Tauri 配置: 图标已配置');
      } else {
        logger.warning('Tauri 配置: 图标未配置');
      }
    } catch {
      logger.error('Tauri 配置: 解析失败');
      allGood = false;
    }
  } else {
    logger.error('Tauri 配置: 未找到 tauri.conf.json');
    allGood = false;
  }

  // 检查图标目录
  const iconsDir = path.join(projectRoot, 'src-tauri/icons');
  if (fs.existsSync(iconsDir)) {
    const icons = fs.readdirSync(iconsDir);
    logger.success(`图标目录: 找到 ${icons.length} 个图标文件`);
  } else {
    logger.warning('图标目录: 未找到 icons 目录');
  }

  return allGood;
}

/**
 * 检查 Rust 目标
 */
function checkRustTargets() {
  logger.newline();
  logger.subtitle('Rust 目标');

  try {
    const installed = executor.exec('rustup target list --installed', {
      silent: true,
    });
    const targets = installed.trim().split('\n');

    logger.info(`已安装 ${targets.length} 个目标:`);
    targets.forEach(target => {
      logger.debug(`  - ${target}`);
    });

    return targets.length > 0;
  } catch {
    logger.error('无法获取 Rust 目标列表');
    return false;
  }
}

/**
 * 主验证函数
 */
function validate() {
  logger.title('构建环境验证');
  logger.newline();

  const checks = [];

  // 基础工具
  logger.subtitle('基础工具');
  checks.push({ name: 'Node.js', passed: checkNodeJS() });
  checks.push({ name: 'npm', passed: checkNpm() });
  checks.push({ name: 'Rust', passed: checkRust() });
  checks.push({ name: 'Tauri CLI', passed: checkTauriCLI() });

  // 平台特定工具
  checks.push({ name: '平台工具', passed: checkPlatformTools() });

  // 项目配置
  checks.push({ name: '项目配置', passed: checkProjectConfig() });

  // Rust 目标
  checks.push({ name: 'Rust 目标', passed: checkRustTargets() });

  // 汇总结果
  logger.newline();
  logger.title('验证结果汇总');
  logger.newline();

  const passedCount = checks.filter(c => c.passed).length;
  const totalCount = checks.length;

  for (const check of checks) {
    if (check.passed) {
      logger.success(`✓ ${check.name}`);
    } else {
      logger.error(`✗ ${check.name}`);
    }
  }

  logger.newline();
  logger.info(`通过: ${passedCount}/${totalCount}`);

  if (passedCount === totalCount) {
    logger.success('所有检查通过！构建环境已就绪。');
    return 0;
  } else {
    logger.warning('部分检查未通过，请安装缺失的工具。');
    logger.info('参考 SETUP.md 获取详细安装指南。');
    return 1;
  }
}

// 运行验证
const exitCode = validate();
process.exit(exitCode);
