use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use objc2::rc::Retained;
use objc2::runtime::Sel;
use objc2::{define_class, msg_send, sel, MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{NSMenu, NSMenuItem, NSStatusBar, NSStatusItem};
use objc2_foundation::{ns_string, NSObject, NSObjectProtocol, NSString, NSTimer};

use crate::codex::{self, CodexUsage};
use crate::config;

// Constants for AppKit status item lengths
const NS_VARIABLE_STATUS_ITEM_LENGTH: f64 = -1.0;

// Shared state for the menu bar UI
pub struct MenuState {
    pub status_item: Retained<NSStatusItem>,
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
        #[unsafe(method(refreshClicked:))]
        fn refresh_clicked(&self, _sender: &NSObject) {
            trigger_refresh();
        }

        #[unsafe(method(openConfigClicked:))]
        fn open_config_clicked(&self, _sender: &NSObject) {
            if let Some(dir) = config::get_codex_home_dir() {
                let _ = std::process::Command::new("open").arg(dir).spawn();
            }
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

/// Helper to safely create an NSMenuItem
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
            // Prevent re-entrant concurrent refreshes
            if state.is_refreshing.swap(true, Ordering::SeqCst) {
                return;
            }

            // Set button to refreshing state
            if let Some(button) = state.status_item.button(state.mtm) {
                button.setTitle(&NSString::from_str("☁ Codex ⟳"));
            }

            let is_refreshing = state.is_refreshing.clone();

            // Run data fetch in background thread so UI never freezes
            std::thread::spawn(move || {
                let usage = codex::fetch_codex_usage();
                is_refreshing.store(false, Ordering::SeqCst);

                // Dispatch UI update back to main thread using dispatch_async
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

            // 1. Update status bar button title
            if let Some(button) = state.status_item.button(mtm) {
                let title_str = usage.menu_bar_title();
                button.setTitle(&NSString::from_str(&title_str));
            }

            // 2. Rebuild the dropdown NSMenu
            let menu = NSMenu::initWithTitle(NSMenu::alloc(mtm), ns_string!("Codex Menu"));
            let handler = MenuActionHandler::new(mtm);

            // Title & Plan item
            let plan = usage.plan_type.as_deref().unwrap_or("Pro");
            let header_text = match usage.primary_percent {
                Some(pct) => format!("☁ Codex: {}% ({})", pct, plan),
                None => format!("☁ Codex ({})", plan),
            };
            menu.addItem(&make_menu_item(mtm, &header_text, None, None, "", false));

            // Separator
            menu.addItem(&NSMenuItem::separatorItem(mtm));

            // Primary window
            let primary_label = format!("{:<14} {}%", usage.primary_window_label(), usage.primary_percent.map(|p| p.to_string()).unwrap_or_else(|| "--".to_string()));
            menu.addItem(&make_menu_item(mtm, &primary_label, None, None, "", false));

            // Secondary window (e.g. Weekly)
            let secondary_label = format!("{:<14} {}%", usage.secondary_window_label(), usage.secondary_percent.map(|p| p.to_string()).unwrap_or_else(|| "--".to_string()));
            menu.addItem(&make_menu_item(mtm, &secondary_label, None, None, "", false));

            // Reset countdown
            let reset_label = format!("{:<14} {}", "Reset in", usage.reset_countdown_label());
            menu.addItem(&make_menu_item(mtm, &reset_label, None, None, "", false));

            // Optional: Credits balance if available
            if let Some(balance) = &usage.credits_balance {
                let credit_label = format!("{:<14} {}", "Credits", balance);
                menu.addItem(&make_menu_item(mtm, &credit_label, None, None, "", false));
            }

            // Separator
            menu.addItem(&NSMenuItem::separatorItem(mtm));

            // Updated time info
            let updated_label = format!("更新时间: {}", usage.updated_at);
            menu.addItem(&make_menu_item(mtm, &updated_label, None, None, "", false));

            // Separator
            menu.addItem(&NSMenuItem::separatorItem(mtm));

            // Refresh action
            let handler_obj: &NSObject = &handler;
            menu.addItem(&make_menu_item(
                mtm,
                "🔄 立即刷新",
                Some(sel!(refreshClicked:)),
                Some(handler_obj),
                "r",
                true,
            ));

            // Open Codex directory
            menu.addItem(&make_menu_item(
                mtm,
                "📁 打开配置目录",
                Some(sel!(openConfigClicked:)),
                Some(handler_obj),
                "",
                true,
            ));

            // Separator
            menu.addItem(&NSMenuItem::separatorItem(mtm));

            // Quit action (target None will send terminate: to NSApp)
            menu.addItem(&make_menu_item(
                mtm,
                "❌ 退出 Codex Monitor",
                Some(sel!(terminate:)),
                None,
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

    let state = Rc::new(MenuState {
        status_item,
        mtm,
        is_refreshing: Arc::new(AtomicBool::new(false)),
    });

    GLOBAL_MENU_STATE.with(|cell| {
        *cell.borrow_mut() = Some(state);
    });

    // Schedule periodic timer on main runloop
    let handler = MenuActionHandler::new(mtm);
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
