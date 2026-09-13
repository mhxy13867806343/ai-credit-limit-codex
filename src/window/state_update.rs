use objc2::sel;
use objc2_app_kit::NSColor;
use objc2_foundation::{NSPoint, NSString, NSTimer};

use super::heatmap_calc;
use super::panel_bars::update_daily_bars;
use super::panel_broadcast::update_broadcast_panel;
use super::panel_models::update_top_models;
use super::skeleton::animate_skeleton_step;
use super::state::DashboardWindow;
use super::types::{HeatmapMode, MainTab};
use super::ui_helpers::cell_date_dynamic;
use crate::codex::{self, CodexUsage};
use crate::config;

impl DashboardWindow {
    pub fn set_main_tab(&self, tab: MainTab) {
        *self.current_main_tab.borrow_mut() = tab;
        let (c_act, c_trans) = (NSColor::colorWithSRGBRed_green_blue_alpha(0.14, 0.38, 0.92, 1.0), NSColor::clearColor());
        let (c_w, c_m) = (NSColor::whiteColor(), NSColor::colorWithSRGBRed_green_blue_alpha(0.55, 0.58, 0.64, 1.0));
        let tabs = [
            (tab == MainTab::Heatmap, &self.header.main_tab_hm_bg, &self.header.main_tab_hm_lbl),
            (tab == MainTab::DailyBars, &self.header.main_tab_bars_bg, &self.header.main_tab_bars_lbl),
            (tab == MainTab::Models, &self.header.main_tab_models_bg, &self.header.main_tab_models_lbl),
            (tab == MainTab::Broadcast, &self.header.main_tab_broad_bg, &self.header.main_tab_broad_lbl),
        ];
        for (act, bg, lbl) in tabs {
            bg.setBackgroundColor(Some(if act { &c_act } else { &c_trans }));
            lbl.setTextColor(Some(if act { &c_w } else { &c_m }));
        }

        if !*self.is_showing_install_guide.borrow() {
            self.hm.panel.setHidden(tab != MainTab::Heatmap);
            self.panel_daily_bars.setHidden(tab != MainTab::DailyBars);
            self.panel_models.setHidden(tab != MainTab::Models);
            self.broadcast.panel.setHidden(tab != MainTab::Broadcast);
        }
    }

    pub fn set_heatmap_mode(&self, mode: HeatmapMode) {
        *self.heatmap_mode.borrow_mut() = mode;
        let colors = [
            NSColor::colorWithSRGBRed_green_blue_alpha(0.14, 0.38, 0.92, 1.0),
            NSColor::colorWithSRGBRed_green_blue_alpha(0.85, 0.15, 0.18, 1.0),
            NSColor::colorWithSRGBRed_green_blue_alpha(0.05, 0.60, 0.40, 1.0),
            NSColor::colorWithSRGBRed_green_blue_alpha(0.55, 0.25, 0.90, 1.0),
        ];
        let (c_trans, c_w, c_m) = (NSColor::clearColor(), NSColor::whiteColor(), NSColor::colorWithSRGBRed_green_blue_alpha(0.55, 0.58, 0.64, 1.0));
        let modes = [
            (mode == HeatmapMode::Daily, &colors[0], &self.hm.tab_daily_bg, &self.hm.tab_daily_lbl),
            (mode == HeatmapMode::Weekly, &colors[1], &self.hm.tab_weekly_bg, &self.hm.tab_weekly_lbl),
            (mode == HeatmapMode::Cumulative, &colors[2], &self.hm.tab_cumul_bg, &self.hm.tab_cumul_lbl),
            (mode == HeatmapMode::All, &colors[3], &self.hm.tab_all_bg, &self.hm.tab_all_lbl),
        ];
        for (act, c, bg, lbl) in modes {
            bg.setBackgroundColor(Some(if act { c } else { &c_trans }));
            lbl.setTextColor(Some(if act { &c_w } else { &c_m }));
        }

        let desc = match mode {
            HeatmapMode::Daily => "🔵 每日模式 (蓝色用量热力图)",
            HeatmapMode::Weekly => "🔴 每周模式 (红色周配额热力图)",
            HeatmapMode::Cumulative => "🟢 累计模式 (绿色用量增长热力图)",
            HeatmapMode::All => "🟣 全景模式 (紫色全周期热力图)",
        };
        self.hm.hm_mode_desc.setStringValue(&NSString::from_str(desc));
        self.render_heatmap(mode);
    }

    pub fn set_refresh_button_state(&self, label: &str, is_cooling: bool) {
        self.header.btn_refresh_lbl.setStringValue(&NSString::from_str(label));
        let c = if is_cooling {
            NSColor::colorWithSRGBRed_green_blue_alpha(0.55, 0.58, 0.64, 1.0)
        } else {
            NSColor::colorWithSRGBRed_green_blue_alpha(0.35, 0.75, 1.0, 1.0)
        };
        self.header.btn_refresh_lbl.setTextColor(Some(&c));
    }

    pub fn step_skeleton_animation(&self) {
        let step = *self.skeleton_anim_step.borrow();
        let next_step = animate_skeleton_step(&self.skeleton_bars, step);
        *self.skeleton_anim_step.borrow_mut() = next_step;
    }

