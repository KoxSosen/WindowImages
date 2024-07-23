#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
use wallpaper;
use wallpaper::Mode;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
fn my_custom_command() {
    println!("I was invoked from JS!");
}

async fn choose_api() {

}

async fn setup_timer() {

}

async fn remove_timers() {

}


fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        window.hide().unwrap();
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![greet])
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// TODO: Implement proper error handling, and return the exception
async fn set_background_with_mode(url: &str, mode: &str) -> Result<(), ()> {
    wallpaper::set_from_url(url).unwrap();
    let mut internal_mode: Mode = Mode::Crop;
    // TODO: Implement more of these.
    match mode {
        "crop" => internal_mode = Mode::Crop,
        "fit" => internal_mode = Mode::Fit,
        "center" => internal_mode = Mode::Center,
        &_ => {internal_mode = Mode::Crop}
    }
    wallpaper::set_mode(internal_mode).unwrap();
    Ok(())
}