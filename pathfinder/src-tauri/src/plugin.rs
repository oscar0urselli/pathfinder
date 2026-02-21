use std::{collections::HashMap, fs, io::BufRead, path::{Path, PathBuf}, process::Command as StdCommand, sync::{Arc, Mutex}, thread};

use elevated_command::Command;
use petgraph::graph::UnGraph;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};

use crate::{database::query_to_json, log::{Log, LogType}, net_graph::{NetEdge, NetNode}, report::Report, settings::Settings, utils::{Modal, ModalData, ModalType, PluginFormField, Toast, ToastType, copy_dir_all}};

#[derive(Serialize, Deserialize)]
pub enum PluginCommand {
    Register,
    Toast { r#type: ToastType, text: String },
    Log { r#type: LogType, message: String },
    ExecuteRawQuery { query: String },
    QueryRawSql { query: String },
    QueryRes { count: Option<usize>, data: Option<serde_json::Value>, error: Option<String> },
    ShowForm { title: String, config: Vec<Vec<PluginFormField>> },
    FormData { dst: String, data: String },
    Exit,
    Terminate { plugin: String },
    GetNetGraph,
    NetGraph { graph: UnGraph<NetNode, NetEdge> },
    AddNetNode { node: NetNode },
    AddNetEdge { src: u32, dst: u32, edge: NetEdge },
    RemoveNetNode { node: u32 },
    RemoveNetEdge { edge: u32 },
    UpdateNetNode { index: u32, node: NetNode }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PluginStatus {
    Running,
    WaitingForm,
    Exiting
}

pub fn init_plugins_server(app_handle: AppHandle, conn: duckdb::Connection, port: u16, active_plugins_arc: Arc<Mutex<HashMap<String, PluginStatus>>>, net_graph_arc: Arc<Mutex<UnGraph<NetNode, NetEdge>>>) {
    thread::spawn(move || {
        let ctx = zmq::Context::new();
        let socket = ctx.socket(zmq::ROUTER).unwrap();

        socket.bind(&format!("tcp://*:{}", port)).unwrap();

        loop {
            let identity = socket.recv_string(0).unwrap().unwrap();
            let message = socket.recv_string(0).unwrap().unwrap();
            let command: PluginCommand = serde_json::from_slice(message.as_ref()).unwrap();

            let new_status = match command {
                PluginCommand::Register => {
                    Log::create(LogType::Info, format!("Plugin `{}` registered.", &identity), &conn).unwrap();
                    Some(PluginStatus::Running)
                },
                PluginCommand::Toast { r#type, text } => {
                    match app_handle.emit("toast", &Toast { r#type, text }) {
                        Ok(_) => Log::create(LogType::Info, format!("Plugin `{}` emit toast.", &identity), &conn).unwrap(),
                        Err(err) => Log::create(LogType::Error, format!("Plugin `{}` emit toast raise error: {}.", &identity, err.to_string()), &conn).unwrap()
                    };
                    None
                },
                PluginCommand::Log { r#type, message } => {
                    Log::create(r#type, format!("Plugin `{}`: {}", &identity, message), &conn).unwrap();
                    None
                },
                PluginCommand::ExecuteRawQuery { query } => {
                    match conn.execute(&query, []) {
                        Ok(c) => {
                            Log::create(LogType::Info, format!("Plugin '{}' execute raw query `{}` returns: {}", &identity, &query, c), &conn).unwrap();
                            socket.send(identity.as_bytes(), zmq::SNDMORE);
                            socket.send(serde_json::to_string(&PluginCommand::QueryRes { count: Some(c), data: None, error: None }).unwrap().as_bytes(), 0);
                        },
                        Err(err) => {
                            Log::create(LogType::Error, format!("Plugin '{}' execute raw query `{}` raise error: {}", &identity, &query, err.to_string()), &conn).unwrap();
                            socket.send(identity.as_bytes(), zmq::SNDMORE);
                            socket.send(serde_json::to_string(&PluginCommand::QueryRes { count: None, data: None, error: Some(err.to_string()) }).unwrap().as_bytes(), 0);
                        }
                    };
                    None
                },
                PluginCommand::QueryRawSql { query } => {
                    match query_to_json(&conn, &query) {
                        Ok(data) => {
                            Log::create(LogType::Info, format!("Plugin '{}' query to database `{}`.", &identity, &query), &conn).unwrap();
                            socket.send(identity.as_bytes(), zmq::SNDMORE);
                            socket.send(serde_json::to_string(&PluginCommand::QueryRes { count: None, data: Some(data), error: None }).unwrap().as_bytes(), 0);
                        },
                        Err(err) => {
                            Log::create(LogType::Error, format!("Plugin '{}' query to database `{}` raise error: {}", &identity, &query, err.to_string()), &conn).unwrap();
                            socket.send(identity.as_bytes(), zmq::SNDMORE);
                            socket.send(serde_json::to_string(&PluginCommand::QueryRes { count: None, data: None, error: Some(err.to_string()) }).unwrap().as_bytes(), 0);
                        }
                    };
                    None
                },
                PluginCommand::QueryRes { count, data, error } => None,
                PluginCommand::ShowForm { title, config } => {
                    Log::create(LogType::Info, format!("Plugin `{}` emit form modal.", &identity), &conn).unwrap();
                    match app_handle.emit("modal", &Modal { title: title, r#type: ModalType::PluginForm, data: ModalData::PluginForm { config }, plugin: Some(identity.clone()) }) {
                        Ok(_) => {},
                        Err(err) => { Log::create(LogType::Error, format!("Plugin `{}` emit form modal raise error: {}.", &identity, err.to_string()), &conn).unwrap(); }
                    };
                    Some(PluginStatus::WaitingForm)
                },
                PluginCommand::FormData { dst, data } => {
                    Log::create(LogType::Info, format!("Form data to plugin `{}`: {}", &dst, &data), &conn).unwrap();
                    socket.send(dst.as_bytes(), zmq::SNDMORE);
                    socket.send(&message, 0);
                    Some(PluginStatus::Running)
                },
                PluginCommand::Exit => {
                    Log::create(LogType::Warn, format!("Plugin `{}` is exiting.", &identity), &conn).unwrap();
                    match app_handle.emit("toast", &Toast { r#type: ToastType::Warning, text: "Plugin terminated".to_string() }) {
                        Ok(_) => {},
                        Err(err) => { Log::create(LogType::Error, format!("Plugin `{}` emit toast raise error: {}.", &identity, err.to_string()), &conn).unwrap(); }
                    };
                    Some(PluginStatus::Exiting)
                },
                PluginCommand::Terminate { plugin } => {
                    Log::create(LogType::Warn, format!("Terminate plugin `{}`.", &plugin), &conn).unwrap();
                    socket.send(plugin.as_bytes(), zmq::SNDMORE);
                    socket.send(&message, 0);
                    None
                },
                PluginCommand::GetNetGraph => {
                    Log::create(LogType::Info, format!("Plugin `{}` get network graph data.", &identity), &conn).unwrap();
                    let net_graph = net_graph_arc.lock().unwrap();
                    
                    socket.send(identity.as_bytes(), zmq::SNDMORE);
                    socket.send(serde_json::to_string(&PluginCommand::NetGraph {
                        graph: net_graph.clone()
                    }).unwrap().as_bytes(), 0);
                    None
                },
                PluginCommand::NetGraph { graph } => { None },
                PluginCommand::AddNetNode { node } => {
                    Log::create(LogType::Info, format!("Plugin `{}` add node to network graph.", &identity), &conn).unwrap();
                    let mut net_graph = net_graph_arc.lock().unwrap();
                    net_graph.add_node(node);
                    
                    app_handle.emit("updateNetGraph", &net_graph.clone()).unwrap();
                    
                    None 
                },
                PluginCommand::AddNetEdge { src, dst, edge } => {
                    Log::create(LogType::Info, format!("Plugin `{}` add edge between `{}` and `{}` to network graph.", &identity, src, dst), &conn).unwrap();
                    let mut net_graph = net_graph_arc.lock().unwrap();
                    net_graph.add_edge(src.into(), dst.into(), edge);
                    
                    app_handle.emit("updateNetGraph", &net_graph.clone()).unwrap();
                    
                    None
                },
                PluginCommand::RemoveNetNode { node } => {
                    Log::create(LogType::Info, format!("Plugin `{}` remove node `{}` from network graph.", &identity, node), &conn).unwrap();
                    let mut net_graph = net_graph_arc.lock().unwrap();
                    net_graph.remove_node(node.into());
                    
                    app_handle.emit("updateNetGraph", &net_graph.clone()).unwrap();
                    
                    None
                },
                PluginCommand::RemoveNetEdge { edge } => {
                    Log::create(LogType::Info, format!("Plugin `{}` remove edge `{}` from network graph.", &identity, edge), &conn).unwrap();
                    let mut net_graph = net_graph_arc.lock().unwrap();
                    net_graph.remove_edge(edge.into());
                    
                    app_handle.emit("updateNetGraph", &net_graph.clone()).unwrap();
                    
                    None
                },
                PluginCommand::UpdateNetNode { index, node } => {
                    Log::create(LogType::Info, format!("Plugin `{}` update node `{}` of network graph.", &identity, index), &conn).unwrap();
                    let mut net_graph = net_graph_arc.lock().unwrap();
                    let mut_node = net_graph.node_weight_mut(index.into()).unwrap();
                    *mut_node = node;
                    
                    app_handle.emit("updateNetGraph", &net_graph.clone()).unwrap();
                    
                    None
                }
            };
            
            if let Some(s) = new_status {
                let mut active_plugins = active_plugins_arc.lock().unwrap();
                if active_plugins.contains_key(&identity) {
                    *active_plugins.get_mut(&identity).unwrap() = s;
                }
                else {
                    active_plugins.insert(identity, s);
                }
                app_handle.emit("active_plugins", active_plugins.clone()).unwrap();
            }
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
    pub language: PluginLanguage,
    pub permissions: String
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

#[tauri::command]
pub fn import_plugin_from_folder(app_handle: AppHandle, plugins: tauri::State<Arc<Mutex<HashMap<String, Plugin>>>>, path: String) {
    let plugins_arc_clone = Arc::clone(&plugins);
    let mut plugins = plugins_arc_clone.lock().unwrap();
    
    let path = PathBuf::from(path);
    let dir = path.file_name().unwrap();
    
    copy_dir_all(&path, app_handle.path().app_local_data_dir().unwrap().join("plugins").join(dir)).unwrap();
    
    for p in fs::read_dir(app_handle.path().app_local_data_dir().unwrap().join("plugins")).unwrap() {
        let cnt = fs::read_to_string(p.as_ref().unwrap().path().join("config.json")).unwrap();
        let config: PluginConfig = serde_json::from_str(&cnt).unwrap();
        
        plugins.insert(config.name.clone(), Plugin {
            path: p.as_ref().unwrap().path().to_str().unwrap().to_owned(),
            folder: p.unwrap().file_name().to_str().unwrap().to_owned(),
            config: config
        });
    }
}

#[tauri::command]
pub fn send_plugin_form_res(settings: tauri::State<Arc<Mutex<Settings>>>, plugin: String, params: String) {
    let settings_arc_clone = Arc::clone(&settings);
    let settings = settings_arc_clone.lock().unwrap();
    
    let data = serde_json::to_string(&PluginCommand::FormData {
        dst: plugin,
        data: params
    }).unwrap();
    
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::DEALER).unwrap();
    socket.set_identity("chomik".as_bytes());

    socket.connect(&format!("tcp://localhost:{}", settings.plugins_server_port)).unwrap();
    
    socket.send(data.as_bytes(), 0);
}

#[tauri::command]
pub fn run_plugin(app_handle: AppHandle, loaded_report: tauri::State<Arc<Mutex<Option<Report>>>>, settings: tauri::State<Arc<Mutex<Settings>>>, plugins: tauri::State<Arc<Mutex<HashMap<String, Plugin>>>>, active_plugins: tauri::State<Arc<Mutex<HashMap<String, PluginStatus>>>>, plugin_name: String) {
    let loaded_report_arc_clone = Arc::clone(&loaded_report);
    let loaded_report = loaded_report_arc_clone.lock().unwrap().clone();
    
    if loaded_report.is_none() {
        app_handle.emit("toast", &Toast { r#type: ToastType::Warning, text: "No report loaded. You must create and/or load a report first.".to_string() }).unwrap()
    }
    
    let settings_arc_clone = Arc::clone(&settings);
    let settings = settings_arc_clone.lock().unwrap().clone();
    
    let plugins_arc_clone = Arc::clone(&plugins);
    let plugins = plugins_arc_clone.lock().unwrap();
    let plugin = plugins.get(&plugin_name).unwrap().clone();
    
    let active_plugins_arc_clone = Arc::clone(&active_plugins);
    
    match plugin.config.language {
        PluginLanguage::Py => {
            match settings.python.clone() {
                Some(path) => {
                    thread::spawn(move || {                        
                        let mut cmd = StdCommand::new(path);
                        cmd.arg(Path::new(&plugin.path).join("src/main.py").to_str().unwrap());
                        cmd.arg("--port");
                        cmd.arg(settings.plugins_server_port.to_string());
                        cmd.arg("--report");
                        cmd.arg(&loaded_report.as_ref().unwrap().id);
                        
                        if Command::is_elevated() && plugin.config.permissions == "admin" || plugin.config.permissions == "user" {
                            match cmd.output() {
                                Ok(r) => {
                                    if r.status.success() {
                                        app_handle.emit("toast", &Toast { r#type: ToastType::Success, text: "Plugin successfully terminated.".to_string() }).unwrap()
                                    }
                                    else {
                                        let mut stderr = String::new();
                                        for line in r.stderr.lines() {
                                            if let Ok(line) = line {
                                                stderr.push_str(&line);
                                            }
                                        }
                                        app_handle.emit("toast", &Toast { r#type: ToastType::Danger, text: stderr }).unwrap()
                                    }
                                },
                                Err(err) => app_handle.emit("toast", &Toast { r#type: ToastType::Danger, text: err.to_string() }).unwrap()
                            }
                        }
                        else {
                            match Command::new(cmd).output() {
                                Ok(r) => {
                                    if r.status.success() {
                                        app_handle.emit("toast", &Toast { r#type: ToastType::Success, text: "Plugin successfully terminated.".to_string() }).unwrap()
                                    }
                                    else {
                                        let mut stderr = String::new();
                                        for line in r.stderr.lines() {
                                            if let Ok(line) = line {
                                                stderr.push_str(&line);
                                            }
                                        }
                                        app_handle.emit("toast", &Toast { r#type: ToastType::Danger, text: stderr }).unwrap()
                                    }
                                },
                                Err(err) => app_handle.emit("toast", &Toast { r#type: ToastType::Danger, text: err.to_string() }).unwrap()
                            }
                        }
                        
                        let mut active_plugins = active_plugins_arc_clone.lock().unwrap();
                        active_plugins.remove(&plugin_name);
                    });
                },
                None => app_handle.emit("toast", &Toast {
                    r#type: ToastType::Danger,
                    text: "Python interpreter path has not been configured.".to_string()
                }).unwrap()
            }
        },
        PluginLanguage::Js => {},
        PluginLanguage::Lua => {}
    };
}

#[tauri::command]
pub fn terminate_plugin(settings: tauri::State<Arc<Mutex<Settings>>>, plugin: String) {
    let settings_arc_clone = Arc::clone(&settings);
    let settings = settings_arc_clone.lock().unwrap();
    
    let data = serde_json::to_string(&PluginCommand::Terminate { plugin }).unwrap();
    
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::DEALER).unwrap();
    socket.set_identity("chomik".as_bytes());

    socket.connect(&format!("tcp://localhost:{}", settings.plugins_server_port)).unwrap();
    
    socket.send(data.as_bytes(), 0);
}

#[tauri::command]
pub fn get_active_plugins(active_plugins: tauri::State<Arc<Mutex<HashMap<String, PluginStatus>>>>) -> HashMap<String, PluginStatus> {
    let active_plugins_arc_clone = Arc::clone(&active_plugins);
    let active_plugins = active_plugins_arc_clone.lock().unwrap();
    
    active_plugins.clone()
}