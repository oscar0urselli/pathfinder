use std::sync::{Arc, Mutex};
use std::fmt;

use duckdb::{self, params};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Log {
    pub id: u64,
    pub ts: u64,
    pub r#type: LogType,
    pub message: String
}

#[derive(Serialize, Deserialize)]
pub enum LogType {
    Error,
    Warn,
    Info,
    Debug,
    Trace
}

impl fmt::Display for LogType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Error => "Error",
            Self::Warn => "Warn",
            Self::Info => "Info",
            Self::Debug => "Debug",
            Self::Trace => "Trace"
        })?;
        Ok(())
    }
}

impl duckdb::types::FromSql for LogType {
    fn column_result(value: duckdb::types::ValueRef<'_>) -> duckdb::types::FromSqlResult<Self> {
        match value {
            duckdb::types::ValueRef::Text(s) => match str::from_utf8(s).unwrap() {
                "Error" => Ok(Self::Error),
                "Warn" => Ok(Self::Warn),
                "Info" => Ok(Self::Info),
                "Debug" => Ok(Self::Debug),
                "Trace" => Ok(Self::Trace),
                _ => Err(duckdb::types::FromSqlError::InvalidType)
            },
            _ => Err(duckdb::types::FromSqlError::InvalidType)
        }
    }
}

impl Log {    
    pub fn create(r#type: LogType, message: String, db: &duckdb::Connection) -> Result<usize, ()> {
        match db.execute("INSERT INTO logs (ts, type, message) VALUES (now(), ?, ?);", params![r#type.to_string(), message]) {
            Ok(v) => Ok(v),
            Err(_) => Err(())
        }
    }
}

#[tauri::command]
pub fn get_logs(conn: tauri::State<Arc<Mutex<duckdb::Connection>>>, time_frame: u64, types: Vec<String>) -> Result<Vec<Log>, String> {
    let conn_arc_clone = Arc::clone(&conn);
    let conn = conn_arc_clone.lock().unwrap().try_clone().unwrap();
    
    let sql = format!("SELECT * FROM logs WHERE type IN {:?} AND CAST(ts AS TIMESTAMP) > CAST(now() AS TIMESTAMP) - INTERVAL {} MINUTES;", types, time_frame).replace("\"", "'");
    let mut stmt = conn.prepare(sql.as_str()).unwrap();
    let data = stmt.query_map([], |row| {
        Ok(Log {
            id: row.get(0).unwrap(),
            ts: row.get(1).unwrap(),
            r#type: row.get(2).unwrap(),
            message: row.get(3).unwrap()
        })
    });
    
    match data {
        Ok(d) => {
            let mut v = Vec::new();
            for i in d {
                v.push(i.unwrap());
            }
            Ok(v)
        }
        Err(err) => Err(err.to_string())
    }
}