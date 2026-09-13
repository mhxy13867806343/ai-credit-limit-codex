use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use objc2::rc::Retained;
use objc2::runtime::Sel;
use objc2::{define_class, msg_send, sel, MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{NSApplication, NSMenu, NSMenuItem, NSStatusBar, NSStatusItem};
use objc2_foundation::{ns_string, NSObject, NSObjectProtocol, NSString, NSTimer};

use crate::codex::{self, CodexUsage};
use crate::config;
use crate::window;

// Constants for AppKit status item lengths
const NS_VARIABLE_STATUS_ITEM_LENGTH: f64 = -1.0;

// Shared state for the menu bar UI
pub struct MenuState {
    pub status_item: Retained<NSStatusItem>,
    pub handler: Retained<MenuActionHandler>,
    pub last_usage: RefCell<CodexUsage>,
    pub mtm: MainThreadMarker,
    pub is_refreshing: Arc<AtomicBool>,
}

thread_local! {
    static GLOBAL_MENU_STATE: RefCell<Option<Rc<MenuState>>> = const { RefCell::new(None) };
}

define_class!(
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    pub struct MenuActionHandler;

    unsafe impl NSObjectProtocol for MenuActionHandler {}

    impl MenuActionHandler {
        #[unsafe(method(openDashboardClicked:))]
        fn open_dashboard_clicked(&self, _sender: &NSObject) {
            GLOBAL_MENU_STATE.with(|cell| {
                let borrowed = cell.borrow();
                if let Some(state) = borrowed.as_ref() {
                    let usage = state.last_usage.borrow().clone();
                    window::open_or_update_dashboard(state.mtm, &usage);
                }
            });
        }

        #[unsafe(method(refreshClicked:))]
        fn refresh_clicked(&self, _sender: &NSObject) {
            trigger_refresh();
        }

        #[unsafe(method(openCodexAppClicked:))]
        fn open_codex_app_clicked(&self, _sender: &NSObject) {
            if let Some(bin) = config::find_codex_bin() {
                if let Ok(_) = std::process::Command::new(&bin).arg("app").spawn() {
                    return;
                }
            }
            let candidates = ["ChatGPT", "Codex", "OpenAI Codex"];
            for app in candidates {
                if std::process::Command::new("open").arg("-a").arg(app).status().map(|s| s.success()).unwrap_or(false) {
                    return;
                }
            }
        }

        #[unsafe(method(openConfigClicked:))]
        fn open_config_clicked(&self, _sender: &NSObject) {
            if let Some(dir) = config::get_codex_home_dir() {
                let _ = std::process::Command::new("open").arg(dir).spawn();
            }
        }

        #[unsafe(method(openResetTrackerClicked:))]
        fn open_reset_tracker_clicked(&self, _sender: &NSObject) {
            let _ = std::process::Command::new("open")
                .arg("https://mhxy13867806343.github.io/fock-codex-resets/")
                .spawn();
        }

        #[unsafe(method(quitClicked:))]
        fn quit_clicked(&self, _sender: &NSObject) {
            let app = NSApplication::sharedApplication(self.mtm());
            app.terminate(None);
        }

        #[unsafe(method(timerFired:))]
        fn timer_fired(&self, _timer: &NSTimer) {
            trigger_refresh();
        }
    }
);

impl MenuActionHandler {
    pub fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(());
        unsafe { msg_send![super(this), init] }
    }
}

/// Generate a neat ASCII progress bar like [██████░░░░]
fn format_progress_bar(percent: i32, width: usize) -> String {
    let clamped = percent.clamp(0, 100) as usize;
    let filled = (clamped * width) / 100;
    let empty = width.saturating_sub(filled);
    let bar: String = "█".repeat(filled) + &"░".repeat(empty);
    format!("[{}]", bar)
}

/// Helper to create an NSMenuItem
fn make_menu_item(
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
        NSMenuItem::initWithTitle_action_keyEquivalent(
            NSMenuItem::alloc(mtm),
            &title_ns,
            action,
            &key_ns,
        )
    };
    if let Some(t) = target {
        unsafe {
            item.setTarget(Some(t));
        }
    }
    item.setEnabled(enabled);
    item
}

