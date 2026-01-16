# RTodo 构建环境搭建指南

本文档介绍如何搭建 RTodo 应用的跨平台构建环境。

---

## 目录

- [通用要求](#通用要求)
- [Windows 构建环境](#windows-构建环境)
- [Linux 构建环境](#linux-构建环境)
- [macOS 构建环境](#macos-构建环境)
- [验证安装](#验证安装)
- [常见问题](#常见问题)

---

## 通用要求

所有平台都需要以下基础工具：

### 1. Node.js 和 npm

**版本要求**:
- Node.js: >= 16.0.0
- npm: >= 8.0.0

**安装方式**:
- 访问 [nodejs.org](https://nodejs.org/) 下载 LTS 版本
- 或使用 [nvm](https://github.com/nvm-sh/nvm) (推荐)

**验证安装**:
```bash
node --version
npm --version
```

### 2. Rust 工具链

**版本要求**:
- Rust: >= 1.70.0
- Cargo: >= 1.70.0

**安装方式**:
访问 [rustup.rs](https://rustup.rs/) 按照指引安装。

**验证安装**:
```bash
rustc --version
cargo --version
```

### 3. Tauri CLI

**安装方式**:
```bash
cargo install tauri-cli --version "^2.0.0"
```

**验证安装**:
```bash
cargo tauri --version
```

---

## Windows 构建环境

### 1. Visual Studio Build Tools

**用途**: 编译 Rust 代码和 C/C++ 依赖

**安装方式**:
1. 访问 [Visual Studio Downloads](https://visualstudio.microsoft.com/downloads/)
2. 下载 "Build Tools for Visual Studio 2022"
3. 运行安装程序，选择 "C++ build tools"
4. 确保勾选以下组件:
   - MSVC v143 - VS 2022 C++ x64/x86 build tools
   - Windows 11 SDK (或 Windows 10 SDK)

**命令行安装**:
```powershell
# 使用 winget
winget install Microsoft.VisualStudio.2022.BuildTools

# 或使用 Chocolatey
choco install visualstudio2022buildtools
```

### 2. WiX Toolset (可选)

**用途**: 生成 MSI 安装包

**版本要求**: WiX Toolset v3.11 或更高

**安装方式**:
1. 访问 [WiX Toolset Releases](https://github.com/wixtoolset/wix3/releases)
2. 下载最新的 `wix311-binaries.zip`
3. 解压到 `C:\Program Files (x86)\WiX Toolset v3.11\`
4. 将 `bin` 目录添加到系统 PATH

**命令行安装**:
```powershell
# 使用 Chocolatey
choco install wixtoolset
```

**验证安装**:
```powershell
candle -?
```

### 3. NSIS (可选)

**用途**: 生成 NSIS 安装包

**版本要求**: NSIS 3.08 或更高

**安装方式**:
1. 访问 [NSIS Website](https://nsis.sourceforge.io/)
2. 下载最新版本安装程序
3. 运行安装程序

**命令行安装**:
```powershell
# 使用 Chocolatey
choco install nsis
```

**验证安装**:
```powershell
makensis /VERSION
```

### 4. WebView2 Runtime (通常已预装)

**用途**: Windows 11/10 通常已预装，确保应用能运行

**检查方式**:
```powershell
Get-ItemProperty "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -ErrorAction SilentlyContinue
```

### 5. Node.js 构建工具 (可能需要)

**用途**: 编译原生 Node.js 模块

**安装方式**:
```powershell
npm install --global windows-build-tools
```

---

## Linux 构建环境

### 1. 系统包管理器依赖

**Debian/Ubuntu**:
```bash
sudo apt update
sudo apt install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

**Fedora/RHEL**:
```bash
sudo dnf install -y \
    webkit2gtk4.1-devel \
    openssl-devel \
    curl \
    wget \
    file \
    libappindicator-gtk3-devel \
    librsvg2-devel
```

**Arch Linux**:
```bash
sudo pacman -Syu --needed \
    webkit2gtk-4.1 \
    base-devel \
    curl \
    wget \
    file \
    openssl \
    appmenu-gtk-module \
    libappindicator-gtk3 \
    librsvg
```

### 2. AppImage 构建工具 (可选)

**用途**: 生成 AppImage 包

**安装方式**:
```bash
# 下载 linuxdeploy
wget https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage
chmod +x linuxdeploy-x86_64.AppImage
sudo mv linuxdeploy-x86_64.AppImage /usr/local/bin/linuxdeploy

# 下载 linuxdeploy-plugin-qt
wget https://github.com/linuxdeploy/linuxdeploy-plugin-qt/releases/download/continuous/linuxdeploy-plugin-qt-x86_64.AppImage
chmod +x linuxdeploy-plugin-qt-x86_64.AppImage
sudo mv linuxdeploy-plugin-qt-x86_64.AppImage /usr/local/bin/linuxdeploy-plugin-qt
```

### 3. RPM 构建工具 (Fedora/RHEL)

**安装方式**:
```bash
sudo dnf install -y rpm-build rpmlint
```

### 4. DEB 构建工具 (Debian/Ubuntu)

**安装方式**:
```bash
sudo apt install -y dpkg-dev debhelper
```

### 5. 交叉编译工具链 (ARM64, 可选)

**安装方式**:
```bash
# Debian/Ubuntu
sudo apt install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu

# Fedora
sudo dnf install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu
```

---

## macOS 构建环境

### 1. Xcode

**用途**: 编译 Rust 代码和系统依赖

**版本要求**: Xcode 14.0 或更高

**安装方式**:
1. 从 App Store 安装 Xcode
2. 运行 Xcode 接受许可协议
3. 安装 Command Line Tools

**命令行方式**:
```bash
xcode-select --install
```

### 2. Command Line Tools

**验证安装**:
```bash
xcode-select -p
```

**如果未安装**:
```bash
xcode-select --install
```

### 3. Xcode Command Line Tools 组件

**确保以下组件已安装**:
```bash
# 检查已安装的 SDK
xcodebuild -showsdks
```

### 4. Rust 工具链 (多架构)

**安装 ARM64 工具链** (Apple Silicon):
```bash
rustup target add aarch64-apple-darwin
```

**安装 x86_64 工具链** (Intel):
```bash
rustup target add x86_64-apple-darwin
```

### 5. 代码签名 (可选，用于发布)

**用途**: 对应用进行签名，避免安全警告

**要求**:
- Apple Developer 账号
- 有效的开发者证书

**配置方式**:
1. 登录 [Apple Developer](https://developer.apple.com/)
2. 创建开发者证书
3. 在钥匙串中安装证书
4. 在 `tauri.conf.json` 中配置签名身份

---

## 验证安装

### 1. 环境验证脚本

运行项目中的验证脚本:
```bash
cd build
npm install
node scripts/validate.js
```

### 2. 手动验证

**通用验证**:
```bash
# 检查 Node.js
node --version  # 应该 >= 16.0.0

# 检查 Rust
rustc --version  # 应该 >= 1.70.0

# 检查 Tauri CLI
cargo tauri --version  # 应该 >= 2.0.0
```

**Windows 验证**:
```powershell
# 检查 MSVC
cl  # 应该能找到编译器

# 检查 WiX (如果安装)
candle -?

# 检查 NSIS (如果安装)
makensis /VERSION
```

**Linux 验证**:
```bash
# 检查 webkit2gtk
pkg-config --modversion webkit2gtk-4.1

# 检查构建工具
gcc --version
g++ --version
```

**macOS 验证**:
```bash
# 检查 Xcode
xcodebuild -version

# 检查目标架构
rustup target list | grep apple-darwin
```

### 3. 测试构建

**开发模式测试**:
```bash
cd F:\program\Rust\tauri\rtodo
cargo tauri dev
```

**生产构建测试**:
```bash
cargo tauri build
```

如果构建成功，说明环境配置正确。

---

## 常见问题

### Q1: Windows 上出现 "link.exe not found"

**原因**: Visual Studio Build Tools 未正确安装

**解决**:
1. 重新安装 Build Tools for Visual Studio 2022
2. 确保勾选 "C++ build tools"
3. 重启命令提示符

### Q2: Linux 上出现 "webkit2gtk not found"

**原因**: webkit2gtk 开发包未安装

**解决**:
```bash
# Debian/Ubuntu
sudo apt install libwebkit2gtk-4.1-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel

# Arch
sudo pacman -S webkit2gtk-4.1
```

### Q3: macOS 上出现 "xcrun: error: invalid active developer path"

**原因**: Xcode Command Line Tools 未正确安装

**解决**:
```bash
sudo xcode-select --reset
sudo xcode-select --install
```

### Q4: Rust 编译时出现 "error: linker not found"

**原因**: 系统链接器未找到

**Windows 解决**:
安装 MSVC (Visual Studio Build Tools)

**Linux 解决**:
```bash
sudo apt install build-essential
```

**macOS 解决**:
```bash
xcode-select --install
```

### Q5: cargo tauri build 失败，提示 "bundle.active is false"

**原因**: Tauri 配置中打包功能未启用

**解决**:
修改 `src-tauri/tauri.conf.json`:
```json
{
  "bundle": {
    "active": true
  }
}
```

### Q6: Windows 上构建 MSI 失败

**原因**: WiX Toolset 未安装或不在 PATH 中

**解决**:
1. 安装 WiX Toolset v3.11+
2. 将 `C:\Program Files (x86)\WiX Toolset v3.11\bin` 添加到 PATH
3. 重启命令提示符

### Q7: 如何交叉编译到其他平台？

**说明**:
- Windows → Linux/macOS: 不支持，需要在对应平台上构建
- Linux → Windows: 可使用 `cargo-xwin` 工具
- macOS → Linux: 可使用 `osxcross` 工具
- **推荐**: 使用 GitHub Actions 在各平台上构建

### Q8: 构建时间太长怎么办？

**优化方法**:
```bash
# 使用更少的并行任务
cargo tauri build --jobs 2

# 跳过依赖更新
cargo tauri build --frozen

# 只构建应用，不打包
cargo tauri build --no-bundle
```

### Q9: 如何清理构建缓存？

```bash
# 清理 Cargo 构建缓存
cargo clean

# 清理 Tauri 缓存
cargo tauri clean

# 清理 Node 依赖
rm -rf node_modules
rm -rf build/node_modules

# 清理构建输出
rm -rf src-tauri/target
```

### Q10: Windows Defender 阻止构建

**解决**:
1. 将项目目录添加到 Windows Defender 排除列表
2. 排除路径: `F:\program\Rust\tauri\rtodo`
3. 排除进程: `cargo.exe`, `rustc.exe`, `cc1.exe`

---

## 下一步

环境搭建完成后，请参考:
- [README.md](./README.md) - 构建脚本使用说明
- [构建指南](https://v2.tauri.app/start/build/) - 官方构建文档

---

## 获取帮助

- **Tauri 官方文档**: https://v2.tauri.app/
- **Tauri Discord**: https://discord.gg/tauri
- **GitHub Issues**: https://github.com/tauri-apps/tauri/issues
