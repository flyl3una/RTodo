# RTodo 构建脚本

RTodo 应用的跨平台构建脚本，使用 Python 实现，支持 Windows、Linux、macOS 多个平台和多种架构。

---

## 快速开始

### 环境要求

**通用依赖**:
- Python 3.7+
- Node.js 16+
- Rust 1.70+
- Cargo

**Windows 附加依赖**:
```powershell
# WiX Toolset (用于 MSI)
choco install wixtoolset

# NSIS (可选，用于 NSIS 安装包)
choco install nsis
```

**Linux 附加依赖**:
```bash
# Ubuntu/Debian
sudo apt install build-essential libwebkit2gtk-4.1-dev
```

**macOS 附加依赖**:
```bash
# 安装 Command Line Tools
xcode-select --install
```

### 构建应用

```bash
cd build

# 构建当前平台
npm run build

# 或直接使用 Python
python build/build.py
```

---

## 可用脚本

### NPM 脚本命令

| 命令 | 说明 |
|------|------|
| `npm run build` | 构建当前平台 |
| `npm run build:win` | 构建 Windows 平台 |
| `npm run build:linux` | 构建 Linux 平台 |
| `npm run build:mac` | 构建 macOS 平台 |
| `npm run build:win-arm64` | 构建 Windows ARM64 |
| `npm run build:mac-arm64` | 构建 macOS ARM64 |
| `npm run build:mac-universal` | 构建 macOS Universal Binary |

### Python 命令

```bash
# 构建当前平台
python build/build.py

# 指定平台
python build/build.py -p windows
python build/build.py -p linux
python build/build.py -p macos

# 指定架构
python build/build.py -p windows -a arm64
python build/build.py -p macos -a arm64
python build/build.py -p macos -a universal

# 详细输出
python build/build.py -v

# 忽略错误继续构建
python build/build.py --ignore-errors
```

---

## 命令行参数

```
usage: build.py [-h] [-p {windows,linux,macos,win,mac,osx}]
                [-a {x86_64,arm64,universal}] [--all-platforms]
                [--ignore-errors] [-v]

RTodo 跨平台构建脚本

optional arguments:
  -h, --help            显示帮助信息
  -p {windows,linux,macos,win,mac,osx}, --platform {windows,linux,macos,win,mac,osx}
                        目标平台
  -a {x86_64,arm64,universal}, --arch {x86_64,arm64,universal}
                        目标架构
  --all-platforms        构建所有平台（仅在当前平台上可用）
  --ignore-errors        忽略错误继续构建
  -v, --verbose          详细输出
  --version             显示版本号
```

---

## 支持的平台和包格式

### Windows

| 架构 | 包格式 | 状态 |
|------|--------|------|
| x86_64 | MSI, NSIS | ✅ 默认启用 |
| arm64 | MSI, NSIS | ⚠️ 需要安装 ARM64 工具链 |

**构建要求**:
- Visual Studio Build Tools
- WiX Toolset (用于 MSI)
- NSIS (用于 NSIS 安装包)

### Linux

| 架构 | 包格式 | 状态 |
|------|--------|------|
| x86_64 | DEB, RPM, AppImage | ✅ 默认启用 |
| arm64 | DEB, AppImage | ⚠️ 需要交叉编译工具链 |

**构建要求**:
- GCC 编译器
- webkit2gtk-4.1 开发包
- linuxdeploy (用于 AppImage)
- rpmbuild (用于 RPM)

### macOS

| 架构 | 包格式 | 状态 |
|------|--------|------|
| x86_64 | APP, DMG | ✅ 默认启用 |
| arm64 | APP, DMG | ✅ 默认启用 |
| universal | APP, DMG | ✅ 默认启用 |

**构建要求**:
- Xcode
- Command Line Tools

---

## 构建产物位置

### Windows

```
src-tauri/target/x86_64-pc-windows-msvc/release/bundle/
├── msi/          # MSI 安装包
│   └── RTodo_0.1.0_x64_en-US.msi
└── nsis/         # NSIS 安装包
    └── RTodo_0.1.0_x64-setup.exe
```

### Linux

```
src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/
├── deb/          # DEB 包
│   └── rtodo_0.1.0_amd64.deb
├── appimage/     # AppImage
│   └── RTodo_0.1.0_amd64.AppImage
└── rpm/          # RPM 包
    └── rtodo-0.1.0-1.x86_64.rpm
```

### macOS

```
src-tauri/target/x86_64-apple-darwin/release/bundle/
├── macos/        # APP 包
│   └── RTodo.app
└── dmg/          # DMG 镜像
    └── RTodo_0.1.0_x64.dmg
```

---

## 项目结构

```
build/
├── README.md                  # 本文档
├── package.json               # NPM 脚本配置
├── build.py                   # 主构建脚本 (Python)
├── platforms/                 # 平台构建模块
│   ├── __init__.py
│   ├── windows.py             # Windows 平台构建
│   ├── linux.py               # Linux 平台构建
│   └── macos.py               # macOS 平台构建
├── utils/                     # 工具模块
│   ├── __init__.py
│   ├── logger.py              # 日志工具
│   └── executor.py            # 命令执行工具
└── configs/                   # 配置文件
    ├── targets.json           # 平台配置
    └── version.json           # 版本配置
```

---

## 配置文件

### targets.json

目标平台配置，定义了每个平台支持的架构和包格式。

### version.json

版本信息配置，包含当前版本、构建号和变更日志。

---

## 常见问题

### Q: 构建失败，提示 "bundle.active is false"

A: 需要修改 `src-tauri/tauri.conf.json`，将 `bundle.active` 设置为 `true`。

### Q: Windows 上找不到 WiX Toolset

A: 运行 `choco install wixtoolset` 安装，或从 [WiX官网](https://wixtoolset.org/releases/) 下载。

### Q: Linux 上构建 AppImage 失败

A: 安装 linuxdeploy: `wget https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage`

### Q: macOS 上签名失败

A: 需要 Apple Developer 账号和有效的开发者证书。或在 `tauri.conf.json` 中设置 `"signingIdentity": null`。

### Q: 如何交叉编译？

A: 跨平台交叉编译有限制：
- Windows → Linux/macOS: 不支持
- Linux → Windows: 可使用 `cargo-xwin`
- macOS → Linux: 可使用 `osxcross`

**推荐**: 使用 GitHub Actions 在各平台上构建。

---

## 相关文档

- [Tauri 官方文档](https://v2.tauri.app/start/build/) - 官方构建指南

---

## 许可证

MIT License - Copyright 2025 RTodo Team
