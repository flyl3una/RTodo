# RTodo 构建脚本

RTodo 应用的跨平台构建脚本，支持 Windows、Linux、macOS 多个平台和多种架构。

---

## 快速开始

### 1. 安装依赖

```bash
cd build
npm install
```

### 2. 验证环境

```bash
npm run validate
```

### 3. 构建应用

```bash
# 构建当前平台
npm run build

# 或使用构建脚本
node scripts/build-all.js
```

---

## 可用脚本

### 主要构建命令

| 命令 | 说明 |
|------|------|
| `npm run build` | 构建当前平台 |
| `npm run build:win` | 构建 Windows 平台 |
| `npm run build:linux` | 构建 Linux 平台 |
| `npm run build:mac` | 构建 macOS 平台 |
| `npm run build:all` | 构建所有平台（当前可用） |

### 辅助命令

| 命令 | 说明 |
|------|------|
| `npm run clean` | 清理构建产物 |
| `npm run validate` | 验证构建环境 |
| `npm run bump:version` | 更新版本号 |

---

## 构建选项

### 指定平台

```bash
# Windows
node scripts/build-all.js --platform windows

# Linux
node scripts/build-all.js --platform linux

# macOS
node scripts/build-all.js --platform macos
```

### 指定架构

```bash
# x86_64
node scripts/build-all.js --arch x86_64

# arm64
node scripts/build-all.js --arch arm64

# macOS Universal Binary
node scripts/build-all.js --arch universal
```

### 组合选项

```bash
# Windows ARM64
node scripts/build-all.js --platform windows --arch arm64

# Linux x86_64
node scripts/build-all.js --platform linux --arch x86_64
```

### 其他选项

```bash
# 详细输出
node scripts/build-all.js --verbose

# 忽略错误继续构建
node scripts/build-all.js --ignore-errors
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

## 版本管理

### 更新版本号

```bash
# 补丁版本 (0.1.0 -> 0.1.1)
node scripts/bump-version.js patch

# 次版本 (0.1.0 -> 0.2.0)
node scripts/bump-version.js minor

# 主版本 (0.1.0 -> 1.0.0)
node scripts/bump-version.js major

# 自动确认
node scripts/bump-version.js patch --yes
```

### 版本配置文件

版本信息存储在 `configs/version.json`:

```json
{
  "current": "0.1.0",
  "buildNumber": 1,
  "releaseDate": "2025-01-16",
  "changelog": [...]
}
```

---

## 清理构建产物

### 基本清理

```bash
npm run clean
```

清理以下目录:
- `src-tauri/target/` - Cargo 构建缓存
- `build/node_modules/` - 构建脚本依赖
- `build/output/` - 构建输出

### 深度清理

```bash
node scripts/clean.js --all
```

额外清理:
- `frontend/node_modules/` - 前端依赖
- `frontend/dist/` - 前端构建产物

---

## 环境验证

### 验证脚本

```bash
npm run validate
```

检查项:
- ✅ Node.js 版本
- ✅ npm 版本
- ✅ Rust 工具链
- ✅ Tauri CLI
- ✅ 平台特定工具
- ✅ 项目配置

### 环境要求

**通用要求**:
- Node.js >= 16.0.0
- Rust >= 1.70.0
- Tauri CLI >= 2.0.0

**Windows 额外要求**:
- Visual Studio Build Tools 2022
- WiX Toolset v3.11+ (可选，用于 MSI)
- NSIS 3.08+ (可选，用于 NSIS)

**Linux 额外要求**:
- GCC 编译器
- libwebkit2gtk-4.1-dev
- linuxdeploy (可选，用于 AppImage)
- rpmbuild (可选，用于 RPM)

**macOS 额外要求**:
- Xcode 14.0+
- Command Line Tools

---

## 配置文件

### targets.json

目标平台配置，定义了每个平台支持的架构和包格式。

```json
{
  "windows": {
    "x86_64": {
      "target": "x86_64-pc-windows-msvc",
      "packages": ["msi", "nsis"],
      "enabled": true
    }
  }
}
```

### version.json

版本信息配置，包含当前版本、构建号和变更日志。

```json
{
  "current": "0.1.0",
  "buildNumber": 1,
  "releaseDate": "2025-01-16",
  "changelog": [...]
}
```

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

## 目录结构

```
build/
├── README.md                  # 本文档
├── SETUP.md                   # 环境搭建指南
├── package.json               # 脚本配置
├── scripts/
│   ├── build-all.js          # 主构建脚本
│   ├── build-windows.js      # Windows 构建
│   ├── build-linux.js        # Linux 构建
│   ├── build-macos.js        # macOS 构建
│   ├── clean.js              # 清理脚本
│   ├── validate.js           # 环境验证
│   ├── bump-version.js       # 版本管理
│   └── utils/
│       ├── logger.js         # 日志工具
│       └── executor.js       # 命令执行
└── configs/
    ├── targets.json          # 平台配置
    └── version.json          # 版本配置
```

---

## 相关文档

- [SETUP.md](./SETUP.md) - 详细的构建环境搭建指南
- [Tauri 官方文档](https://v2.tauri.app/start/build/) - 官方构建指南

---

## 许可证

MIT
