# Copyright 2025 RTodo Team. All rights reserved.
# SPDX-License-Identifier: MIT
"""
Linux 平台构建脚本
支持 x86_64 和 arm64 架构
生成 DEB, RPM, AppImage, tar.gz 包
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


def get_package_formats(arch: str, no_appimage: bool = False) -> List[str]:
    """获取构建的包格式"""
    formats = ['deb', 'tar.gz']  # DEB 和 tar.gz 是默认格式

    # AppImage 需要 linuxdeploy（可以通过 --no-appimage 跳过）
    if not no_appimage and (executor.command_exists('linuxdeploy') or arch == 'x86_64'):
        formats.append('appimage')

    # RPM 需要 rpmbuild
    if executor.command_exists('rpmbuild') or executor.command_exists('rpm'):
        formats.append('rpm')

    return formats


def find_binary_path(target: str, project_root: str, app_name: str = 'rtodo') -> str:
    """
    查找二进制文件路径
    Tauri 2.x 可能在不同位置输出二进制文件
    :param target: Rust 目标架构
    :param project_root: 项目根目录
    :param app_name: 应用名称
    :return: 二进制文件完整路径
    :raises FileNotFoundError: 如果找不到二进制文件
    """
    # 尝试多个可能的路径
    possible_paths = [
        # Tauri 2.x 标准路径（项目根目录的 target/）
        os.path.join(project_root, f'target/{target}/release/{app_name}'),
        # Tauri 1.x 或某些配置下的路径
        os.path.join(project_root, f'src-tauri/target/{target}/release/{app_name}'),
    ]

    for path in possible_paths:
        if os.path.exists(path):
            return path

    # 如果都找不到，抛出异常
    raise FileNotFoundError(
        f"未找到二进制文件 '{app_name}'，尝试了以下路径：\n" +
        "\n".join(f"  - {p}" for p in possible_paths)
    )


def create_tarball(target: str, project_root: str, version: dict) -> str:
    """
    手动创建 tar.gz 包
    :param target: Rust 目标架构
    :param project_root: 项目根目录
    :param version: 版本信息
    :return: tar.gz 文件路径
    """
    import tarfile
    import shutil

    app_name = 'rtodo'
    app_version = version.get('current', '0.1.1')
    arch = 'x86_64' if 'x86_64' in target else 'arm64'

    # 查找二进制文件
    binary_src = find_binary_path(target, project_root, app_name)
    # 不直接输出路径以避免 Windows 编码问题
    logger.info(f"找到二进制文件，准备打包...")

    # 临时目录（使用项目根目录的 target/）
    temp_base_dir = os.path.join(project_root, 'target', target, 'release')
    temp_dir = os.path.join(temp_base_dir, 'tarball-temp')
    if os.path.exists(temp_dir):
        shutil.rmtree(temp_dir)
    os.makedirs(temp_dir)

    # 创建目录结构
    package_dir = os.path.join(temp_dir, f'{app_name}-{app_version}-{arch}')
    os.makedirs(package_dir)

    # 复制二进制文件
    binary_dst = os.path.join(package_dir, app_name)
    shutil.copy2(binary_src, binary_dst)
    os.chmod(binary_dst, 0o755)

    # 创建 README
    readme_content = f"""# {app_name} v{app_version} ({arch})

## Installation

1. Extract the archive:
   tar -xzf {app_name}-{app_version}-{arch}.tar.gz

2. Enter the directory:
   cd {app_name}-{app_version}-{arch}

3. Run the application:
   ./{app_name}

## System Requirements

- GTK 3
- WebKit2GTK
- libayatana-appindicator (optional, for system tray)

### Ubuntu/Debian:
```bash
sudo apt install libgtk-3-0 libwebkit2gtk-4.1-0 libayatana-appindicator3-1
```

### Fedora/RHEL:
```bash
sudo dnf install gtk3 webkit2gtk3 libappindicator-gtk3
```

