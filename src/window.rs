use std::cell::RefCell;
use std::rc::Rc;

use objc2::rc::Retained;
use objc2::{MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{
    NSApplication, NSBackingStoreType, NSColor, NSFont, NSTextAlignment, NSTextField, NSWindow,
    NSWindowStyleMask,
};
use objc2_foundation::{NSPoint, NSRect, NSSize, NSString};

use crate::codex::CodexUsage;

pub struct DashboardWindow {
    pub window: Retained<NSWindow>,
    pub label_title: Retained<NSTextField>,
    pub label_percent: Retained<NSTextField>,
    pub label_used: Retained<NSTextField>,
    pub label_reset: Retained<NSTextField>,
    pub label_coupon: Retained<NSTextField>,
    pub label_time: Retained<NSTextField>,
}

thread_local! {
    static GLOBAL_DASHBOARD: RefCell<Option<Rc<DashboardWindow>>> = const { RefCell::new(None) };
}

impl DashboardWindow {
    pub fn new(mtm: MainThreadMarker) -> Rc<Self> {
        let frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(480.0, 420.0));
        let window = unsafe {
            NSWindow::initWithContentRect_styleMask_backing_defer(
                NSWindow::alloc(mtm),
                frame,
                NSWindowStyleMask::Titled
                    | NSWindowStyleMask::Closable
                    | NSWindowStyleMask::Miniaturizable,
                NSBackingStoreType::Buffered,
                false,
            )
        };

        window.setTitle(&NSString::from_str("Codex 额度监控看板 (Pro 5x)"));
        unsafe { window.setReleasedWhenClosed(false) };
        window.center();

        let view = window.contentView().expect("Must have content view");

        window.setBackgroundColor(Some(&NSColor::colorWithSRGBRed_green_blue_alpha(
            0.11, 0.11, 0.13, 1.0,
        )));

        // 1. Header Title: "artaGrace • Pro (5x)"
        let label_title = create_label(
            mtm,
            "Codex • Pro (5x)",
            NSRect::new(NSPoint::new(24.0, 360.0), NSSize::new(432.0, 32.0)),
            22.0,
            true,
            Some(&NSColor::whiteColor()),
            NSTextAlignment::Left,
        );
        view.addSubview(&label_title);

        // Subtitle
        let label_sub = create_label(
            mtm,
            "OpenAI 官方限额实时追踪 (与桌面端同步)",
            NSRect::new(NSPoint::new(24.0, 335.0), NSSize::new(432.0, 20.0)),
            13.0,
            false,
            Some(&NSColor::colorWithSRGBRed_green_blue_alpha(0.6, 0.6, 0.65, 1.0)),
            NSTextAlignment::Left,
        );
        view.addSubview(&label_sub);

        // 2. Big Highlight: "剩余 40%"
        let label_percent = create_label(
            mtm,
            "剩余 40%",
            NSRect::new(NSPoint::new(24.0, 260.0), NSSize::new(432.0, 60.0)),
            46.0,
            true,
            Some(&NSColor::colorWithSRGBRed_green_blue_alpha(0.2, 0.85, 0.5, 1.0)),
            NSTextAlignment::Center,
        );
        view.addSubview(&label_percent);

        // 3. Used details: "周使用限额: [██████░░░░] 59%"
        let label_used = create_label(
            mtm,
            "周使用限额: 已用 59%",
            NSRect::new(NSPoint::new(24.0, 225.0), NSSize::new(432.0, 24.0)),
            15.0,
            false,
            Some(&NSColor::whiteColor()),
            NSTextAlignment::Center,
        );
        view.addSubview(&label_used);

        // 4. Reset countdown: "⏳ 额度将于 6d 6h 后重置"
        let label_reset = create_label(
            mtm,
            "⏳ 额度将于 6d 6h 后完全刷新重置",
            NSRect::new(NSPoint::new(24.0, 180.0), NSSize::new(432.0, 24.0)),
            14.0,
            true,
            Some(&NSColor::colorWithSRGBRed_green_blue_alpha(1.0, 0.78, 0.28, 1.0)),
            NSTextAlignment::Center,
        );
        view.addSubview(&label_reset);

        // 5. Reset credit card: "🎁 重置特权: 完全重置 (可用 1 次 • 10/5 到期)"
        let label_coupon = create_label(
            mtm,
            "🎁 使用限额重置: 完全重置 (可用 1 次，将于 10/5 GMT+8 06:16 到期)",
            NSRect::new(NSPoint::new(24.0, 130.0), NSSize::new(432.0, 36.0)),
            13.0,
            false,
            Some(&NSColor::colorWithSRGBRed_green_blue_alpha(0.7, 0.85, 1.0, 1.0)),
            NSTextAlignment::Center,
        );
        view.addSubview(&label_coupon);

        // 6. Tips footer: "关闭此窗口后，Codex Monitor 依然常驻在顶部菜单栏中"
        let label_tips = create_label(
            mtm,
            "💡 关闭此窗口后，应用依然常驻在顶部菜单栏，随时可以点击再次打开",
            NSRect::new(NSPoint::new(24.0, 60.0), NSSize::new(432.0, 20.0)),
            12.0,
            false,
            Some(&NSColor::colorWithSRGBRed_green_blue_alpha(0.5, 0.5, 0.55, 1.0)),
            NSTextAlignment::Center,
        );
        view.addSubview(&label_tips);

        // 7. Last update time
        let label_time = create_label(
            mtm,
            "更新时间: 11:52:00",
            NSRect::new(NSPoint::new(24.0, 25.0), NSSize::new(432.0, 20.0)),
            11.0,
            false,
            Some(&NSColor::colorWithSRGBRed_green_blue_alpha(0.4, 0.4, 0.45, 1.0)),
            NSTextAlignment::Center,
        );
        view.addSubview(&label_time);

        Rc::new(Self {
            window,
            label_title,
            label_percent,
            label_used,
            label_reset,
            label_coupon,
            label_time,
        })
    }

    pub fn update(&self, usage: &CodexUsage) {
        let plan = usage.display_plan_name();
        self.label_title.setStringValue(&NSString::from_str(&format!(
            "Codex • {}",
            plan
        )));

        if let Some(rem) = usage.remaining_percent() {
            self.label_percent
                .setStringValue(&NSString::from_str(&format!("剩余 {}%", rem)));
        } else {
            self.label_percent
                .setStringValue(&NSString::from_str("☁ 加载中..."));
        }

        let used = usage.primary_percent.unwrap_or(0);
        let rem = usage.remaining_percent().unwrap_or(100 - used);
        self.label_used.setStringValue(&NSString::from_str(&format!(
            "{} • 剩余 {}% (已用 {}%)",
            usage.primary_window_label(),
            rem,
            used
        )));

        self.label_reset.setStringValue(&NSString::from_str(&format!(
            "⏳ 额度将于 {} 后完全重置",
            usage.reset_countdown_label()
        )));

        if let Some(coupon) = usage.reset_credit_label() {
            self.label_coupon.setStringValue(&NSString::from_str(&format!(
                "🎁 使用限额重置: {}",
                coupon
            )));
        } else {
            self.label_coupon.setStringValue(&NSString::from_str(
                "🎁 暂无可用的手动完全重置券",
            ));
        }

        self.label_time.setStringValue(&NSString::from_str(&format!(
            "更新时间: {}",
            usage.updated_at
        )));
    }

    pub fn show(&self, mtm: MainThreadMarker) {
        self.window.makeKeyAndOrderFront(None);
        #[allow(deprecated)]
        NSApplication::sharedApplication(mtm).activateIgnoringOtherApps(true);
    }
}

fn create_label(
    mtm: MainThreadMarker,
    text: &str,
    frame: NSRect,
    font_size: f64,
    is_bold: bool,
    color: Option<&NSColor>,
    align: NSTextAlignment,
) -> Retained<NSTextField> {
    let ns_text = NSString::from_str(text);
    let tf = NSTextField::labelWithString(&ns_text, mtm);
    tf.setFrame(frame);
    if is_bold {
        tf.setFont(Some(&NSFont::boldSystemFontOfSize(font_size)));
    } else {
        tf.setFont(Some(&NSFont::systemFontOfSize(font_size)));
    }
    if let Some(c) = color {
        tf.setTextColor(Some(c));
    }
    tf.setAlignment(align);
    tf
}

pub fn open_or_update_dashboard(mtm: MainThreadMarker, usage: &CodexUsage) {
    GLOBAL_DASHBOARD.with(|cell| {
        let mut borrowed = cell.borrow_mut();
        if borrowed.is_none() {
            *borrowed = Some(DashboardWindow::new(mtm));
        }
        if let Some(dash) = borrowed.as_ref() {
            dash.update(usage);
            dash.show(mtm);
        }
    });
}
