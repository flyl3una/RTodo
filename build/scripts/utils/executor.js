// 命令执行工具模块
import { execSync, spawn } from 'child_process';
import { promisify } from 'util';
import * as logger from './logger.js';

const execAsync = promisify(exec);

/**
 * 执行选项
 * @typedef {Object} ExecOptions
 * @property {string} cwd - 工作目录
 * @property {boolean} silent - 是否静默输出
 * @property {boolean} ignoreError - 是否忽略错误
 * @property {number} timeout - 超时时间（毫秒）
 * @property {Object} env - 环境变量
 */

/**
 * 默认执行选项
 * @type {ExecOptions}
 */
const defaultOptions = {
  cwd: process.cwd(),
  silent: false,
  ignoreError: false,
  timeout: 300000, // 5 分钟
  env: process.env,
};

/**
 * 执行命令（同步）
 * @param {string} command - 要执行的命令
 * @param {ExecOptions} options - 执行选项
 * @returns {string} 命令输出
 */
export function exec(command, options = {}) {
  const opts = { ...defaultOptions, ...options };

  if (!opts.silent) {
    logger.info(`执行命令: ${command}`);
  }

  try {
    const output = execSync(command, {
      cwd: opts.cwd,
      encoding: 'utf-8',
      stdio: opts.silent ? 'pipe' : 'inherit',
      timeout: opts.timeout,
      env: opts.env,
    });

    if (!opts.silent && output) {
      logger.success(`命令执行成功`);
    }

    return output;
  } catch (error) {
    if (!opts.ignoreError) {
      logger.error(`命令执行失败: ${error.message}`);
      throw error;
    }
    return '';
  }
}

/**
 * 执行命令（异步）
 * @param {string} command - 要执行的命令
 * @param {ExecOptions} options - 执行选项
 * @returns {Promise<{stdout: string, stderr: string}>} 命令输出
 */
export async function execAsyncCmd(command, options = {}) {
  const opts = { ...defaultOptions, ...options };

  if (!opts.silent) {
    logger.info(`执行命令: ${command}`);
  }

  try {
    const { stdout, stderr } = await execAsync(command, {
      cwd: opts.cwd,
      encoding: 'utf-8',
      stdio: opts.silent ? 'pipe' : 'inherit',
      timeout: opts.timeout,
      env: opts.env,
    });

    if (!opts.silent) {
      logger.success(`命令执行成功`);
    }

    return { stdout, stderr };
  } catch (error) {
    if (!opts.ignoreError) {
      logger.error(`命令执行失败: ${error.message}`);
      throw error;
    }
    return { stdout: '', stderr: '' };
  }
}

/**
 * 执行命令并返回 JSON（同步）
 * @param {string} command - 要执行的命令
 * @param {ExecOptions} options - 执行选项
 * @returns {Object} 解析后的 JSON 对象
 */
export function execJSON(command, options = {}) {
  const output = exec(command, { ...options, silent: true });
  try {
    return JSON.parse(output);
  } catch (error) {
    logger.error(`解析 JSON 失败: ${error.message}`);
    logger.debug(`原始输出: ${output}`);
    throw error;
  }
}

/**
 * 执行命令并返回 JSON（异步）
 * @param {string} command - 要执行的命令
 * @param {ExecOptions} options - 执行选项
 * @returns {Promise<Object>} 解析后的 JSON 对象
 */
export async function execJSONAsync(command, options = {}) {
  const { stdout } = await execAsyncCmd(command, { ...options, silent: true });
  try {
    return JSON.parse(stdout);
  } catch (error) {
    logger.error(`解析 JSON 失败: ${error.message}`);
    logger.debug(`原始输出: ${stdout}`);
    throw error;
  }
}

/**
 * 检查命令是否存在
 * @param {string} command - 命令名称
 * @returns {boolean} 命令是否存在
 */
export function commandExists(command) {
  const platform = process.platform;

  try {
    if (platform === 'win32') {
      exec(`where ${command}`, { silent: true });
    } else {
      exec(`which ${command}`, { silent: true });
    }
    return true;
  } catch {
    return false;
  }
}

/**
 * 获取命令路径
 * @param {string} command - 命令名称
 * @returns {string|null} 命令路径
 */
export function getCommandPath(command) {
  const platform = process.platform;

  try {
    if (platform === 'win32') {
      return exec(`where ${command}`, { silent: true }).trim().split('\n')[0];
    } else {
      return exec(`which ${command}`, { silent: true }).trim();
    }
  } catch {
    return null;
  }
}

