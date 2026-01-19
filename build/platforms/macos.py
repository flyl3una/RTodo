# Copyright 2025 RTodo Team. All rights reserved.
# SPDX-License-Identifier: MIT
"""
macOS 平台构建脚本
支持 x86_64 (Intel), arm64 (Apple Silicon), 和 Universal Binary
生成 APP 和 DMG 包
"""

import os
import sys
import shutil
from typing import Dict, Any
from ..utils.logger import logger, Spinner
from ..utils.executor import executor


# macOS 构建目标
MACOS_TARGETS = {
    'x86_64': 'x86_64-apple-darwin',
    'arm64': 'aarch64-apple-darwin',
    'universal': 'universal-apple-darwin',
}


def check_xcode() -> bool:
    """检查 Xcode"""
    try:
        version = executor.exec('xcodebuild -version', silent=True)
        logger.success('Xcode: 已安装')
        logger.debug(version.strip())
        return True
    except:
        logger.error('Xcode: 未安装')
        logger.info('安装方式: 从 App Store 安装 Xcode')
        return False


def check_command_line_tools() -> bool:
    """检查 Command Line Tools"""
    try:
        path = executor.exec('xcode-select -p', silent=True)
        logger.success('Command Line Tools: 已安装')
        return True
    except:
        logger.error('Command Line Tools: 未安装')
        logger.info('安装方式: xcode-select --install')
        return False


def get_package_formats(arch: str) -> list:
    """获取构建的包格式"""
    # macOS 默认生成 APP 和 DMG
    return ['app', 'dmg']


def build_single_arch(target: str, project_root: str) -> None:
    """
    构建单个架构的 macOS 应用
    :param target: Rust 目标
    :param project_root: 项目根目录
    """
    logger.info(f"构建目标: {target}")

    build_command = f'cargo tauri build --target {target}'

    spinner = Spinner(f"正在构建 {target}...")
    spinner.start()

    try:
        executor.exec(build_command, cwd=project_root, timeout=600, silent=False)
        spinner.succeed(f"{target} 构建完成")
    except Exception as e:
        spinner.fail(f"{target} 构建失败")
        raise


def create_universal_binary(project_root: str) -> None:
    """
    创建 Universal Binary
    :param project_root: 项目根目录
    """
    logger.info('创建 Universal Binary (合并 x86_64 和 arm64)')

    x86_64_target = 'x86_64-apple-darwin'
    arm64_target = 'aarch64-apple-darwin'

    # 构建两个架构
    build_single_arch(x86_64_target, project_root)
    build_single_arch(arm64_target, project_root)

    # 使用 lipo 合并二进制文件
    x86_64_binary = os.path.join(
        project_root,
        f'src-tauri/target/{x86_64_target}/release/bundle/macos/RTodo.app/Contents/MacOS/rtodo'
    )

    arm64_binary = os.path.join(
        project_root,
        f'src-tauri/target/{arm64_target}/release/bundle/macos/RTodo.app/Contents/MacOS/rtodo'
    )

    universal_binary = os.path.join(
        project_root,
        'src-tauri/target/universal-apple-darwin/release/bundle/macos/RTodo.app/Contents/MacOS/rtodo'
    )

    # 创建输出目录
    os.makedirs(os.path.dirname(universal_binary), exist_ok=True)

    # 合并二进制
    logger.info('使用 lipo 合并二进制文件...')
    executor.exec(
        f'lipo -create "{x86_64_binary}" "{arm64_binary}" -output "{universal_binary}"'
    )

    # 复制 APP 包的其他文件
    logger.info('复制 APP 包资源...')
    source_app = os.path.join(
        project_root,
        f'src-tauri/target/{x86_64_target}/release/bundle/macos/RTodo.app'
    )
    dest_app = os.path.join(
        project_root,
        'src-tauri/target/universal-apple-darwin/release/bundle/macos/RTodo.app'
    )

    # 递归复制除了 MacOS 目录外的所有内容
    def copy_directory(src, dst):
        """递归复制目录，排除 MacOS"""
        os.makedirs(dst, exist_ok=True)
        for item in os.listdir(src):
            src_path = os.path.join(src, item)
            dst_path = os.path.join(dst, item)

            if os.path.isdir(src_path):
                if item != 'MacOS':
                    copy_directory(src_path, dst_path)
            else:
                shutil.copy2(src_path, dst_path)

    copy_directory(source_app, dest_app)

    logger.success('Universal Binary 创建完成')


