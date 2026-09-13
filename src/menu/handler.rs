use std::process::Command;

use objc2::rc::Retained;
use objc2::{define_class, msg_send, MainThreadMarker, MainThreadOnly};
use objc2_app_kit::NSApplication;
use objc2_foundation::{NSObject, NSObjectProtocol, NSTimer};

use crate::config;
use crate::window;
use super::trigger_refresh;
use super::GLOBAL_MENU_STATE;

define_class!(
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    pub struct MenuActionHandler;

    unsafe impl NSObjectProtocol for MenuActionHandler {}

    impl MenuActionHandler {
        #[unsafe(method(openDashboardClicked:))]
        fn open_dashboard_clicked(&self, _sender: &NSObject) {
            GLOBAL_MENU_STATE.with(|cell| {
                if let Some(state) = cell.borrow().as_ref() {
                    let usage = state.last_usage.borrow().clone();
                    window::open_or_update_dashboard(state.mtm, &usage);
                }
            });
        }

        #[unsafe(method(openInstallGuideClicked:))]
        fn open_install_guide_clicked(&self, _sender: &NSObject) {
            GLOBAL_MENU_STATE.with(|cell| {
                if let Some(state) = cell.borrow().as_ref() {
                    let dash = window::get_or_create_dashboard(state.mtm);
                    dash.show_install_guide();
                    dash.show(state.mtm);
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
                if let Ok(_) = Command::new(&bin).arg("app").spawn() {
                    return;
                }
            }
            let candidates = ["ChatGPT", "Codex", "OpenAI Codex"];
            for app in candidates {
                if Command::new("open").arg("-a").arg(app).status().map(|s| s.success()).unwrap_or(false) {
                    return;
                }
            }
        }

        #[unsafe(method(openConfigClicked:))]
        fn open_config_clicked(&self, _sender: &NSObject) {
            if let Some(dir) = config::get_codex_home_dir() {
                let _ = Command::new("open").arg(dir).spawn();
            }
        }

        #[unsafe(method(openResetTrackerClicked:))]
        fn open_reset_tracker_clicked(&self, _sender: &NSObject) {
            let _ = Command::new("open").arg("https://chatgpt.com/codex").spawn();
        }

        #[unsafe(method(openInBrowserClicked:))]
        fn open_in_browser_clicked(&self, _sender: &NSObject) {
            let _ = Command::new("open").arg("https://chatgpt.com/codex").spawn();
        }

        #[unsafe(method(mainTabHeatmapClicked:))]
        fn main_tab_heatmap_clicked(&self, _sender: &NSObject) {
            crate::window::select_main_tab(crate::window::MainTab::Heatmap);
        }

        #[unsafe(method(mainTabBarsClicked:))]
        fn main_tab_bars_clicked(&self, _sender: &NSObject) {
            crate::window::select_main_tab(crate::window::MainTab::DailyBars);
        }

        #[unsafe(method(mainTabModelsClicked:))]
        fn main_tab_models_clicked(&self, _sender: &NSObject) {
            crate::window::select_main_tab(crate::window::MainTab::Models);
        }

        #[unsafe(method(mainTabBroadcastClicked:))]
        fn main_tab_broadcast_clicked(&self, _sender: &NSObject) {
            crate::window::select_main_tab(crate::window::MainTab::Broadcast);
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
