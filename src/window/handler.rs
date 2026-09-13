use std::io::Write;
use std::process::{Command, Stdio};

use objc2::rc::Retained;
use objc2::{define_class, msg_send, MainThreadMarker, MainThreadOnly};
use objc2_foundation::{NSObject, NSObjectProtocol, NSString, NSTimer};

use super::types::{HeatmapMode, MainTab};
use super::GLOBAL_DASHBOARD;

define_class!(
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    pub struct DashboardActionHandler;

    unsafe impl NSObjectProtocol for DashboardActionHandler {}

    impl DashboardActionHandler {
        // Heatmap 4 modes
        #[unsafe(method(dailyModeClicked:))]
        fn daily_mode_clicked(&self, _sender: &NSObject) {
            GLOBAL_DASHBOARD.with(|c| if let Some(d) = c.borrow().as_ref() { d.set_heatmap_mode(HeatmapMode::Daily); });
        }

        #[unsafe(method(weeklyModeClicked:))]
        fn weekly_mode_clicked(&self, _sender: &NSObject) {
            GLOBAL_DASHBOARD.with(|c| if let Some(d) = c.borrow().as_ref() { d.set_heatmap_mode(HeatmapMode::Weekly); });
        }

        #[unsafe(method(cumulativeModeClicked:))]
        fn cumulative_mode_clicked(&self, _sender: &NSObject) {
            GLOBAL_DASHBOARD.with(|c| if let Some(d) = c.borrow().as_ref() { d.set_heatmap_mode(HeatmapMode::Cumulative); });
        }

        #[unsafe(method(allModeClicked:))]
        fn all_mode_clicked(&self, _sender: &NSObject) {
            GLOBAL_DASHBOARD.with(|c| if let Some(d) = c.borrow().as_ref() { d.set_heatmap_mode(HeatmapMode::All); });
        }

        // Main Tabs
        #[unsafe(method(mainTabHeatmapClicked:))]
        fn main_tab_heatmap_clicked(&self, _sender: &NSObject) {
            GLOBAL_DASHBOARD.with(|c| if let Some(d) = c.borrow().as_ref() { d.set_main_tab(MainTab::Heatmap); });
        }

        #[unsafe(method(mainTabBarsClicked:))]
        fn main_tab_bars_clicked(&self, _sender: &NSObject) {
            GLOBAL_DASHBOARD.with(|c| if let Some(d) = c.borrow().as_ref() { d.set_main_tab(MainTab::DailyBars); });
        }

        #[unsafe(method(mainTabModelsClicked:))]
        fn main_tab_models_clicked(&self, _sender: &NSObject) {
            GLOBAL_DASHBOARD.with(|c| if let Some(d) = c.borrow().as_ref() { d.set_main_tab(MainTab::Models); });
        }

        #[unsafe(method(mainTabBroadcastClicked:))]
        fn main_tab_broadcast_clicked(&self, _sender: &NSObject) {
            GLOBAL_DASHBOARD.with(|c| if let Some(d) = c.borrow().as_ref() { d.set_main_tab(MainTab::Broadcast); });
        }

        // Header Actions
        #[unsafe(method(refreshClicked:))]
        fn refresh_clicked(&self, _sender: &NSObject) {
            if crate::menu::is_refresh_cooling_down() {
                return;
            }
            GLOBAL_DASHBOARD.with(|c| if let Some(d) = c.borrow().as_ref() { d.set_refreshing(true); });
            crate::menu::trigger_refresh();
        }

        #[unsafe(method(openInBrowserClicked:))]
        fn open_in_browser_clicked(&self, _sender: &NSObject) {
            let _ = Command::new("open").arg("https://chatgpt.com/codex").spawn();
        }

        #[unsafe(method(openCodexResetsClicked:))]
        fn open_codex_resets_clicked(&self, _sender: &NSObject) {
            let _ = Command::new("open").arg("https://codex-resets.com/").spawn();
        }

        #[unsafe(method(openAccountClicked:))]
        fn open_account_clicked(&self, _sender: &NSObject) {
            let _ = Command::new("open").arg("https://chatgpt.com/#settings/account").spawn();
        }

        #[unsafe(method(skeletonPulseFired:))]
        fn skeleton_pulse_fired(&self, _timer: &NSTimer) {
            GLOBAL_DASHBOARD.with(|c| if let Some(d) = c.borrow().as_ref() { d.step_skeleton_animation(); });
        }

        // Install Guide Actions
        #[unsafe(method(openCodexDownloadClicked:))]
        fn open_codex_download_clicked(&self, _sender: &NSObject) {
            let _ = Command::new("open").arg("https://chatgpt.com/codex").spawn();
        }

        #[unsafe(method(copyCliInstallClicked:))]
        fn copy_cli_install_clicked(&self, _sender: &NSObject) {
            let cmd = "npm install -g @openai/codex";
            if let Ok(mut child) = Command::new("pbcopy").stdin(Stdio::piped()).spawn() {
                if let Some(mut stdin) = child.stdin.take() {
                    let _ = stdin.write_all(cmd.as_bytes());
                }
            }
            GLOBAL_DASHBOARD.with(|c| {
                if let Some(d) = c.borrow().as_ref() {
                    d.install_panel.btn_copy_label.setStringValue(&NSString::from_str("✓ 已复制到剪贴板！"));
                }
            });
        }

        #[unsafe(method(recheckInstallClicked:))]
        fn recheck_install_clicked(&self, _sender: &NSObject) {
            GLOBAL_DASHBOARD.with(|c| {
                if let Some(d) = c.borrow().as_ref() {
                    d.refresh_install_status();
                }
            });
            crate::menu::trigger_refresh();
        }

        #[unsafe(method(switchToDashboardClicked:))]
        fn switch_to_dashboard_clicked(&self, _sender: &NSObject) {
            GLOBAL_DASHBOARD.with(|c| {
                if let Some(d) = c.borrow().as_ref() {
                    d.show_dashboard_view();
                }
            });
        }
    }
);

impl DashboardActionHandler {
    pub fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(());
        unsafe { msg_send![super(this), init] }
    }
}
