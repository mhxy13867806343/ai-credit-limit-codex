use objc2::rc::Retained;
use objc2_app_kit::NSTextField;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HeatmapMode {
    Daily,
    Weekly,
    Cumulative,
    All,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MainTab {
    Heatmap,
    DailyBars,
    Models,
    Broadcast,
}

pub struct BarComponent {
    pub label_val: Retained<NSTextField>,
    #[allow(dead_code)]
    pub track: Retained<NSTextField>,
    pub fill: Retained<NSTextField>,
    pub label_date: Retained<NSTextField>,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub max_height: f64,
}

#[allow(dead_code)]
pub struct ModelBarComponent {
    pub label_info: Retained<NSTextField>,
    pub track: Retained<NSTextField>,
    pub fill: Retained<NSTextField>,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
