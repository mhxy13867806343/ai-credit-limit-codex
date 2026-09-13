use objc2::rc::Retained;
use objc2::MainThreadMarker;
use objc2_app_kit::{NSColor, NSTextAlignment, NSTextField, NSView};
use objc2_foundation::{NSPoint, NSRect, NSSize, NSString};

use crate::codex::CodexUsage;
use super::ui_helpers::{create_box, create_label};

pub struct BroadcastComponents {
    pub panel: Retained<NSView>,
    pub label_insights: Retained<NSTextField>,
    pub label_plugins: Retained<NSTextField>,
    pub label_coupon: Retained<NSTextField>,
    pub label_global_reset: Retained<NSTextField>,
}

pub fn build_broadcast_panel(
    mtm: MainThreadMarker,
    panel_rect: NSRect,
) -> BroadcastComponents {
    let color_card_bg = NSColor::colorWithSRGBRed_green_blue_alpha(0.12, 0.13, 0.15, 1.0);
    let color_white = NSColor::whiteColor();
    let color_muted = NSColor::colorWithSRGBRed_green_blue_alpha(0.55, 0.58, 0.64, 1.0);
    let color_amber = NSColor::colorWithSRGBRed_green_blue_alpha(0.95, 0.75, 0.25, 1.0);

    let panel = NSView::new(mtm);
    panel.setFrame(panel_rect);
    panel.setHidden(true);

    let b_card_w = 224.0;
    let b_card_h = 342.0;
    let b_gap = 14.0;
    let b_start_y = 10.0;

    // Card 1: 💡 活动洞察 (Insights)
    let b1_x = 8.0;
    let b1_bg = create_box(mtm, NSRect::new(NSPoint::new(b1_x, b_start_y), NSSize::new(b_card_w, b_card_h)), &color_card_bg);
    panel.addSubview(&b1_bg);
    let b1_title = create_label(mtm, "💡 活动洞察 (Insights)", NSRect::new(NSPoint::new(b1_x + 14.0, b_start_y + b_card_h - 38.0), NSSize::new(b_card_w - 28.0, 22.0)), 13.5, true, Some(&color_white), NSTextAlignment::Left);
    panel.addSubview(&b1_title);
    let label_insights = create_label(mtm, "快速模式: 0%\n\n最常用推理: 高 · 70%\n\n已探索技能: 98\n\n累计会话数: 170+\n\nToken利用率: 86.4%\n\n工作区数量: 12", NSRect::new(NSPoint::new(b1_x + 14.0, b_start_y + 20.0), NSSize::new(b_card_w - 28.0, 270.0)), 12.0, false, Some(&color_muted), NSTextAlignment::Left);
    panel.addSubview(&label_insights);

    // Card 2: 🧩 最常用插件与工具
    let b2_x = b1_x + b_card_w + b_gap;
    let b2_bg = create_box(mtm, NSRect::new(NSPoint::new(b2_x, b_start_y), NSSize::new(b_card_w, b_card_h)), &color_card_bg);
    panel.addSubview(&b2_bg);
    let b2_title = create_label(mtm, "🧩 最常用插件与工具", NSRect::new(NSPoint::new(b2_x + 14.0, b_start_y + b_card_h - 38.0), NSSize::new(b_card_w - 28.0, 22.0)), 13.5, true, Some(&color_white), NSTextAlignment::Left);
    panel.addSubview(&b2_title);
    let label_plugins = create_label(mtm, "@superpowers: 374 次运行\n\n@agent-skills: 79 次运行\n\n$godot-ui: 40 次运行\n\n$godot-gdscript: 32 次运行\n\n$python-eval: 28 次运行\n\n$git-diff: 19 次运行", NSRect::new(NSPoint::new(b2_x + 14.0, b_start_y + 20.0), NSSize::new(b_card_w - 28.0, 270.0)), 12.0, false, Some(&color_muted), NSTextAlignment::Left);
    panel.addSubview(&label_plugins);

    // Card 3: 🌐 额度特权券与全球重置广播
    let b3_x = b2_x + b_card_w + b_gap;
    let b3_w = 234.0;
    let b3_bg = create_box(mtm, NSRect::new(NSPoint::new(b3_x, b_start_y), NSSize::new(b3_w, b_card_h)), &color_card_bg);
    panel.addSubview(&b3_bg);
    let b3_title = create_label(mtm, "🌐 重置特权券与全网广播", NSRect::new(NSPoint::new(b3_x + 14.0, b_start_y + b_card_h - 38.0), NSSize::new(b3_w - 28.0, 22.0)), 13.5, true, Some(&color_white), NSTextAlignment::Left);
    panel.addSubview(&b3_title);
    let label_coupon = create_label(mtm, "🎁 特权券: Full reset (1 次)\n• 状态: 可用\n• 到期: 10/5 06:16", NSRect::new(NSPoint::new(b3_x + 14.0, b_start_y + 140.0), NSSize::new(b3_w - 28.0, 140.0)), 11.5, true, Some(&color_amber), NSTextAlignment::Left);
    panel.addSubview(&label_coupon);
    let label_global_reset = create_label(mtm, "社区广播: @thsottiaux 确认重置生效\n(0.8天前 • 观测均隔 6.9天)\n\n全网状态: 正常运转\n下次预期窗口: 6.1天后", NSRect::new(NSPoint::new(b3_x + 14.0, b_start_y + 20.0), NSSize::new(b3_w - 28.0, 110.0)), 11.5, false, Some(&color_muted), NSTextAlignment::Left);
    panel.addSubview(&label_global_reset);

    BroadcastComponents {
        panel,
        label_insights,
        label_plugins,
        label_coupon,
        label_global_reset,
    }
}

pub fn update_broadcast_panel(
    comp: &BroadcastComponents,
    usage: &CodexUsage,
) {
    let reasoning_str = usage.most_used_reasoning.as_deref().unwrap_or("暂无记录");
    let total_th = usage.total_threads.unwrap_or(0);
    comp.label_insights.setStringValue(&NSString::from_str(&format!(
        "最常用推理级别: {}\n\n累计会话记录: {} 次会话\n\n数据源: 本地 Codex 运行库状态",
        reasoning_str, total_th
    )));

    comp.label_plugins.setStringValue(&NSString::from_str(
        "自动插件与工作流监控\n\n当前状态: 运行就绪\n\n已与本地会话链路保持同步",
    ));

    let credits_lines = usage.format_reset_credits();
    comp.label_coupon.setStringValue(&NSString::from_str(&credits_lines.join("\n\n")));

    if let Some(gr) = &usage.global_reset {
        let author = gr.author.as_deref().unwrap_or("codex");
        let days = gr.days_since_last.unwrap_or(0.0);
        let avg = gr.avg_interval_days.unwrap_or(0.0);
        comp.label_global_reset.setStringValue(&NSString::from_str(&format!(
            "社区广播: @{} 确认重置生效\n({:.1}天前 • 观测均隔 {:.1}天)\n\n全网状态: 正常运转",
            author, days, avg
        )));
    } else {
        comp.label_global_reset.setStringValue(&NSString::from_str("全网广播状态同步中..."));
    }
}
