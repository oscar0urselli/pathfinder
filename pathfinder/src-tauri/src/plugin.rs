use std::{collections::HashMap, io::BufRead, process::Command, sync::{Arc, Mutex}, thread};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

use crate::{report::Report, settings::Settings, utils::Toast};

#[derive(Serialize, Deserialize)]
pub enum PluginCommand {
    Register,
    Toast { alert_type: u8, text: String },
    ExecuteRawQuery { query: String },
    FormReq { data: PluginFormData },
    FormRes { dst: String, data: String },
    Exit,
    Terminate { plugins: Vec<String> }
}

#[derive(Serialize, Deserialize)]
pub struct PluginFormData {
    name: String,
    config: HashMap<String, PluginFormConfig>
}

#[derive(Serialize, Deserialize)]
pub struct PluginFormConfig {
    title: String,
    r#type: String,
    options: Option<Vec<String>>,
    min: Option<String>,
    max: Option<String>,
    step: Option<String>,
    regex: Option<String>,
    default: Option<String>
}

pub fn init_plugins_server(app_handle: AppHandle, conn: duckdb::Connection, port: u16) {
    thread::spawn(move || {
        let ctx = zmq::Context::new();
        let socket = ctx.socket(zmq::ROUTER).unwrap();

        socket.bind(&format!("tcp://*:{}", port)).unwrap();

        loop {
            let identity = String::from_utf8(socket.recv_msg(0).unwrap().to_vec()).unwrap();
            let message = socket.recv_msg(0).unwrap().to_vec();
            let command: PluginCommand = serde_json::from_slice(message.as_ref()).unwrap();

            match command {
                PluginCommand::Register => {
                    println!("Register plugin with ID: {}", identity);
                }
                PluginCommand::Toast { alert_type, text } => {
                    app_handle
                        .emit(
                            "toast",
                            &Toast {
                                alert_type: Toast::alert_type_to_string(alert_type).unwrap(),
                                text,
                            },
                        )
                        .unwrap();
                }
                PluginCommand::ExecuteRawQuery { query } => {
                    conn.execute(&query, []);
                },
                PluginCommand::FormReq { data } => {
                    app_handle.emit("form", &data).unwrap();
                },
                PluginCommand::FormRes { dst, data } => {
                    socket.send(dst.as_bytes(), zmq::SNDMORE);
                    socket.send(data.as_bytes(), 0);
                },
                PluginCommand::Exit => {},
                PluginCommand::Terminate { plugins } => {
                    
                }
            };
        }
    });
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Plugin {
    pub path: String,
    pub folder: String,
    pub config: PluginConfig
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PluginConfig {
    pub name: String,
    pub author: String,
    pub license: String,
    pub repository: String,
    pub version: String,
    pub language: PluginLanguage
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PluginLanguage {
    Py,
    Js,
    Lua
}

#[tauri::command]
pub fn get_plugins(plugins: tauri::State<Arc<Mutex<HashMap<String, Plugin>>>>) -> HashMap<String, Plugin> {
    let plugins_arc_clone = Arc::clone(&plugins);
    let plugins = plugins_arc_clone.lock().unwrap();
    
    plugins.clone()
}

#[derive(Serialize, Deserialize)]
pub struct FormRes {
    dst: String,
    data: String
}

#[tauri::command]
pub fn send_plugin_form_res(app_handle: AppHandle, settings: tauri::State<Arc<Mutex<Settings>>>, plugin: String, params: String) {
    let settings_arc_clone = Arc::clone(&settings);
    let settings = settings_arc_clone.lock().unwrap();
    
    let data = serde_json::to_string(&FormRes {
        dst: plugin,
        data: params
    }).unwrap();
    
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::DEALER).unwrap();

    socket.connect(&format!("tcp://*:{}", settings.plugins_server_port)).unwrap();
    
    socket.send("chomik", zmq::SNDMORE);
    socket.send(data.as_bytes(), 0);
}

#[tauri::command]
pub fn run_plugin(app_handle: AppHandle, loaded_report: tauri::State<Arc<Mutex<Option<Report>>>>, settings: tauri::State<Arc<Mutex<Settings>>>, plugins: tauri::State<Arc<Mutex<HashMap<String, Plugin>>>>, plugin_name: String) {
    let loaded_report_arc_clone = Arc::clone(&loaded_report);
    let loaded_report = loaded_report_arc_clone.lock().unwrap().clone();
    
    if loaded_report.is_none() {
        app_handle.emit("toast", &Toast { alert_type: "warning".to_string(), text: "No report loaded. You must create and/or load a report first.".to_string() }).unwrap()
    }
    
    let settings_arc_clone = Arc::clone(&settings);
    let settings = settings_arc_clone.lock().unwrap().clone();
    
    let plugins_arc_clone = Arc::clone(&plugins);
    let plugins = plugins_arc_clone.lock().unwrap();
    let plugin = plugins.get(&plugin_name).unwrap().clone();
    
    match plugin.config.language {
        PluginLanguage::Py => {
            match settings.python.clone() {
                Some(path) => {
                    thread::spawn(move || {
                        app_handle.emit("toast", &Toast { alert_type: "warning".to_string(), text: "Plugin started.".to_string() }).unwrap();
                        match Command::new(path)
                            .current_dir(&plugin.path)
                            .arg("src/main.py")
                            .arg("--port")
                            .arg(settings.plugins_server_port.to_string())
                            .arg("--report")
                            .arg(&loaded_report.as_ref().unwrap().id)
                            .output() {
                                Ok(r) => {
                                    if r.status.success() {
                                        app_handle.emit("toast", &Toast { alert_type: "success".to_string(), text: "Plugin successfully terminated.".to_string() }).unwrap()
                                    }
                                    else {
                                        let mut stderr = String::new();
                                        for line in r.stderr.lines() {
                                            if let Ok(line) = line {
                                                stderr.push_str(&line);
                                            }
                                        }
                                        app_handle.emit("toast", &Toast { alert_type: "danger".to_string(), text: stderr }).unwrap()
                                    }
                                },
                                Err(err) => app_handle.emit("toast", &Toast { alert_type: "danger".to_string(), text: err.to_string() }).unwrap()
                            }
                    });
                },
                None => app_handle.emit("toast", &Toast {
                    alert_type: "danger".to_string(),
                    text: "Python interpreter path has not been configured.".to_string()
                }).unwrap()
            }
        },
        PluginLanguage::Js => {},
        PluginLanguage::Lua => {}
    };
}