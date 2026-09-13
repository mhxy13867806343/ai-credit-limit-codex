use std::cell::RefCell;
use std::rc::Rc;

use objc2::rc::Retained;
use objc2::{MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{
    NSApplication, NSBackingStoreType, NSColor, NSTextAlignment, NSTextField,
    NSView, NSWindow, NSWindowStyleMask,
};
use objc2_foundation::{NSPoint, NSRect, NSSize, NSString, NSTimer};

use super::handler::DashboardActionHandler;
use super::header::{build_header, HeaderComponents};
use super::panel_bars::build_daily_bars_panel;
use super::panel_broadcast::{build_broadcast_panel, BroadcastComponents};
use super::panel_heatmap::{build_heatmap_panel, HeatmapComponents};
use super::panel_install::{build_install_panel, InstallPanelComponents};
use super::panel_models::build_models_panel;
use super::skeleton::build_skeleton_panel;
use super::types::{BarComponent, HeatmapMode, MainTab, ModelBarComponent};
use super::ui_helpers::create_label;
use crate::codex::DailyUsageItem;

pub struct DashboardWindow {
    pub window: Retained<NSWindow>,
    pub header: HeaderComponents,
    pub current_main_tab: RefCell<MainTab>,
    pub panel_skeleton: Retained<NSView>,
    pub skeleton_bars: Vec<Retained<NSTextField>>,
    pub skeleton_anim_step: RefCell<usize>,
    pub skeleton_timer: RefCell<Option<Retained<NSTimer>>>,
    pub heatmap_mode: RefCell<HeatmapMode>,
    pub cached_buckets: RefCell<Vec<DailyUsageItem>>,
    pub cached_lifetime: RefCell<i64>,
    pub hm: HeatmapComponents,
    pub daily_bars: Vec<BarComponent>,
    pub panel_daily_bars: Retained<NSView>,
    pub model_bars: Vec<ModelBarComponent>,
    pub panel_models: Retained<NSView>,
    pub broadcast: BroadcastComponents,
    pub label_time: Retained<NSTextField>,
    pub install_panel: InstallPanelComponents,
    pub is_showing_install_guide: RefCell<bool>,
    pub handler: Retained<DashboardActionHandler>,
}

impl DashboardWindow {
    pub fn new(mtm: MainThreadMarker) -> Rc<Self> {
        let frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(760.0, 630.0));
        let window = unsafe {
            NSWindow::initWithContentRect_styleMask_backing_defer(
                NSWindow::alloc(mtm),
                frame,
                NSWindowStyleMask::Titled | NSWindowStyleMask::Closable | NSWindowStyleMask::Miniaturizable,
                NSBackingStoreType::Buffered,
                false,
            )
        };
        window.setTitle(&NSString::from_str("Codex 个人资料与 Token 用量看板 (Pro 5x)"));
        unsafe { window.setReleasedWhenClosed(false) };
        window.center();

        let view = window.contentView().expect("Must have content view");
        window.setBackgroundColor(Some(&NSColor::colorWithSRGBRed_green_blue_alpha(0.08, 0.08, 0.09, 1.0)));

        let handler = DashboardActionHandler::new(mtm);
        let header = build_header(mtm, &view, &handler);

        let panel_rect = NSRect::new(NSPoint::new(24.0, 36.0), NSSize::new(712.0, 362.0));
        let (panel_skeleton, mut skeleton_bars) = build_skeleton_panel(mtm, panel_rect);
        view.addSubview(&panel_skeleton);

        for sk in &header.card_skeletons {
            skeleton_bars.push(sk.clone());
        }
        for sk in &header.rate_skeletons {
            skeleton_bars.push(sk.clone());
        }

        let hm = build_heatmap_panel(mtm, panel_rect, &handler);
        view.addSubview(&hm.panel);

        let (panel_daily_bars, daily_bars) = build_daily_bars_panel(mtm, panel_rect);
        view.addSubview(&panel_daily_bars);

        let (panel_models, model_bars) = build_models_panel(mtm, panel_rect);
        view.addSubview(&panel_models);

        let broadcast = build_broadcast_panel(mtm, panel_rect);
        view.addSubview(&broadcast.panel);

        let color_muted = NSColor::colorWithSRGBRed_green_blue_alpha(0.55, 0.58, 0.64, 1.0);
        let label_time = create_label(
            mtm,
            "数据加载中... • 关闭此窗口后，应用依然常驻在顶部菜单栏",
            NSRect::new(NSPoint::new(24.0, 10.0), NSSize::new(712.0, 18.0)),
            10.5,
            false,
            Some(&color_muted),
            NSTextAlignment::Center,
        );
        view.addSubview(&label_time);

        let install_frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(760.0, 630.0));
        let install_panel = build_install_panel(mtm, install_frame, &handler);
        view.addSubview(&install_panel.panel);
        install_panel.panel.setHidden(true);

        let dash = Rc::new(Self {
            window,
            header,
            current_main_tab: RefCell::new(MainTab::Heatmap),
            panel_skeleton,
            skeleton_bars,
            skeleton_anim_step: RefCell::new(0),
            skeleton_timer: RefCell::new(None),
            heatmap_mode: RefCell::new(HeatmapMode::Daily),
            cached_buckets: RefCell::new(Vec::new()),
            cached_lifetime: RefCell::new(0),
            hm,
            daily_bars,
            panel_daily_bars,
            model_bars,
            panel_models,
            broadcast,
            label_time,
            install_panel,
            is_showing_install_guide: RefCell::new(false),
            handler,
        });
        dash.set_refreshing(true);
        dash
    }

    pub fn show(&self, mtm: MainThreadMarker) {
        self.window.makeKeyAndOrderFront(None);
        #[allow(deprecated)]
        NSApplication::sharedApplication(mtm).activateIgnoringOtherApps(true);
    }
}