/**
 * 执行交互式命令
 * @param {string} command - 要执行的命令
 * @param {string[]} args - 命令参数
 * @param {ExecOptions} options - 执行选项
 * @returns {Promise<number>} 退出代码
 */
export function execInteractive(command, args = [], options = {}) {
  const opts = { ...defaultOptions, ...options };

  return new Promise((resolve, reject) => {
    if (!opts.silent) {
      logger.info(`执行交互式命令: ${command} ${args.join(' ')}`);
    }

    const child = spawn(command, args, {
      cwd: opts.cwd,
      stdio: 'inherit',
      env: opts.env,
      shell: true,
    });

    child.on('close', (code) => {
      if (code === 0) {
        if (!opts.silent) {
          logger.success(`命令执行成功`);
        }
        resolve(code);
      } else {
        if (!opts.ignoreError) {
          logger.error(`命令退出代码: ${code}`);
          reject(new Error(`Command failed with exit code ${code}`));
        } else {
          resolve(code);
        }
      }
    });

    child.on('error', (error) => {
      if (!opts.ignoreError) {
        logger.error(`命令执行错误: ${error.message}`);
        reject(error);
      } else {
        resolve(1);
      }
    });
  });
}

/**
 * 执行一系列命令
 * @param {Array<string|{command: string, options?: ExecOptions}>} commands - 命令列表
 * @returns {Array<string>} 所有命令的输出
 */
export function execSeries(commands) {
  const outputs = [];

  for (const cmd of commands) {
    let command, options;

    if (typeof cmd === 'string') {
      command = cmd;
      options = {};
    } else {
      command = cmd.command;
      options = cmd.options || {};
    }

    try {
      const output = exec(command, options);
      outputs.push(output);
    } catch (error) {
      if (!options?.ignoreError) {
        throw error;
      }
      outputs.push('');
    }
  }

  return outputs;
}

/**
 * 并行执行多个命令
 * @param {Array<string|{command: string, options?: ExecOptions}>} commands - 命令列表
 * @returns {Promise<Array<string>>} 所有命令的输出
 */
export async function execParallel(commands) {
  const promises = commands.map(cmd => {
    let command, options;

    if (typeof cmd === 'string') {
      command = cmd;
      options = {};
    } else {
      command = cmd.command;
      options = cmd.options || {};
    }

    return execAsyncCmd(command, options).catch(error => {
      if (!options?.ignoreError) {
        throw error;
      }
      return { stdout: '', stderr: '' };
    });
  });

  const results = await Promise.all(promises);
  return results.map(r => r.stdout);
}

/**
 * 重试执行命令
 * @param {string} command - 要执行的命令
 * @param {ExecOptions} options - 执行选项
 * @param {number} retries - 重试次数
 * @param {number} delay - 重试延迟（毫秒）
 * @returns {Promise<string>} 命令输出
 */
export async function execWithRetry(command, options = {}, retries = 3, delay = 1000) {
  for (let i = 0; i < retries; i++) {
    try {
      if (i > 0) {
        logger.warning(`重试 (${i}/${retries - 1})...`);
      }
      return await execAsyncCmd(command, options);
    } catch (error) {
      if (i === retries - 1) {
        throw error;
      }
      logger.debug(`等待 ${delay}ms 后重试...`);
      await new Promise(resolve => setTimeout(resolve, delay));
    }
  }
}

/**
 * 读取环境变量
 * @param {string} name - 变量名
 * @param {string} defaultValue - 默认值
 * @returns {string} 环境变量值
 */
export function getEnv(name, defaultValue = '') {
  return process.env[name] || defaultValue;
}

/**
 * 设置环境变量
 * @param {string} name - 变量名
 * @param {string} value - 变量值
 */
export function setEnv(name, value) {
  process.env[name] = value;
}

/**
 * 获取平台信息
 * @returns {Object} 平台信息
 */
export function getPlatformInfo() {
  return {
    platform: process.platform,
    arch: process.arch,
    nodeVersion: process.version,
    isWindows: process.platform === 'win32',
    isMacOS: process.platform === 'darwin',
    isLinux: process.platform === 'linux',
  };
}

export default {
  exec,
  execAsyncCmd,
  execJSON,
  execJSONAsync,
  commandExists,
  getCommandPath,
  execInteractive,
  execSeries,
  execParallel,
  execWithRetry,
  getEnv,
  setEnv,
  getPlatformInfo,
};
