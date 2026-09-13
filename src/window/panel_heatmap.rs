use objc2::rc::Retained;
use objc2::{sel, MainThreadMarker};
use objc2_app_kit::{NSButton, NSColor, NSScrollView, NSTextAlignment, NSTextField, NSView};
use objc2_foundation::{ns_string, NSObject, NSPoint, NSRect, NSSize};

use super::ui_helpers::{cell_date_dynamic, create_box, create_label};

pub struct HeatmapComponents {
    pub panel: Retained<NSView>,
    pub tab_daily_bg: Retained<NSTextField>,
    pub tab_weekly_bg: Retained<NSTextField>,
    pub tab_cumul_bg: Retained<NSTextField>,
    pub tab_all_bg: Retained<NSTextField>,
    pub tab_daily_lbl: Retained<NSTextField>,
    pub tab_weekly_lbl: Retained<NSTextField>,
    pub tab_cumul_lbl: Retained<NSTextField>,
    pub tab_all_lbl: Retained<NSTextField>,
    pub hm_mode_desc: Retained<NSTextField>,
    pub hm_summary_badge: Retained<NSTextField>,
    pub hm_total_metric: Retained<NSTextField>,
    pub hm_scroll_view: Retained<NSScrollView>,
    pub heatmap_tiles: Vec<Retained<NSTextField>>,
    pub leg_tiles: Vec<Retained<NSTextField>>,
}