/// Trigger an asynchronous data fetch and update the UI
pub fn trigger_refresh() {
    GLOBAL_MENU_STATE.with(|cell| {
        let borrowed = cell.borrow();
        if let Some(state) = borrowed.as_ref() {
            if state.is_refreshing.swap(true, Ordering::SeqCst) {
                return;
            }

            if let Some(button) = state.status_item.button(state.mtm) {
                button.setTitle(&NSString::from_str("☁ Codex ⟳"));
            }

            let is_refreshing = state.is_refreshing.clone();

            std::thread::spawn(move || {
                let usage = codex::fetch_codex_usage();
                is_refreshing.store(false, Ordering::SeqCst);

                dispatch_to_main_thread(move || {
                    update_menu_with_usage(&usage);
                });
            });
        }
    });
}

/// Update menu items and title with new CodexUsage data
pub fn update_menu_with_usage(usage: &CodexUsage) {
    GLOBAL_MENU_STATE.with(|cell| {
        let borrowed = cell.borrow();
        if let Some(state) = borrowed.as_ref() {
            let mtm = state.mtm;
            let handler_obj: &NSObject = &state.handler;

            // Save last usage snapshot
            state.last_usage.replace(usage.clone());

            // 1. Update status bar title: "☁ 剩余 40%" (or "☁ 剩余 41%")
            if let Some(button) = state.status_item.button(mtm) {
                let title_str = usage.menu_bar_title();
                button.setTitle(&NSString::from_str(&title_str));
            }

            // 2. Rebuild the dropdown NSMenu
            let menu = NSMenu::initWithTitle(NSMenu::alloc(mtm), ns_string!("Codex Menu"));
            menu.setAutoenablesItems(false);

            // Item 1: Open Dashboard Window (Top Action)
            let plan = usage.display_plan_name();
            let rem_str = usage.remaining_percent().map(|r| format!(" • 剩余 {}%", r)).unwrap_or_default();
            let header_text = format!("🖥️ 打开监控看板  ({}{})", plan, rem_str);
            menu.addItem(&make_menu_item(
                mtm,
                &header_text,
                Some(sel!(openDashboardClicked:)),
                Some(handler_obj),
                "d",
                true,
            ));

            // Separator
            menu.addItem(&NSMenuItem::separatorItem(mtm));

            // Item 2: Primary window usage
            let used_pct = usage.primary_percent.unwrap_or(0);
            let rem_pct = usage.remaining_percent().unwrap_or(100 - used_pct);
            let p_bar = format_progress_bar(used_pct, 10);
            let p_label = format!("{:<10} {} 剩余 {}% (已用 {}%)", usage.primary_window_label(), p_bar, rem_pct, used_pct);
            menu.addItem(&make_menu_item(
                mtm,
                &p_label,
                Some(sel!(openDashboardClicked:)),
                Some(handler_obj),
                "",
                true,
            ));

            // Item 3: Secondary window (only if distinct and present)
            if let Some(s_pct) = usage.secondary_percent {
                let s_bar = format_progress_bar(s_pct, 10);
                let s_label = format!("{:<10} {} 已用 {}%", usage.secondary_window_mins.map(codex::format_window_name).unwrap_or_else(|| "备用窗口".to_string()), s_bar, s_pct);
                menu.addItem(&make_menu_item(
                    mtm,
                    &s_label,
                    Some(sel!(openDashboardClicked:)),
                    Some(handler_obj),
                    "",
                    true,
                ));
            }

            // Item 4: Reset countdown
            let reset_label = format!("⏳ 额度重置    {}", usage.reset_countdown_label());
            menu.addItem(&make_menu_item(
                mtm,
                &reset_label,
                Some(sel!(openDashboardClicked:)),
                Some(handler_obj),
                "",
                true,
            ));

            // Item 5: Reset Credits (使用限额重置券)
            if let Some(credit_text) = usage.reset_credit_label() {
                let credit_item_label = format!("🎁 使用限额重置    {}", credit_text);
                menu.addItem(&make_menu_item(
                    mtm,
                    &credit_item_label,
                    Some(sel!(openDashboardClicked:)),
                    Some(handler_obj),
                    "",
                    true,
                ));
            } else if let Some(balance) = &usage.credits_balance {
                let credit_label = format!("💳 账户余额        {}", balance);
                menu.addItem(&make_menu_item(
                    mtm,
                    &credit_label,
                    Some(sel!(openDashboardClicked:)),
                    Some(handler_obj),
                    "",
                    true,
                ));
            }

            // Separator
            menu.addItem(&NSMenuItem::separatorItem(mtm));

            // Status & Time
            let status_msg = if usage.is_rate_limited {
                "⚠️ 状态: 已达用量限制"
            } else {
                "⚡ 状态: 额度正常可用"
            };
            menu.addItem(&make_menu_item(mtm, status_msg, None, None, "", true));

            let updated_label = format!("🕒 更新时间: {}", usage.updated_at);
            menu.addItem(&make_menu_item(mtm, &updated_label, None, None, "", true));

            // Separator
            menu.addItem(&NSMenuItem::separatorItem(mtm));

            // Action: Launch / Focus Codex Desktop App
            menu.addItem(&make_menu_item(
                mtm,
                "🚀 打开 Codex 桌面端 (App)",
                Some(sel!(openCodexAppClicked:)),
                Some(handler_obj),
                "o",
                true,
            ));

            // Action: Open Codex Resets Tracker
            menu.addItem(&make_menu_item(
                mtm,
                "🌐 全网重置日历 (codex-resets)",
                Some(sel!(openResetTrackerClicked:)),
                Some(handler_obj),
                "",
                true,
            ));

            // Action: Manual Refresh
            menu.addItem(&make_menu_item(
                mtm,
                "🔄 立即刷新数据",
                Some(sel!(refreshClicked:)),
                Some(handler_obj),
                "r",
                true,
            ));

            // Action: Open config directory
            menu.addItem(&make_menu_item(
                mtm,
                "📁 打开配置目录 (~/.codex)",
                Some(sel!(openConfigClicked:)),
                Some(handler_obj),
                "",
                true,
            ));

            // Separator
            menu.addItem(&NSMenuItem::separatorItem(mtm));

            // Action: Quit
            menu.addItem(&make_menu_item(
                mtm,
                "❌ 退出 Codex Monitor",
                Some(sel!(quitClicked:)),
                Some(handler_obj),
                "q",
                true,
            ));

            state.status_item.setMenu(Some(&menu));
        }
    });
}

