# Copyright 2025 RTodo Team. All rights reserved.
# SPDX-License-Identifier: MIT
"""
日志工具模块
提供彩色日志输出、进度条、加载器等功能
"""

import sys
import time
import threading
from enum import IntEnum
from datetime import datetime
from typing import Optional, List, Union


class LogLevel(IntEnum):
    """日志级别"""
    DEBUG = 0
    INFO = 1
    SUCCESS = 2
    WARNING = 3
    ERROR = 4


class Colors:
    """ANSI 颜色代码"""
    RESET = '\033[0m'
    BOLD = '\033[1m'

    # 前景色
    BLACK = '\033[30m'
    RED = '\033[31m'
    GREEN = '\033[32m'
    YELLOW = '\033[33m'
    BLUE = '\033[34m'
    MAGENTA = '\033[35m'
    CYAN = '\033[36m'
    WHITE = '\033[37m'

    # 亮色
    GRAY = '\033[90m'
    BRIGHT_RED = '\033[91m'
    BRIGHT_GREEN = '\033[92m'
    BRIGHT_YELLOW = '\033[93m'
    BRIGHT_BLUE = '\033[94m'
    BRIGHT_MAGENTA = '\033[95m'
    BRIGHT_CYAN = '\033[96m'

    # Windows 下禁用颜色（如果需要）
    DISABLED = ''


# 检测是否在 Windows 上且支持颜色
def _supports_color() -> bool:
    """检测终端是否支持颜色"""
    if sys.platform == 'win32':
        try:
            import ctypes
            import ctypes.wintypes

            kernel32 = ctypes.windll.kernel32
            kernel32.GetConsoleMode(kernel32.GetStdHandle(-11), ctypes.byref(ctypes.wintypes.DWORD()))
            return True
        except:
            return False
    return sys.stdout.isatty()


_supports_color = _supports_color()

if not _supports_color:
    # 禁用所有颜色代码
    for attr in dir(Colors):
        if not attr.startswith('_') and attr.isupper():
            setattr(Colors, attr, '')


class Logger:
    """日志工具类"""

    def __init__(self, level: LogLevel = LogLevel.INFO):
        """
        初始化日志工具
        :param level: 日志级别
        """
        self.level = level
        self._spinners = {}

    def set_level(self, level: LogLevel):
        """设置日志级别"""
        self.level = level

    def _get_timestamp(self) -> str:
        """获取格式化的时间戳"""
        now = datetime.now()
        return now.strftime('%H:%M:%S')

    def _log(self, level: LogLevel, message: str, *args, **kwargs):
        """
        内部日志输出方法
        :param level: 日志级别
        :param message: 日志消息
        :param args: 额外参数
        :param kwargs: 额外关键字参数
        """
        if level < self.level:
            return

        timestamp = self._get_timestamp()
        level_name = LogLevel(level).name

        # 根据级别设置颜色
        if level == LogLevel.DEBUG:
            level_color = Colors.GRAY
            message_color = Colors.GRAY
        elif level == LogLevel.INFO:
            level_color = Colors.BLUE
            message_color = Colors.WHITE
        elif level == LogLevel.SUCCESS:
            level_color = Colors.GREEN
            message_color = Colors.GREEN
        elif level == LogLevel.WARNING:
            level_color = Colors.YELLOW
            message_color = Colors.YELLOW
        elif level == LogLevel.ERROR:
            level_color = Colors.RED
            message_color = Colors.RED
        else:
            level_color = ''
            message_color = ''

        formatted_message = f"{message_color}{message}{Colors.RESET}"
        if args:
            formatted_message += ' ' + ' '.join(str(arg) for arg in args)

        print(f"[{timestamp}] [{level_color}{level_name}{Colors.RESET}] {formatted_message}")

    def debug(self, message: str, *args):
        """调试日志"""
        self._log(LogLevel.DEBUG, message, *args)

    def info(self, message: str, *args):
        """信息日志"""
        self._log(LogLevel.INFO, message, *args)

    def success(self, message: str, *args):
        """成功日志"""
        self._log(LogLevel.SUCCESS, message, *args)

    def warning(self, message: str, *args):
        """警告日志"""
        self._log(LogLevel.WARNING, message, *args)

    def error(self, message: str, *args):
        """错误日志"""
        self._log(LogLevel.ERROR, message, *args)

    def separator(self, char: str = '=', length: int = 50):
        """输出分隔线"""
        print(char * length)

    def title(self, text: str):
        """输出标题"""
        self.separator('=', 60)
        print(f"{Colors.BOLD}{Colors.CYAN}{text}{Colors.RESET}")
        self.separator('=', 60)

    def subtitle(self, text: str):
        """输出子标题"""
        print(f"{Colors.BOLD}{Colors.YELLOW}{text}{Colors.RESET}")
        self.separator('-', 40)

    def list(self, items: List[Union[str, tuple]]):
        """输出列表"""
        for item in items:
            if isinstance(item, tuple):
                symbol, text = item
                print(f"  {symbol} {text}")
            else:
                print(f"  • {item}")

    def key_value(self, data: dict):
        """输出键值对"""
        max_key_len = max(len(str(k)) for k in data.keys())
        for key, value in data.items():
            padded_key = str(key).ljust(max_key_len)
            print(f"  {Colors.CYAN}{padded_key}{Colors.RESET}: {value}")

    def newline(self):
        """输出空行"""
        print()

    def clear(self):
        """清屏"""
        print('\033[2J\033[H', end='')


