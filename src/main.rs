mod codex;
mod config;
mod menu;
mod window;

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{define_class, msg_send, MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate};
use objc2_foundation::{NSNotification, NSObject, NSObjectProtocol};

define_class!(
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    struct AppDelegate;

    unsafe impl NSObjectProtocol for AppDelegate {}

    unsafe impl NSApplicationDelegate for AppDelegate {
        #[unsafe(method(applicationDidFinishLaunching:))]
        fn did_finish_launching(&self, _notification: &NSNotification) {
            let mtm = self.mtm();
            let app = NSApplication::sharedApplication(mtm);

            // Set as Regular application to show system Main Menu Bar (文件, 查看, 窗口, 帮助)
            app.setActivationPolicy(NSApplicationActivationPolicy::Regular);

            let cfg = config::Config::default();
            menu::setup_menu_bar(mtm, &cfg);

            let usage = codex::fetch_codex_usage();
            window::open_or_update_dashboard(mtm, &usage);

            #[allow(deprecated)]
            app.activateIgnoringOtherApps(true);
        }

        #[unsafe(method(applicationShouldHandleReopen:hasVisibleWindows:))]
        fn application_should_handle_reopen(&self, _app: &NSApplication, has_visible_windows: bool) -> bool {
            if !has_visible_windows {
                let mtm = self.mtm();
                let usage = codex::fetch_codex_usage();
                window::open_or_update_dashboard(mtm, &usage);
            }
            true
        }
    }
);

impl AppDelegate {
    fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(());
        unsafe { msg_send![super(this), init] }
    }
}

fn main() {
    let mtm = MainThreadMarker::new().expect("Must initialize on the main thread");
    let app = NSApplication::sharedApplication(mtm);

    let delegate = AppDelegate::new(mtm);
    app.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));

    println!("[codex-monitor] Starting native macOS menu bar app...");
    app.run();
}