pub fn build_heatmap_panel(
    mtm: MainThreadMarker,
    panel_rect: NSRect,
    handler_obj: &NSObject,
) -> HeatmapComponents {
    let color_card_bg = NSColor::colorWithSRGBRed_green_blue_alpha(0.12, 0.13, 0.15, 1.0);
    let color_white = NSColor::whiteColor();
    let color_muted = NSColor::colorWithSRGBRed_green_blue_alpha(0.55, 0.58, 0.64, 1.0);
    let color_cyan = NSColor::colorWithSRGBRed_green_blue_alpha(0.35, 0.75, 1.0, 1.0);
    let color_tab_active = NSColor::colorWithSRGBRed_green_blue_alpha(0.14, 0.38, 0.92, 1.0);
    let color_transparent = NSColor::clearColor();

    let panel = NSView::new(mtm);
    panel.setFrame(panel_rect);

    let p1_bg = create_box(mtm, NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(712.0, 362.0)), &color_card_bg);
    panel.addSubview(&p1_bg);

    let hm_header = create_label(mtm, "Token 活动", NSRect::new(NSPoint::new(14.0, 324.0), NSSize::new(82.0, 24.0)), 15.0, true, Some(&color_white), NSTextAlignment::Left);
    panel.addSubview(&hm_header);

    let hm_summary_badge = create_label(mtm, "用量统计加载中...", NSRect::new(NSPoint::new(104.0, 326.0), NSSize::new(220.0, 18.0)), 12.0, false, Some(&color_muted), NSTextAlignment::Left);
    panel.addSubview(&hm_summary_badge);

    let hm_mode_desc = create_label(mtm, "🔵 每日模式 (蓝色用量热力图)", NSRect::new(NSPoint::new(326.0, 326.0), NSSize::new(138.0, 18.0)), 11.0, false, Some(&color_cyan), NSTextAlignment::Left);
    panel.addSubview(&hm_mode_desc);

    let tab_box_bg = NSColor::colorWithSRGBRed_green_blue_alpha(0.14, 0.15, 0.18, 1.0);
    let tab_container = create_box(mtm, NSRect::new(NSPoint::new(456.0, 320.0), NSSize::new(246.0, 26.0)), &tab_box_bg);
    panel.addSubview(&tab_container);

    let tab_w = 60.0;
    let tab_h = 24.0;
    let tab_y = 321.0;

    let tab_daily_bg = create_box(mtm, NSRect::new(NSPoint::new(458.0, tab_y), NSSize::new(tab_w, tab_h)), &color_tab_active);
    panel.addSubview(&tab_daily_bg);
    let tab_daily_lbl = create_label(mtm, "🔵 每日", NSRect::new(NSPoint::new(458.0, tab_y + 4.0), NSSize::new(tab_w, 16.0)), 11.0, false, Some(&color_white), NSTextAlignment::Center);
    panel.addSubview(&tab_daily_lbl);
    let btn_daily = unsafe { NSButton::buttonWithTitle_target_action(ns_string!(""), Some(handler_obj), Some(sel!(dailyModeClicked:)), mtm) };
    btn_daily.setFrame(NSRect::new(NSPoint::new(458.0, tab_y), NSSize::new(tab_w, tab_h)));
    btn_daily.setTransparent(true);
    panel.addSubview(&btn_daily);

    let tab_weekly_bg = create_box(mtm, NSRect::new(NSPoint::new(519.0, tab_y), NSSize::new(tab_w, tab_h)), &color_transparent);
    panel.addSubview(&tab_weekly_bg);
    let tab_weekly_lbl = create_label(mtm, "🔴 每周", NSRect::new(NSPoint::new(519.0, tab_y + 4.0), NSSize::new(tab_w, 16.0)), 11.0, false, Some(&color_muted), NSTextAlignment::Center);
    panel.addSubview(&tab_weekly_lbl);
    let btn_weekly = unsafe { NSButton::buttonWithTitle_target_action(ns_string!(""), Some(handler_obj), Some(sel!(weeklyModeClicked:)), mtm) };
    btn_weekly.setFrame(NSRect::new(NSPoint::new(519.0, tab_y), NSSize::new(tab_w, tab_h)));
    btn_weekly.setTransparent(true);
    panel.addSubview(&btn_weekly);

    let tab_cumul_bg = create_box(mtm, NSRect::new(NSPoint::new(580.0, tab_y), NSSize::new(tab_w, tab_h)), &color_transparent);
    panel.addSubview(&tab_cumul_bg);
    let tab_cumul_lbl = create_label(mtm, "🟢 累计", NSRect::new(NSPoint::new(580.0, tab_y + 4.0), NSSize::new(tab_w, 16.0)), 11.0, false, Some(&color_muted), NSTextAlignment::Center);
    panel.addSubview(&tab_cumul_lbl);
    let btn_cumul = unsafe { NSButton::buttonWithTitle_target_action(ns_string!(""), Some(handler_obj), Some(sel!(cumulativeModeClicked:)), mtm) };
    btn_cumul.setFrame(NSRect::new(NSPoint::new(580.0, tab_y), NSSize::new(tab_w, tab_h)));
    btn_cumul.setTransparent(true);
    panel.addSubview(&btn_cumul);

    let tab_all_bg = create_box(mtm, NSRect::new(NSPoint::new(641.0, tab_y), NSSize::new(tab_w, tab_h)), &color_transparent);
    panel.addSubview(&tab_all_bg);
    let tab_all_lbl = create_label(mtm, "🟣 所有", NSRect::new(NSPoint::new(641.0, tab_y + 4.0), NSSize::new(tab_w, 16.0)), 11.0, false, Some(&color_muted), NSTextAlignment::Center);
    panel.addSubview(&tab_all_lbl);
    let btn_all = unsafe { NSButton::buttonWithTitle_target_action(ns_string!(""), Some(handler_obj), Some(sel!(allModeClicked:)), mtm) };
    btn_all.setFrame(NSRect::new(NSPoint::new(641.0, tab_y), NSSize::new(tab_w, tab_h)));
    btn_all.setTransparent(true);
    panel.addSubview(&btn_all);

    let hm_scroll_view = NSScrollView::new(mtm);
    hm_scroll_view.setFrame(NSRect::new(NSPoint::new(10.0, 52.0), NSSize::new(692.0, 246.0)));
    hm_scroll_view.setHasHorizontalScroller(true);
    hm_scroll_view.setHasVerticalScroller(false);
    hm_scroll_view.setAutohidesScrollers(true);
    hm_scroll_view.setDrawsBackground(false);

    let hm_cols = 24;
    let hm_rows = 7;
    let tile_w = 34.0;
    let tile_h = 22.0;
    let tile_gap_y = 5.0;
    let col_step = 39.0;
    let left_pad = 28.0;
    let doc_w = left_pad + hm_cols as f64 * col_step + 16.0;

    let doc_view = NSView::new(mtm);
    doc_view.setFrame(NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(doc_w, 240.0)));

    let wd_color = NSColor::colorWithSRGBRed_green_blue_alpha(0.55, 0.58, 0.64, 1.0);
    for (day_text, row_idx) in [("一", 6.0), ("三", 4.0), ("五", 2.0)] {
        let wd_lbl = create_label(mtm, day_text, NSRect::new(NSPoint::new(6.0, row_idx * 27.0 + 10.0), NSSize::new(18.0, 18.0)), 11.0, false, Some(&wd_color), NSTextAlignment::Center);
        doc_view.addSubview(&wd_lbl);
    }

    let month_markers_fn = || {
        let mut markers = Vec::new();
        let mut prev_m = 0;
        for c in 0..hm_cols {
            let (_, m, _) = cell_date_dynamic(c, 0, hm_cols);
            if m != prev_m {
                prev_m = m;
                markers.push((c, format!("{}月", m)));
            }
        }
        markers
    };
    for (w_col, m_text) in month_markers_fn() {
        let mx = left_pad + w_col as f64 * col_step;
        let m_lbl = create_label(mtm, &m_text, NSRect::new(NSPoint::new(mx, 208.0), NSSize::new(45.0, 18.0)), 11.0, true, Some(&color_muted), NSTextAlignment::Left);
        doc_view.addSubview(&m_lbl);
    }

    let empty_tile_color = NSColor::colorWithSRGBRed_green_blue_alpha(0.13, 0.14, 0.17, 1.0);
    let mut heatmap_tiles = Vec::with_capacity(hm_cols * hm_rows);
    for col in 0..hm_cols {
        for row in 0..hm_rows {
            let tx = left_pad + col as f64 * col_step;
            let ty = (6 - row) as f64 * (tile_h + tile_gap_y) + 6.0;
            let tile = create_box(mtm, NSRect::new(NSPoint::new(tx, ty), NSSize::new(tile_w, tile_h)), &empty_tile_color);
            doc_view.addSubview(&tile);
            heatmap_tiles.push(tile);
        }
    }

    hm_scroll_view.setDocumentView(Some(&doc_view));
    panel.addSubview(&hm_scroll_view);

    let leg_y = 16.0;
    let hm_total_metric = create_label(mtm, "📊 用量统计加载中...", NSRect::new(NSPoint::new(14.0, leg_y), NSSize::new(470.0, 20.0)), 11.5, false, Some(&color_cyan), NSTextAlignment::Left);
    panel.addSubview(&hm_total_metric);

    let leg_lbl_less = create_label(mtm, "少", NSRect::new(NSPoint::new(546.0, leg_y), NSSize::new(20.0, 20.0)), 10.5, false, Some(&color_muted), NSTextAlignment::Right);
    panel.addSubview(&leg_lbl_less);

    let leg_colors = [
        NSColor::colorWithSRGBRed_green_blue_alpha(0.14, 0.15, 0.18, 1.0),
        NSColor::colorWithSRGBRed_green_blue_alpha(0.10, 0.28, 0.55, 1.0),
        NSColor::colorWithSRGBRed_green_blue_alpha(0.13, 0.42, 0.85, 1.0),
        NSColor::colorWithSRGBRed_green_blue_alpha(0.16, 0.55, 1.0, 1.0),
        NSColor::colorWithSRGBRed_green_blue_alpha(0.45, 0.78, 1.0, 1.0),
    ];
    let mut leg_tiles = Vec::with_capacity(leg_colors.len());
    for (idx, color) in leg_colors.iter().enumerate() {
        let bx = 566.0 + idx as f64 * 15.0;
        let box_view = create_box(mtm, NSRect::new(NSPoint::new(bx, leg_y + 4.0), NSSize::new(12.0, 10.0)), color);
        panel.addSubview(&box_view);
        leg_tiles.push(box_view);
    }

    let leg_lbl_more = create_label(mtm, "多", NSRect::new(NSPoint::new(566.0 + 6.0 * 15.0 + 2.0, leg_y), NSSize::new(20.0, 20.0)), 10.5, false, Some(&color_muted), NSTextAlignment::Left);
    panel.addSubview(&leg_lbl_more);

    HeatmapComponents {
        panel,
        tab_daily_bg,
        tab_weekly_bg,
        tab_cumul_bg,
        tab_all_bg,
        tab_daily_lbl,
        tab_weekly_lbl,
        tab_cumul_lbl,
        tab_all_lbl,
        hm_mode_desc,
        hm_summary_badge,
        hm_total_metric,
        hm_scroll_view,
        heatmap_tiles,
        leg_tiles,
    }
}
