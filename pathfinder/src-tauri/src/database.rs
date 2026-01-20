use std::{collections::HashMap, sync::{Arc, Mutex}};

use duckdb::{Connection, Row};
use serde::Serialize;
use serde_json::{Map, Value};
use tauri::{AppHandle, Emitter};

use crate::utils::Toast;


fn column_to_value(row: &Row, i: usize) -> Value {
    if let Ok(opt_i64) = row.get::<usize, Option<i64>>(i) {
        return opt_i64
            .map(|v| Value::Number(v.into()))
            .unwrap_or(Value::Null);
    }
    
    if let Ok(opt_f64) = row.get::<usize, Option<f64>>(i) {
        return opt_f64
            .map(|v| Value::Number(serde_json::Number::from_f64(v).unwrap()))
            .unwrap_or(Value::Null);
    }
    
    if let Ok(opt_bool) = row.get::<usize, Option<bool>>(i) {
        return opt_bool
            .map(|v| Value::Bool(v.into()))
            .unwrap_or(Value::Null);
    }
    
    if let Ok(opt_str) = row.get::<usize, Option<String>>(i) {
        return opt_str
            .map(|v| Value::String(v))
            .unwrap_or(Value::Null);
    }
    
    Value::Null
}

pub fn query_to_json(conn: &Connection, sql: &str) -> Result<Value, String> {
    let mut stmt = match conn.prepare(sql) {
        Ok(s) => s,
        Err(err) => return Err(err.to_string())
    };
    
    let mut rows = stmt.query([]).unwrap();
    let column_count = rows.as_ref().unwrap().column_count();
    let column_names = rows.as_ref().unwrap().column_names();
    
    let mut out_rows = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let mut map = Map::new();
        for i in 0..column_count {
            let json_val = column_to_value(&row, i);
            map.insert(column_names[i].clone(), json_val);
        }
        out_rows.push(Value::Object(map));
    }
    
    Ok(Value::Array(out_rows))
}

#[tauri::command]
pub fn get_table(app_handle: AppHandle, conn: tauri::State<Arc<Mutex<Connection>>>, table: String) -> Option<Value> {
    let conn_arc_clone = Arc::clone(&conn);
    let conn = conn_arc_clone.lock().unwrap().try_clone().unwrap();
    
    match query_to_json(&conn, format!("SELECT * FROM {};", table).as_str()) {
        Ok(data) => Some(data),
        Err(err) => {
            app_handle.emit("toast", &Toast {
                alert_type: "danger".to_string(),
                text: err
            }).unwrap();
            None
        }
    }
}

#[derive(Serialize, Clone)]
pub struct SqlTableColumn {
    pub table: String,
    pub name: String,
    pub r#type: String
}

#[tauri::command]
pub fn get_tables_list(conn: tauri::State<Arc<Mutex<Connection>>>) -> HashMap<String, Vec<SqlTableColumn>> {
    let conn_arc_clone = Arc::clone(&conn);
    let conn = conn_arc_clone.lock().unwrap().try_clone().unwrap();
    
    let mut stmt = conn.prepare("SELECT table_name, column_name, data_type FROM information_schema.columns WHERE table_schema NOT IN ('information_schema', 'pg_catalog');").unwrap();
    
    let data = stmt.query_map([], |row| {        
        Ok(SqlTableColumn{
            table: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            r#type: row.get(2).unwrap()
        })
    }).unwrap();
    
    let mut tables = HashMap::new();
    for r in data {
        if !tables.contains_key(&r.as_ref().unwrap().table) {
            tables.insert(r.as_ref().unwrap().table.clone(), Vec::new());
        }
        
        tables.get_mut(&r.as_ref().unwrap().table).unwrap().push(r.unwrap());
    }
    
    tables.clone()
}