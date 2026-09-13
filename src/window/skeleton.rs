use objc2::rc::Retained;
use objc2::MainThreadMarker;
use objc2_app_kit::{NSColor, NSTextAlignment, NSTextField, NSView};
use objc2_foundation::{NSPoint, NSRect, NSSize};

use super::ui_helpers::{create_box, create_label};

pub fn build_skeleton_panel(
    mtm: MainThreadMarker,
    panel_rect: NSRect,
) -> (Retained<NSView>, Vec<Retained<NSTextField>>) {
    let color_card_bg = NSColor::colorWithSRGBRed_green_blue_alpha(0.12, 0.13, 0.15, 1.0);
    let color_cyan = NSColor::colorWithSRGBRed_green_blue_alpha(0.35, 0.75, 1.0, 1.0);
    let sk_init_color = NSColor::colorWithSRGBRed_green_blue_alpha(0.18, 0.20, 0.24, 1.0);

    let panel_skeleton = NSView::new(mtm);
    panel_skeleton.setFrame(panel_rect);
    panel_skeleton.setHidden(true);

    let sk_bg = create_box(mtm, NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(712.0, 362.0)), &color_card_bg);
    panel_skeleton.addSubview(&sk_bg);

    let mut skeleton_bars = Vec::new();

    // Loading hint
    let sk_hint = create_label(
        mtm,
        "⚡ 正在同步最新数据...",
        NSRect::new(NSPoint::new(24.0, 324.0), NSSize::new(220.0, 20.0)),
        12.0,
        true,
        Some(&color_cyan),
        NSTextAlignment::Left,
    );
    panel_skeleton.addSubview(&sk_hint);

    // Header placeholders: Title bar, Badge bar, Mode pills
    let sk_title = create_box(mtm, NSRect::new(NSPoint::new(24.0, 282.0), NSSize::new(140.0, 24.0)), &sk_init_color);
    panel_skeleton.addSubview(&sk_title);
    skeleton_bars.push(sk_title);

    let sk_badge = create_box(mtm, NSRect::new(NSPoint::new(180.0, 284.0), NSSize::new(210.0, 20.0)), &sk_init_color);
    panel_skeleton.addSubview(&sk_badge);
    skeleton_bars.push(sk_badge);

    for i in 0..4 {
        let sk_pill = create_box(
            mtm,
            NSRect::new(NSPoint::new(456.0 + i as f64 * 62.0, 282.0), NSSize::new(56.0, 24.0)),
            &sk_init_color,
        );
        panel_skeleton.addSubview(&sk_pill);
        skeleton_bars.push(sk_pill);
    }

    // Body paragraph rows (simulating cards / charts / ranking)
    let sk_row_configs = [
        (664.0, 226.0, 24.0),
        (610.0, 186.0, 24.0),
        (640.0, 146.0, 24.0),
        (570.0, 106.0, 24.0),
        (620.0, 66.0, 24.0),
    ];
    for (w, y, h) in sk_row_configs {
        let sk_row = create_box(mtm, NSRect::new(NSPoint::new(24.0, y), NSSize::new(w, h)), &sk_init_color);
        panel_skeleton.addSubview(&sk_row);
        skeleton_bars.push(sk_row);
    }

    // Footer placeholder
    let sk_foot1 = create_box(mtm, NSRect::new(NSPoint::new(24.0, 22.0), NSSize::new(320.0, 18.0)), &sk_init_color);
    panel_skeleton.addSubview(&sk_foot1);
    skeleton_bars.push(sk_foot1);

    let sk_foot2 = create_box(mtm, NSRect::new(NSPoint::new(550.0, 22.0), NSSize::new(138.0, 18.0)), &sk_init_color);
    panel_skeleton.addSubview(&sk_foot2);
    skeleton_bars.push(sk_foot2);

    (panel_skeleton, skeleton_bars)
}

pub fn animate_skeleton_step(skeleton_bars: &[Retained<NSTextField>], step: usize) -> usize {
    let levels = [0.15, 0.18, 0.23, 0.30, 0.23, 0.18];
    for (i, bar) in skeleton_bars.iter().enumerate() {
        let lvl = levels[(step + i) % levels.len()];
        let color = NSColor::colorWithSRGBRed_green_blue_alpha(lvl, lvl * 1.05, lvl * 1.15, 1.0);
        bar.setBackgroundColor(Some(&color));
    }
    (step + 1) % levels.len()
}
