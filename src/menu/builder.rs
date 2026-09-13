use objc2::rc::Retained;
use objc2::runtime::Sel;
use objc2::{sel, MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{NSMenu, NSMenuItem};
use objc2_foundation::{ns_string, NSObject, NSString};

use crate::codex::{self, CodexUsage};

pub fn format_progress_bar(percent: i32, width: usize) -> String {
    let clamped = percent.clamp(0, 100) as usize;
    let filled = (clamped * width) / 100;
    let empty = width.saturating_sub(filled);
    let bar: String = "█".repeat(filled) + &"░".repeat(empty);
    format!("[{}]", bar)
}

pub fn make_menu_item(
    mtm: MainThreadMarker,
    title: &str,
    action: Option<Sel>,
    target: Option<&NSObject>,
    key_equivalent: &str,
    enabled: bool,
) -> Retained<NSMenuItem> {
    let title_ns = NSString::from_str(title);
    let key_ns = NSString::from_str(key_equivalent);
    let item = unsafe {
        NSMenuItem::initWithTitle_action_keyEquivalent(NSMenuItem::alloc(mtm), &title_ns, action, &key_ns)
    };
    if let Some(t) = target {
        unsafe { item.setTarget(Some(t)); }
    }
    item.setEnabled(enabled);
    item
}

pub fn build_menu(
    mtm: MainThreadMarker,
    usage: &CodexUsage,
    handler_obj: &NSObject,
) -> Retained<NSMenu> {
    let menu = NSMenu::initWithTitle(NSMenu::alloc(mtm), ns_string!("Codex Menu"));
    menu.setAutoenablesItems(false);

    // 1. Dashboard Window
    let plan = usage.display_plan_name();
    let rem_str = usage.remaining_percent().map(|r| format!(" • 剩余 {}%", r)).unwrap_or_default();
    let header_text = format!("🖥️ 打开监控看板  ({}{})", plan, rem_str);
    menu.addItem(&make_menu_item(mtm, &header_text, Some(sel!(openDashboardClicked:)), Some(handler_obj), "d", true));

    // Install Guide entry
    menu.addItem(&make_menu_item(mtm, "📦 Codex 下载与安装指引...", Some(sel!(openInstallGuideClicked:)), Some(handler_obj), "i", true));

    menu.addItem(&NSMenuItem::separatorItem(mtm));

    // 2. Primary window
    let used_pct = usage.primary_percent.unwrap_or(0);
    let rem_pct = usage.remaining_percent().unwrap_or(100 - used_pct);
    let p_bar = format_progress_bar(used_pct, 10);
    let p_label = format!("周配额     {} 剩余 {}% (已用 {}%)", p_bar, rem_pct, used_pct);
    menu.addItem(&make_menu_item(mtm, &p_label, Some(sel!(openDashboardClicked:)), Some(handler_obj), "", true));

    // 3. Secondary window
    if let Some(s_pct) = usage.secondary_percent {
        let s_bar = format_progress_bar(s_pct, 10);
        let s_name = usage.secondary_window_mins.map(codex::format_window_name).unwrap_or_else(|| "备用窗口".to_string());
        let s_label = format!("{:<8} {} 已用 {}%", s_name, s_bar, s_pct);
        menu.addItem(&make_menu_item(mtm, &s_label, Some(sel!(openDashboardClicked:)), Some(handler_obj), "", true));
    }

    // 4. Reset & Stats
    if let Some(resets_at) = usage.primary_resets_at {
        let reset_label = format!("⏳ 额度重置    {}", codex::format_relative_time(resets_at));
        menu.addItem(&make_menu_item(mtm, &reset_label, Some(sel!(openDashboardClicked:)), Some(handler_obj), "", true));
    }

    let hourly_str = usage.format_hourly_tokens();
    let pace_label = format!("⏱ 消耗速率    {}/h", hourly_str);
    menu.addItem(&make_menu_item(mtm, &pace_label, Some(sel!(openDashboardClicked:)), Some(handler_obj), "", true));

    let week_str = usage.format_week_tokens();
    let week_label = format!("📅 本周用量    {}", week_str);
    menu.addItem(&make_menu_item(mtm, &week_label, Some(sel!(openDashboardClicked:)), Some(handler_obj), "", true));

    if let Some(tok) = usage.today_tokens {
        let today_label = format!("📊 今日消耗    {}", codex::format_tokens_compact(tok));
        menu.addItem(&make_menu_item(mtm, &today_label, Some(sel!(openDashboardClicked:)), Some(handler_obj), "", true));
    }

    // 5. Top Model
    if let Some(top_m) = usage.top_models.first() {
        let m_tok = codex::format_tokens_compact(top_m.tokens_used);
        let model_label = format!("🤖 最多使用    {} • {} ({:.1}%)", top_m.model_name, m_tok, top_m.percentage);
        menu.addItem(&make_menu_item(mtm, &model_label, Some(sel!(openDashboardClicked:)), Some(handler_obj), "", true));
    }

    menu.addItem(&NSMenuItem::separatorItem(mtm));

    // Status & Time
    let status_msg = if usage.is_rate_limited { "⚠️ 状态: 已达用量限制" } else { "⚡ 状态: 额度正常可用" };
    menu.addItem(&make_menu_item(mtm, status_msg, None, None, "", true));

    let updated_label = format!("🕒 更新时间: {}", usage.updated_at);
    menu.addItem(&make_menu_item(mtm, &updated_label, None, None, "", true));

    menu.addItem(&NSMenuItem::separatorItem(mtm));

    // Actions
    menu.addItem(&make_menu_item(mtm, "🚀 打开 Codex 桌面端 (App)", Some(sel!(openCodexAppClicked:)), Some(handler_obj), "o", true));
    menu.addItem(&make_menu_item(mtm, "🔄 立即刷新数据", Some(sel!(refreshClicked:)), Some(handler_obj), "r", true));
    menu.addItem(&make_menu_item(mtm, "📁 打开配置目录 (~/.codex)", Some(sel!(openConfigClicked:)), Some(handler_obj), "", true));

    menu.addItem(&NSMenuItem::separatorItem(mtm));
    menu.addItem(&make_menu_item(mtm, "退出 Codex 监控器", Some(sel!(quitClicked:)), Some(handler_obj), "q", true));

    menu
}
