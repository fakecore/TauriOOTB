// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{WindowBuilder};
mod core;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .menu(core::InitMenu())
        .on_menu_event(move |event|core::MenuEvenHandle(event))
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app|{
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}