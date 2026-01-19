# Copyright 2025 RTodo Team. All rights reserved.
# SPDX-License-Identifier: MIT
"""
平台构建模块
"""

from .windows import build_windows
from .linux import build_linux
from .macos import build_macos

__all__ = ['build_windows', 'build_linux', 'build_macos']
