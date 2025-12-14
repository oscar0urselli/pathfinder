mod arp_scan;

use std::fs;
use std::sync::{Arc, Mutex};
use tauri::Manager;

use duckdb::{self, params};


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![arp_scan::arp_scan_info, arp_scan::arp_scan])
        .setup(|app| {
            if !app.path().app_local_data_dir().unwrap().join("duckdb").exists() {
                let _ = fs::create_dir(app.path().app_local_data_dir().unwrap().join("duckdb"));
            }
            
            let conn = duckdb::Connection::open(app.path().app_local_data_dir().unwrap().join("duckdb").join("pathfinder.ddb")).unwrap();

            let _ = conn.execute("CREATE TABLE IF NOT EXISTS reports (id UUID PRIMARY KEY, last_access_datetime DATETIME, place STRING, author STRING, device STRING, version STRING);", params![]);
            
            let _ = conn.execute("CREATE TABLE IF NOT EXISTS arp_scans (id UUID PRIMARY KEY, report UUID, arp_count UINT64, duration_ms UINT128, packet_count UINT64, interface STRING, network STRING, timeout UINT64, interval UINT64, retry UINT64, src_ip STRING, src_mac STRING, dst_mac STRING, vlan_id UINT16);", params![]);
            let _ = conn.execute("CREATE TABLE IF NOT EXISTS arp (id UINT64 PRIMARY KEY, ipv4 STRING, mac STRING, hostname STRING, vendor STRING, scan UUID);", params![]);
            
            app.manage(Arc::new(Mutex::new(conn)));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
