use objc2::rc::Retained;
use objc2::MainThreadMarker;
use objc2_app_kit::{NSColor, NSTextAlignment, NSView};
use objc2_foundation::{NSPoint, NSRect, NSSize, NSString};

use crate::codex::ModelUsageItem;
use super::types::ModelBarComponent;
use super::ui_helpers::{create_box, create_label};

pub fn build_models_panel(
    mtm: MainThreadMarker,
    panel_rect: NSRect,
) -> (Retained<NSView>, Vec<ModelBarComponent>) {
    let color_card_bg = NSColor::colorWithSRGBRed_green_blue_alpha(0.12, 0.13, 0.15, 1.0);
    let color_card_subtle = NSColor::colorWithSRGBRed_green_blue_alpha(0.16, 0.17, 0.20, 1.0);
    let color_white = NSColor::whiteColor();
    let color_cyan = NSColor::colorWithSRGBRed_green_blue_alpha(0.35, 0.75, 1.0, 1.0);

    let panel = NSView::new(mtm);
    panel.setFrame(panel_rect);
    panel.setHidden(true);

    let p3_bg = create_box(mtm, NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(712.0, 362.0)), &color_card_bg);
    panel.addSubview(&p3_bg);

    let model_header = create_label(
        mtm,
        "🤖 各模型 Token 消耗排行与占比",
        NSRect::new(NSPoint::new(18.0, 324.0), NSSize::new(600.0, 22.0)),
        14.5,
        true,
        Some(&color_white),
        NSTextAlignment::Left,
    );
    panel.addSubview(&model_header);

    let max_bars = 8;
    let m_bar_w = 676.0;
    let m_bar_h = 7.0;
    let m_row_h = 34.0;
    let m_start_y = 278.0;

    let mut model_bars = Vec::with_capacity(max_bars);

    for i in 0..max_bars {
        let my = m_start_y - i as f64 * m_row_h;

        let label_info = create_label(
            mtm,
            "",
            NSRect::new(NSPoint::new(18.0, my + 9.0), NSSize::new(m_bar_w, 16.0)),
            12.0,
            false,
            Some(&color_white),
            NSTextAlignment::Left,
        );
        panel.addSubview(&label_info);

        let track = create_box(
            mtm,
            NSRect::new(NSPoint::new(18.0, my), NSSize::new(m_bar_w, m_bar_h)),
            &color_card_subtle,
        );
        panel.addSubview(&track);

        let fill = create_box(
            mtm,
            NSRect::new(NSPoint::new(18.0, my), NSSize::new(0.0, m_bar_h)),
            &color_cyan,
        );
        panel.addSubview(&fill);

        model_bars.push(ModelBarComponent {
            label_info,
            track,
            fill,
            x: 18.0,
            y: my,
            width: m_bar_w,
            height: m_bar_h,
        });
    }

    (panel, model_bars)
}

pub fn update_top_models(model_bars: &[ModelBarComponent], models: &[ModelUsageItem]) {
    for (i, bar) in model_bars.iter().enumerate() {
        if i < models.len() {
            let m = &models[i];
            bar.track.setHidden(false);
            bar.fill.setHidden(false);
            bar.label_info.setHidden(false);

            let tokens_compact = crate::codex::format_tokens_compact(m.tokens_used);
            let b_val = m.tokens_used as f64 / 1_000_000_000.0;
            let text = if m.tokens_used > 0 {
                format!("{}  •  {} ({:.2}B)  ({} 次会话 • {:.1}%)", m.model_name, tokens_compact, b_val, m.thread_count, m.percentage)
            } else {
                format!("{}  •  0 (0.00B)  ({} 次会话 • {:.1}%)", m.model_name, m.thread_count, m.percentage)
            };
            bar.label_info.setStringValue(&NSString::from_str(&text));

            let fill_w = if m.tokens_used > 0 {
                (bar.width * (m.percentage / 100.0)).clamp(4.0, bar.width)
            } else {
                0.0
            };
            bar.fill.setFrame(NSRect::new(NSPoint::new(bar.x, bar.y), NSSize::new(fill_w, bar.height)));
        } else {
            bar.track.setHidden(true);
            bar.fill.setHidden(true);
            bar.label_info.setHidden(true);
        }
    }
}
