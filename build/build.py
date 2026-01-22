#!/usr/bin/env python3
# Copyright 2025 RTodo Team. All rights reserved.
# SPDX-License-Identifier: MIT
"""
RTodo 主构建脚本
支持多平台、多架构的自动化构建
"""

import os
import sys
import json
import argparse
from pathlib import Path
from .utils.logger import logger, LogLevel
from .utils.executor import executor
from .platforms import build_windows, build_linux, build_macos


# 获取项目根目录
SCRIPT_DIR = Path(__file__).parent
PROJECT_ROOT = SCRIPT_DIR.parent


def load_config(filename: str) -> dict:
    """
    加载配置文件
    :param filename: 配置文件名
    :return: 配置对象
    """
    config_path = SCRIPT_DIR / 'configs' / filename

    if not config_path.exists():
        logger.error(f"配置文件不存在: {config_path}")
        sys.exit(1)

    try:
        with open(config_path, 'r', encoding='utf-8') as f:
            return json.load(f)
    except json.JSONDecodeError as e:
        logger.error(f"解析配置文件失败: {e}")
        sys.exit(1)
    except Exception as e:
        logger.error(f"读取配置文件失败: {e}")
        sys.exit(1)


def check_environment():
    """检查构建环境"""
    logger.title('检查构建环境')

    platform_info = executor.platform_info
    logger.info(f"检测到平台: {platform_info['platform']}")
    logger.info(f"架构: {platform_info['arch']}")

    # 检查必需的命令
    required_commands = ['node', 'cargo', 'rustc']
    for cmd in required_commands:
        if executor.command_exists(cmd):
            logger.success(f"{cmd}: 已安装")
        else:
            logger.error(f"{cmd}: 未安装")
            sys.exit(1)

    # 检查 Tauri CLI
    if executor.command_exists('cargo-tauri') or executor.command_exists('tauri'):
        logger.success("Tauri CLI: 已安装")
    else:
        logger.error("Tauri CLI: 未安装")
        logger.info("运行: cargo install tauri-cli --version '^2.0.0'")
        sys.exit(1)

    logger.newline()


def build(options: argparse.Namespace):
    """
    构建项目
    :param options: 命令行选项
    """
    logger.title('RTodo 跨平台构建')
    logger.newline()

    # 检查环境
    check_environment()

    # 加载配置
    targets = load_config('targets.json')
    version = load_config('version.json')

    logger.info(f"应用版本: {version.get('current', 'unknown')}")
    logger.info(f"构建号: {version.get('buildNumber', 'unknown')}")
    logger.newline()

    # 确定要构建的平台
    platforms_to_build = []

    if options.all_platforms:
        # 构建所有平台（仅限当前平台）
        if sys.platform == 'win32':
            platforms_to_build.append('windows')
        elif sys.platform == 'darwin':
            platforms_to_build.append('macos')
        elif sys.platform.startswith('linux'):
            platforms_to_build.append('linux')
    elif options.platform:
        # 用户指定的平台
        platform_map = {
            'win': 'windows',
            'windows': 'windows',
            'linux': 'linux',
            'mac': 'macos',
            'macos': 'macos',
            'osx': 'macos',
        }
        platform_name = platform_map.get(options.platform.lower(), options.platform)
        platforms_to_build.append(platform_name)
    else:
        # 自动检测当前平台
        if sys.platform == 'win32':
            platforms_to_build.append('windows')
        elif sys.platform == 'darwin':
            platforms_to_build.append('macos')
        elif sys.platform.startswith('linux'):
            platforms_to_build.append('linux')

    # 执行构建
    build_results = []

    for platform_name in platforms_to_build:
        try:
            build_options = {
                'arch': options.arch or 'x86_64',
                'targets': targets,
                'version': version,
                'project_root': str(PROJECT_ROOT),
            }

            if platform_name == 'windows':
                result = build_windows(build_options)
            elif platform_name == 'linux':
                result = build_linux(build_options)
            elif platform_name == 'macos':
                result = build_macos(build_options)
            else:
                logger.error(f"不支持的平台: {platform_name}")
                sys.exit(1)

            build_results.append(result)

        except Exception as e:
            logger.error(f"构建 {platform_name} 失败: {e}")
            if not options.ignore_errors:
                sys.exit(1)

    logger.newline()
    logger.title('构建完成')

    # 显示构建产物位置
    output_dir = PROJECT_ROOT / 'src-tauri' / 'target'
    logger.info(f"构建产物位置: {output_dir}")
    logger.newline()

    return build_results


def main():
    """主函数"""
    # 解析命令行参数
    parser = argparse.ArgumentParser(
        description='RTodo 跨平台构建脚本',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
示例:
  python build.py                    # 构建当前平台
  python build.py -p windows         # 构建 Windows 平台
  python build.py -p macos -a arm64  # 构建 macOS ARM64
  python build.py --all-platforms    # 构建所有可用平台
        """
    )

    parser.add_argument(
        '-p', '--platform',
        type=str,
        choices=['windows', 'linux', 'macos', 'win', 'mac', 'osx'],
        help='目标平台 (windows, linux, macos)'
    )

    parser.add_argument(
        '-a', '--arch',
        type=str,
        choices=['x86_64', 'arm64', 'universal'],
        help='目标架构 (x86_64, arm64, universal)'
    )

    parser.add_argument(
        '--all-platforms',
        action='store_true',
        help='构建所有平台（仅在当前平台上可用）'
    )

    parser.add_argument(
        '--ignore-errors',
        action='store_true',
        help='忽略错误继续构建'
    )

    parser.add_argument(
        '-v', '--verbose',
        action='store_true',
        help='详细输出'
    )

    parser.add_argument(
        '--version',
        action='version',
        version='%(prog)s 1.0.0'
    )

    args = parser.parse_args()

    # 设置日志级别
    if args.verbose:
        logger.set_level(LogLevel.DEBUG)

    # 执行构建
    try:
        build(args)
    except KeyboardInterrupt:
        logger.warning('\n构建已取消')
        sys.exit(1)
    except Exception as e:
        logger.error(f'构建失败: {e}')
        if args.verbose:
            import traceback
            traceback.print_exc()
        sys.exit(1)


if __name__ == '__main__':
    main()
