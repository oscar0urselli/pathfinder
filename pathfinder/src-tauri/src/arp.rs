use std::net::IpAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use arp_scan_lib::scan_options::ScanOptions;
use duckdb::{self, params};
use serde::{Deserialize, Serialize};

use crate::report::Report;

const REQUEST_MS_INTERVAL: u64 = 10;
const HOST_RETRY_DEFAULT: usize = 1;
const TIMEOUT_MS_FAST: u64 = 800;
const TIMEOUT_MS_DEFAULT: u64 = 2000;

#[derive(Serialize, Deserialize)]
pub struct ArpScanSettings {
    interface: String,
    network: String,
    timeout: u64,
    interval: u32,
    retry: u32,
    src_ip: String,
    src_mac: String,
    dst_mac: String,
    vlan_id: Option<u16>,
}

#[derive(Serialize, Deserialize)]
pub struct ArpScan {
    id: String,
    report: String,
    arp_count: u64,
    duration_ms: u64,
    packet_count: u64,
    interface: String,
    network: String,
    timeout: u64,
    interval: u32,
    retry: u32,
    src_ip: String,
    src_mac: String,
    dst_mac: String,
    vlan_id: Option<u16>,
    scans: Option<Vec<Arp>>,
}

#[derive(Serialize, Deserialize)]
pub struct IpInfo {
    ip: String,
    network: String,
    prefix: u8,
}

