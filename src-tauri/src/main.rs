// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{Manager, WindowBuilder};
mod core;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_config() -> String {
    "hello".to_string()
}

fn main() {
    core::init_config();

    tauri::Builder::default()
        .menu(core::init_menu())
        .on_menu_event(move |event| core::menu_even_handle(event))
        .invoke_handler(tauri::generate_handler![greet, get_config])
        .setup(|app| {
            // let main_window = app.get_window("main").unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
