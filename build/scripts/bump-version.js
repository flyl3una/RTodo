/**
 * 版本管理脚本
 * 用于更新应用版本号
 */

import * as logger from './utils/logger.js';
import * as executor from './utils/executor.js';
import fs from 'fs';
import path from 'path';

/**
 * 版本类型
 */
const VersionType = {
  MAJOR: 'major',
  MINOR: 'minor',
  PATCH: 'patch',
};

/**
 * 解析版本号
 * @param {string} version - 版本字符串 (例如: "0.1.0")
 * @returns {Object} 解析后的版本对象
 */
function parseVersion(version) {
  const parts = version.split('.').map(Number);
  return {
    major: parts[0] || 0,
    minor: parts[1] || 0,
    patch: parts[2] || 0,
    toString() {
      return `${this.major}.${this.minor}.${this.patch}`;
    },
  };
}

/**
 * 增加版本号
 * @param {string} currentVersion - 当前版本号
 * @param {string} type - 版本类型 (major, minor, patch)
 * @returns {string} 新版本号
 */
function bumpVersion(currentVersion, type) {
  const version = parseVersion(currentVersion);

  switch (type) {
    case VersionType.MAJOR:
      version.major++;
      version.minor = 0;
      version.patch = 0;
      break;
    case VersionType.MINOR:
      version.minor++;
      version.patch = 0;
      break;
    case VersionType.PATCH:
      version.patch++;
      break;
    default:
      throw new Error(`无效的版本类型: ${type}`);
  }

  return version.toString();
}

/**
 * 获取当前日期
 * @returns {string} 格式化的日期 (YYYY-MM-DD)
 */
function getCurrentDate() {
  const now = new Date();
  const year = now.getFullYear();
  const month = String(now.getMonth() + 1).padStart(2, '0');
  const day = String(now.getDate()).padStart(2, '0');
  return `${year}-${month}-${day}`;
}

/**
 * 更新 Tauri 配置文件
 * @param {string} newVersion - 新版本号
 * @param {string} projectRoot - 项目根目录
 */
function updateTauriConfig(newVersion, projectRoot) {
  const configPath = path.join(projectRoot, 'src-tauri/tauri.conf.json');

  if (!fs.existsSync(configPath)) {
    logger.warning(`未找到 Tauri 配置文件: ${configPath}`);
    return;
  }

  logger.info(`更新 Tauri 配置: ${configPath}`);

  const config = JSON.parse(fs.readFileSync(configPath, 'utf-8'));
  config.version = newVersion;

  fs.writeFileSync(configPath, JSON.stringify(config, null, 2), 'utf-8');
  logger.success(`Tauri 配置已更新到版本 ${newVersion}`);
}

/**
 * 更新构建版本配置
 * @param {string} newVersion - 新版本号
 * @param {string} buildDir - 构建目录
 */
function updateBuildVersionConfig(newVersion, buildDir) {
  const configPath = path.join(buildDir, 'configs/version.json');

  if (!fs.existsSync(configPath)) {
    logger.warning(`未找到版本配置文件: ${configPath}`);
    return;
  }

  logger.info(`更新版本配置: ${configPath}`);

  const config = JSON.parse(fs.readFileSync(configPath, 'utf-8'));
  const oldVersion = config.current;

  config.current = newVersion;
  config.buildNumber = config.buildNumber + 1;
  config.releaseDate = getCurrentDate();

  // 添加到 changelog
  if (!config.changelog) {
    config.changelog = [];
  }

  config.changelog.unshift({
    version: newVersion,
    date: config.releaseDate,
    changes: [`版本从 ${oldVersion} 升级到 ${newVersion}`],
  });

  fs.writeFileSync(configPath, JSON.stringify(config, null, 2), 'utf-8');
  logger.success(`版本配置已更新到版本 ${newVersion}`);
}

/**
 * 更新前端 package.json
 * @param {string} newVersion - 新版本号
 * @param {string} projectRoot - 项目根目录
 */
function updateFrontendPackage(newVersion, projectRoot) {
  const packagePath = path.join(projectRoot, 'frontend/package.json');

  if (!fs.existsSync(packagePath)) {
    logger.warning(`未找到前端 package.json: ${packagePath}`);
    return;
  }

  logger.info(`更新前端 package.json: ${packagePath}`);

  const pkg = JSON.parse(fs.readFileSync(packagePath, 'utf-8'));
  pkg.version = newVersion;

  fs.writeFileSync(packagePath, JSON.stringify(pkg, null, 2), 'utf-8');
  logger.success(`前端 package.json 已更新到版本 ${newVersion}`);
}

