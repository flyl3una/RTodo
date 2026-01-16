// 日志工具模块
import chalk from 'chalk';

/**
 * 日志级别
 */
export const LogLevel = {
  DEBUG: 0,
  INFO: 1,
  SUCCESS: 2,
  WARNING: 3,
  ERROR: 4,
};

/**
 * 当前日志级别
 */
let currentLevel = LogLevel.INFO;

/**
 * 设置日志级别
 * @param {number} level - 日志级别
 */
export function setLogLevel(level) {
  currentLevel = level;
}

/**
 * 获取日志级别名称
 * @param {number} level - 日志级别
 * @returns {string} 级别名称
 */
function getLevelName(level) {
  const names = {
    [LogLevel.DEBUG]: 'DEBUG',
    [LogLevel.INFO]: 'INFO',
    [LogLevel.SUCCESS]: 'SUCCESS',
    [LogLevel.WARNING]: 'WARNING',
    [LogLevel.ERROR]: 'ERROR',
  };
  return names[level] || 'UNKNOWN';
}

/**
 * 格式化时间戳
 * @returns {string} 格式化的时间戳
 */
function getTimestamp() {
  const now = new Date();
  const hours = String(now.getHours()).padStart(2, '0');
  const minutes = String(now.getMinutes()).padStart(2, '0');
  const seconds = String(now.getSeconds()).padStart(2, '0');
  return `${hours}:${minutes}:${seconds}`;
}

/**
 * 日志输出函数
 * @param {number} level - 日志级别
 * @param {string} message - 日志消息
 * @param {...any} args - 额外参数
 */
function log(level, message, ...args) {
  if (level < currentLevel) return;

  const timestamp = getTimestamp();
  const levelName = getLevelName(level);

  let coloredMessage;
  let coloredLevel;

  switch (level) {
    case LogLevel.DEBUG:
      coloredLevel = chalk.gray(levelName);
      coloredMessage = chalk.gray(message);
      break;
    case LogLevel.INFO:
      coloredLevel = chalk.blue(levelName);
      coloredMessage = chalk.white(message);
      break;
    case LogLevel.SUCCESS:
      coloredLevel = chalk.green(levelName);
      coloredMessage = chalk.green(message);
      break;
    case LogLevel.WARNING:
      coloredLevel = chalk.yellow(levelName);
      coloredMessage = chalk.yellow(message);
      break;
    case LogLevel.ERROR:
      coloredLevel = chalk.red(levelName);
      coloredMessage = chalk.red(message);
      break;
    default:
      coloredLevel = levelName;
      coloredMessage = message;
  }

  console.log(`[${timestamp}] [${coloredLevel}]`, coloredMessage, ...args);
}

/**
 * 调试日志
 * @param {string} message - 日志消息
 * @param {...any} args - 额外参数
 */
export function debug(message, ...args) {
  log(LogLevel.DEBUG, message, ...args);
}

/**
 * 信息日志
 * @param {string} message - 日志消息
 * @param {...any} args - 额外参数
 */
export function info(message, ...args) {
  log(LogLevel.INFO, message, ...args);
}

/**
 * 成功日志
 * @param {string} message - 日志消息
 * @param {...any} args - 额外参数
 */
export function success(message, ...args) {
  log(LogLevel.SUCCESS, message, ...args);
}

/**
 * 警告日志
 * @param {string} message - 日志消息
 * @param {...any} args - 额外参数
 */
export function warning(message, ...args) {
  log(LogLevel.WARNING, message, ...args);
}

/**
 * 错误日志
 * @param {string} message - 日志消息
 * @param {...any} args - 额外参数
 */
export function error(message, ...args) {
  log(LogLevel.ERROR, message, ...args);
}

/**
 * 创建进度条
 * @param {string} text - 进度条文本
 * @param {number} total - 总数
 * @returns {Object} 进度条对象
 */
export function createProgressBar(text, total) {
  let current = 0;

  return {
    /**
     * 更新进度
     * @param {number} value - 当前进度值
     */
    update(value) {
      current = value;
      const percentage = Math.floor((current / total) * 100);
      const filled = Math.floor(percentage / 2);
      const empty = 50 - filled;
      const bar = '█'.repeat(filled) + '░'.repeat(empty);

      process.stdout.write(`\r${text}: [${bar}] ${percentage}%`);
      if (current === total) {
        process.stdout.write('\n');
      }
    },

    /**
     * 增加进度
     * @param {number} value - 增加的值
     */
    increment(value = 1) {
      this.update(current + value);
    },

    /**
     * 完成进度
     */
    complete() {
      this.update(total);
    },
  };
}

/**
 * 创建旋转加载器
 * @param {string} text - 加载文本
 * @param {string[]} chars - 旋转字符数组
 * @returns {Object} 加载器对象
 */
export function createSpinner(text, chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏']) {
  let index = 0;
  let interval = null;
  let currentText = text;

  return {
    /**
     * 开始旋转
     */
    start() {
      interval = setInterval(() => {
        process.stdout.write(`\r${chars[index]} ${currentText}`);
        index = (index + 1) % chars.length;
      }, 80);
    },

    /**
     * 停止旋转
     * @param {string} finalText - 最终文本
     * @param {boolean} success - 是否成功
     */
    stop(finalText, success = true) {
      if (interval) {
        clearInterval(interval);
        interval = null;
      }
      const symbol = success ? '✓' : '✗';
      const coloredSymbol = success ? chalk.green(symbol) : chalk.red(symbol);
      process.stdout.write(`\r${coloredSymbol} ${finalText || currentText}\n`);
    },

    /**
     * 更新文本
     * @param {string} text - 新文本
     */
    setText(text) {
      currentText = text;
    },

    /**
     * 成功完成
     * @param {string} text - 成功文本
     */
    succeed(text) {
      this.stop(text, true);
    },

    /**
     * 失败完成
     * @param {string} text - 失败文本
     */
    fail(text) {
      this.stop(text, false);
    },
  };
}

/**
 * 分隔线
 * @param {string} char - 分隔字符
 * @param {number} length - 长度
 */
export function separator(char = '=', length = 50) {
  console.log(char.repeat(length));
}

/**
 * 标题
 * @param {string} text - 标题文本
 */
export function title(text) {
  separator('=', 60);
  console.log(chalk.bold.cyan(text));
  separator('=', 60);
}

/**
 * 子标题
 * @param {string} text - 子标题文本
 */
export function subtitle(text) {
  console.log(chalk.bold.yellow(text));
  separator('-', 40);
}

/**
 * 列表
 * @param {Array<string|Array>} items - 列表项，可以是字符串或 [符号, 文本] 数组
 */
export function list(items) {
  items.forEach(item => {
    if (Array.isArray(item)) {
      const [symbol, text] = item;
      console.log(`  ${symbol} ${text}`);
    } else {
      console.log(`  • ${item}`);
    }
  });
}

/**
 * 键值对列表
 * @param {Object} obj - 键值对对象
 */
export function keyValue(obj) {
  const maxKeyLength = Math.max(...Object.keys(obj).map(k => k.length));

  Object.entries(obj).forEach(([key, value]) => {
    const paddedKey = key.padEnd(maxKeyLength);
    console.log(`  ${chalk.cyan(paddedKey)}: ${value}`);
  });
}

/**
 * 空行
 */
export function newline() {
  console.log();
}

/**
 * 清屏
 */
export function clear() {
  console.clear();
}

export default {
  debug,
  info,
  success,
  warning,
  error,
  createProgressBar,
  createSpinner,
  separator,
  title,
  subtitle,
  list,
  keyValue,
  newline,
  clear,
  setLogLevel,
  LogLevel,
};
