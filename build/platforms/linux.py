# Copyright 2025 RTodo Team. All rights reserved.
# SPDX-License-Identifier: MIT
"""
Linux 平台构建脚本
支持 x86_64 和 arm64 架构
生成 DEB, RPM, AppImage 包
"""

import os
import sys
from typing import Dict, Any, List
from ..utils.logger import logger, Spinner
from ..utils.executor import executor


# Linux 构建目标
LINUX_TARGETS = {
    'x86_64': 'x86_64-unknown-linux-gnu',
    'arm64': 'aarch64-unknown-linux-gnu',
}


def check_linux_dependencies() -> bool:
    """检查 Linux 依赖"""
    deps = [
        {'cmd': 'gcc', 'name': 'GCC compiler'},
        {'cmd': 'pkg-config', 'name': 'pkg-config'},
    ]

    all_found = True

    for dep in deps:
        if executor.command_exists(dep['cmd']):
            logger.success(f"{dep['name']}: 已安装")
        else:
            logger.warning(f"{dep['name']}: 未安装")
            all_found = False

    # 检查 webkit2gtk
    try:
        executor.exec('pkg-config --exists webkit2gtk-4.1', silent=True)
        logger.success('webkit2gtk-4.1: 已安装')
    except:
        logger.warning('webkit2gtk-4.1: 未安装')
        logger.info('安装方式: sudo apt install libwebkit2gtk-4.1-dev')
        all_found = False

    return all_found


def get_package_formats(arch: str) -> List[str]:
    """获取构建的包格式"""
    formats = ['deb']  # DEB 是默认格式

    # AppImage 需要 linuxdeploy
    if executor.command_exists('linuxdeploy') or arch == 'x86_64':
        formats.append('appimage')

    # RPM 需要 rpmbuild
    if executor.command_exists('rpmbuild') or executor.command_exists('rpm'):
        formats.append('rpm')

    return formats


def build_linux(options: Dict[str, Any]) -> Dict[str, Any]:
    """
    构建 Linux 应用
    :param options: 构建选项
    :return: 构建结果信息
    """
    arch = options.get('arch', 'x86_64')
    targets = options.get('targets', {})
    version = options.get('version', {})
    project_root = options.get('project_root', '')

    logger.title('Linux 平台构建')
    logger.newline()

    # 检查架构支持
    target = LINUX_TARGETS.get(arch)
    if not target:
        logger.error(f"不支持的架构: {arch}")
        logger.info(f"支持的架构: {', '.join(LINUX_TARGETS.keys())}")
        sys.exit(1)

    logger.info(f"目标架构: {arch}")
    logger.info(f"Rust 目标: {target}")
    logger.newline()

    # 检查依赖
    check_linux_dependencies()
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

    build_results = []

    for format_type in package_formats:
        logger.info(f"构建 {format_type.upper()} 包...")

        build_command = f'cargo tauri build --target {target} --bundles {format_type}'

        logger.info(f"执行: {build_command}")

        spinner = Spinner(f"正在构建 {format_type.upper()}...")
        spinner.start()

        try:
            executor.exec(build_command, cwd=project_root, timeout=600, silent=False)
            spinner.succeed(f"{format_type.upper()} 构建完成")
            build_results.append(format_type)
        except Exception as e:
            spinner.fail(f"{format_type.upper()} 构建失败")
            if format_type != 'appimage':
                # AppImage 可能失败，但不影响其他格式
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
    for format_type in build_results:
        format_dir = os.path.join(bundle_dir, format_type)
        if os.path.exists(format_dir):
            try:
                files = os.listdir(format_dir)
                if files:
                    logger.info(f"{format_type.upper()} 包:")
                    for f in files:
                        file_path = os.path.join(format_dir, f)
                        if os.path.isfile(file_path):
                            size = os.path.getsize(file_path)
                            size_mb = size / (1024 * 1024)
                            print(f"  - {f} ({size_mb:.2f} MB)")
            except Exception as e:
                logger.warning(f"无法列出 {format_type.upper()} 目录: {e}")
        else:
            logger.warning(f"未找到 {format_type.upper()} 包")

    logger.newline()
    logger.success(f'Linux {arch} 构建完成!')

    return {
        'platform': 'linux',
        'arch': arch,
        'target': target,
        'package_formats': build_results,
        'bundle_dir': bundle_dir,
    }
