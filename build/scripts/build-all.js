#!/usr/bin/env node
/**
 * RTodo 主构建脚本
 * 支持多平台、多架构的自动化构建
 */

import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';
import path from 'path';
import { fileURLToPath } from 'url';
import * as logger from './utils/logger.js';
import * as executor from './utils/executor.js';
import fs from 'fs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// 获取项目根目录
const projectRoot = path.join(__dirname, '../..');

/**
 * 加载配置文件
 * @param {string} filename - 配置文件名
 * @returns {Object} 配置对象
 */
function loadConfig(filename) {
  const configPath = path.join(__dirname, '../configs', filename);

  if (!fs.existsSync(configPath)) {
    logger.error(`配置文件不存在: ${configPath}`);
    process.exit(1);
  }

  try {
    const content = fs.readFileSync(configPath, 'utf-8');
    return JSON.parse(content);
  } catch (error) {
    logger.error(`解析配置文件失败: ${error.message}`);
    process.exit(1);
  }
}

/**
 * 检查环境
 */
function checkEnvironment() {
  logger.title('检查构建环境');

  const { platform, isWindows, isMacOS, isLinux } = executor.getPlatformInfo();

  logger.info(`检测到平台: ${platform}`);
  logger.info(`架构: ${process.arch}`);

  // 检查必需的命令
  const requiredCommands = ['node', 'cargo', 'rustc'];
  for (const cmd of requiredCommands) {
    if (executor.commandExists(cmd)) {
      logger.success(`${cmd}: 已安装`);
    } else {
      logger.error(`${cmd}: 未安装`);
      process.exit(1);
    }
  }

  // 检查 Tauri CLI
  if (executor.commandExists('cargo-tauri') || executor.commandExists('tauri')) {
    logger.success(`Tauri CLI: 已安装`);
  } else {
    logger.error(`Tauri CLI: 未安装`);
    logger.info(`运行: cargo install tauri-cli --version "^2.0.0"`);
    process.exit(1);
  }

  logger.newline();
}

/**
 * 构建项目
 * @param {Object} options - 构建选项
 */
async function build(options) {
  const { platform, arch, allPlatforms } = options;

  logger.title('RTodo 跨平台构建');
  logger.newline();

  // 检查环境
  checkEnvironment();

  // 加载配置
  const targets = loadConfig('targets.json');
  const version = loadConfig('version.json');

  logger.info(`应用版本: ${version.current}`);
  logger.info(`构建号: ${version.buildNumber}`);
  logger.newline();

  // 确定要构建的平台
  let platformsToBuild = [];

  if (allPlatforms) {
    // 构建所有平台
    if (process.platform === 'win32') {
      platformsToBuild.push('windows');
    } else if (process.platform === 'darwin') {
      platformsToBuild.push('macos');
    } else if (process.platform === 'linux') {
      platformsToBuild.push('linux');
    }
  } else if (platform) {
    // 用户指定的平台
    const platformMap = {
      'win': 'windows',
      'windows': 'windows',
      'linux': 'linux',
      'mac': 'macos',
      'macos': 'macos',
      'osx': 'macos',
    };
    platformsToBuild.push(platformMap[platform.toLowerCase()] || platform);
  } else {
    // 自动检测当前平台
    const currentPlatform = process.platform;
    if (currentPlatform === 'win32') {
      platformsToBuild.push('windows');
    } else if (currentPlatform === 'darwin') {
      platformsToBuild.push('macos');
    } else if (currentPlatform === 'linux') {
      platformsToBuild.push('linux');
    }
  }

  // 执行构建
  for (const platformName of platformsToBuild) {
    try {
      const buildModule = await import(`./build-${platformName}.js`);
      await buildModule.default({ arch, targets, version, projectRoot });
    } catch (error) {
      logger.error(`构建 ${platformName} 失败: ${error.message}`);
      if (!options.ignoreErrors) {
        process.exit(1);
      }
    }
  }

  logger.newline();
  logger.title('构建完成');

  // 显示构建产物位置
  const outputDir = path.join(projectRoot, 'src-tauri/target');
  logger.info(`构建产物位置: ${outputDir}`);
  logger.newline();
}

/**
 * 主函数
 */
async function main() {
  // 解析命令行参数
  const argv = yargs(hideBin(process.argv))
    .option('platform', {
      alias: 'p',
      type: 'string',
      description: '目标平台 (windows, linux, macos)',
      choices: ['windows', 'linux', 'macos', 'win', 'mac', 'osx'],
    })
    .option('arch', {
      alias: 'a',
      type: 'string',
      description: '目标架构 (x86_64, arm64, universal)',
      choices: ['x86_64', 'arm64', 'universal'],
    })
    .option('all-platforms', {
      type: 'boolean',
      description: '构建所有平台（仅在当前平台上可用）',
      default: false,
    })
    .option('ignore-errors', {
      type: 'boolean',
      description: '忽略错误继续构建',
      default: false,
    })
    .option('verbose', {
      alias: 'v',
      type: 'boolean',
      description: '详细输出',
      default: false,
    })
    .help()
    .alias('help', 'h')
    .version(false)
    .strict()
    .argv;

  // 设置日志级别
  if (argv.verbose) {
    logger.setLogLevel(logger.LogLevel.DEBUG);
  }

  // 执行构建
  await build({
    platform: argv.platform,
    arch: argv.arch,
    allPlatforms: argv.allPlatforms,
    ignoreErrors: argv.ignoreErrors,
  });
}

// 运行主函数
main().catch(error => {
  logger.error(`构建失败: ${error.message}`);
  logger.debug(error.stack);
  process.exit(1);
});