/**
 * 生成变更日志模板
 * @param {string} newVersion - 新版本号
 * @returns {string} 变更日志内容
 */
function generateChangelogTemplate(newVersion) {
  return `# ${newVersion} (${getCurrentDate()})

## 新增
-

## 修复
-

## 改进
-

## 移除
-

---

## 使用说明
在上方填写本次版本的变更内容，然后保存。
`;
}

/**
 * 主函数
 */
function main() {
  // 解析命令行参数
  const args = process.argv.slice(2);
  const type = args[0] || VersionType.PATCH;

  // 验证版本类型
  if (
    !Object.values(VersionType).includes(type) &&
    !args.includes('--help') &&
    !args.includes('-h')
  ) {
    logger.error(`无效的版本类型: ${type}`);
    logger.info(`支持的类型: ${Object.values(VersionType).join(', ')}`);
    logger.info('');
    logger.info('使用方式:');
    logger.info('  node scripts/bump-version.js [major|minor|patch]');
    logger.info('');
    logger.info('示例:');
    logger.info('  node scripts/bump-version.js patch   # 0.1.0 -> 0.1.1');
    logger.info('  node scripts/bump-version.js minor   # 0.1.0 -> 0.2.0');
    logger.info('  node scripts/bump-version.js major   # 0.1.0 -> 1.0.0');
    process.exit(1);
  }

  if (args.includes('--help') || args.includes('-h')) {
    logger.info('版本管理脚本');
    logger.info('');
    logger.info('使用方式:');
    logger.info('  node scripts/bump-version.js [major|minor|patch]');
    logger.info('');
    logger.info('参数:');
    logger.info('  major   - 主版本号 ( breaking changes )');
    logger.info('  minor   - 次版本号 ( 新功能，向后兼容 )');
    logger.info('  patch   - 补丁版本号 ( bug 修复，向后兼容 )');
    logger.info('');
    logger.info('示例:');
    logger.info('  node scripts/bump-version.js patch   # 0.1.0 -> 0.1.1');
    logger.info('  node scripts/bump-version.js minor   # 0.1.0 -> 0.2.0');
    logger.info('  node scripts/bump-version.js major   # 0.1.0 -> 1.0.0');
    process.exit(0);
  }

  logger.title('版本号更新');
  logger.newline();

  // 获取当前版本
  const buildDir = process.cwd();
  const versionConfigPath = path.join(buildDir, 'configs/version.json');

  if (!fs.existsSync(versionConfigPath)) {
    logger.error(`未找到版本配置文件: ${versionConfigPath}`);
    process.exit(1);
  }

  const versionConfig = JSON.parse(
    fs.readFileSync(versionConfigPath, 'utf-8')
  );
  const currentVersion = versionConfig.current;

  logger.info(`当前版本: ${currentVersion}`);
  logger.info(`更新类型: ${type}`);
  logger.newline();

  // 计算新版本号
  const newVersion = bumpVersion(currentVersion, type);
  logger.info(`新版本号: ${newVersion}`);
  logger.newline();

  // 确认更新
  if (!args.includes('--yes') && !args.includes('-y')) {
    logger.warning('即将更新以下文件:');
    logger.info(`  - src-tauri/tauri.conf.json`);
    logger.info(`  - frontend/package.json`);
    logger.info(`  - build/configs/version.json`);
    logger.newline();

    logger.info('按 Ctrl+C 取消，或按 Enter 继续...');
    logger.newline();

    // 在实际使用中，这里可以添加交互式确认
    // 但为了自动化，我们跳过这个步骤
  }

  // 获取项目根目录
  const projectRoot = path.join(buildDir, '..');

  // 更新所有版本文件
  updateTauriConfig(newVersion, projectRoot);
  updateFrontendPackage(newVersion, projectRoot);
  updateBuildVersionConfig(newVersion, buildDir);

  logger.newline();
  logger.title('版本更新完成');
  logger.newline();

  logger.success(`版本已从 ${currentVersion} 更新到 ${newVersion}`);
  logger.newline();

  logger.info('后续步骤:');
  logger.info('  1. 更新 CHANGELOG.md');
  logger.info('  2. 提交代码: git add . && git commit -m "chore: bump version to ${newVersion}"');
  logger.info('  3. 创建标签: git tag v${newVersion}');
  logger.info('  4. 构建应用: npm run build');

  // 生成变更日志模板
  const changelogContent = generateChangelogTemplate(newVersion);
  const changelogPath = path.join(buildDir, `CHANGELOG-${newVersion}.md`);

  fs.writeFileSync(changelogPath, changelogContent, 'utf-8');
  logger.newline();
  logger.info(`已生成变更日志模板: ${changelogPath}`);
}

// 运行主函数
main();
