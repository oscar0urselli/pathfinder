use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use duckdb;


#[derive(Serialize, Deserialize)]
pub struct NewReport {
    title: String,
    author: String,
    device: String,
    place: String
}

#[derive(Serialize, Deserialize)]
pub struct Report {
    id: String,
    last_access_tsz: u64,
    title: String,
    author: String,
    device: String,
    place: String,
    version: String
}

#[tauri::command]
pub fn new_report(conn: tauri::State<Arc<Mutex<duckdb::Connection>>>, report: NewReport) -> Result<Report, String> {
    let conn_arc_clone = Arc::clone(&conn);
    let conn = conn_arc_clone.lock().unwrap().try_clone().unwrap();
    
    let mut stmt = conn.prepare("INSERT INTO reports (id, last_access_tsz, title, author, device, place, version) VALUES (uuidv7(), now(), ?, ?, ?, ?, ?) RETURNING *;").unwrap();
    let data = stmt.query_one([
        report.title,
        report.author,
        report.place,
        report.device,
        "0".to_string()
    ], |row| {
        Ok(Report {
            id: row.get(0).unwrap(),
            last_access_tsz: row.get(1).unwrap(),
            title: row.get(2).unwrap(),
            author: row.get(3).unwrap(),
            device: row.get(4).unwrap(),
            place: row.get(5).unwrap(),
            version: row.get(6).unwrap()
        })
    });
    
    match data {
        Ok(d) => Ok(d),
        Err(err) => Err(err.to_string())
    }
}

#[tauri::command]
pub fn list_reports(conn: tauri::State<Arc<Mutex<duckdb::Connection>>>) -> Result<Vec<Report>, String> {
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
            version: row.get(6).unwrap()
        })
    });
    
    match data {
        Ok(d) => {
            let mut v: Vec<Report> = Vec::new();
            for i in d {
                v.push(i.unwrap());
            }
            Ok(v)
        },
        Err(err) => Err(err.to_string())
    }
}

#[tauri::command]
pub fn update_access_tsz_report(conn: tauri::State<Arc<Mutex<duckdb::Connection>>>, id: String) -> Result<u64, String> {
    let conn_arc_clone = Arc::clone(&conn);
    let conn = conn_arc_clone.lock().unwrap().try_clone().unwrap();
    
    let mut stmt = conn.prepare("UPDATE reports SET last_access_tsz = now() WHERE id = ? RETURNING last_access_tsz;").unwrap();
    let data = stmt.query_one([id], |row| {
        Ok(row.get(0).unwrap())
    });
    
    match data {
        Ok(d) => Ok(d),
        Err(err) => Err(err.to_string())
    }
}