/// Initialize the native menu bar status item and start the timer
pub fn setup_menu_bar(mtm: MainThreadMarker, cfg: &config::Config) {
    let status_bar = NSStatusBar::systemStatusBar();
    let status_item = status_bar.statusItemWithLength(NS_VARIABLE_STATUS_ITEM_LENGTH);

    if let Some(button) = status_item.button(mtm) {
        button.setTitle(ns_string!("☁ Codex ..."));
    }

    let handler = MenuActionHandler::new(mtm);

    let state = Rc::new(MenuState {
        status_item,
        handler: handler.clone(),
        last_usage: RefCell::new(CodexUsage::default()),
        mtm,
        is_refreshing: Arc::new(AtomicBool::new(false)),
    });

    GLOBAL_MENU_STATE.with(|cell| {
        *cell.borrow_mut() = Some(state);
    });

    // Schedule periodic timer on main runloop
    let interval = cfg.refresh_interval_secs as f64;
    unsafe {
        NSTimer::scheduledTimerWithTimeInterval_target_selector_userInfo_repeats(
            interval,
            &handler,
            sel!(timerFired:),
            None,
            true,
        );
    }

    // Initial immediate data refresh
    trigger_refresh();
}

// ---------------------------------------------------------------------------
// macOS libdispatch wrapper to safely dispatch work to the Main Thread
// ---------------------------------------------------------------------------

extern "C" {
    static mut _dispatch_main_q: std::ffi::c_void;
    fn dispatch_async_f(
        queue: *mut std::ffi::c_void,
        context: *mut std::ffi::c_void,
        work: extern "C" fn(*mut std::ffi::c_void),
    );
}

extern "C" fn run_boxed_closure(context: *mut std::ffi::c_void) {
    let closure = unsafe { Box::from_raw(context as *mut Box<dyn FnOnce()>) };
    closure();
}

fn dispatch_to_main_thread<F: FnOnce() + Send + 'static>(f: F) {
    let boxed: Box<Box<dyn FnOnce()>> = Box::new(Box::new(f));
    let raw = Box::into_raw(boxed) as *mut std::ffi::c_void;
    unsafe {
        let queue = std::ptr::addr_of_mut!(_dispatch_main_q);
        dispatch_async_f(queue, raw, run_boxed_closure);
    }
}
