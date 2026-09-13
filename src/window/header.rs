use objc2::rc::Retained;
use objc2::{sel, MainThreadMarker};
use objc2_app_kit::{NSButton, NSColor, NSTextAlignment, NSTextField, NSView};
use objc2_foundation::{ns_string, NSObject, NSPoint, NSRect, NSSize, NSString};

use super::ui_helpers::{create_box, create_label};

#[allow(dead_code)]
pub struct HeaderComponents {
    pub container: Retained<NSView>,
    pub avatar_text: Retained<NSTextField>,
    pub label_name: Retained<NSTextField>,
    pub label_handle: Retained<NSTextField>,
    pub plan_lbl: Retained<NSTextField>,
    pub btn_reset_lbl: Retained<NSTextField>,
    pub btn_refresh_lbl: Retained<NSTextField>,
    pub card_vals: [Retained<NSTextField>; 5],
    pub card_subs: [Retained<NSTextField>; 5],
    pub card_skeletons: [Retained<NSTextField>; 5],
    pub rate_hourly_val: Retained<NSTextField>,
    pub rate_weekly_val: Retained<NSTextField>,
    pub rate_quota_val: Retained<NSTextField>,
    pub rate_skeletons: [Retained<NSTextField>; 3],
    pub main_tab_hm_bg: Retained<NSTextField>,
    pub main_tab_hm_lbl: Retained<NSTextField>,
    pub main_tab_bars_bg: Retained<NSTextField>,
    pub main_tab_bars_lbl: Retained<NSTextField>,
    pub main_tab_models_bg: Retained<NSTextField>,
    pub main_tab_models_lbl: Retained<NSTextField>,
    pub main_tab_broad_bg: Retained<NSTextField>,
    pub main_tab_broad_lbl: Retained<NSTextField>,
}

