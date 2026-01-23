#!/usr/bin/env python3
# Copyright 2025 RTodo Team. All rights reserved.
# SPDX-License-Identifier: MIT
"""
AppRun 文件下载助手
解决网络问题导致 AppImage 构建失败
"""

import os
import sys
import urllib.request
from pathlib import Path

# AppRun 下载 URL
APPRUN_URL_X86_64 = "https://github.com/tauri-apps/binary-releases/releases/download/apprun-old/AppRun-x86_64"
APPRUN_URL_AARCH64 = "https://github.com/tauri-apps/binary-releases/releases/download/apprun-old/AppRun-aarch64"


def download_apprun(arch: str = 'x86_64') -> Path:
    """
    下载 AppRun 文件到本地缓存目录
    :param arch: 架构 (x86_64 或 aarch64)
    :return: 下载的文件路径
    """
    # 确定缓存目录
    cache_dir = Path.home() / '.cache' / 'tauri' / 'apprun'
    cache_dir.mkdir(parents=True, exist_ok=True)

    # 确定文件名和 URL
    if arch == 'aarch64':
        filename = 'AppRun-aarch64'
        url = APPRUN_URL_AARCH64
    else:
        filename = 'AppRun-x86_64'
        url = APPRUN_URL_X86_64

    dest_file = cache_dir / filename

    # 检查是否已存在
    if dest_file.exists():
        print(f"✓ AppRun 文件已存在: {dest_file}")
        return dest_file

    print(f"正在下载 AppRun ({arch})...")
    print(f"URL: {url}")
    print(f"目标: {dest_file}")

    try:
        # 下载文件（带超时和重试）
        import socket

        # 设置超时
        socket.setdefaulttimeout(60)

        # 使用代理（如果设置了）
        proxies = {}
        http_proxy = os.environ.get('HTTP_PROXY') or os.environ.get('http_proxy')
        https_proxy = os.environ.get('HTTPS_PROXY') or os.environ.get('https_proxy')

        if http_proxy:
            proxies['http'] = http_proxy
            print(f"使用 HTTP 代理: {http_proxy}")
        if https_proxy:
            proxies['https'] = https_proxy
            print(f"使用 HTTPS 代理: {https_proxy}")

        if proxies:
            proxy_handler = urllib.request.ProxyHandler(proxies)
            opener = urllib.request.build_opener(proxy_handler)
            urllib.request.install_opener(opener)

        # 下载文件
        urllib.request.urlretrieve(url, dest_file)
        print(f"✓ 下载完成: {dest_file}")

        # 设置可执行权限
        os.chmod(dest_file, 0o755)

        return dest_file

    except urllib.error.URLError as e:
        print(f"✗ 下载失败: {e}")
        print("\n提示：")
        print("1. 检查网络连接")
        print("2. 配置代理：")
        print("   export HTTP_PROXY=http://127.0.0.1:7890")
        print("   export HTTPS_PROXY=http://127.0.0.1:7890")
        print("3. 或手动下载后放到以下目录：")
        print(f"   {cache_dir}")
        sys.exit(1)
    except Exception as e:
        print(f"✗ 下载失败: {e}")
        sys.exit(1)


def main():
    """主函数"""
    import argparse

    parser = argparse.ArgumentParser(description='下载 AppRun 文件')
    parser.add_argument(
        '-a', '--arch',
        type=str,
        choices=['x86_64', 'aarch64', 'arm64'],
        default='x86_64',
        help='目标架构 (默认: x86_64)'
    )

    args = parser.parse_args()

    # 处理 arm64 别名
    arch = 'aarch64' if args.arch == 'arm64' else args.arch

    download_apprun(arch)


if __name__ == '__main__':
    main()