### Arch Linux:
```bash
sudo pacman -S gtk3 webkit2gtk libappindicator-gtk3
```

## License

Copyright (c) 2025 RTodo Team. All rights reserved.
"""
    readme_path = os.path.join(package_dir, 'README.md')
    with open(readme_path, 'w', encoding='utf-8') as f:
        f.write(readme_content)

    # 创建 tar.gz 文件
    tarball_name = f'{app_name}-{app_version}-{arch}.tar.gz'
    tarball_path = os.path.join(
        project_root,
        f'target/{target}/release/bundle/tarball',
        tarball_name
    )

    # 确保目标目录存在
    os.makedirs(os.path.dirname(tarball_path), exist_ok=True)

    with tarfile.open(tarball_path, 'w:gz') as tar:
        tar.add(package_dir, arcname=os.path.basename(package_dir))

    # 清理临时目录
    shutil.rmtree(temp_dir)

    return tarball_path


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
    no_appimage = options.get('no_appimage', False)

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
    package_formats = get_package_formats(arch, no_appimage)
    logger.info(f"将生成的包格式: {', '.join(package_formats)}")

    if no_appimage and 'appimage' not in package_formats:
        logger.info("已跳过 AppImage 构建 (--no-appimage)")

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

    # 设置网络超时环境变量（针对 AppImage 下载）
    os.environ['TAURI_BUNDLE_SKIP_SIGNING'] = '1'

    build_results = []
    base_built = False  # 标记是否已经构建了基础二进制文件

    # 分离出需要通过 Tauri 构建的格式和手动构建的格式
    tauri_formats = []
    manual_formats = []

    for format_type in package_formats:
        if format_type == 'tar.gz':
            manual_formats.append(format_type)
        else:
            tauri_formats.append(format_type)

    # 检查二进制文件是否存在
    try:
        binary_path = find_binary_path(target, project_root, 'rtodo')
        logger.info("二进制文件已存在，跳过编译")
        base_built = True
    except FileNotFoundError:
        logger.info("需要构建基础二进制文件...")

        # 如果有 Tauri 格式要构建，使用第一个格式来构建（会同时编译二进制文件）
        # 如果没有 Tauri 格式（只要 tar.gz），则只编译二进制文件
        if tauri_formats:
            first_format = tauri_formats[0]
            logger.info(f"构建 {first_format.upper()} 包（同时编译二进制文件）...")

            build_command = f'cargo tauri build --target {target} --bundles {first_format}'
            logger.info(f"执行: {build_command}")

            spinner = Spinner(f"正在编译和打包 {first_format.upper()}...")
            spinner.start()

            try:
                timeout = 1200 if first_format == 'appimage' else 600
                executor.exec(build_command, cwd=project_root, timeout=timeout, silent=False)
                spinner.succeed(f"{first_format.upper()} 构建完成")
                build_results.append(first_format)
                base_built = True
            except Exception as e:
                spinner.fail(f"{first_format.upper()} 构建失败")
                # 检查二进制文件是否已生成
                try:
                    binary_path = find_binary_path(target, project_root, 'rtodo')
                    logger.warning(f"{first_format.upper()} 打包失败，但二进制文件已构建")
                    logger.success("使用已构建的二进制文件继续...")
                    base_built = True
                except FileNotFoundError:
                    logger.error(f"{first_format.upper()} 构建失败，且二进制文件也未生成")
                    # 如果是 AppImage 失败，给出额外提示
                    if first_format == 'appimage':
                        logger.info("建议使用 --no-appimage 跳过 AppImage")
                    raise
        else:
            # 只有 tar.gz，只编译二进制文件
            logger.info("编译二进制文件（用于 tar.gz 打包）...")

            # 尝试不指定任何 bundles（Tauri 2.x 默认行为）
            base_command = f'cargo tauri build --target {target}'

            spinner = Spinner("正在编译 Tauri 应用...")
            spinner.start()

            try:
                executor.exec(base_command, cwd=project_root, timeout=600, silent=False)
                spinner.succeed("二进制文件构建完成")
                base_built = True
            except Exception as e:
                spinner.fail("二进制文件构建失败")
                raise
    else:
        logger.info("二进制文件已存在，跳过编译")
        base_built = True

    # 构建其余的 bundle 格式（除了第一个）
    if tauri_formats:
        for format_type in tauri_formats[1:]:
            logger.info(f"构建 {format_type.upper()} 包...")

            bundle_command = f'cargo tauri build --target {target} --bundles {format_type}'
            logger.info(f"执行: {bundle_command}")

            spinner = Spinner(f"正在打包 {format_type.upper()}...")
            spinner.start()

            try:
                # AppImage 需要更长的超时时间（下载文件）
                timeout = 1200 if format_type == 'appimage' else 600
                executor.exec(bundle_command, cwd=project_root, timeout=timeout, silent=False)
                spinner.succeed(f"{format_type.upper()} 构建完成")
                build_results.append(format_type)
            except Exception as e:
                spinner.fail(f"{format_type.upper()} 构建失败")
                if format_type == 'appimage':
                    # AppImage 失败时给出详细提示
                    logger.warning("AppImage 构建失败，可能是因为网络问题")
                    logger.info("提示：")
                    logger.info("1. 检查网络连接或配置代理：")
                    logger.info("   export HTTP_PROXY=http://127.0.0.1:7890")
                    logger.info("   export HTTPS_PROXY=http://127.0.0.1:7890")
                    logger.info("")
                    logger.info("2. 手动下载 AppRun 文件：")
                    logger.info("   python -m build.utils.apprun-helper")
                    logger.info("")
                    logger.info("3. 或者跳过 AppImage：")
                    logger.info("   python build-cli.py --no-appimage")
                else:
                    # 其他格式的错误应该抛出
                    raise
    else:
        # 如果只有 tar.gz，直接构建二进制文件（不打包任何 bundle）
        logger.info("构建二进制文件（用于 tar.gz 打包）...")

        # 使用 --none 参数告诉 Tauri 不构建任何 bundle
        base_build_command = f'cargo tauri build --target {target} --bundles none'

        spinner = Spinner("正在编译 Tauri 应用...")
        spinner.start()

        try:
            executor.exec(base_build_command, cwd=project_root, timeout=600, silent=False)
            spinner.succeed("二进制文件构建完成")
            base_built = True
        except Exception as e:
            spinner.fail("二进制文件构建失败")
            # 如果 --bundles none 不支持，尝试不指定任何 bundles
            logger.info("尝试使用默认构建...")
            try:
                fallback_command = f'cargo tauri build --target {target}'
                executor.exec(fallback_command, cwd=project_root, timeout=600, silent=False)
                spinner.succeed("二进制文件构建完成")
                base_built = True
            except Exception as e2:
                spinner.fail("二进制文件构建失败")
                raise e2

    # 手动构建 tar.gz 等格式
    for format_type in manual_formats:
        if format_type == 'tar.gz':
            logger.info(f"打包 {format_type.upper()}...")

            spinner = Spinner(f"正在创建 {format_type.upper()}...")
            spinner.start()

            try:
                tarball_path = create_tarball(target, project_root, version)
                spinner.succeed(f"{format_type.upper()} 创建完成")
                build_results.append(format_type)

                # 显示文件信息
                file_size = os.path.getsize(tarball_path)
                size_mb = file_size / (1024 * 1024)
                logger.info(f"  生成文件: {os.path.basename(tarball_path)} ({size_mb:.2f} MB)")
            except Exception as e:
                spinner.fail(f"{format_type.upper()} 创建失败")
                logger.error(f"错误: {e}")
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
        # tar.gz 在 bundle/tarball 目录下，其他格式在 bundle/{format_type} 目录下
        if format_type == 'tar.gz':
            format_dir = os.path.join(bundle_dir, 'tarball')
        else:
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
