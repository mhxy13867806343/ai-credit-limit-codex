use objc2::rc::Retained;
use objc2::{sel, MainThreadMarker};
use objc2_app_kit::{NSColor, NSTextAlignment, NSTextField, NSView};
use objc2_foundation::{NSObject, NSPoint, NSRect, NSSize, NSString};

use super::state::DashboardWindow;
use super::ui_helpers::{
    color_bg, color_card, color_cyan, color_muted, color_primary, create_box,
    create_clickable_box, create_label,
};
use crate::config;

pub struct InstallPanelComponents {
    pub panel: Retained<NSView>,
    pub status_label: Retained<NSTextField>,
    pub btn_copy_label: Retained<NSTextField>,
    pub btn_to_dash: Retained<NSTextField>,
}

pub fn build_install_panel(
    mtm: MainThreadMarker,
    frame: NSRect,
    handler: &NSObject,
) -> InstallPanelComponents {
    let panel = NSView::new(mtm);
    panel.setFrame(frame);

    let bg = create_box(mtm, frame, &color_bg());
    panel.addSubview(&bg);

    let color_white = NSColor::whiteColor();
    let c_card = color_card();
    let c_cyan = color_cyan();
    let c_muted = color_muted();
    let c_blue = color_primary();
    let c_term_bg = NSColor::colorWithSRGBRed_green_blue_alpha(0.06, 0.07, 0.08, 1.0);

    // Header
    let icon_lbl = create_label(mtm, "⚡", NSRect::new(NSPoint::new(32.0, 560.0), NSSize::new(696.0, 44.0)), 36.0, true, Some(&c_cyan), NSTextAlignment::Center);
    panel.addSubview(&icon_lbl);

    let title_lbl = create_label(mtm, "未检测到本地 Codex 客户端", NSRect::new(NSPoint::new(32.0, 524.0), NSSize::new(696.0, 32.0)), 22.0, true, Some(&color_white), NSTextAlignment::Center);
    panel.addSubview(&title_lbl);

    let sub_lbl = create_label(mtm, "Codex 监控器需要本地已安装桌面客户端或 CLI 命令行工具，请选择以下方式安装：", NSRect::new(NSPoint::new(32.0, 496.0), NSSize::new(696.0, 20.0)), 12.0, false, Some(&c_muted), NSTextAlignment::Center);
    panel.addSubview(&sub_lbl);

    // Card 1: Desktop App
    let card1 = create_box(mtm, NSRect::new(NSPoint::new(32.0, 140.0), NSSize::new(336.0, 335.0)), &c_card);
    panel.addSubview(&card1);

    let c1_icon = create_label(mtm, "📥", NSRect::new(NSPoint::new(48.0, 420.0), NSSize::new(40.0, 32.0)), 24.0, false, Some(&color_white), NSTextAlignment::Left);
    panel.addSubview(&c1_icon);
    let c1_title = create_label(mtm, "方式一：下载桌面客户端", NSRect::new(NSPoint::new(88.0, 424.0), NSSize::new(260.0, 24.0)), 16.0, true, Some(&color_white), NSTextAlignment::Left);
    panel.addSubview(&c1_title);
    let c1_badge = create_label(mtm, "推荐 • 完整官方功能", NSRect::new(NSPoint::new(88.0, 404.0), NSSize::new(260.0, 16.0)), 11.0, false, Some(&c_cyan), NSTextAlignment::Left);
    panel.addSubview(&c1_badge);

    let c1_desc = "OpenAI 官方推出的独立原生桌面应用：\n\n• 提供完整的独立图形窗口与快捷键调用\n• 自动管理登录态与 Pro 配额额度\n• 内置完整后台 API 服务，即开即连";
    let c1_desc_lbl = create_label(mtm, c1_desc, NSRect::new(NSPoint::new(48.0, 270.0), NSSize::new(304.0, 120.0)), 12.0, false, Some(&c_muted), NSTextAlignment::Left);
    panel.addSubview(&c1_desc_lbl);

    create_clickable_box(mtm, &panel, NSRect::new(NSPoint::new(48.0, 180.0), NSSize::new(304.0, 38.0)), "🌐 打开官网下载页面 ↗", 13.0, &c_blue, &color_white, handler, sel!(openCodexDownloadClicked:));

    // Card 2: CLI
    let card2 = create_box(mtm, NSRect::new(NSPoint::new(392.0, 140.0), NSSize::new(336.0, 335.0)), &c_card);
    panel.addSubview(&card2);

    let c2_icon = create_label(mtm, "💻", NSRect::new(NSPoint::new(408.0, 420.0), NSSize::new(40.0, 32.0)), 24.0, false, Some(&color_white), NSTextAlignment::Left);
    panel.addSubview(&c2_icon);
    let c2_title = create_label(mtm, "方式二：命令行安装 CLI", NSRect::new(NSPoint::new(448.0, 424.0), NSSize::new(260.0, 24.0)), 16.0, true, Some(&color_white), NSTextAlignment::Left);
    panel.addSubview(&c2_title);
    let c2_badge = create_label(mtm, "开发者推荐 • 轻量高效", NSRect::new(NSPoint::new(448.0, 404.0), NSSize::new(260.0, 16.0)), 11.0, false, Some(&c_cyan), NSTextAlignment::Left);
    panel.addSubview(&c2_badge);

    let c2_sub = create_label(mtm, "通过 npm 全局安装官方 Codex CLI 工具：", NSRect::new(NSPoint::new(408.0, 368.0), NSSize::new(304.0, 20.0)), 12.0, false, Some(&c_muted), NSTextAlignment::Left);
    panel.addSubview(&c2_sub);

    let term_bg = create_box(mtm, NSRect::new(NSPoint::new(408.0, 310.0), NSSize::new(304.0, 44.0)), &c_term_bg);
    panel.addSubview(&term_bg);
    let term_lbl = create_label(mtm, "$ npm i -g @openai/codex", NSRect::new(NSPoint::new(418.0, 322.0), NSSize::new(284.0, 20.0)), 12.5, true, Some(&color_white), NSTextAlignment::Left);
    panel.addSubview(&term_lbl);

    let (_, btn_copy_label) = create_clickable_box(mtm, &panel, NSRect::new(NSPoint::new(408.0, 256.0), NSSize::new(304.0, 36.0)), "📋 复制安装命令", 12.5, &c_blue, &color_white, handler, sel!(copyCliInstallClicked:));

    let c2_hint = "安装后在终端运行:\n$ codex login\n完成登录授权后，点击下方「重新检测」即可！";
    let c2_hint_lbl = create_label(mtm, c2_hint, NSRect::new(NSPoint::new(408.0, 164.0), NSSize::new(304.0, 76.0)), 11.0, false, Some(&c_muted), NSTextAlignment::Left);
    panel.addSubview(&c2_hint_lbl);

    // Bottom Bar
    let bot_bg = create_box(mtm, NSRect::new(NSPoint::new(32.0, 36.0), NSSize::new(696.0, 80.0)), &c_card);
    panel.addSubview(&bot_bg);

    let status_label = create_label(mtm, "🔍 正在检测本地环境...", NSRect::new(NSPoint::new(48.0, 64.0), NSSize::new(380.0, 24.0)), 12.5, false, Some(&c_cyan), NSTextAlignment::Left);
    panel.addSubview(&status_label);

    create_clickable_box(mtm, &panel, NSRect::new(NSPoint::new(438.0, 58.0), NSSize::new(130.0, 36.0)), "🔄 重新检测", 12.0, &c_card, &color_white, handler, sel!(recheckInstallClicked:));

    let (_, btn_to_dash) = create_clickable_box(mtm, &panel, NSRect::new(NSPoint::new(580.0, 58.0), NSSize::new(136.0, 36.0)), "📊 进入看板 ↗", 12.0, &c_blue, &color_white, handler, sel!(switchToDashboardClicked:));

    InstallPanelComponents {
        panel,
        status_label,
        btn_copy_label,
        btn_to_dash,
    }
}

