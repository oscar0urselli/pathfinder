use std::{
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Emitter};

use crate::utils::{Toast, ToastType};

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub notification_pos: String,
    pub plugins_server_port: u16,
    pub python: Option<String>,
    pub node_js: Option<String>,
    pub lua: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            notification_pos: "top-center".to_string(),
            plugins_server_port: 5555,
            python: None,
            node_js: None,
            lua: None,
        }
    }
}

impl Settings {
    /// Save the struct in a file called `config.json`
    /// It will be saved in the suggested direcotory for configuration files.
    /// If the saving fails, the file won't be saved.
    pub fn save(&self, path: PathBuf) -> Result<String, String> {
        match fs::write(
            path.join("config.json"),
            serde_json::to_string_pretty(self).unwrap(),
        ) {
            Ok(_) => Ok("`config.json` saved successfully.".to_string()),
            Err(err) => Err(err.to_string()),
        }
    }

    /// Guaranteed to return the struct.
    /// If the file `config.json` doesn't exists or it's invalid, default values will be used.
    pub fn load(path: PathBuf) -> Self {
        let cnt = fs::read_to_string(path.join("config.json"));
        match cnt {
            Ok(c) => serde_json::from_str(&c).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }
}

#[tauri::command]
pub fn get_settings(settings: tauri::State<Arc<Mutex<Settings>>>) -> Settings {
    let settings_arc_clone = Arc::clone(&settings);
    let settings = settings_arc_clone.lock().unwrap();

    settings.clone()
}

#[tauri::command]
pub fn set_notifications_pos(settings: tauri::State<Arc<Mutex<Settings>>>, pos: String) {
    let settings_arc_clone = Arc::clone(&settings);
    let mut settings = settings_arc_clone.lock().unwrap();

    settings.notification_pos = pos;
}

#[tauri::command]
pub fn set_plugins_server_port(app_handle: AppHandle, settings: tauri::State<Arc<Mutex<Settings>>>, port: u16) {
    let settings_arc_clone = Arc::clone(&settings);
    let mut settings = settings_arc_clone.lock().unwrap();

    settings.plugins_server_port = port;

    settings
        .save(app_handle.path().app_local_data_dir().unwrap())
        .unwrap();

    app_handle.restart();
} 

#[tauri::command]
pub fn set_python_interpreter(app_handle: AppHandle, settings: tauri::State<Arc<Mutex<Settings>>>, path: String) -> Option<String> {
    let settings_arc_clone = Arc::clone(&settings);
    let mut settings = settings_arc_clone.lock().unwrap();
    
    match fs::exists(&path) {
        Ok(b) => match b {
            true => {
                settings.python = Some(path);
                settings
                    .save(app_handle.path().app_local_data_dir().unwrap())
                    .unwrap();
                app_handle.emit("toast", &Toast {
                    r#type: ToastType::Success,
                    text: "Python interpreter updated.".to_string()
                }).unwrap();
            },
            false => app_handle.emit("toast", &Toast {
                r#type: ToastType::Danger,
                text: "The provided Python interpreter doesn't exists.".to_string()
            }).unwrap()
        },
        Err(err) => app_handle.emit("toast", &Toast {
            r#type: ToastType::Danger,
            text: err.to_string()
        }).unwrap()
    };
    
    settings.python.clone()
}

#[tauri::command]
pub fn set_node_js_interpreter(app_handle: AppHandle, settings: tauri::State<Arc<Mutex<Settings>>>, path: String) -> Option<String> {
    let settings_arc_clone = Arc::clone(&settings);
    let mut settings = settings_arc_clone.lock().unwrap();
    
    match fs::exists(&path) {
        Ok(b) => match b {
            true => {
                settings.node_js = Some(path);
                settings
                    .save(app_handle.path().app_local_data_dir().unwrap())
                    .unwrap();
                app_handle.emit("toast", &Toast {
                    r#type: ToastType::Success,
                    text: "Node.js interpreter updated.".to_string()
                }).unwrap();
            },
            false => app_handle.emit("toast", &Toast {
                r#type: ToastType::Danger,
                text: "The provided Node.js interpreter doesn't exists.".to_string()
            }).unwrap()
        },
        Err(err) => app_handle.emit("toast", &Toast {
            r#type: ToastType::Danger,
            text: err.to_string()
        }).unwrap()
    };
    
    settings.node_js.clone()
}

#[tauri::command]
pub fn set_lua_interpreter(app_handle: AppHandle, settings: tauri::State<Arc<Mutex<Settings>>>, path: String) -> Option<String> {
    let settings_arc_clone = Arc::clone(&settings);
    let mut settings = settings_arc_clone.lock().unwrap();
    
    match fs::exists(&path) {
        Ok(b) => match b {
            true => {
                settings.lua = Some(path);
                settings
                    .save(app_handle.path().app_local_data_dir().unwrap())
                    .unwrap();
                app_handle.emit("toast", &Toast {
                    r#type: ToastType::Success,
                    text: "Lua interpreter updated.".to_string()
                }).unwrap();
            },
            false => app_handle.emit("toast", &Toast {
                r#type: ToastType::Danger,
                text: "The provided Lua interpreter doesn't exists.".to_string()
            }).unwrap()
        },
        Err(err) => app_handle.emit("toast", &Toast {
            r#type: ToastType::Danger,
            text: err.to_string()
        }).unwrap()
    };
    
    settings.lua.clone()
}