use std::{net::{IpAddr, SocketAddr}, str::FromStr, sync::{Arc, Mutex}, thread};

use duckdb::{self, params, types::{FromSql, FromSqlError, ValueRef}};
use hickory_client::{client::{Client, ClientHandle}, proto::{rr::{DNSClass, Name, RecordType}, runtime::TokioRuntimeProvider, tcp::TcpClientStream}};
use serde::{Deserialize, Serialize};
use crate::report::{Report};


#[derive(Serialize, Deserialize)]
pub struct DnsQuerySettings {
    host: String,
    port: u16,
    protocol: String,
    domain: String,
    a: bool,
    aaaa: bool,
    caa: bool,
    cname: bool,
    ptr: bool,
    alias: bool,
    mx: bool,
    ns: bool,
    srv: bool,
    txt: bool,
    hinfo: bool
}

#[derive(Serialize, Deserialize)]
pub struct DnsRecord {
    name: String,
    rtype: String,
    class: String,
    ttl: u32,
    data: String
}

impl DnsRecord {
    pub fn to_sql_string(&self) -> String {
        format!("{{'name': '{}', 'rtype': '{}', 'class': '{}', 'ttl': {}, 'data': '{}'}}", self.name, self.rtype, self.class, self.ttl, self.data)
    }
}

#[derive(Serialize, Deserialize)]
pub struct DnsQuery {
    id: String,
    report: String,
    host: String,
    port: u16,
    protocol: String,
    domain: String,
    records: Option<Vec<DnsRecord>>
}


/// **TODO**: Find a nicer way to insert a list of structs into DuckDB. At the moment String concatenation with raw query is used.
#[tauri::command]
pub fn dns_query(conn: tauri::State<Arc<Mutex<duckdb::Connection>>>, loaded_report: tauri::State<Arc<Mutex<Option<Report>>>>, settings: DnsQuerySettings) -> Result<(), String> {
    let conn_arc_clone = Arc::clone(&conn);
    let conn = conn_arc_clone.lock().unwrap().try_clone().unwrap();
    
    let loaded_report_arc_clone = Arc::clone(&loaded_report);
    let loaded_report = loaded_report_arc_clone.lock().unwrap().clone();
    
    if loaded_report.is_none() {
        return Err("No report loaded. You must create and/or load a report first.".to_string());
    }
    
    thread::spawn(move || {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let (stream, sender) = TcpClientStream::new(SocketAddr::new(settings.host.parse::<IpAddr>().unwrap(), settings.port), None, None, TokioRuntimeProvider::new());
            
            let client = Client::new(stream, sender, None);
            
            let (mut client, bg) = client.await.expect("Connection failed.");
            
            tokio::spawn(bg);
            
            let mut record_types: Vec<RecordType> = Vec::new();
            if settings.a {
                record_types.push(RecordType::A);
            }
            if settings.aaaa {
                record_types.push(RecordType::AAAA);
            }
            if settings.caa {
                record_types.push(RecordType::CAA);
            }
            if settings.cname {
                record_types.push(RecordType::CNAME);
            }
            if settings.alias {
                record_types.push(RecordType::ANAME);
            }
            if settings.ptr {
                record_types.push(RecordType::PTR);
            }
            if settings.mx {
                record_types.push(RecordType::MX);
            }
            if settings.ns {
                record_types.push(RecordType::NS);
            }
            if settings.srv {
                record_types.push(RecordType::SRV);
            }
            if settings.txt {
                record_types.push(RecordType::TXT);
            }
            if settings.hinfo {
                record_types.push(RecordType::HINFO);
            }
            
            let mut stmt = conn.prepare("INSERT INTO dns (id, report, host, port, protocol, domain) VALUES (uuidv7(), ?, ?, ?, ?, ?) RETURNING id;").unwrap();
            let dns_query_id = stmt.query_one(params![
                loaded_report.as_ref().unwrap().id,
                settings.host,
                settings.port,
                settings.protocol,
                settings.domain
            ], |row| {
                Ok(row.get::<usize, String>(0).unwrap())
            }).unwrap();
            
            let mut records: Vec<DnsRecord> = Vec::new();
            for record in record_types {
                let query = client.query(Name::from_str(&settings.domain).unwrap(), DNSClass::IN, record);
                
                let response = query.await.unwrap();
                
                for rr in response.answers() {
                    records.push(DnsRecord {
                        name: rr.name().to_string(),
                        rtype: rr.record_type().to_string(),
                        class: "IN".to_string(),
                        ttl: rr.ttl(),
                        data: rr.data().to_string() 
                    });
                }
            }
            let records_str = serde_json::to_string(&records).unwrap();
            
            conn.execute("UPDATE dns SET records = ? WHERE id = ?;", params![records_str, dns_query_id]).unwrap();
        });
    });
    
    Ok(())
}

#[tauri::command]
pub fn get_dns_queries(conn: tauri::State<Arc<Mutex<duckdb::Connection>>>, loaded_report: tauri::State<Arc<Mutex<Option<Report>>>>) -> Result<Vec<DnsQuery>, String> {
    let conn_arc_clone = Arc::clone(&conn);
    let conn = conn_arc_clone.lock().unwrap().try_clone().unwrap();
    
    let loaded_report_arc_clone = Arc::clone(&loaded_report);
    let loaded_report = loaded_report_arc_clone.lock().unwrap();
    
    let (mut stmt, p) = match loaded_report.clone() {
        Some(r) => (conn.prepare("SELECT * FROM dns WHERE report = ?;").unwrap(), params![r.id.clone()]),
        None => (conn.prepare("SELECT * FROM dns;").unwrap(), params![])
    };
    
    let data = stmt.query_map(p, |row| {
        let records_str: Option<String> = row.get(6).unwrap();
        let records: Option<Vec<DnsRecord>> = match records_str {
            Some(s) => Some(serde_json::from_str(&s).unwrap()),
            None => None
        };
        
        Ok(DnsQuery {
            id: row.get(0).unwrap(),
            report: row.get(1).unwrap(),
            host: row.get(2).unwrap(),
            port: row.get(3).unwrap(),
            protocol: row.get(4).unwrap(),
            domain: row.get(5).unwrap(),
            records
        })
    });
    
    match data {
        Ok(d) => {
            let mut v: Vec<DnsQuery> = Vec::new();
            for i in d {
                v.push(i.unwrap());
            }
            Ok(v)
        },
        Err(err) => Err(err.to_string())
    }
}