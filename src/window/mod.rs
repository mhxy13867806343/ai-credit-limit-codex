pub mod handler;
pub mod header;
pub mod heatmap_calc;
pub mod panel_bars;
pub mod panel_broadcast;
pub mod panel_heatmap;
pub mod panel_install;
pub mod panel_models;
pub mod skeleton;
pub mod state;
pub mod state_update;
pub mod types;
pub mod ui_helpers;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;

use objc2::MainThreadMarker;

pub use state::DashboardWindow;
pub use types::{HeatmapMode, MainTab};
use crate::codex::CodexUsage;

thread_local! {
    pub static GLOBAL_DASHBOARD: RefCell<Option<Rc<DashboardWindow>>> = const { RefCell::new(None) };
}

static LAST_AUTO_REFRESH: Mutex<Option<std::time::Instant>> = Mutex::new(None);

pub fn trigger_open_auto_refresh() {
    let should_refresh = {
        let mut guard = LAST_AUTO_REFRESH.lock().unwrap();
        match *guard {
            Some(last) if last.elapsed().as_secs() < 25 => false,
            _ => {
                *guard = Some(std::time::Instant::now());
                true
            }
        }
    };
    if should_refresh {
        crate::menu::trigger_refresh();
    }
}

pub fn get_or_create_dashboard(mtm: MainThreadMarker) -> Rc<DashboardWindow> {
    GLOBAL_DASHBOARD.with(|cell| {
        let mut borrow = cell.borrow_mut();
        if let Some(w) = borrow.as_ref() {
            return Rc::clone(w);
        }
        let w = DashboardWindow::new(mtm);
        *borrow = Some(Rc::clone(&w));
        w
    })
}

pub fn open_or_update_dashboard(mtm: MainThreadMarker, usage: &CodexUsage) {
    let dash = get_or_create_dashboard(mtm);
    dash.update(usage);
    dash.show(mtm);
    trigger_open_auto_refresh();
}

pub fn update_dashboard_if_visible(usage: &CodexUsage) {
    GLOBAL_DASHBOARD.with(|cell| {
        if let Some(dash) = cell.borrow().as_ref() {
            dash.update(usage);
        }
    });
}

pub fn set_dashboard_refreshing(refreshing: bool) {
    GLOBAL_DASHBOARD.with(|cell| {
        if let Some(dash) = cell.borrow().as_ref() {
            dash.set_refreshing(refreshing);
        }
    });
}

pub fn update_refresh_cooldown(rem_secs: u64) {
    GLOBAL_DASHBOARD.with(|cell| {
        if let Some(dash) = cell.borrow().as_ref() {
            if rem_secs > 0 {
                dash.set_refresh_button_state(&format!("🔄 刷新 ({}s)", rem_secs), true);
            } else {
                dash.set_refresh_button_state("🔄 刷新", false);
            }
        }
    });
}

#[allow(dead_code)]
pub fn select_main_tab(tab: MainTab) {
    GLOBAL_DASHBOARD.with(|cell| {
        if let Some(dash) = cell.borrow().as_ref() {
            dash.set_main_tab(tab);
        }
    });
}

#[allow(dead_code)]
pub fn select_heatmap_mode(mode: HeatmapMode) {
    GLOBAL_DASHBOARD.with(|cell| {
        if let Some(dash) = cell.borrow().as_ref() {
            dash.set_main_tab(MainTab::Heatmap);
            dash.set_heatmap_mode(mode);
            dash.render_heatmap(mode);
        }
    });
}
