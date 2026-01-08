mod arp;
mod plugin;
mod report;
mod database;
mod settings;
mod utils;

use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};
use tauri::Manager;

use duckdb::{self, params};

use crate::plugin::{Plugin, PluginConfig};
use crate::settings::Settings;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            report::get_loaded_report,
            report::new_report,
            report::list_reports,
            report::load_report,
            plugin::run_plugin,
            plugin::get_plugins,
            plugin::send_plugin_form_res,
            settings::get_settings,
            settings::set_notifications_pos,
            settings::set_plugins_server_port,
            settings::set_python_interpreter,
            settings::set_node_js_interpreter,
            settings::set_lua_interpreter,
            database::get_table,
            database::get_tables_list,
            arp::arp_scan_info,
            arp::arp_scan,
            arp::get_arp_scans
        ])
        .setup(|app| {
            if !app.path().app_local_data_dir().unwrap().join("plugins").exists() {
                let _ = fs::create_dir(app.path().app_local_data_dir().unwrap().join("plugins"));
            }
            
            if !app.path().app_local_data_dir().unwrap().join("duckdb").exists() {
                let _ = fs::create_dir(app.path().app_local_data_dir().unwrap().join("duckdb"));
            }
            
            let conn = duckdb::Connection::open(app.path().app_local_data_dir().unwrap().join("duckdb").join("pathfinder.ddb")).unwrap();

            match conn.execute("CREATE TABLE IF NOT EXISTS reports (id UUID PRIMARY KEY, last_access_tsz TIMESTAMPTZ, title STRING, place STRING, author STRING, device STRING, version STRING);", params![]) {
                Ok(_) => println!("'reports' table created."),
                Err(err) => println!("Table 'reports' not created: {}", err)
            };
            
            let loaded_report: Option<report::Report> = None; 
            app.manage(Arc::new(Mutex::new(loaded_report)));
            
            let settings = Settings::load(app.path().app_local_data_dir().unwrap());
            
            plugin::init_plugins_server(app.app_handle().clone(), conn.try_clone().unwrap(), settings.plugins_server_port);
            
            app.manage(Arc::new(Mutex::new(conn)));
            app.manage(Arc::new(Mutex::new(settings)));
            
            let mut plugins: HashMap<String, Plugin> = HashMap::new();
            for p in fs::read_dir(app.app_handle().path().app_local_data_dir().unwrap().join("plugins")).unwrap() {
                let cnt = fs::read_to_string(p.as_ref().unwrap().path().join("config.json")).unwrap();
                let config: PluginConfig = serde_json::from_str(&cnt).unwrap();
                
                plugins.insert(config.name.clone(), Plugin {
                    path: p.as_ref().unwrap().path().to_str().unwrap().to_owned(),
                    folder: p.unwrap().file_name().to_str().unwrap().to_owned(),
                    config: config
                });
            }
            app.manage(Arc::new(Mutex::new(plugins)));
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
