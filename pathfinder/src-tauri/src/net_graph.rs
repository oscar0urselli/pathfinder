use std::{collections::HashMap, net::IpAddr, sync::{Arc, Mutex}};

use petgraph::graph::UnGraph;
use serde::{Deserialize, Serialize};

use crate::utils::{IpCidr, MacAddr};


#[derive(Serialize, Deserialize, Clone)]
pub enum NetNodeType {
    Unknown,
    Switch,
    Router,
    Server,
    Database,
    Pc
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NetNodeInterface {
    pub mac: MacAddr,
    pub ips: Vec<IpCidr>
}

#[derive(Serialize, Deserialize, Clone)]
pub enum NetNodeServiceBindingProtocol {
    Tcp,
    Udp
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NetNodeServiceBinding {
    pub ip: IpAddr,
    pub port: u16,
    pub protocol: NetNodeServiceBindingProtocol
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NetNodeService {
    pub name: String,
    pub bindings: Vec<NetNodeServiceBinding>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NetNode {
    pub name: String,
    pub r#type: NetNodeType,
    pub interfaces: HashMap<String, NetNodeInterface>,
    pub services: Vec<NetNodeService>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NetEdge {
    pub a_index: u32,
    pub a_node: String,
    pub a_interface: Option<String>,
    pub b_index: u32,
    pub b_node: String,
    pub b_interface: Option<String>
}


#[tauri::command]
pub fn get_net_graph(net_graph: tauri::State<Arc<Mutex<UnGraph<NetNode, NetEdge>>>>) -> UnGraph<NetNode, NetEdge> {
    let net_graph_arc_clone = Arc::clone(&net_graph);
    let net_graph = net_graph_arc_clone.lock().unwrap();
    
    net_graph.clone()
}

#[tauri::command]
pub fn add_net_node(net_graph: tauri::State<Arc<Mutex<UnGraph<NetNode, NetEdge>>>>, node: NetNode) {
    let net_graph_arc_clone = Arc::clone(&net_graph);
    let mut net_graph = net_graph_arc_clone.lock().unwrap();
    
    net_graph.add_node(node);
}

#[tauri::command]
pub fn add_net_edge(net_graph: tauri::State<Arc<Mutex<UnGraph<NetNode, NetEdge>>>>, src: u32, dst: u32, edge: NetEdge) {
    let net_graph_arc_clone = Arc::clone(&net_graph);
    let mut net_graph = net_graph_arc_clone.lock().unwrap();
    
    net_graph.add_edge(src.into(), dst.into(), edge);
}

#[tauri::command]
pub fn remove_net_node(net_graph: tauri::State<Arc<Mutex<UnGraph<NetNode, NetEdge>>>>, node: u32) {
    let net_graph_arc_clone = Arc::clone(&net_graph);
    let mut net_graph = net_graph_arc_clone.lock().unwrap();
    
    net_graph.remove_node(node.into());
}

#[tauri::command]
pub fn remove_net_edge(net_graph: tauri::State<Arc<Mutex<UnGraph<NetNode, NetEdge>>>>, edge: u32) {
    let net_graph_arc_clone = Arc::clone(&net_graph);
    let mut net_graph = net_graph_arc_clone.lock().unwrap();
    
    net_graph.remove_edge(edge.into());
}

#[tauri::command]
pub fn edit_net_node(net_graph: tauri::State<Arc<Mutex<UnGraph<NetNode, NetEdge>>>>, index: u32, node: NetNode) {
    let net_graph_arc_clone = Arc::clone(&net_graph);
    let mut net_graph = net_graph_arc_clone.lock().unwrap();
    
    let mut_node = net_graph.node_weight_mut(index.into()).unwrap();
    *mut_node = node;
}