impl DashboardWindow {
    pub fn show_install_guide(&self) {
        *self.is_showing_install_guide.borrow_mut() = true;
        self.install_panel.panel.setHidden(false);
        self.header.container.setHidden(true);
        self.label_time.setHidden(true);
        self.panel_skeleton.setHidden(true);
        self.hm.panel.setHidden(true);
        self.panel_daily_bars.setHidden(true);
        self.panel_models.setHidden(true);
        self.broadcast.panel.setHidden(true);
        self.refresh_install_status();
    }

    pub fn show_dashboard_view(&self) {
        *self.is_showing_install_guide.borrow_mut() = false;
        self.install_panel.panel.setHidden(true);
        self.header.container.setHidden(false);
        self.label_time.setHidden(false);
        let tab = *self.current_main_tab.borrow();
        self.set_main_tab(tab);
    }

    pub fn refresh_install_status(&self) {
        let app_ok = config::is_codex_desktop_app_installed();
        let cli_ok = config::is_codex_cli_installed();
        let status_text = format!(
            "🖥️ 桌面应用: {}  •  💻 CLI 工具: {}",
            if app_ok { "已安装 ✓" } else { "未检测到" },
            if cli_ok { "已安装 ✓" } else { "未检测到" }
        );
        self.install_panel.status_label.setStringValue(&NSString::from_str(&status_text));
        self.install_panel.btn_to_dash.setHidden(!config::is_codex_installed());
    }
}