def build_macos(options: Dict[str, Any]) -> Dict[str, Any]:
    """
    构建 macOS 应用
    :param options: 构建选项
    :return: 构建结果信息
    """
    arch = options.get('arch', 'universal')
    targets = options.get('targets', {})
    version = options.get('version', {})
    project_root = options.get('project_root', '')

    logger.title('macOS 平台构建')
    logger.newline()

    # 检查架构支持
    target = MACOS_TARGETS.get(arch)
    if not target:
        logger.error(f"不支持的架构: {arch}")
        logger.info(f"支持的架构: {', '.join(MACOS_TARGETS.keys())}")
        sys.exit(1)

    logger.info(f"目标架构: {arch}")
    logger.info(f"Rust 目标: {target}")
    logger.newline()

    # 检查开发工具
    check_xcode()
    check_command_line_tools()
    logger.newline()

    # 确定要构建的包格式
    package_formats = get_package_formats(arch)
    logger.info(f"将生成的包格式: {', '.join(package_formats)}")
    logger.newline()

    # 检查并添加 Rust 目标
    targets_to_add = []
    if arch == 'universal' or arch == 'x86_64':
        targets_to_add.append('x86_64-apple-darwin')
    if arch == 'universal' or arch == 'arm64':
        targets_to_add.append('aarch64-apple-darwin')

    installed_targets = executor.exec('rustup target list --installed', silent=True)
    installed_targets_list = [t.strip() for t in installed_targets.split('\n') if t.strip()]

    for t in targets_to_add:
        if t not in installed_targets_list:
            logger.info(f"添加 Rust 目标: {t}")
            executor.exec(f'rustup target add {t}')

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

    if arch == 'universal':
        # 构建通用二进制
        create_universal_binary(project_root)
    else:
        # 构建单个架构
        build_single_arch(target, project_root)

    logger.newline()

    # 显示构建产物
    logger.subtitle('构建产物')

    actual_target = 'universal-apple-darwin' if arch == 'universal' else target
    bundle_dir = os.path.join(
        project_root,
        'src-tauri/target',
        actual_target,
        'release/bundle'
    )

    logger.info(f"构建产物目录: {bundle_dir}")
    logger.newline()

    # 列出生成的文件
    macos_dir = os.path.join(bundle_dir, 'macos')
    if os.path.exists(macos_dir):
        try:
            files = os.listdir(macos_dir)
            if files:
                logger.info('APP 包:')
                for f in files:
                    print(f"  - {f}")
                logger.newline()
        except Exception as e:
            logger.warning('无法列出 APP 包')
    else:
        logger.warning('未找到 APP 包')

    dmg_dir = os.path.join(bundle_dir, 'dmg')
    if os.path.exists(dmg_dir):
        try:
            files = os.listdir(dmg_dir)
            if files:
                logger.info('DMG 镜像:')
                for f in files:
                    file_path = os.path.join(dmg_dir, f)
                    if os.path.isfile(file_path):
                        size = os.path.getsize(file_path)
                        size_mb = size / (1024 * 1024)
                        print(f"  - {f} ({size_mb:.2f} MB)")
        except Exception as e:
            logger.warning('无法列出 DMG 镜像')
    else:
        logger.warning('未找到 DMG 镜像')

    logger.newline()
    logger.success(f'macOS {arch} 构建完成!')

    return {
        'platform': 'macos',
        'arch': arch,
        'target': actual_target,
        'package_formats': package_formats,
        'bundle_dir': bundle_dir,
    }
