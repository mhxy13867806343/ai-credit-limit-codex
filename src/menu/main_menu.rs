use objc2::rc::Retained;
use objc2::{sel, MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{NSMenu, NSMenuItem};
use std::cell::RefCell;
use objc2_foundation::{ns_string, NSObject, NSString};

use super::builder::make_menu_item;

thread_local! {
    static REFRESH_MENU_ITEM: RefCell<Option<Retained<NSMenuItem>>> = const { RefCell::new(None) };
}

pub fn set_main_menu_refresh_state(title: &str, enabled: bool) {
    REFRESH_MENU_ITEM.with(|cell| {
        if let Some(item) = cell.borrow().as_ref() {
            item.setTitle(&NSString::from_str(title));
            item.setEnabled(enabled);
        }
    });
}

pub fn build_system_main_menu(
    mtm: MainThreadMarker,
    handler_obj: &NSObject,
) -> Retained<NSMenu> {
    let main_menu = NSMenu::new(mtm);

    // 1. App Menu (Codex Monitor)
    let app_item = NSMenuItem::new(mtm);
    let app_menu = NSMenu::initWithTitle(NSMenu::alloc(mtm), ns_string!("Codex Monitor"));
    app_menu.addItem(&make_menu_item(mtm, "关于 Codex Monitor", Some(sel!(orderFrontStandardAboutPanel:)), None, "", true));
    app_menu.addItem(&NSMenuItem::separatorItem(mtm));
    app_menu.addItem(&make_menu_item(mtm, "偏好设置...", Some(sel!(openConfigClicked:)), Some(handler_obj), ",", true));
    app_menu.addItem(&NSMenuItem::separatorItem(mtm));
    app_menu.addItem(&make_menu_item(mtm, "隐藏 Codex Monitor", Some(sel!(hide:)), None, "h", true));
    app_menu.addItem(&make_menu_item(mtm, "隐藏其他", Some(sel!(hideOtherApplications:)), None, "h", true));
    app_menu.addItem(&make_menu_item(mtm, "显示全部", Some(sel!(unhideAllApplications:)), None, "", true));
    app_menu.addItem(&NSMenuItem::separatorItem(mtm));
    app_menu.addItem(&make_menu_item(mtm, "退出 Codex Monitor", Some(sel!(quitClicked:)), Some(handler_obj), "q", true));
    app_item.setSubmenu(Some(&app_menu));
    main_menu.addItem(&app_item);

    // 2. 文件 (File)
    let file_item = NSMenuItem::new(mtm);
    let file_menu = NSMenu::initWithTitle(NSMenu::alloc(mtm), ns_string!("文件"));
    file_menu.addItem(&make_menu_item(mtm, "打开监控看板", Some(sel!(openDashboardClicked:)), Some(handler_obj), "d", true));
    let ref_item = make_menu_item(mtm, "立即刷新数据", Some(sel!(refreshClicked:)), Some(handler_obj), "r", true);
    file_menu.addItem(&ref_item);
    REFRESH_MENU_ITEM.with(|cell| {
        *cell.borrow_mut() = Some(ref_item);
    });
    file_menu.addItem(&make_menu_item(mtm, "在浏览器中打开", Some(sel!(openInBrowserClicked:)), Some(handler_obj), "b", true));
    file_menu.addItem(&NSMenuItem::separatorItem(mtm));
    file_menu.addItem(&make_menu_item(mtm, "关闭窗口", Some(sel!(performClose:)), None, "w", true));
    file_item.setSubmenu(Some(&file_menu));
    main_menu.addItem(&file_item);

    // 3. 编辑 (Edit)
    let edit_item = NSMenuItem::new(mtm);
    let edit_menu = NSMenu::initWithTitle(NSMenu::alloc(mtm), ns_string!("编辑"));
    edit_menu.addItem(&make_menu_item(mtm, "撤销", Some(sel!(undo:)), None, "z", true));
    edit_menu.addItem(&make_menu_item(mtm, "重做", Some(sel!(redo:)), None, "Z", true));
    edit_menu.addItem(&NSMenuItem::separatorItem(mtm));
    edit_menu.addItem(&make_menu_item(mtm, "剪切", Some(sel!(cut:)), None, "x", true));
    edit_menu.addItem(&make_menu_item(mtm, "复制", Some(sel!(copy:)), None, "c", true));
    edit_menu.addItem(&make_menu_item(mtm, "粘贴", Some(sel!(paste:)), None, "v", true));
    edit_menu.addItem(&make_menu_item(mtm, "全选", Some(sel!(selectAll:)), None, "a", true));
    edit_item.setSubmenu(Some(&edit_menu));
    main_menu.addItem(&edit_item);

    // 4. 显示 (View)
    let view_item = NSMenuItem::new(mtm);
    let view_menu = NSMenu::initWithTitle(NSMenu::alloc(mtm), ns_string!("显示"));
    view_menu.addItem(&make_menu_item(mtm, "🔥 活动热力图", Some(sel!(mainTabHeatmapClicked:)), Some(handler_obj), "1", true));
    view_menu.addItem(&make_menu_item(mtm, "📊 每日用量", Some(sel!(mainTabBarsClicked:)), Some(handler_obj), "2", true));
    view_menu.addItem(&make_menu_item(mtm, "🤖 模型排行", Some(sel!(mainTabModelsClicked:)), Some(handler_obj), "3", true));
    view_menu.addItem(&make_menu_item(mtm, "🌐 特权券与广播", Some(sel!(mainTabBroadcastClicked:)), Some(handler_obj), "4", true));
    view_item.setSubmenu(Some(&view_menu));
    main_menu.addItem(&view_item);

    // 5. 窗口 (Window)
    let win_item = NSMenuItem::new(mtm);
    let win_menu = NSMenu::initWithTitle(NSMenu::alloc(mtm), ns_string!("窗口"));
    win_menu.addItem(&make_menu_item(mtm, "最小化", Some(sel!(performMiniaturize:)), None, "m", true));
    win_menu.addItem(&make_menu_item(mtm, "缩放", Some(sel!(performZoom:)), None, "", true));
    win_menu.addItem(&NSMenuItem::separatorItem(mtm));
    win_menu.addItem(&make_menu_item(mtm, "前置所有窗口", Some(sel!(arrangeInFront:)), None, "", true));
    win_item.setSubmenu(Some(&win_menu));
    main_menu.addItem(&win_item);

    // 6. 帮助 (Help)
    let help_item = NSMenuItem::new(mtm);
    let help_menu = NSMenu::initWithTitle(NSMenu::alloc(mtm), ns_string!("帮助"));
    help_menu.addItem(&make_menu_item(mtm, "Codex 安装与下载指引", Some(sel!(openInstallGuideClicked:)), Some(handler_obj), "", true));
    help_menu.addItem(&make_menu_item(mtm, "启动 Codex 桌面应用", Some(sel!(openCodexAppClicked:)), Some(handler_obj), "", true));
    help_item.setSubmenu(Some(&help_menu));
    main_menu.addItem(&help_item);

    main_menu
}
