#!/usr/bin/env python3
# Copyright 2025 RTodo Team. All rights reserved.
# SPDX-License-Identifier: MIT
"""
RTodo 构建入口脚本
解决 Python 相对导入问题
"""

import sys
from pathlib import Path

# 将项目根目录添加到 Python 路径
PROJECT_ROOT = Path(__file__).parent
sys.path.insert(0, str(PROJECT_ROOT))

# 导入并执行主构建脚本
from build.build import main

if __name__ == '__main__':
    main()
