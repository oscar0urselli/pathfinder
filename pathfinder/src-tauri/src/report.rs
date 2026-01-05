use std::sync::{Arc, Mutex};

use duckdb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewReport {
    title: String,
    author: String,
    device: String,
    place: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Report {
    pub id: String,
    pub last_access_tsz: u64,
    pub title: String,
    pub author: String,
    pub device: String,
    pub place: String,
    pub version: String,
}

#[tauri::command]
pub fn get_loaded_report(
    loaded_report: tauri::State<Arc<Mutex<Option<Report>>>>,
) -> Option<Report> {
    let loaded_report_arc_clone = Arc::clone(&loaded_report);
    let loaded_report = loaded_report_arc_clone.lock().unwrap();

    loaded_report.clone()
}

#[tauri::command]
pub fn new_report(
    conn: tauri::State<Arc<Mutex<duckdb::Connection>>>,
    loaded_report: tauri::State<Arc<Mutex<Option<Report>>>>,
    report: NewReport,
) -> Result<Report, String> {
    let conn_arc_clone = Arc::clone(&conn);
    let conn = conn_arc_clone.lock().unwrap().try_clone().unwrap();

    let loaded_report_arc_clone = Arc::clone(&loaded_report);
    let mut loaded_report = loaded_report_arc_clone.lock().unwrap();

    let mut stmt = conn.prepare("INSERT INTO reports (id, last_access_tsz, title, author, device, place, version) VALUES (uuidv7(), now(), ?, ?, ?, ?, ?) RETURNING *;").unwrap();
    let data = stmt.query_one(
        [
            report.title,
            report.author,
            report.place,
            report.device,
            "0".to_string(),
        ],
        |row| {
            Ok(Report {
                id: row.get(0).unwrap(),
                last_access_tsz: row.get(1).unwrap(),
                title: row.get(2).unwrap(),
                author: row.get(3).unwrap(),
                device: row.get(4).unwrap(),
                place: row.get(5).unwrap(),
                version: row.get(6).unwrap(),
            })
        },
    );

    match data {
        Ok(d) => {
            *loaded_report = Some(d.clone());
            Ok(d)
        }
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn list_reports(
    conn: tauri::State<Arc<Mutex<duckdb::Connection>>>,
) -> Result<Vec<Report>, String> {
    let conn_arc_clone = Arc::clone(&conn);
    let conn = conn_arc_clone.lock().unwrap().try_clone().unwrap();

    let mut stmt = conn.prepare("SELECT * FROM reports;").unwrap();
    let data = stmt.query_map([], |row| {
        Ok(Report {
            id: row.get(0).unwrap(),
            last_access_tsz: row.get(1).unwrap(),
            title: row.get(2).unwrap(),
            author: row.get(3).unwrap(),
            device: row.get(4).unwrap(),
            place: row.get(5).unwrap(),
            version: row.get(6).unwrap(),
        })
    });

    match data {
        Ok(d) => {
            let mut v: Vec<Report> = Vec::new();
            for i in d {
                v.push(i.unwrap());
            }
            Ok(v)
        }
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn load_report(
    conn: tauri::State<Arc<Mutex<duckdb::Connection>>>,
    loaded_report: tauri::State<Arc<Mutex<Option<Report>>>>,
    id: String,
) -> Result<Report, String> {
    let conn_arc_clone = Arc::clone(&conn);
    let conn = conn_arc_clone.lock().unwrap().try_clone().unwrap();

    let loaded_report_arc_clone = Arc::clone(&loaded_report);
    let mut loaded_report = loaded_report_arc_clone.lock().unwrap();

    let mut stmt = conn
        .prepare("UPDATE reports SET last_access_tsz = now() WHERE id = ? RETURNING *;")
        .unwrap();
    let data = stmt.query_one([id], |row| {
        Ok(Report {
            id: row.get(0).unwrap(),
            last_access_tsz: row.get(1).unwrap(),
            title: row.get(2).unwrap(),
            author: row.get(3).unwrap(),
            device: row.get(4).unwrap(),
            place: row.get(5).unwrap(),
            version: row.get(6).unwrap(),
        })
    });

    match data {
        Ok(d) => {
            *loaded_report = Some(d.clone());
            Ok(d)
        }
        Err(err) => Err(err.to_string()),
    }
}
