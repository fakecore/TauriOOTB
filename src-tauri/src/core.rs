use tauri::{App, CustomMenuItem, Manager, Menu, MenuEvent, MenuItem, Submenu, Window, Wry, WindowMenuEvent, window};




//menu, move to core/menu.rs

pub fn InitMenu() -> Menu {
// here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.

    let settings = CustomMenuItem::new("settings".to_string(), "Settings");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let mainSubmenu = Submenu::new("main",
                               Menu::new()
                                   .add_item(settings)
                                   .add_item(quit)
                                   .add_item(close)
                            );

    Menu::new()
        .add_submenu(mainSubmenu)
}

pub fn MenuEvenHandle(event: WindowMenuEvent<Wry>){

    let window = event.window();
    let handler = window.app_handle();
    match event.menu_item_id() {
        "settings" =>{
            if window.get_window("settings").is_some() {
                window.get_window("settings").unwrap().show();
            }else{
                let settings_window = tauri::WindowBuilder::new(
                    &handler,
                    "settings",
                    tauri::WindowUrl::App("index.html".into())
                ).build();
                settings_window.unwrap().show();
            }
        }
        "quit" => {
            std::process::exit(0);
        }
        "close" => {
            // event.window().close().unwrap();
        }
        _ => {}
    }

}

pub fn MenuUpdate(app: &mut App<Wry>){
}