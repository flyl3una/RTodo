# Copyright 2025 RTodo Team. All rights reserved.
# SPDX-License-Identifier: MIT
"""
命令执行工具模块
提供跨平台命令执行、环境变量管理等功能
"""

import os
import sys
import platform
import subprocess
import shutil
from typing import Optional, Dict, List, Union, Tuple
from concurrent.futures import ThreadPoolExecutor, as_completed
from .logger import logger


class ExecResult:
    """命令执行结果"""

    def __init__(self, stdout: str, stderr: str, returncode: int):
        self.stdout = stdout
        self.stderr = stderr
        self.returncode = returncode

    @property
    def success(self) -> bool:
        """是否执行成功"""
        return self.returncode == 0

    def __str__(self) -> str:
        return self.stdout or self.stderr


class Executor:
    """命令执行工具类"""

    def __init__(self):
        """初始化执行器"""
        self.platform_info = self._get_platform_info()

    def _get_platform_info(self) -> dict:
        """获取平台信息"""
        return {
            'platform': sys.platform,
            'arch': platform.machine(),
            'is_windows': sys.platform == 'win32',
            'is_macos': sys.platform == 'darwin',
            'is_linux': sys.platform.startswith('linux'),
        }

    def exec(self, command: str, cwd: Optional[str] = None,
             silent: bool = False, ignore_error: bool = False,
             timeout: Optional[int] = None, env: Optional[dict] = None) -> str:
        """
        执行命令（同步）
        :param command: 要执行的命令
        :param cwd: 工作目录
        :param silent: 是否静默输出
        :param ignore_error: 是否忽略错误
        :param timeout: 超时时间（秒）
        :param env: 环境变量
        :return: 命令输出
        """
        if not silent:
            logger.info(f"执行命令: {command}")

        # 处理工作目录
        work_dir = cwd or os.getcwd()

        # 合并环境变量
        exec_env = os.environ.copy()
        if env:
            exec_env.update(env)

        try:
            # Windows 下特殊处理
            if self.platform_info['is_windows']:
                # 使用 shell=True 来支持管道和重定向
                result = subprocess.run(
                    command,
                    shell=True,
                    cwd=work_dir,
                    env=exec_env,
                    capture_output=silent,
                    text=True,
                    timeout=timeout,
                    encoding='utf-8',
                    errors='replace'
                )

                if silent:
                    output = result.stdout
                else:
                    output = result.stdout
                    logger.success("命令执行成功")

                if result.returncode != 0 and not ignore_error:
                    error_msg = result.stderr or f"命令返回错误码: {result.returncode}"
                    logger.error(f"命令执行失败: {error_msg}")
                    raise RuntimeError(f"{command}\n{error_msg}")

                return output

            else:
                # Unix-like 系统
                result = subprocess.run(
                    command,
                    shell=True,
                    cwd=work_dir,
                    env=exec_env,
                    capture_output=silent,
                    text=True,
                    timeout=timeout,
                    encoding='utf-8',
                    errors='replace'
                )

                if silent:
                    output = result.stdout
                else:
                    output = result.stdout
                    logger.success("命令执行成功")

                if result.returncode != 0 and not ignore_error:
                    error_msg = result.stderr or f"命令返回错误码: {result.returncode}"
                    logger.error(f"命令执行失败: {error_msg}")
                    raise RuntimeError(f"{command}\n{error_msg}")

                return output

        except subprocess.TimeoutExpired:
            logger.error(f"命令执行超时: {command}")
            raise
        except Exception as e:
            if not ignore_error:
                logger.error(f"命令执行异常: {e}")
                raise
            return ''

    def exec_result(self, command: str, cwd: Optional[str] = None,
                   silent: bool = False, timeout: Optional[int] = None,
                   env: Optional[dict] = None) -> ExecResult:
        """
        执行命令并返回完整结果
        :param command: 要执行的命令
        :param cwd: 工作目录
        :param silent: 是否静默输出
        :param timeout: 超时时间（秒）
        :param env: 环境变量
        :return: 执行结果对象
        """
        if not silent:
            logger.info(f"执行命令: {command}")

        work_dir = cwd or os.getcwd()
        exec_env = os.environ.copy()
        if env:
            exec_env.update(env)

        try:
            result = subprocess.run(
                command,
                shell=True,
                cwd=work_dir,
                env=exec_env,
                capture_output=True,
                text=True,
                timeout=timeout,
                encoding='utf-8',
                errors='replace'
            )

            if not silent and result.returncode == 0:
                logger.success("命令执行成功")

            return ExecResult(
                stdout=result.stdout,
                stderr=result.stderr,
                returncode=result.returncode
            )

        except subprocess.TimeoutExpired:
            logger.error(f"命令执行超时: {command}")
            raise

    def command_exists(self, command: str) -> bool:
        """
        检查命令是否存在
        :param command: 命令名称
        :return: 命令是否存在
        """
        try:
            return shutil.which(command) is not None
        except Exception:
            return False

    def get_command_path(self, command: str) -> Optional[str]:
        """
        获取命令路径
        :param command: 命令名称
        :return: 命令路径
        """
        try:
            return shutil.which(command)
        except Exception:
            return None

    def exec_series(self, commands: List[Union[str, dict]]) -> List[str]:
        """
        顺序执行一系列命令
        :param commands: 命令列表，可以是字符串或 {'command': str, 'options': dict} 格式
        :return: 所有命令的输出列表
        """
        outputs = []

        for cmd in commands:
            if isinstance(cmd, str):
                command = cmd
                options = {}
            else:
                command = cmd.get('command', '')
                options = cmd.get('options', {})

            try:
                output = self.exec(command, **options)
                outputs.append(output)
            except Exception as e:
                if not options.get('ignore_error', False):
                    raise
                outputs.append('')

        return outputs

    def exec_parallel(self, commands: List[Union[str, dict]],
                     max_workers: int = 4) -> List[ExecResult]:
        """
        并行执行多个命令
        :param commands: 命令列表
        :param max_workers: 最大并行数
        :return: 所有命令的执行结果
        """
        def run_command(cmd):
            if isinstance(cmd, str):
                return self.exec_result(cmd, silent=True)
            else:
                command = cmd.get('command', '')
                options = {k: v for k, v in cmd.items() if k != 'command'}
                return self.exec_result(command, silent=True, **options)

        results = []
        with ThreadPoolExecutor(max_workers=max_workers) as executor:
            future_to_cmd = {executor.submit(run_command, cmd): cmd for cmd in commands}

            for future in as_completed(future_to_cmd):
                try:
                    result = future.result()
                    results.append(result)
                except Exception as e:
                    logger.error(f"并行命令执行失败: {e}")
                    results.append(ExecResult('', '', 1))

        return results

    def get_env(self, name: str, default: str = '') -> str:
        """
        读取环境变量
        :param name: 变量名
        :param default: 默认值
        :return: 环境变量值
        """
        return os.environ.get(name, default)

    def set_env(self, name: str, value: str):
        """
        设置环境变量
        :param name: 变量名
        :param value: 变量值
        """
        os.environ[name] = value


# 全局执行器实例
executor = Executor()
