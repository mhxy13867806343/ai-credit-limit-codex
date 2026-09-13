use objc2::rc::Retained;
use objc2::runtime::Sel;
use objc2::MainThreadMarker;
use objc2_app_kit::{NSButton, NSColor, NSFont, NSTextAlignment, NSTextField, NSView};
use objc2_foundation::{ns_string, NSObject, NSRect, NSString};

pub fn color_bg() -> Retained<NSColor> {
    NSColor::colorWithSRGBRed_green_blue_alpha(0.08, 0.08, 0.09, 1.0)
}
pub fn color_card() -> Retained<NSColor> {
    NSColor::colorWithSRGBRed_green_blue_alpha(0.12, 0.13, 0.15, 1.0)
}
#[allow(dead_code)]
pub fn color_card_subtle() -> Retained<NSColor> {
    NSColor::colorWithSRGBRed_green_blue_alpha(0.14, 0.15, 0.18, 1.0)
}
pub fn color_primary() -> Retained<NSColor> {
    NSColor::colorWithSRGBRed_green_blue_alpha(0.14, 0.38, 0.92, 1.0)
}
pub fn color_cyan() -> Retained<NSColor> {
    NSColor::colorWithSRGBRed_green_blue_alpha(0.35, 0.75, 1.0, 1.0)
}
#[allow(dead_code)]
pub fn color_amber() -> Retained<NSColor> {
    NSColor::colorWithSRGBRed_green_blue_alpha(0.95, 0.75, 0.25, 1.0)
}
pub fn color_muted() -> Retained<NSColor> {
    NSColor::colorWithSRGBRed_green_blue_alpha(0.55, 0.58, 0.64, 1.0)
}

pub fn create_box(mtm: MainThreadMarker, frame: NSRect, bg: &NSColor) -> Retained<NSTextField> {
    let tf = NSTextField::labelWithString(&NSString::from_str(""), mtm);
    tf.setFrame(frame);
    tf.setDrawsBackground(true);
    tf.setBackgroundColor(Some(bg));
    tf
}

pub fn create_label(
    mtm: MainThreadMarker,
    text: &str,
    frame: NSRect,
    size: f64,
    bold: bool,
    color: Option<&NSColor>,
    align: NSTextAlignment,
) -> Retained<NSTextField> {
    let tf = NSTextField::labelWithString(&NSString::from_str(text), mtm);
    tf.setFrame(frame);
    tf.setDrawsBackground(false);
    tf.setBezeled(false);
    tf.setEditable(false);
    tf.setSelectable(false);
    tf.setAlignment(align);
    let font = if bold { NSFont::boldSystemFontOfSize(size) } else { NSFont::systemFontOfSize(size) };
    tf.setFont(Some(&font));
    if let Some(c) = color { tf.setTextColor(Some(c)); }
    tf
}

pub fn create_transparent_button(
    mtm: MainThreadMarker,
    frame: NSRect,
    target: Option<&NSObject>,
    action: Option<Sel>,
) -> Retained<NSButton> {
    let btn = unsafe { NSButton::buttonWithTitle_target_action(ns_string!(""), target.map(|v| &**v), action, mtm) };
    btn.setFrame(frame);
    btn.setTransparent(true);
    btn
}

pub fn create_clickable_box(
    mtm: MainThreadMarker,
    parent: &NSView,
    frame: NSRect,
    text: &str,
    font_size: f64,
    bg_color: &NSColor,
    text_color: &NSColor,
    target: &NSObject,
    action: Sel,
) -> (Retained<NSTextField>, Retained<NSTextField>) {
    let bg = create_box(mtm, frame, bg_color);
    parent.addSubview(&bg);
    let lbl = create_label(mtm, text, frame, font_size, true, Some(text_color), NSTextAlignment::Center);
    parent.addSubview(&lbl);
    let btn = create_transparent_button(mtm, frame, Some(target), Some(action));
    parent.addSubview(&btn);
    (bg, lbl)
}

pub fn cell_date_dynamic(col: usize, row: usize, total_cols: usize) -> (i32, i32, i32) {
    let mut now: libc::time_t = 0;
    unsafe { libc::time(&mut now) };
    let mut tm: libc::tm = unsafe { std::mem::zeroed() };
    unsafe { libc::localtime_r(&now, &mut tm) };
    let cur_wday = (tm.tm_wday + 6) % 7;
    let days_to_sunday = 6 - cur_wday;
    let end_week_sunday = now + (days_to_sunday as i64) * 86400;
    let weeks_back = (total_cols - 1).saturating_sub(col);
    let cell_time = end_week_sunday - (weeks_back as i64 * 7 + (6 - row) as i64) * 86400;
    let mut cell_tm: libc::tm = unsafe { std::mem::zeroed() };
    unsafe { libc::localtime_r(&cell_time, &mut cell_tm) };
    (cell_tm.tm_year + 1900, cell_tm.tm_mon + 1, cell_tm.tm_mday)
}
