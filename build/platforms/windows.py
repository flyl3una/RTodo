# Copyright 2025 RTodo Team. All rights reserved.
# SPDX-License-Identifier: MIT
"""
Windows 平台构建脚本
支持 x86_64 和 arm64 架构
生成 MSI 和 NSIS 安装包
"""

import os
import sys
from typing import Dict, Any
from ..utils.logger import logger, Spinner
from ..utils.executor import executor


# Windows 构建目标
WINDOWS_TARGETS = {
    'x86_64': 'x86_64-pc-windows-msvc',
    'arm64': 'aarch64-pc-windows-msvc',
}


def check_wix() -> bool:
    """检查 WiX Toolset"""
    if executor.command_exists('candle'):
        logger.success('WiX Toolset: 已安装')
        return True
    logger.warning('WiX Toolset 未安装，无法生成 MSI 安装包')
    logger.info('安装方式: choco install wixtoolset')
    logger.info('或访问: https://wixtoolset.org/releases/')
    return False


def check_nsis() -> bool:
    """检查 NSIS"""
    if executor.command_exists('makensis'):
        logger.success('NSIS: 已安装')
        return True
    logger.warning('NSIS 未安装，跳过 NSIS 安装包')
    logger.info('安装方式: choco install nsis')
    return False


def get_package_formats(arch: str) -> list:
    """获取构建的包格式"""
    formats = ['msi']  # MSI 是默认格式

    # NSIS 可选
    if executor.command_exists('makensis'):
        formats.append('nsis')

    return formats


def build_windows(options: Dict[str, Any]) -> Dict[str, Any]:
    """
    构建 Windows 应用
    :param options: 构建选项
    :return: 构建结果信息
    """
    arch = options.get('arch', 'x86_64')
    targets = options.get('targets', {})
    version = options.get('version', {})
    project_root = options.get('project_root', '')

    logger.title('Windows 平台构建')
    logger.newline()

    # 检查架构支持
    target = WINDOWS_TARGETS.get(arch)
    if not target:
        logger.error(f"不支持的架构: {arch}")
        logger.info(f"支持的架构: {', '.join(WINDOWS_TARGETS.keys())}")
        sys.exit(1)

    logger.info(f"目标架构: {arch}")
    logger.info(f"Rust 目标: {target}")
    logger.newline()

    # 检查构建工具
    has_wix = check_wix()
    has_nsis = check_nsis()
    logger.newline()

    # 确定要构建的包格式
    package_formats = get_package_formats(arch)
    logger.info(f"将生成的包格式: {', '.join(package_formats)}")
    logger.newline()

    # 检查是否需要添加 Rust 目标
    installed_targets = executor.exec('rustup target list --installed', silent=True)
    installed_targets_list = [t.strip() for t in installed_targets.split('\n') if t.strip()]

    if target not in installed_targets_list:
        logger.info(f"添加 Rust 目标: {target}")
        executor.exec(f'rustup target add {target}')
        logger.success("Rust 目标已添加")
        logger.newline()

    # 构建前端
    logger.subtitle('构建前端')
    frontend_dir = os.path.join(project_root, 'frontend')
    logger.info("安装依赖...")
    executor.exec('npm install', cwd=frontend_dir)
    logger.info("构建前端...")
    executor.exec('npm run build', cwd=frontend_dir)
    logger.success('前端构建完成')
    logger.newline()

    # 构建 Tauri 应用
    logger.subtitle('构建 Tauri 应用')

    build_command = f'cargo tauri build --target {target}'
    logger.info(f"执行构建: {build_command}")
    logger.info("这可能需要几分钟...")

    spinner = Spinner("正在构建...")
    spinner.start()

    try:
        executor.exec(build_command, cwd=project_root, timeout=600, silent=False)
        spinner.succeed('构建完成')
    except Exception as e:
        spinner.fail('构建失败')
        raise

    logger.newline()

    # 显示构建产物
    logger.subtitle('构建产物')
    bundle_dir = os.path.join(
        project_root,
        'src-tauri/target',
        target,
        'release/bundle'
    )

    logger.info(f"构建产物目录: {bundle_dir}")
    logger.newline()

    # 列出生成的文件
    if 'msi' in package_formats:
        msi_dir = os.path.join(bundle_dir, 'msi')
        if os.path.exists(msi_dir):
            try:
                files = os.listdir(msi_dir)
                if files:
                    logger.info('MSI 安装包:')
                    for f in files:
                        print(f"  - {f}")
            except Exception as e:
                logger.warning(f'无法列出 MSI 目录: {e}')
        else:
            logger.warning('未找到 MSI 安装包')

    if 'nsis' in package_formats:
        nsis_dir = os.path.join(bundle_dir, 'nsis')
        if os.path.exists(nsis_dir):
            try:
                files = os.listdir(nsis_dir)
                if files:
                    logger.info('NSIS 安装包:')
                    for f in files:
                        print(f"  - {f}")
            except Exception as e:
                logger.warning(f'无法列出 NSIS 目录: {e}')
        else:
            logger.warning('未找到 NSIS 安装包')

    logger.newline()
    logger.success(f'Windows {arch} 构建完成!')

    return {
        'platform': 'windows',
        'arch': arch,
        'target': target,
        'package_formats': package_formats,
        'bundle_dir': bundle_dir,
    }
