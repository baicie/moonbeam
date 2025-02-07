// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod brightness;
// mod tray;

fn main() {
    tauri::Builder::default()
        // .system_tray(tray::create_tray())
        // .on_system_tray_event(tray::handle_tray_event)
        .invoke_handler(tauri::generate_handler![
            brightness::get_brightness,
            brightness::set_brightness,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
