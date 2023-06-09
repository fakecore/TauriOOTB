use std::{
    fs::{self, File},
    io::Write,
    path::{self, Path, PathBuf},
    string,
};

use tauri::{App, CustomMenuItem, Manager, Menu, Submenu, WindowMenuEvent, Wry};

//menu, move to core/menu.rs

pub fn init_menu() -> Menu {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.

    let settings = CustomMenuItem::new("settings".to_string(), "Settings");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let main_submenu = Submenu::new(
        "main",
        Menu::new()
            .add_item(settings)
            .add_item(quit)
            .add_item(close),
    );

    Menu::new().add_submenu(main_submenu)
}

pub fn menu_even_handle(event: WindowMenuEvent<Wry>) {
    let window = event.window();
    window.eval(&format!(
        "window.location.replace('http://localhost:{}')",
        "1234"
    ));
    let handler = window.app_handle();
    match event.menu_item_id() {
        "settings" => {
            if window.get_window("settings").is_some() {
                window.get_window("settings").unwrap().show();
            } else {
                let settings_window = tauri::WindowBuilder::new(
                    &handler,
                    "settings",
                    // tauri::WindowUrl::App("http://127.0.0.1:1420/config".into()),
                    tauri::WindowUrl::App("config".into()),
                )
                .build();
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

pub fn menu_update(app: &mut App<Wry>) {}

pub fn init_config() {
    let user_path = get_user_dir_path();
    //create a dir to save config
    let conf_path = match create_dir_and_config_file(&user_path, |conf_path: &PathBuf| {}) {
        Ok(path) => path,
        Err(err) => panic!("{}", err),
    };
}

fn get_user_dir_path() -> PathBuf {
    match home::home_dir() {
        Some(path) => path,
        None => match std::env::current_dir() {
            Ok(path) => path,
            Err(_) => ".".to_string().into(),
        },
    }
}

fn create_dir_and_config_file<F>(
    dir_path: &PathBuf,
    init_conf_cb: F,
) -> Result<PathBuf, std::io::Error>
where
    F: Fn(&PathBuf),
{
    let dir = Path::new(dir_path);
    let folder_name = ".tauriootb";
    let folder_path = dir.join(folder_name);

    // 判断文件夹是否存在，不存在则创建
    if !folder_path.exists() {
        fs::create_dir(&folder_path)?;
    }

    // Build the path to the config.toml file
    let config_file_path = folder_path.join("config.toml");

    // Check if the config.toml file exists, create it if it doesn't
    if !config_file_path.exists() {
        File::create(&config_file_path)?;
        init_conf_cb(&config_file_path);
    }
    //todo
    //rewrite again for debug
    // if debug
    if true {
        init_conf_cb(&config_file_path);
    }
    println!("config file path: {:?}", config_file_path);
    Ok(config_file_path)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::{create_dir_and_config_file, get_user_dir_path};

    #[test]
    fn test_get_user_dir_config() {
        let user_dir_binding = get_user_dir_path();
        let user_dir = user_dir_binding.to_str().unwrap_or("");
        print!("{:?}", user_dir);
        assert_eq!(
            create_dir_and_config_file(&get_user_dir_path(), |conf_path: &PathBuf| {})
                .unwrap()
                .to_str()
                .unwrap(),
            user_dir.to_owned() + &"/.tauriootb/config.toml".clone()
        );
    }
}
