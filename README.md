# Codex Monitor ☁

<p align="center">
  <img src="https://img.shields.io/badge/Platform-macOS-black?logo=apple" alt="Platform macOS" />
  <img src="https://img.shields.io/badge/Language-Rust%202021-orange?logo=rust" alt="Rust" />
  <img src="https://img.shields.io/badge/Framework-Pure%20AppKit%20(objc2)-blue" alt="AppKit" />
  <img src="https://img.shields.io/badge/Package%20Size-1.3%20MB-green" alt="Size" />
  <img src="https://img.shields.io/badge/License-MIT-purple" alt="License" />
</p>

<p align="center">
  <b>极速、超轻量、纯原生的 macOS OpenAI Codex 实时用量监控与数据看板</b><br>
  无需 Electron · 无需 Tauri · 无需 WebView · 仅占用 ~6MB 内存 · 安装包仅 1.3MB
</p>

<p align="center">
  <a href="README.md"><b>简体中文</b></a> | <a href="README_EN.md"><b>English</b></a>
</p>

---

## 📥 下载与安装 (macOS)

本项目已提供即开即用的 DMG 与 ZIP 预打包文件，无需任何编译环境即可直接安装使用：

| 文件 | 格式 | 说明 | 下载/路径 |
| :--- | :--- | :--- | :--- |
| **Codex-Monitor-macOS.dmg** | macOS DMG 镜像 | **推荐**。双击挂载后将 App 拖入 Applications 目录 | [dist/Codex-Monitor-macOS.dmg](dist/Codex-Monitor-macOS.dmg) |
| **Codex-Monitor-macOS.zip** | ZIP 压缩包 | 解压后即可直接双击运行 `Codex Monitor.app` | [dist/Codex-Monitor-macOS.zip](dist/Codex-Monitor-macOS.zip) |

> **安装提示**：首次打开如遇 macOS 安全提示，可在系统「设置」->「隐私与安全性」中点击「仍要打开」，或在终端执行 `xattr -cr "/Applications/Codex Monitor.app"` 即可正常启动。

---

## ✨ 核心特性

### 1. 🖥️ 原生全景用量监控看板 (760 × 630)
- **顶部用户信息 & 实时身份**：自动提取 Codex 账户邮箱、前缀用户名、头像缩写、以及 Pro (5x) / Team 等订阅计划徽章。
- **5 大核心指标卡**：
  - **累计 Token**（支持亿/万/Compact 动态格式化）
  - **峰值 Token**
  - **最长聊天时长**（智能格式化为小时/分钟，防止文字截断）
  - **当前连续活跃天数**
  - **最长连续活跃天数**
- **3 大速率与配额胶囊条**：
  - ⏱ 每小时消耗速率
  - 📅 本周消耗总量
  - ⚡ 实时周配额剩余比例（百分比高亮）
- **4 大核心数据视图灵活切换**：
  - 🔥 **活动热力图 (Activity Heatmap)**：24 周跨度 GitHub 风格网格，支持【每日】、【每周】、【累计增长】、【全景概览】4 种色彩与聚合模式，悬浮气泡即时显示详细 Token。
  - 📊 **每日用量 (Daily Bars)**：日维度条形消耗分布统计图。
  - 🤖 **模型排行 (Model Ranking)**：自动遍历 SQLite 本地会话与缓存，动态展示使用量最高的模型排行榜与推理等级（Reasoning Effort）占比。
  - 🌐 **特权券与广播 (Perks & Broadcast)**：重置特权券状态检测、本地会话链路洞察与全网重置播报。

### 2. ⚡ 实时骨架屏 (Skeleton Loading)
- 刷新与加载期间，顶部 5 张指标卡、3 个速率条与下方内容面板**全面接入骨架屏呼吸动效**。
- 高帧率明暗渐变动画同步脉冲，数据就绪后平滑无缝还原。