pub fn build_header(
    mtm: MainThreadMarker,
    view: &NSView,
    handler_obj: &NSObject,
) -> HeaderComponents {
    let container = NSView::new(mtm);
    container.setFrame(NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(760.0, 630.0)));
    view.addSubview(&container);

    let c_card = NSColor::colorWithSRGBRed_green_blue_alpha(0.12, 0.13, 0.15, 1.0);
    let c_subtle = NSColor::colorWithSRGBRed_green_blue_alpha(0.16, 0.17, 0.20, 1.0);
    let c_white = NSColor::whiteColor();
    let c_muted = NSColor::colorWithSRGBRed_green_blue_alpha(0.55, 0.58, 0.64, 1.0);
    let c_cyan = NSColor::colorWithSRGBRed_green_blue_alpha(0.35, 0.75, 1.0, 1.0);
    let c_green = NSColor::colorWithSRGBRed_green_blue_alpha(0.09, 0.65, 0.45, 1.0);
    let c_amber = NSColor::colorWithSRGBRed_green_blue_alpha(0.95, 0.75, 0.25, 1.0);
    let c_act = NSColor::colorWithSRGBRed_green_blue_alpha(0.14, 0.38, 0.92, 1.0);
    let c_trans = NSColor::clearColor();

    // 1. Profile avatar & user info
    let avatar_bg = create_box(mtm, NSRect::new(NSPoint::new(24.0, 566.0), NSSize::new(44.0, 44.0)), &c_green);
    container.addSubview(&avatar_bg);
    let avatar_text = create_label(mtm, "--", NSRect::new(NSPoint::new(24.0, 576.0), NSSize::new(44.0, 24.0)), 18.0, true, Some(&c_white), NSTextAlignment::Center);
    container.addSubview(&avatar_text);
    let label_name = create_label(mtm, "加载中...", NSRect::new(NSPoint::new(78.0, 588.0), NSSize::new(220.0, 24.0)), 18.0, true, Some(&c_white), NSTextAlignment::Left);
    container.addSubview(&label_name);
    let label_handle = create_label(mtm, "--", NSRect::new(NSPoint::new(78.0, 568.0), NSSize::new(220.0, 18.0)), 12.5, false, Some(&c_muted), NSTextAlignment::Left);
    container.addSubview(&label_handle);

    // 2. Action buttons
    let btn_y = 572.0;
    let b_rst = create_box(mtm, NSRect::new(NSPoint::new(248.0, btn_y), NSSize::new(118.0, 28.0)), &c_subtle);
    container.addSubview(&b_rst);
    let btn_reset_lbl = create_label(mtm, "🌐 重置: 24h前 ↗", NSRect::new(NSPoint::new(248.0, btn_y + 4.0), NSSize::new(118.0, 20.0)), 11.0, true, Some(&c_cyan), NSTextAlignment::Center);
    btn_reset_lbl.setToolTip(Some(&NSString::from_str("Codex 全网限额重置追踪 (codex-resets.com)\n点击在浏览器中打开 codex-resets.com")));
    container.addSubview(&btn_reset_lbl);
    let btn_rst_click = unsafe { NSButton::buttonWithTitle_target_action(ns_string!(""), Some(handler_obj), Some(sel!(openCodexResetsClicked:)), mtm) };
    btn_rst_click.setFrame(NSRect::new(NSPoint::new(248.0, btn_y), NSSize::new(118.0, 28.0)));
    btn_rst_click.setTransparent(true);
    container.addSubview(&btn_rst_click);

    let b_alpha = create_box(mtm, NSRect::new(NSPoint::new(372.0, btn_y), NSSize::new(120.0, 28.0)), &c_subtle);
    container.addSubview(&b_alpha);
    container.addSubview(&create_label(mtm, "[alpha] 网页端 ↗", NSRect::new(NSPoint::new(372.0, btn_y + 4.0), NSSize::new(120.0, 20.0)), 11.5, false, Some(&c_white), NSTextAlignment::Center));
    let btn_b_click = unsafe { NSButton::buttonWithTitle_target_action(ns_string!(""), Some(handler_obj), Some(sel!(openInBrowserClicked:)), mtm) };
    btn_b_click.setFrame(NSRect::new(NSPoint::new(372.0, btn_y), NSSize::new(120.0, 28.0)));
    btn_b_click.setTransparent(true);
    container.addSubview(&btn_b_click);

    let b_acc = create_box(mtm, NSRect::new(NSPoint::new(498.0, btn_y), NSSize::new(72.0, 28.0)), &c_subtle);
    container.addSubview(&b_acc);
    container.addSubview(&create_label(mtm, "⚙️ 账户 ↗", NSRect::new(NSPoint::new(498.0, btn_y + 4.0), NSSize::new(72.0, 20.0)), 11.5, false, Some(&c_white), NSTextAlignment::Center));
    let btn_a_click = unsafe { NSButton::buttonWithTitle_target_action(ns_string!(""), Some(handler_obj), Some(sel!(openAccountClicked:)), mtm) };
    btn_a_click.setFrame(NSRect::new(NSPoint::new(498.0, btn_y), NSSize::new(72.0, 28.0)));
    btn_a_click.setTransparent(true);
    container.addSubview(&btn_a_click);

    let b_ref = create_box(mtm, NSRect::new(NSPoint::new(576.0, btn_y), NSSize::new(78.0, 28.0)), &c_subtle);
    container.addSubview(&b_ref);
    let btn_refresh_lbl = create_label(mtm, "🔄 刷新", NSRect::new(NSPoint::new(576.0, btn_y + 4.0), NSSize::new(78.0, 20.0)), 11.5, true, Some(&c_cyan), NSTextAlignment::Center);
    container.addSubview(&btn_refresh_lbl);
    let btn_r_click = unsafe { NSButton::buttonWithTitle_target_action(ns_string!(""), Some(handler_obj), Some(sel!(refreshClicked:)), mtm) };
    btn_r_click.setFrame(NSRect::new(NSPoint::new(576.0, btn_y), NSSize::new(78.0, 28.0)));
    btn_r_click.setTransparent(true);
    container.addSubview(&btn_r_click);

    let b_plan = create_box(mtm, NSRect::new(NSPoint::new(660.0, btn_y), NSSize::new(76.0, 28.0)), &c_subtle);
    container.addSubview(&b_plan);
    let plan_lbl = create_label(mtm, "⚡ ...", NSRect::new(NSPoint::new(660.0, btn_y + 4.0), NSSize::new(76.0, 20.0)), 11.5, true, Some(&c_amber), NSTextAlignment::Center);
    container.addSubview(&plan_lbl);

    // 3. Top 5 Major Metric Cards Row
    let card_subs_text = ["累计 Token", "峰值 Token", "最长聊天时长", "当前连续天数", "最长连续天数"];
    let card_colors = [&c_white, &c_white, &c_amber, &c_white, &c_cyan];
    let sk_init_color = NSColor::colorWithSRGBRed_green_blue_alpha(0.18, 0.20, 0.24, 1.0);
    let mut card_vals = Vec::new();
    let mut card_subs = Vec::new();
    let mut card_skeletons = Vec::new();

    for i in 0..5 {
        let cx = 24.0 + i as f64 * 144.0;
        container.addSubview(&create_box(mtm, NSRect::new(NSPoint::new(cx, 502.0), NSSize::new(136.0, 56.0)), &c_card));
        let sk_val = create_box(mtm, NSRect::new(NSPoint::new(cx + 28.0, 532.0), NSSize::new(80.0, 16.0)), &sk_init_color);
        sk_val.setHidden(true);
        container.addSubview(&sk_val);
        card_skeletons.push(sk_val);

        let val_l = create_label(mtm, "--", NSRect::new(NSPoint::new(cx + 4.0, 527.0), NSSize::new(128.0, 24.0)), 17.5, true, Some(card_colors[i]), NSTextAlignment::Center);
        container.addSubview(&val_l);
        card_vals.push(val_l);
        let sub_l = create_label(mtm, card_subs_text[i], NSRect::new(NSPoint::new(cx + 4.0, 510.0), NSSize::new(128.0, 16.0)), 10.5, false, Some(&c_muted), NSTextAlignment::Center);
        container.addSubview(&sub_l);
        card_subs.push(sub_l);
    }

    // 4. Rate cards row
    let mut rate_skeletons = Vec::new();
    let rate_xs = [24.0, 264.0, 504.0];
    for &rx in &rate_xs {
        container.addSubview(&create_box(mtm, NSRect::new(NSPoint::new(rx, 450.0), NSSize::new(232.0, 42.0)), &c_subtle));
        let sk = create_box(mtm, NSRect::new(NSPoint::new(rx + 26.0, 462.0), NSSize::new(180.0, 18.0)), &sk_init_color);
        sk.setHidden(true);
        container.addSubview(&sk);
        rate_skeletons.push(sk);
    }
    let rate_hourly_val = create_label(mtm, "⏱ 每小时速率: --/h", NSRect::new(NSPoint::new(34.0, 461.0), NSSize::new(212.0, 20.0)), 12.0, true, Some(&c_cyan), NSTextAlignment::Center);
    container.addSubview(&rate_hourly_val);

    let rate_weekly_val = create_label(mtm, "📅 本周用量: -- Token", NSRect::new(NSPoint::new(274.0, 461.0), NSSize::new(212.0, 20.0)), 12.0, true, Some(&c_white), NSTextAlignment::Center);
    container.addSubview(&rate_weekly_val);

    let rate_quota_val = create_label(mtm, "⚡ 周配额: --", NSRect::new(NSPoint::new(514.0, 461.0), NSSize::new(212.0, 20.0)), 12.0, true, Some(&c_green), NSTextAlignment::Center);
    container.addSubview(&rate_quota_val);

    // 5. Main Tab Switcher Bar
    container.addSubview(&create_box(mtm, NSRect::new(NSPoint::new(24.0, 406.0), NSSize::new(712.0, 36.0)), &c_subtle));
    let tab_titles = ["🔥 活动热力图", "📊 每日用量", "🤖 模型排行", "🌐 特权券与广播"];
    let tab_sels = [sel!(mainTabHeatmapClicked:), sel!(mainTabBarsClicked:), sel!(mainTabModelsClicked:), sel!(mainTabBroadcastClicked:)];
    let mut tab_bgs = Vec::new();
    let mut tab_lbls = Vec::new();

    for i in 0..4 {
        let tx = 28.0 + i as f64 * 179.0;
        let bg = create_box(mtm, NSRect::new(NSPoint::new(tx, 410.0), NSSize::new(173.0, 28.0)), if i == 0 { &c_act } else { &c_trans });
        container.addSubview(&bg);
        tab_bgs.push(bg);
        let lbl = create_label(mtm, tab_titles[i], NSRect::new(NSPoint::new(tx, 415.0), NSSize::new(173.0, 18.0)), 12.0, i == 0, Some(if i == 0 { &c_white } else { &c_muted }), NSTextAlignment::Center);
        container.addSubview(&lbl);
        tab_lbls.push(lbl);
        let btn = unsafe { NSButton::buttonWithTitle_target_action(ns_string!(""), Some(handler_obj), Some(tab_sels[i]), mtm) };
        btn.setFrame(NSRect::new(NSPoint::new(tx, 410.0), NSSize::new(173.0, 28.0)));
        btn.setTransparent(true);
        container.addSubview(&btn);
    }

    HeaderComponents {
        container,
        avatar_text,
        label_name,
        label_handle,
        plan_lbl,
        btn_reset_lbl,
        btn_refresh_lbl,
        card_vals: card_vals.try_into().unwrap(),
        card_subs: card_subs.try_into().unwrap(),
        card_skeletons: card_skeletons.try_into().unwrap(),
        rate_hourly_val,
        rate_weekly_val,
        rate_quota_val,
        rate_skeletons: rate_skeletons.try_into().unwrap(),
        main_tab_hm_bg: tab_bgs.remove(0),
        main_tab_bars_bg: tab_bgs.remove(0),
        main_tab_models_bg: tab_bgs.remove(0),
        main_tab_broad_bg: tab_bgs.remove(0),
        main_tab_hm_lbl: tab_lbls.remove(0),
        main_tab_bars_lbl: tab_lbls.remove(0),
        main_tab_models_lbl: tab_lbls.remove(0),
        main_tab_broad_lbl: tab_lbls.remove(0),
    }
}
