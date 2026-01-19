# 摊盒 Booth-Kernel

<!-- 徽章区域 -->
[![Tauri](https://img.shields.io/badge/Tauri-v2-24C8D5?logo=tauri&logoColor=white)](https://tauri.app)
[![Rust](https://img.shields.io/badge/Rust-1.75+-black?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Vue](https://img.shields.io/badge/Vue-3.x-4FC08D?logo=vue.js&logoColor=white)](https://vuejs.org)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
![Platform](https://img.shields.io/badge/平台-Windows%20%7C%20Android-blue)

> **基于 Tauri v2 + Rust 构建的现代化同人展会收银与库存管理工具。**

---

## 🚩 普通用户请看这里

如果您是**社团摊主**，正在寻找软件下载链接或使用教程，请访问我们的官方文档网站。本仓库仅包含源代码与开发文档。

👉 **[点击前往官网与下载 (使用手册)](https://booth-kernel.secret-sealing.club)**

---

## ✨ 项目简介

**摊盒 (Booth Kernel)** 是一款专为漫展现场设计的本地化收银系统。为了解决展会现场信号拥堵、记账繁琐的痛点，我们采用了 **"主机-客户端"** 的局域网架构：

*   **主机端 (Host)**: 运行 Tauri 桌面应用（Windows/Android），内置 Axum HTTP 服务与 SQLite 数据库，负责核心逻辑与数据存储。
*   **客户端 (Client)**: 局域网内的任意设备（闲置手机/平板），直接通过浏览器访问主机，充当“扫码枪”或“点单屏”，无需安装 App。

### 核心特性

*   **离线优先**: 100% 基于局域网 (LAN) 运行，无外部互联网依赖，断网也能飞快记账。
*   **隐私安全**: 所有数据（库存、流水、图片）仅存储于本地 SQLite (`data.db`)，绝不上传云端。
*   **极致性能**: Rust 后端确保了极低的内存占用和高并发处理能力，旧电脑也能流畅带飞多台设备。
*   **跨平台**: 主机端完美支持 Windows 和 Android 平板 (Linux/macOS 适配中)。

## 🛠️ 技术栈

| 模块 | 技术选型 | 说明 |
| :--- | :--- | :--- |
| **核心框架** | [Tauri v2](https://v2.tauri.app/) | 下一代跨平台应用构建框架 |
| **后端逻辑** | [Rust](https://www.rust-lang.org/) | 负责系统调用、高性能计算 |
| **HTTP服务** | [Axum](https://github.com/tokio-rs/axum) | 嵌入式 REST API 服务器 |
| **数据库** | [SQLx](https://github.com/launchbadge/sqlx) + SQLite | 异步 ORM，数据本地落盘 |
| **前端框架** | [Vue 3](https://vuejs.org/) + [Vite](https://vitejs.dev/) | 响应式 Web 界面 |
| **UI 组件库** | [Naive UI](https://www.naiveui.com/) | 统一的现代化设计风格 |
| **状态管理** | [Pinia](https://pinia.vuejs.org/) | 前端全局状态管理 |

## 📸 界面预览

<!-- 请替换为你实际的截图链接，建议放在 docs/public/images/ 文件夹或图床 -->
| 电脑管理后台 | 手机摊主视图 | 平板顾客视图 |
| :---: | :---: | :---: |
| ![后台](https://via.placeholder.com/300x200?text=管理后台) | ![手机](https://via.placeholder.com/300x200?text=摊主端) | ![平板](https://via.placeholder.com/300x200?text=顾客端) |

## 💻 开发指南

### 环境要求

*   **Node.js**: v18 或更高版本
*   **Rust**: v1.75+ (请通过 [rustup](https://rustup.rs/) 安装)
*   **包管理器**: 推荐使用 `pnpm`
*   **构建工具**:
    *   Windows: 需安装 C++ 生成工具 (通过 Visual Studio Installer)
    *   Android: 需安装 Android Studio & SDK / NDK (如需构建移动端)

### 快速开始

1.  **克隆仓库**
    ```bash
    git clone https://github.com/Academy-of-Boundary-Landscape/ABL-BoothApp.git
    cd ABL-BoothApp
    ```

2.  **安装依赖**
    ```bash
    pnpm install
    ```

3.  **启动开发模式** (桌面端)
    ```bash
    pnpm tauri dev
    ```
    *此命令会同时启动 Vite 前端服务和 Tauri Rust 后端。*

4.  **启动安卓模拟器** (移动端)
    ```bash
    pnpm tauri android init
    pnpm tauri android dev
    ```

### 打包构建

*   **构建 Windows 版本 (exe/msi)**
    ```bash
    pnpm tauri build
    ```

*   **构建 Android 版本 (apk/aab)**
    ```bash
    pnpm tauri android build
    ```
    *注意：首次构建安卓版本前，需在 `src-tauri/gen/android/keystore.properties` 中配置签名密钥。*
    *注意2：我修改了默认的 ABI 支持，仅打包 arm64-v8a 和 armeabi-v7a，以减小包体积，这可能导致手机模拟器或许无法运行本软件。*

* **构建更简洁的 Android 版本**
由于默认的 Android 构建会包含多种 CPU 架构的支持，导致 APK 包体积较大。如果您的目标设备仅为 ARM 架构（如大多数现代安卓手机和平板），可以通过以下步骤构建更小的 APK：
    在项目根目录下运行以下命令以仅构建 ARM64 架构 (绝大多数现代手机) 的 APK：
    ```bash
    npm run tauri -- android build --apk true -t aarch64
    ```
    或者对于较老的设备，运行这个进行构建
    ```bash
    npm run tauri -- android build --apk true -t armv7
    ```
## 📂 项目结构

```text
.
├── src/                # Vue 3 前端代码
│   ├── assets/         # 静态资源
│   ├── components/     # Vue 组件
│   ├── views/          # 页面视图 (Admin/Vendor/Customer)
│   ├── stores/         # Pinia 状态仓库
│   └── services/       # Axios API 封装
├── src-tauri/          # Rust 后端代码
│   ├── src/
│   │   ├── main.rs     # 程序入口
│   │   ├── lib.rs      # 库入口
│   │   ├── db/         # 数据库迁移与 Schema
│   │   ├── http/       # Axum 路由与处理器
│   │   └── utils/      # 工具类 (IP获取, JWT, 加密等)
│   ├── tauri.conf.json # Tauri 核心配置文件
│   └── capabilities/   # 权限配置
└── ...
```

## 🤝 参与贡献

我们非常欢迎来自社区的贡献！无论是修复 Bug、完善文档还是提交新功能。

1.  **Fork** 本仓库。
2.  创建您的特性分支 (`git checkout -b feature/AmazingFeature`)。
3.  提交您的更改 (`git commit -m 'feat: 添加了一个很棒的功能'`)。
4.  推送到分支 (`git push origin feature/AmazingFeature`)。
5.  在 GitHub 上发起 **Pull Request**。

## 📄 开源协议

本项目采用 **MIT 协议** 进行分发。详情请参阅 `LICENSE` 文件。

这意味着您可以自由地使用、修改和分发本项目，但请保留原作者信息。我们**不推荐**将其包装为付费商业软件进行销售，请尊重同人社区的开源精神。

## 👥 作者与致谢

*   **[Renko_1055]** - *项目发起与核心开发* - [Github主页链接]
*   **境界景观学会** - *所属社团*

---

Designed with ❤️ for Doujin Circles.
