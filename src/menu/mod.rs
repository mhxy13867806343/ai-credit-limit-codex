pub mod builder;
pub mod handler;
pub mod main_menu;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use objc2::rc::Retained;
use objc2::{sel, MainThreadMarker};
use objc2_app_kit::{NSStatusBar, NSStatusItem};
use objc2_foundation::{NSString, NSTimer};

use crate::codex::{self, CodexUsage};
use crate::config::Config;
use crate::window;
use builder::build_menu;
pub use handler::MenuActionHandler;

const NS_VARIABLE_STATUS_ITEM_LENGTH: f64 = -1.0;

pub struct MenuState {
    pub status_item: Retained<NSStatusItem>,
    pub handler: Retained<MenuActionHandler>,
    pub last_usage: RefCell<CodexUsage>,
    pub mtm: MainThreadMarker,
    pub is_refreshing: Arc<AtomicBool>,
}

thread_local! {
    pub static GLOBAL_MENU_STATE: RefCell<Option<Rc<MenuState>>> = const { RefCell::new(None) };
}

use std::sync::Mutex;
use std::time::Instant;

static LAST_REFRESH_TIME: Mutex<Option<Instant>> = Mutex::new(None);
static COOLDOWN_REMAINING: Mutex<u64> = Mutex::new(0);

pub fn is_refresh_cooling_down() -> bool {
    let guard = COOLDOWN_REMAINING.lock().unwrap();
    *guard > 0
}

pub fn trigger_refresh() {
    let now = Instant::now();
    {
        let mut guard = LAST_REFRESH_TIME.lock().unwrap();
        if let Some(last) = *guard {
            if now.duration_since(last).as_secs() < 15 {
                return;
            }
        }
        *guard = Some(now);
    }

    *COOLDOWN_REMAINING.lock().unwrap() = 15;
    main_menu::set_main_menu_refresh_state("立即刷新数据 (15s)", false);
    window::update_refresh_cooldown(15);

    std::thread::spawn(|| {
        for sec in (1..15).rev() {
            std::thread::sleep(std::time::Duration::from_secs(1));
            *COOLDOWN_REMAINING.lock().unwrap() = sec;
            dispatch_to_main_thread(move || {
                main_menu::set_main_menu_refresh_state(&format!("立即刷新数据 ({}s)", sec), false);
                window::update_refresh_cooldown(sec);
            });
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
        *COOLDOWN_REMAINING.lock().unwrap() = 0;
        dispatch_to_main_thread(|| {
            main_menu::set_main_menu_refresh_state("立即刷新数据", true);
            window::update_refresh_cooldown(0);
        });
    });

    GLOBAL_MENU_STATE.with(|cell| {
        if let Some(state) = cell.borrow().as_ref() {
            if state.is_refreshing.swap(true, Ordering::SeqCst) {
                return;
            }
            if let Some(button) = state.status_item.button(state.mtm) {
                button.setTitle(&NSString::from_str("☁ Codex ⟳"));
            }
            window::set_dashboard_refreshing(true);

            let is_refreshing = state.is_refreshing.clone();
            std::thread::spawn(move || {
                let usage = codex::fetch_codex_usage();
                std::thread::sleep(std::time::Duration::from_millis(650));
                is_refreshing.store(false, Ordering::SeqCst);
                dispatch_to_main_thread(move || {
                    update_menu_with_usage(&usage);
                });
            });
        }
    });
}

pub fn update_menu_with_usage(usage: &CodexUsage) {
    GLOBAL_MENU_STATE.with(|cell| {
        if let Some(state) = cell.borrow().as_ref() {
            let mtm = state.mtm;
            state.last_usage.replace(usage.clone());

            if let Some(button) = state.status_item.button(mtm) {
                let title_str = usage.menu_bar_title();
                button.setTitle(&NSString::from_str(&title_str));
            }

            let menu = build_menu(mtm, usage, &state.handler);
            state.status_item.setMenu(Some(&menu));
            window::update_dashboard_if_visible(usage);
        }
    });
}

pub fn setup_menu_bar(mtm: MainThreadMarker, cfg: &Config) {
    let status_bar = NSStatusBar::systemStatusBar();
    let status_item = status_bar.statusItemWithLength(NS_VARIABLE_STATUS_ITEM_LENGTH);
    let handler = MenuActionHandler::new(mtm);
    let initial_usage = CodexUsage::default();

    if let Some(button) = status_item.button(mtm) {
        button.setTitle(&NSString::from_str(&initial_usage.menu_bar_title()));
    }

    let menu = build_menu(mtm, &initial_usage, &handler);
    status_item.setMenu(Some(&menu));

    let app = objc2_app_kit::NSApplication::sharedApplication(mtm);
    let sys_main_menu = main_menu::build_system_main_menu(mtm, &handler);
    app.setMainMenu(Some(&sys_main_menu));

    let interval = cfg.refresh_interval_secs as f64;
    unsafe {
        let _ = NSTimer::scheduledTimerWithTimeInterval_target_selector_userInfo_repeats(
            interval,
            &handler,
            sel!(timerFired:),
            None,
            true,
        );
    }

    let state = Rc::new(MenuState {
        status_item,
        handler,
        last_usage: RefCell::new(initial_usage),
        mtm,
        is_refreshing: Arc::new(AtomicBool::new(false)),
    });

    GLOBAL_MENU_STATE.with(|cell| {
        *cell.borrow_mut() = Some(state);
    });

    trigger_refresh();
}

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

pub fn dispatch_to_main_thread<F: FnOnce() + Send + 'static>(f: F) {
    let boxed: Box<Box<dyn FnOnce()>> = Box::new(Box::new(f));
    let raw = Box::into_raw(boxed) as *mut std::ffi::c_void;
    unsafe {
        let queue = std::ptr::addr_of_mut!(_dispatch_main_q);
        dispatch_async_f(queue, raw, run_boxed_closure);
    }
}
