use objc2::rc::Retained;
use objc2::MainThreadMarker;
use objc2_app_kit::{NSColor, NSTextAlignment, NSView};
use objc2_foundation::{NSPoint, NSRect, NSSize, NSString};

use crate::codex::{self, DailyUsageItem};
use super::types::BarComponent;
use super::ui_helpers::{create_box, create_label};

pub fn build_daily_bars_panel(
    mtm: MainThreadMarker,
    panel_rect: NSRect,
) -> (Retained<NSView>, Vec<BarComponent>) {
    let color_card_bg = NSColor::colorWithSRGBRed_green_blue_alpha(0.12, 0.13, 0.15, 1.0);
    let color_card_subtle = NSColor::colorWithSRGBRed_green_blue_alpha(0.16, 0.17, 0.20, 1.0);
    let color_white = NSColor::whiteColor();
    let color_muted = NSColor::colorWithSRGBRed_green_blue_alpha(0.55, 0.58, 0.64, 1.0);
    let color_cyan = NSColor::colorWithSRGBRed_green_blue_alpha(0.35, 0.75, 1.0, 1.0);

    let panel = NSView::new(mtm);
    panel.setFrame(panel_rect);
    panel.setHidden(true);

    let p2_bg = create_box(mtm, NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(712.0, 362.0)), &color_card_bg);
    panel.addSubview(&p2_bg);

    let bar_header = create_label(
        mtm,
        "📊 近期每日 Token 消耗趋势 (Daily Usage Chart · 最近 10 天)",
        NSRect::new(NSPoint::new(18.0, 324.0), NSSize::new(550.0, 22.0)),
        14.5,
        true,
        Some(&color_white),
        NSTextAlignment::Left,
    );
    panel.addSubview(&bar_header);

    let num_bars = 10;
    let bar_w = 54.0;
    let bar_gap = 16.0;
    let bar_start_x = 18.0;
    let bar_base_y = 56.0;
    let bar_max_h = 210.0;

    let mut daily_bars = Vec::with_capacity(num_bars);

    for i in 0..num_bars {
        let bx = bar_start_x + i as f64 * (bar_w + bar_gap);

        let val_label = create_label(
            mtm,
            "--",
            NSRect::new(NSPoint::new(bx - 6.0, bar_base_y + bar_max_h + 4.0), NSSize::new(bar_w + 12.0, 18.0)),
            11.0,
            true,
            Some(&color_cyan),
            NSTextAlignment::Center,
        );
        panel.addSubview(&val_label);

        let track = create_box(
            mtm,
            NSRect::new(NSPoint::new(bx, bar_base_y), NSSize::new(bar_w, bar_max_h)),
            &color_card_subtle,
        );
        panel.addSubview(&track);

        let fill = create_box(
            mtm,
            NSRect::new(NSPoint::new(bx, bar_base_y), NSSize::new(bar_w, 20.0)),
            &color_cyan,
        );
        panel.addSubview(&fill);

        let date_label = create_label(
            mtm,
            "--",
            NSRect::new(NSPoint::new(bx - 6.0, bar_base_y - 24.0), NSSize::new(bar_w + 12.0, 18.0)),
            11.0,
            false,
            Some(&color_muted),
            NSTextAlignment::Center,
        );
        panel.addSubview(&date_label);

        daily_bars.push(BarComponent {
            label_val: val_label,
            track,
            fill,
            label_date: date_label,
            x: bx,
            y: bar_base_y,
            width: bar_w,
            max_height: bar_max_h,
        });
    }

    (panel, daily_bars)
}

pub fn update_daily_bars(daily_bars: &[BarComponent], buckets: &[DailyUsageItem]) {
    let recent = if buckets.len() > 10 {
        &buckets[buckets.len() - 10..]
    } else {
        buckets
    };

    let max_val = recent.iter().map(|b| b.tokens).max().unwrap_or(1).max(1);

    for (i, bar) in daily_bars.iter().enumerate() {
        if i < recent.len() {
            let b = &recent[i];
            let short_date = if b.date.len() >= 10 {
                &b.date[5..]
            } else {
                &b.date
            };
            bar.label_date.setStringValue(&NSString::from_str(short_date));

            let val_str = codex::format_tokens_compact(b.tokens);
            bar.label_val.setStringValue(&NSString::from_str(&val_str));

            let ratio = (b.tokens as f64 / max_val as f64).clamp(0.02, 1.0);
            let h = (bar.max_height * ratio).max(4.0);

            bar.fill.setFrame(NSRect::new(
                NSPoint::new(bar.x, bar.y),
                NSSize::new(bar.width, h),
            ));
        } else {
            bar.label_date.setStringValue(&NSString::from_str("--"));
            bar.label_val.setStringValue(&NSString::from_str("--"));
            bar.fill.setFrame(NSRect::new(
                NSPoint::new(bar.x, bar.y),
                NSSize::new(bar.width, 0.0),
            ));
        }
    }
}