### 3. 🌐 全网限额重置追踪 (codex-resets.com)
- 顶部导航栏内嵌 **`🌐 重置: X小时前 ↗`** 专属交互胶囊，动态抓取并显示 [codex-resets.com](https://codex-resets.com/) 最新全网额度重置时间。
- 点击直接跳转官网查看历史重置记录与统计分析。

### 4. ⏱️ 15 秒防刷冷却机制 (Throttle Cooldown)
- 点击刷新后立即进入 **15 秒保护冷却期**，防止过度调用或触发频控。
- 窗口刷新按钮与系统菜单栏「立即刷新数据」项同步开启 15s 倒计时动画（如 `🔄 刷新 (14s)`），冷却期间自动禁用点击与快捷键触发。

### 5. 📦 智能环境检测与安装指引
- 启动时自动探测本机是否安装 Codex 桌面客户端或 CLI。
- 未安装时自动展示优雅的原生引导界面，提供官网客户端一键跳转下载以及 `npm i -g @openai/codex` 命令行**一键复制**功能与实时状态自检。

### 6.  原生系统主菜单与状态栏托盘
- **窗口置顶时**：接管 macOS 系统顶栏（` Codex Monitor 文件 编辑 显示 窗口 帮助`），支持全套标准快捷键（⌘D 看板、⌘R 刷新、⌘1-4 切标签、⌘W 关闭、⌘Q 退出）。
- **后台常驻时**：顶部菜单栏保留原生 `NSStatusItem` 托盘图标（`☁ Codex: 25% (Pro 5x)`），随时展开下拉菜单快捷操作。

---

## ⚡ 极致性能对比

| 指标 | 本项目 (Rust + AppKit) | Tauri 2 | PyQt / Python | Electron |
| :--- | :--- | :--- | :--- | :--- |
| **内存占用** | **~6 MB** | 30 ~ 60 MB | 100 MB+ | 150 ~ 300 MB |
| **安装包体积** | **1.3 MB** | 10 ~ 20 MB | 50 MB+ | 80 ~ 150 MB |
| **启动速度** | **< 30 ms** | ~ 500 ms | ~ 1.5 s | ~ 2 s |
| **运行时依赖**| **无任何外部依赖** | WebKit 引擎 | Python 解释器 | Chromium + Node.js |

---

## 🛠️ 项目架构与工程规范

本项目遵循极高标准的代码质量与工程规范：
- **严格模块化**：项目内 **所有 26 个 `.rs` 文件代码量均严格控制在 200 行以内**，无任何臃肿长文件。
- **零硬编码**：无硬编码的模型列表、日期或常量，所有数据均来自官方 JSON-RPC 通信通道 (`codex app-server --stdio`)、SQLite 动态解析或实时网络抓取。

```
src/
├── main.rs                 # 应用启动入口与 NSApplication 生命周期管理
├── config.rs               # 环境变量、Codex 二进制与配置探测
├── codex/                  # 数据层模块
│   ├── mod.rs              # 模块暴露
│   ├── models.rs           # 数据实体与序列化结构
│   ├── format.rs           # 统一 Token、时长与时间格式化引擎
│   ├── analytics.rs        # SQLite 会话解析与模型统计动态挖掘
│   ├── rpc.rs              # codex app-server JSON-RPC 通信协议
│   └── api.rs              # 网络回退抓取与 codex-resets.com 实时重置解析
├── menu/                   # 系统与托盘菜单模块
│   ├── mod.rs              # 状态管理、自动刷新与 15s 冷却倒计时调度
│   ├── builder.rs          # 托盘下拉菜单构建
│   ├── handler.rs          # 菜单事件动作监听器
│   └── main_menu.rs        # macOS 顶栏原生主菜单构建与状态联动
└── window/                 # 看板窗口与原生 AppKit UI 组件
    ├── mod.rs              # 窗口全局句柄与外部触发器
    ├── types.rs            # 视图模式与组件枚举定义
    ├── state.rs            # 窗口根容器构建与生命周期
    ├── state_update.rs     # 数据驱动视图更新与刷新状态同步
    ├── header.rs           # 用户卡片、重置链接与指标卡头部构建
    ├── ui_helpers.rs       # AppKit 视图创建与调色板工具
    ├── skeleton.rs         # 骨架屏布局与高帧率呼吸波浪渲染
    ├── heatmap_calc.rs     # 热力图数据聚合并发计算与工具提示
    ├── panel_heatmap.rs    # 24 周热力图网格面板渲染
    ├── panel_bars.rs       # 每日用量条形图面板
    ├── panel_models.rs     # 模型排行榜面板
    ├── panel_broadcast.rs  # 特权券与广播洞察面板
    ├── panel_install.rs    # 未安装 Codex 时的原生引导界面
    └── handler.rs          # 窗口交互事件处理器
```

---

## 🔨 本地开发与构建

如需自行修改源码或重新打包：

```bash
# 1. 检查语法与编译
cargo check

# 2. 本地调试运行
cargo run

# 3. 编译发布包并生成 macOS Application Bundle
./scripts/build-app.sh

# 4. 生成可分发的 DMG 安装包与 ZIP 压缩包
./scripts/package-release.sh
```

---

## 📄 开源许可证

本项目基于 [MIT License](LICENSE) 开源。
