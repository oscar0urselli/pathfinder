mod report;
mod arp_scan;
mod dns;

use std::fs;
use std::sync::{Arc, Mutex};
use tauri::Manager;

use duckdb::{self, params};


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            report::get_loaded_report,
            report::new_report,
            report::list_reports,
            report::load_report,
            arp_scan::arp_scan_info,
            arp_scan::arp_scan,
            arp_scan::get_arp_scans,
            arp_scan::get_arps,
            dns::dns_query,
            dns::get_dns_queries
        ])
        .setup(|app| {
            if !app.path().app_local_data_dir().unwrap().join("duckdb").exists() {
                let _ = fs::create_dir(app.path().app_local_data_dir().unwrap().join("duckdb"));
            }
            
            let conn = duckdb::Connection::open(app.path().app_local_data_dir().unwrap().join("duckdb").join("pathfinder.ddb")).unwrap();

            match conn.execute("CREATE TABLE IF NOT EXISTS reports (id UUID PRIMARY KEY, last_access_tsz TIMESTAMPTZ, title STRING, place STRING, author STRING, device STRING, version STRING);", params![]) {
                Ok(_) => println!("'reports' table created."),
                Err(err) => println!("Table 'reports' not created: {}", err)
            };
            
            let _ = conn.execute("CREATE TABLE IF NOT EXISTS arp_scans (id UUID PRIMARY KEY, report UUID, arp_count UINT64, duration_ms UINT64, packet_count UINT64, interface STRING, network STRING, timeout UINT64, interval UINT64, retry UINT64, src_ip STRING, src_mac STRING, dst_mac STRING, vlan_id UINT16);", params![]);
            
            let _ = conn.execute("CREATE SEQUENCE id_sequence_arp START 1;", []);
            let _ = conn.execute("CREATE TABLE IF NOT EXISTS arp (id UINT64 PRIMARY KEY DEFAULT nextval('id_sequence_arp'), ipv4 STRING, mac STRING, hostname STRING, vendor STRING, scan UUID);", params![]);
            
            let _ = conn.execute("CREATE TABLE IF NOT EXISTS dns (id UUID PRIMARY KEY, report UUID, host STRING, port UINT16, protocol STRING, domain STRING, records STRING);", []);
            
            app.manage(Arc::new(Mutex::new(conn)));
            
            let loaded_report: Option<report::Report> = None; 
            app.manage(Arc::new(Mutex::new(loaded_report)));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
