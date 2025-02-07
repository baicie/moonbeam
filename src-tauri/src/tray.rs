use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, SystemTraySubmenu, Window, WindowBuilder, WindowUrl,
};

// 创建托盘菜单
pub fn create_tray() -> SystemTray {
    let brightness_50 = CustomMenuItem::new("brightness_50".to_string(), "亮度 50%");
    let brightness_100 = CustomMenuItem::new("brightness_100".to_string(), "亮度 100%");

    let brightness_submenu = SystemTraySubmenu::new(
        "亮度调节",
        SystemTrayMenu::new()
            .add_item(brightness_50)
            .add_item(brightness_100),
    );

    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let show = CustomMenuItem::new("show".to_string(), "显示主窗口");

    let tray_menu = SystemTrayMenu::new()
        .add_submenu(brightness_submenu)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(show)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

// 处理托盘事件
pub fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        // 左键点击显示菜单
        SystemTrayEvent::LeftClick { .. } => {
            let window = app.get_window("main").unwrap();
            if window.is_visible().unwrap() {
                window.hide().unwrap();
            } else {
                window.show().unwrap();
            }
        }
        // 右键点击显示小页面
        SystemTrayEvent::RightClick { .. } => {
            show_popup_window(app);
        }
        // 处理菜单项点击
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                app.exit(0);
            }
            "show" => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            "brightness_50" => {
                // 调用亮度设置函数
                let _ = crate::brightness::set_brightness(0.5);
            }
            "brightness_100" => {
                // 调用亮度设置函数
                let _ = crate::brightness::set_brightness(1.0);
            }
            _ => {}
        },
        _ => {}
    }
}

// 创建小弹出窗口
fn show_popup_window(app: &AppHandle) {
    // 检查是否已存在弹出窗口
    if let Some(popup) = app.get_window("popup") {
        popup.show().unwrap();
        return;
    }

    // 创建新的弹出窗口
    let popup = WindowBuilder::new(app, "popup", WindowUrl::App("popup.html".into()))
        .title("亮度调节")
        .inner_size(300.0, 200.0)
        .decorations(false) // 无边框窗口
        .always_on_top(true)
        .skip_taskbar(true)
        .center()
        .build()
        .unwrap();

    // 失去焦点时自动隐藏
    let popup_clone = popup.clone();
    popup.on_window_event(move |event| {
        if let tauri::WindowEvent::Focused(focused) = event {
            if !focused {
                let _ = popup_clone.hide();
            }
        }
    });
}