class Spinner:
    """旋转加载器"""

    SPINNER_CHARS = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏']

    def __init__(self, text: str):
        """
        初始化加载器
        :param text: 加载文本
        """
        self.text = text
        self._running = False
        self._thread = None
        self._index = 0

    def _spin(self):
        """旋转动画"""
        while self._running:
            sys.stdout.write(f"\r{Colors.CYAN}{self.SPINNER_CHARS[self._index]}{Colors.RESET} {self.text}")
            sys.stdout.flush()
            self._index = (self._index + 1) % len(self.SPINNER_CHARS)
            time.sleep(0.08)

    def start(self):
        """开始旋转"""
        if not self._running:
            self._running = True
            self._thread = threading.Thread(target=self._spin, daemon=True)
            self._thread.start()

    def stop(self, final_text: Optional[str] = None, success: bool = True):
        """
        停止旋转
        :param final_text: 最终文本
        :param success: 是否成功
        """
        self._running = False
        if self._thread:
            self._thread.join(timeout=0.5)
            self._thread = None

        # 使用 ASCII 字符以兼容 Windows GBK 编码
        # 在 Windows 上，Unicode 符号 ✓ 和 ✗ 会导致编码错误
        if success:
            symbol = '[OK]'
        else:
            symbol = '[FAIL]'

        color = Colors.GREEN if success else Colors.RED
        text = final_text or self.text
        sys.stdout.write(f"\r{color}{symbol}{Colors.RESET} {text}\n")
        sys.stdout.flush()

    def succeed(self, text: str):
        """成功完成"""
        self.stop(text, True)

    def fail(self, text: str):
        """失败完成"""
        self.stop(text, False)


class ProgressBar:
    """进度条"""

    def __init__(self, text: str, total: int):
        """
        初始化进度条
        :param text: 进度条文本
        :param total: 总数
        """
        self.text = text
        self.total = total
        self.current = 0

    def update(self, value: int):
        """
        更新进度
        :param value: 当前进度值
        """
        self.current = min(value, self.total)
        percentage = int((self.current / self.total) * 100)
        filled = percentage // 2
        empty = 50 - filled
        bar = '█' * filled + '░' * empty

        sys.stdout.write(f"\r{self.text}: [{bar}] {percentage}%")
        if self.current >= self.total:
            sys.stdout.write('\n')
        sys.stdout.flush()

    def increment(self, value: int = 1):
        """
        增加进度
        :param value: 增加的值
        """
        self.update(self.current + value)

    def complete(self):
        """完成进度"""
        self.update(self.total)


# 全局日志实例
logger = Logger()
