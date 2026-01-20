use std::{collections::HashMap, io::BufRead, path::Path, process::Command as StdCommand, sync::{Arc, Mutex}, thread};

use elevated_command::Command;
use petgraph::graph::UnGraph;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

use crate::{report::Report, settings::Settings, utils::Toast};

#[derive(Serialize, Deserialize)]
pub enum PluginCommand {
    Register,
    ToastReq { alert_type: u8, text: String },
    ExecuteRawQueryReq { query: String },
    FormReq { data: PluginFormData },
    FormRes { dst: String, data: String },
    Exit,
    Terminate { plugin: String },
    GetNetGraph,
    NetGraph { graph: UnGraph<NetNode, ()> },
    AddNetNode { node: NetNode },
    AddNetEdge { src: u32, dst: u32 },
    RemoveNetNode { node: u32 },
    RemoveNetEdge { edge: u32 },
    UpdateNetNode { index: u32, node: NetNode }
}

#[derive(Serialize, Deserialize)]
pub struct PluginFormData {
    name: String,
    config: Vec<Vec<PluginFormField>>
}

#[derive(Serialize, Deserialize)]
pub struct PluginFormField {
    name: String,
    title: String,
    r#type: String,
    options: Option<Vec<String>>,
    min: Option<String>,
    max: Option<String>,
    step: Option<String>,
    regex: Option<String>,
    default: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PluginStatus {
    Running,
    WaitingForm,
    Exiting
}

pub fn init_plugins_server(app_handle: AppHandle, conn: duckdb::Connection, port: u16, active_plugins_arc: Arc<Mutex<HashMap<String, PluginStatus>>>, net_graph_arc: Arc<Mutex<UnGraph<NetNode, ()>>>) {
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
                    Some(PluginStatus::Running)
                },
                PluginCommand::ToastReq { alert_type, text } => {
                    app_handle.emit(
                        "toast",
                        &Toast {
                            alert_type: Toast::alert_type_to_string(alert_type).unwrap(),
                            text,
                        },
                    ).unwrap();
                    None
                },
                PluginCommand::ExecuteRawQueryReq { query } => {
                    match conn.execute(&query, []) {
                        Ok(_) => {},
                        Err(err) => app_handle.emit(
                            "toast",
                            &Toast {
                                alert_type: "danger".to_string(),
                                text: err.to_string(),
                            }
                        ).unwrap()
                    };
                    None
                },
                PluginCommand::FormReq { data } => {
                    app_handle.emit("form", &data).unwrap();
                    Some(PluginStatus::WaitingForm)
                },
                PluginCommand::FormRes { dst, data } => {
                    socket.send(dst.as_bytes(), zmq::SNDMORE);
                    socket.send(&message, 0);
                    Some(PluginStatus::Running)
                },
                PluginCommand::Exit => {
                    app_handle.emit(
                        "toast",
                        &Toast {
                            alert_type: "warning".to_string(),
                            text: "Plugin terminated".to_string()
                        }
                    ).unwrap();
                    Some(PluginStatus::Exiting)
                },
                PluginCommand::Terminate { plugin } => {
                    socket.send(plugin.as_bytes(), zmq::SNDMORE);
                    socket.send(&message, 0);
                    None
                },
                PluginCommand::GetNetGraph => {
                    let net_graph = net_graph_arc.lock().unwrap();
                    
                    socket.send(identity.as_bytes(), zmq::SNDMORE);
                    socket.send(serde_json::to_string(&PluginCommand::NetGraph {
                        graph: net_graph.clone()
                    }).unwrap().as_bytes(), 0);
                    
                    None
                },
                PluginCommand::NetGraph { graph } => { None },
                PluginCommand::AddNetNode { node } => {
                    let mut net_graph = net_graph_arc.lock().unwrap();
                    net_graph.add_node(node);
                    
                    app_handle.emit("updateNetGraph", &net_graph.clone()).unwrap();
                    
                    None 
                },
                PluginCommand::AddNetEdge { src, dst } => {
                    let mut net_graph = net_graph_arc.lock().unwrap();
                    net_graph.add_edge(src.into(), dst.into(), ());
                    
                    app_handle.emit("updateNetGraph", &net_graph.clone()).unwrap();
                    
                    None
                },
                PluginCommand::RemoveNetNode { node } => {
                    let mut net_graph = net_graph_arc.lock().unwrap();
                    net_graph.remove_node(node.into());
                    
                    app_handle.emit("updateNetGraph", &net_graph.clone()).unwrap();
                    
                    None
                },
                PluginCommand::RemoveNetEdge { edge } => {
                    let mut net_graph = net_graph_arc.lock().unwrap();
                    net_graph.remove_edge(edge.into());
                    
                    app_handle.emit("updateNetGraph", &net_graph.clone()).unwrap();
                    
                    None
                },
                PluginCommand::UpdateNetNode { index, node } => {
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
                app_handle.emit(
                    "active_plugins",
                    active_plugins.clone()
                ).unwrap();
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
pub fn send_plugin_form_res(app_handle: AppHandle, settings: tauri::State<Arc<Mutex<Settings>>>, plugin: String, params: String) {
    let settings_arc_clone = Arc::clone(&settings);
    let settings = settings_arc_clone.lock().unwrap();
    
    let data = serde_json::to_string(&PluginCommand::FormRes {
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
        app_handle.emit("toast", &Toast { alert_type: "warning".to_string(), text: "No report loaded. You must create and/or load a report first.".to_string() }).unwrap()
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
                        }
                        else {
                            match Command::new(cmd).output() {
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
                        }
                        
                        let mut active_plugins = active_plugins_arc_clone.lock().unwrap();
                        active_plugins.remove(&plugin_name);
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

#[derive(Serialize, Deserialize, Clone)]
pub struct NetNode {
    pub name: String,
    pub r#type: NetNodeType,
    pub interfaces: HashMap<String, NetNodeInterface>,
    pub services: Vec<NetNodeService>
}

#[derive(Serialize, Deserialize, Clone)]
pub enum NetNodeType {
    Unknown,
    Switch,
    Router,
    Server,
    Pc
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NetNodeInterface {
    pub mac: String,
    pub ips: Vec<String>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NetNodeService {
    pub ip: String,
    pub name: String,
    pub port: u16,
    pub transport_protocol: String
}

#[tauri::command]
pub fn get_net_graph(net_graph: tauri::State<Arc<Mutex<UnGraph<NetNode, ()>>>>) -> UnGraph<NetNode, ()> {
    let net_graph_arc_clone = Arc::clone(&net_graph);
    let net_graph = net_graph_arc_clone.lock().unwrap();
    
    net_graph.clone()
}

#[tauri::command]
pub fn add_net_node(net_graph: tauri::State<Arc<Mutex<UnGraph<NetNode, ()>>>>, node: NetNode) {
    let net_graph_arc_clone = Arc::clone(&net_graph);
    let mut net_graph = net_graph_arc_clone.lock().unwrap();
    
    net_graph.add_node(node);
}

#[tauri::command]
pub fn add_net_edge(net_graph: tauri::State<Arc<Mutex<UnGraph<NetNode, ()>>>>, src: u32, dst: u32) {
    let net_graph_arc_clone = Arc::clone(&net_graph);
    let mut net_graph = net_graph_arc_clone.lock().unwrap();
    
    net_graph.add_edge(src.into(), dst.into(), ());
}

#[tauri::command]
pub fn remove_net_node(net_graph: tauri::State<Arc<Mutex<UnGraph<NetNode, ()>>>>, node: u32) {
    let net_graph_arc_clone = Arc::clone(&net_graph);
    let mut net_graph = net_graph_arc_clone.lock().unwrap();
    
    net_graph.remove_node(node.into());
}

#[tauri::command]
pub fn remove_net_edge(net_graph: tauri::State<Arc<Mutex<UnGraph<NetNode, ()>>>>, edge: u32) {
    let net_graph_arc_clone = Arc::clone(&net_graph);
    let mut net_graph = net_graph_arc_clone.lock().unwrap();
    
    net_graph.remove_edge(edge.into());
}

#[tauri::command]
pub fn edit_net_node(net_graph: tauri::State<Arc<Mutex<UnGraph<NetNode, ()>>>>, index: u32, node: NetNode) {
    let net_graph_arc_clone = Arc::clone(&net_graph);
    let mut net_graph = net_graph_arc_clone.lock().unwrap();
    
    let mut_node = net_graph.node_weight_mut(index.into()).unwrap();
    *mut_node = node;
}