    pub fn set_refreshing(&self, refreshing: bool) {
        let c_cyan = NSColor::colorWithSRGBRed_green_blue_alpha(0.35, 0.75, 1.0, 1.0);
        let c_amber = NSColor::colorWithSRGBRed_green_blue_alpha(0.95, 0.75, 0.25, 1.0);

        for i in 0..5 {
            self.header.card_vals[i].setHidden(refreshing);
            self.header.card_skeletons[i].setHidden(!refreshing);
        }
        for v in [&self.header.rate_hourly_val, &self.header.rate_weekly_val, &self.header.rate_quota_val] {
            v.setHidden(refreshing);
        }
        for sk in &self.header.rate_skeletons { sk.setHidden(!refreshing); }

        if refreshing {
            self.header.btn_refresh_lbl.setStringValue(&NSString::from_str("⟳ 刷新中..."));
            self.header.btn_refresh_lbl.setTextColor(Some(&c_amber));
            if !*self.is_showing_install_guide.borrow() {
                self.panel_skeleton.setHidden(false);
                self.hm.panel.setHidden(true);
                self.panel_daily_bars.setHidden(true);
                self.panel_models.setHidden(true);
                self.broadcast.panel.setHidden(true);
            }
            if self.skeleton_timer.borrow().is_none() {
                unsafe {
                    let timer = NSTimer::scheduledTimerWithTimeInterval_target_selector_userInfo_repeats(
                        0.10, &self.handler, sel!(skeletonPulseFired:), None, true,
                    );
                    self.skeleton_timer.replace(Some(timer));
                }
            }
        } else {
            self.header.btn_refresh_lbl.setStringValue(&NSString::from_str("🔄 刷新"));
            self.header.btn_refresh_lbl.setTextColor(Some(&c_cyan));
            if let Some(timer) = self.skeleton_timer.borrow_mut().take() { timer.invalidate(); }
            self.panel_skeleton.setHidden(true);
            if !*self.is_showing_install_guide.borrow() {
                let tab = *self.current_main_tab.borrow();
                self.set_main_tab(tab);
            }
        }
    }

    pub fn update(&self, usage: &CodexUsage) {
        self.set_refreshing(false);
        if !config::is_codex_installed() {
            self.show_install_guide();
            return;
        }

        if !*self.is_showing_install_guide.borrow() {
            self.show_dashboard_view();
        }

        let plan = usage.display_plan_name();
        self.header.plan_lbl.setStringValue(&NSString::from_str(&format!("⚡ {}", plan)));
        if let Some(gr) = &usage.global_reset {
            if let Some(t) = &gr.latest_text {
                self.header.btn_reset_lbl.setStringValue(&NSString::from_str(&format!("🌐 重置: {} ↗", t)));
            }
        }
        if let Some(name) = &usage.user_display_name {
            self.header.label_name.setStringValue(&NSString::from_str(name));
            let initials: String = name.chars().take(2).collect::<String>().to_uppercase();
            self.header.avatar_text.setStringValue(&NSString::from_str(&initials));
        }
        let handle = usage.account_email.as_deref().unwrap_or("user");
        let handle_short = handle.split('@').next().unwrap_or(handle);
        self.header.label_handle.setStringValue(&NSString::from_str(&format!("@{} · {}", handle_short, plan)));

        let card_strs = [
            usage.format_lifetime_tokens_compact(), usage.format_peak_daily_tokens_compact(),
            usage.format_turn_duration(), format!("{} 天", usage.current_streak_days.unwrap_or(0)),
            format!("{} 天", usage.longest_streak_days.unwrap_or(0)),
        ];
        for (i, s) in card_strs.into_iter().enumerate() {
            self.header.card_vals[i].setStringValue(&NSString::from_str(&s));
        }

        let (used, rem) = (usage.primary_percent.unwrap_or(0), usage.remaining_percent().unwrap_or(100 - usage.primary_percent.unwrap_or(0)));
        self.header.rate_hourly_val.setStringValue(&NSString::from_str(&format!("⏱ 每小时速率: {}/h", usage.format_hourly_tokens())));
        self.header.rate_weekly_val.setStringValue(&NSString::from_str(&format!("📅 本周用量: {}", usage.format_week_tokens())));
        self.header.rate_quota_val.setStringValue(&NSString::from_str(&format!("⚡ 周配额: 剩余 {}% (已用 {}%)", rem, used)));

        self.cached_buckets.replace(usage.daily_buckets.clone());
        if let Some(lt) = usage.lifetime_tokens { self.cached_lifetime.replace(lt); }
        let (sy, sm, sd) = cell_date_dynamic(0, 0, 24);
        let range_label = format!("{}月至今", sm);
        let start_date_str = format!("{:04}-{:02}-{:02}", sy, sm, sd);
        let range_total: i64 = usage.daily_buckets.iter().filter(|b| b.date >= start_date_str).map(|b| b.tokens).sum();
        let life_total = *self.cached_lifetime.borrow();
        self.hm.hm_summary_badge.setStringValue(&NSString::from_str(&format!(
            "{}: {}  •  历史累计: {}", range_label, codex::format_tokens_compact(range_total), codex::format_tokens_compact(life_total)
        )));

        let cur_mode = *self.heatmap_mode.borrow();
        self.render_heatmap(cur_mode);

        self.hm.hm_scroll_view.contentView().scrollToPoint(NSPoint::new(260.0, 0.0));
        self.hm.hm_scroll_view.reflectScrolledClipView(&self.hm.hm_scroll_view.contentView());

        update_daily_bars(&self.daily_bars, &usage.daily_buckets);
        update_top_models(&self.model_bars, &usage.top_models);
        update_broadcast_panel(&self.broadcast, usage);
        self.label_time.setStringValue(&NSString::from_str(&format!("更新时间: {} • 关闭此窗口后，应用依然常驻在顶部菜单栏", usage.updated_at)));
    }

    pub fn render_heatmap(&self, mode: HeatmapMode) {
        heatmap_calc::render_heatmap_grid(mode, &self.cached_buckets.borrow(), &self.hm.heatmap_tiles, &self.hm.leg_tiles, &self.hm.hm_total_metric, *self.cached_lifetime.borrow());
    }
}