#[derive(Serialize, Deserialize)]
pub struct ArpScanInterface {
    name: String,
    mac: String,
    ips: Vec<IpInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct ArpScanInfo {
    interfaces: Vec<ArpScanInterface>,
}

#[derive(Serialize, Deserialize)]
pub struct Arp {
    ipv4: String,
    mac: String,
    hostname: Option<String>,
    vendor: Option<String>,
}

#[tauri::command]
pub fn arp_scan_info() -> ArpScanInfo {
    ArpScanInfo {
        interfaces: pnet_datalink::interfaces()
            .into_iter()
            .filter(|x| x.is_running())
            .map(|x| ArpScanInterface {
                name: x.name,
                mac: match x.mac {
                    Some(m) => m.to_string(),
                    None => "".to_string(),
                },
                ips: x
                    .ips
                    .into_iter()
                    .filter(|i| i.is_ipv4())
                    .map(|i| IpInfo {
                        ip: i.ip().to_string(),
                        network: i.network().to_string(),
                        prefix: i.prefix(),
                    })
                    .collect(),
            })
            .collect(),
    }
}

/// Start a thread in which ARP scan is runned
///
/// # Params:
/// - *settings*: ArpScanSettings - Settings used fot ARP scan
#[tauri::command(async)]
pub fn arp_scan(
    conn: tauri::State<Arc<Mutex<duckdb::Connection>>>,
    loaded_report: tauri::State<Arc<Mutex<Option<Report>>>>,
    settings: ArpScanSettings,
) -> Result<(), String> {
    let conn_arc_clone = Arc::clone(&conn);
    let conn = conn_arc_clone.lock().unwrap().try_clone().unwrap();

    let loaded_report_arc_clone = Arc::clone(&loaded_report);
    let loaded_report = loaded_report_arc_clone.lock().unwrap();

    if loaded_report.is_none() {
        return Err("No report loaded. You must create and/or load a report first.".to_string());
    }

    let interfaces = pnet_datalink::interfaces();

    let profile = arp_scan_lib::scan_options::ProfileType::Chaos;

    let scan_options = Arc::new(ScanOptions {
        interface_name: Some(settings.interface.clone()),
        interface_index: Some(
            interfaces
                .iter()
                .rfind(|x| x.name == settings.interface)
                .unwrap()
                .index,
        ),
        timeout_ms: settings.timeout,
        source_ipv4: Some(settings.src_ip.parse().unwrap()),
        source_mac: Some(settings.src_mac.parse().unwrap()),
        destination_mac: Some(settings.dst_mac.parse().unwrap()),
        vlan_id: settings.vlan_id,
        retry_count: settings.retry as usize,
        oui_file: "/usr/share/arp-scan/ieee-oui.csv".to_string(),
        hw_type: None,
        hw_addr: None,
        proto_addr: None,
        arp_operation: None,
        output: arp_scan_lib::scan_options::OutputFormat::Plain,
        proto_type: None,
        randomize_targets: matches!(
            profile,
            arp_scan_lib::scan_options::ProfileType::Stealth
                | arp_scan_lib::scan_options::ProfileType::Chaos
        ),
        packet_help: true,
        scan_timing: match profile {
            arp_scan_lib::scan_options::ProfileType::Stealth => {
                arp_scan_lib::scan_options::ScanTiming::Interval(REQUEST_MS_INTERVAL * 2)
            }
            arp_scan_lib::scan_options::ProfileType::Fast => {
                arp_scan_lib::scan_options::ScanTiming::Interval(0)
            }
            _ => arp_scan_lib::scan_options::ScanTiming::Interval(REQUEST_MS_INTERVAL),
        },
        resolve_hostname: !matches!(profile, arp_scan_lib::scan_options::ProfileType::Stealth),
        network_range: Some(vec![settings.network.parse().unwrap()]),
    });

    let (selected_interface, ip_networks) =
        arp_scan_lib::network::compute_network_configuration(&interfaces, &scan_options);

    let channel_config = pnet_datalink::Config {
        read_timeout: Some(Duration::from_millis(
            arp_scan_lib::network::DATALINK_RCV_TIMEOUT,
        )),
        ..pnet_datalink::Config::default()
    };

    let (mut tx, mut rx) = match pnet_datalink::channel(selected_interface, channel_config) {
        Ok(pnet_datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => return Err("Expected an Ethernet datakink channel".to_string()),
        Err(error) => return Err(format!("Datalink channel creation failed ({})", error)),
    };

    let timed_out = Arc::new(AtomicBool::new(false));
    let cloned_timed_out = Arc::clone(&timed_out);

    let mut vendor_list = arp_scan_lib::vendor::Vendor::new(&scan_options.oui_file);

    let cloned_options = Arc::clone(&scan_options);
    let arp_responses = thread::spawn(move || {
        arp_scan_lib::network::receive_arp_responses(
            &mut rx,
            cloned_options,
            cloned_timed_out,
            &mut vendor_list,
        )
    });

    let network_size = arp_scan_lib::utils::compute_network_size(&ip_networks);

    let estimations = arp_scan_lib::network::compute_scan_estimation(network_size, &scan_options);
    let interval_ms = estimations.interval_ms;

    let has_reached_timeout = Arc::new(AtomicBool::new(false));
    let cloned_reached_timeout = Arc::clone(&has_reached_timeout);

    let source_ip =
        arp_scan_lib::network::find_source_ip(selected_interface, scan_options.source_ipv4);

    for _ in 0..scan_options.retry_count {
        if has_reached_timeout.load(Ordering::Relaxed) {
            break;
        }

        let ip_addresses = arp_scan_lib::network::NetworkIterator::new(
            &ip_networks,
            scan_options.randomize_targets,
        );

        for ip_address in ip_addresses {
            if has_reached_timeout.load(Ordering::Relaxed) {
                break;
            }

            if let IpAddr::V4(ipv4_address) = ip_address {
                arp_scan_lib::network::send_arp_request(
                    &mut tx,
                    selected_interface,
                    source_ip,
                    ipv4_address,
                    Arc::clone(&scan_options),
                );
                thread::sleep(Duration::from_millis(interval_ms));
            }
        }
    }

    let mut sleep_ms_mount: u64 = 0;
    while !has_reached_timeout.load(Ordering::Relaxed) && sleep_ms_mount < scan_options.timeout_ms {
        thread::sleep(Duration::from_millis(100));
        sleep_ms_mount += 100;
    }
    timed_out.store(true, Ordering::Relaxed);

    let (response_summary, target_details) = arp_responses.join().unwrap();

    let mut scans: Vec<Arp> = Vec::new();
    for t in target_details {
        scans.push(Arp {
            ipv4: t.ipv4.to_string(),
            mac: t.mac.to_string(),
            hostname: t.hostname,
            vendor: t.vendor,
        });
    }
    let scans_str = serde_json::to_string(&scans).unwrap();

    let _ = conn.execute("CREATE TABLE IF NOT EXISTS arp (id UUID PRIMARY KEY, report UUID, arp_count UINT64, duration_ms UINT64, packet_count UINT64, interface STRING, network STRING, timeout UINT64, interval UINT64, retry UINT64, src_ip STRING, src_mac STRING, dst_mac STRING, vlan_id UINT16, scans STRING);", params![]);

    let mut stmt = conn.prepare("INSERT INTO arp (id, report, arp_count, duration_ms, packet_count, interface, network, timeout, interval, retry, src_ip, src_mac, dst_mac, vlan_id, scans) VALUES (uuidv7(), ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING id;").unwrap();
    let arp_scan_id = stmt
        .query_one(
            params![
                loaded_report.as_ref().unwrap().id,
                response_summary.arp_count,
                response_summary.duration_ms as u64,
                response_summary.packet_count,
                settings.interface,
                settings.network,
                settings.timeout,
                settings.interval,
                settings.retry,
                settings.src_ip,
                settings.src_mac,
                settings.dst_mac,
                settings.vlan_id,
                scans_str
            ],
            |row| Ok(row.get::<usize, String>(0).unwrap()),
        )
        .unwrap();

    Ok(())
}

#[tauri::command]
pub fn get_arp_scans(
    conn: tauri::State<Arc<Mutex<duckdb::Connection>>>,
    loaded_report: tauri::State<Arc<Mutex<Option<Report>>>>,
) -> Result<Vec<ArpScan>, String> {
    let conn_arc_clone = Arc::clone(&conn);
    let conn = conn_arc_clone.lock().unwrap().try_clone().unwrap();

    let loaded_report_arc_clone = Arc::clone(&loaded_report);
    let loaded_report = loaded_report_arc_clone.lock().unwrap();

    let (mut stmt, p) = match loaded_report.clone() {
        Some(r) => (
            conn.prepare("SELECT * FROM arp WHERE report = ?;")
                .unwrap(),
            params![r.id.clone()],
        ),
        None => (conn.prepare("SELECT * FROM arp;").unwrap(), params![]),
    };
    let data = stmt.query_map(p, |row| {
        let scans_str: Option<String> = row.get(14).unwrap();
        let scans: Option<Vec<Arp>> = match scans_str {
            Some(s) => Some(serde_json::from_str(&s).unwrap()),
            None => None,
        };

        Ok(ArpScan {
            id: row.get(0).unwrap(),
            report: row.get(1).unwrap(),
            arp_count: row.get(2).unwrap(),
            duration_ms: row.get(3).unwrap(),
            packet_count: row.get(4).unwrap(),
            interface: row.get(5).unwrap(),
            network: row.get(6).unwrap(),
            timeout: row.get(7).unwrap(),
            interval: row.get(8).unwrap(),
            retry: row.get(9).unwrap(),
            src_ip: row.get(10).unwrap(),
            src_mac: row.get(11).unwrap(),
            dst_mac: row.get(12).unwrap(),
            vlan_id: row.get(13).unwrap(),
            scans,
        })
    });

    match data {
        Ok(d) => {
            let mut v: Vec<ArpScan> = Vec::new();
            for i in d {
                v.push(i.unwrap());
            }
            Ok(v)
        }
        Err(err) => Err(err.to_string()),
    }
}
