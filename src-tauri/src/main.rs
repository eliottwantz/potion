// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use tauri::Manager;
// use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};
// use tauri::{SystemTray, SystemTrayEvent};

fn main() {
    // let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    // let show = CustomMenuItem::new("Open Potion".to_string(), "Show");
    // let tray_menu = SystemTrayMenu::new()
    //     .add_item(quit)
    //     .add_native_item(SystemTrayMenuItem::Separator)
    //     .add_item(show);

    // let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        // .system_tray(system_tray)
        // .on_system_tray_event(|app, event| {
        //     if let SystemTrayEvent::MenuItemClick { id, .. } = event {
        //         if let "quit" = id.as_str() {
        //             std::process::exit(0);
        //         } else if let "show" = id.as_str() {
        //             let window = app.get_window("main").unwrap();
        //             window.show().unwrap();
        //         }
        //     }
        // })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    // .on_window_event(|event| {
    //     if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
    //         event.window().hide().unwrap();
    //         api.prevent_close();
    //     }
    // })
    // .build(tauri::generate_context!())
    // .expect("error while building tauri application")
    // .run(|_app_handle, event| {
    //     if let tauri::RunEvent::ExitRequested { api, .. } = event {
    //         api.prevent_exit();
    //     }
    // });
}
