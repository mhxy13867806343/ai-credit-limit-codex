# Codex Monitor ☁

> 超轻量级 macOS 原生菜单栏 OpenAI Codex 额度与状态监控工具，使用纯 **Rust + macOS 原生 AppKit (objc2)** 构建。

无需 Electron、无需 Tauri、无需 WebView，运行时内存仅 **~5MB ~ 10MB**，发布包体积仅 **1.2MB**。

---

## 效果预览

菜单栏常驻显示：
```
☁ Codex 72%
```

点击展开原生下拉菜单：
```
☁ Codex: 72% (Pro)
──────────────────────────
5h Window        72%
Weekly Limit     48%
Reset in         2h 13m
──────────────────────────
更新时间: 11:39:10
──────────────────────────
🔄 立即刷新        ⌘R
📁 打开配置目录
──────────────────────────
❌ 退出 Codex Monitor ⌘Q
```

---

## 技术架构

```
Codex Monitor.app
│
├── AppKit 原生菜单栏 (objc2 / objc2-app-kit)
│   ├── NSStatusBar & NSStatusItem (菜单栏常驻图标与文字)
│   ├── NSMenu & NSMenuItem (原生菜单组件)
│   └── LSUIElement = true (不占 Dock 栏，纯菜单栏常驻)
│
├── 额度抓取引擎 (src/codex.rs)
│   ├── 通道 1: Codex CLI JSON-RPC (`codex app-server --stdio`)
│   │   └── 调用 initialize 与 account/rateLimits/read，复用官方登录态与 Token 刷新机制
│   └── 通道 2: 本地 Token 直连 (~/.codex/auth.json + ureq)
│
└── 异步调度 (src/menu.rs)
    ├── 后台线程轮询，防止阻塞 UI
    └── 通过 libdispatch 安全分发至 macOS 主事件循环
```

---

## 性能对比

| 指标 | 本项目 (Rust + AppKit) | Tauri 2 | PyQt / Python | Electron |
| :--- | :--- | :--- | :--- | :--- |
| **内存占用** | **~6 MB** | 30 ~ 60 MB | 100 MB+ | 150 ~ 300 MB |
| **安装包体积** | **1.2 MB** | 10 ~ 20 MB | 50 MB+ | 80 ~ 150 MB |
| **启动速度** | **< 50 ms** | ~ 500 ms | ~ 1.5 s | ~ 2 s |
| **系统侵入** | 纯菜单栏 (无 Dock) | 依赖 WebKit | 需 Python 运行环境 | 依赖 Chromium |

---

## 快速上手

### 编译与运行

```bash
# 1. 运行测试
cargo test --bin codex-monitor

# 2. 编译并打包为 macOS 应用程序
./scripts/build-app.sh

# 3. 运行应用程序
open "target/Codex Monitor.app"
```

或者直接开发模式启动：
```bash
cargo run
```

---

## 目录结构

```
ai-credit-limit-codex/
├── src/
│   ├── main.rs          # 应用入口与 NSApplication 生命周期
│   ├── menu.rs          # macOS 菜单栏 (NSStatusBar / NSMenu)
│   ├── codex.rs         # Codex CLI RPC 协议交互与额度解析
│   └── config.rs        # 路径探测与配置参数
├── scripts/
│   ├── build-app.sh     # 自动构建并打包 .app 脚本
│   └── Info.plist       # macOS 应用元数据配置 (LSUIElement)
├── assets/              # 图标等静态资源
└── Cargo.toml           # 依赖与编译优化配置
```

---

## 后续扩展规划

该架构为后续扩展预留了充足空间，可平滑升级为综合 **AI 开发者状态监控中心**：

```
AI Monitor
├── ☁ OpenAI Codex 额度与重置时间
├── ⚡ Cursor 额度与用量
├── 🚀 Antigravity 运行状态
└── 💻 系统指标 (CPU / 内存 / GPU / 网络)
```

---

## License

MIT License.
