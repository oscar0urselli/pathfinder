use std::net::IpAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use arp_scan_lib::scan_options::ScanOptions;
use serde::{Deserialize, Serialize};
use duckdb;


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
    vlan_id: Option<u16>
}

#[derive(Serialize, Deserialize)]
pub struct IpInfo {
    ip: String,
    network: String,
    prefix: u8
}

#[derive(Serialize, Deserialize)]
pub struct ArpScanInterface {
    name: String,
    mac: String,
    ips: Vec<IpInfo>
}

#[derive(Serialize, Deserialize)]
pub struct ArpScanInfo {
    interfaces: Vec<ArpScanInterface>
}

#[tauri::command]
pub fn arp_scan_info() -> ArpScanInfo {
    ArpScanInfo {
        interfaces: pnet_datalink::interfaces().into_iter().filter(|x| x.is_running()).map(|x| ArpScanInterface {
            name: x.name,
            mac: match x.mac {
                Some(m) => m.to_string(),
                None => "".to_string()
            },
            ips: x.ips.into_iter().filter(|i| i.is_ipv4()).map(|i| IpInfo { ip: i.ip().to_string(), network: i.network().to_string(), prefix: i.prefix() }).collect()
        }).collect()
    }
}

/// Start a thread in which ARP scan is runned
/// 
/// # Params:
/// - *settings*: ArpScanSettings - Settings used fot ARP scan
#[tauri::command(async)]
pub fn arp_scan(conn: tauri::State<Arc<Mutex<duckdb::Connection>>>, settings: ArpScanSettings) -> Result<String, String> {
    let conn_arc_clone = Arc::clone(&conn);
    let conn = conn_arc_clone.lock().unwrap().try_clone().unwrap();
    
    let interfaces = pnet_datalink::interfaces();

    let profile = arp_scan_lib::scan_options::ProfileType::Chaos;
    
    let scan_options = Arc::new(ScanOptions {
        interface_name: Some(settings.interface.clone()),
        interface_index: Some(interfaces.iter().rfind(|x| x.name == settings.interface).unwrap().index),
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
        randomize_targets: matches!(profile, arp_scan_lib::scan_options::ProfileType::Stealth | arp_scan_lib::scan_options::ProfileType::Chaos),
        packet_help: true,
        scan_timing: match profile {
            arp_scan_lib::scan_options::ProfileType::Stealth => arp_scan_lib::scan_options::ScanTiming::Interval(REQUEST_MS_INTERVAL * 2),
            arp_scan_lib::scan_options::ProfileType::Fast => arp_scan_lib::scan_options::ScanTiming::Interval(0),
            _ => arp_scan_lib::scan_options::ScanTiming::Interval(REQUEST_MS_INTERVAL)
        },
        resolve_hostname: !matches!(profile, arp_scan_lib::scan_options::ProfileType::Stealth),
        network_range: Some(vec![settings.network.parse().unwrap()])
    });
    
    let (selected_interface, ip_networks) = arp_scan_lib::network::compute_network_configuration(&interfaces, &scan_options);
    
    let channel_config = pnet_datalink::Config {
        read_timeout: Some(Duration::from_millis(arp_scan_lib::network::DATALINK_RCV_TIMEOUT)),
        ..pnet_datalink::Config::default()
    };
    
    let (mut tx, mut rx) = match pnet_datalink::channel(selected_interface, channel_config) {
        Ok(pnet_datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => return Err("Expected an Ethernet datakink channel".to_string()),
        Err(error) => return Err(format!("Datalink channel creation failed ({})", error))
    };
    
    let timed_out = Arc::new(AtomicBool::new(false));
    let cloned_timed_out = Arc::clone(&timed_out);
    
    let mut vendor_list = arp_scan_lib::vendor::Vendor::new(&scan_options.oui_file);
    
    let cloned_options = Arc::clone(&scan_options);
    let arp_responses = thread::spawn(move || {
        arp_scan_lib::network::receive_arp_responses(&mut rx, cloned_options, cloned_timed_out, &mut vendor_list)
    });
    
    let network_size = arp_scan_lib::utils::compute_network_size(&ip_networks);
    
    let estimations = arp_scan_lib::network::compute_scan_estimation(network_size, &scan_options);
    let interval_ms = estimations.interval_ms;
    
    let has_reached_timeout = Arc::new(AtomicBool::new(false));
    let cloned_reached_timeout = Arc::clone(&has_reached_timeout);
    
    let source_ip = arp_scan_lib::network::find_source_ip(selected_interface, scan_options.source_ipv4);
    
    for _ in 0..scan_options.retry_count {
        if has_reached_timeout.load(Ordering::Relaxed) {
            break;
        }
        
        let ip_addresses = arp_scan_lib::network::NetworkIterator::new(&ip_networks, scan_options.randomize_targets);
        
        for ip_address in ip_addresses {
            if has_reached_timeout.load(Ordering::Relaxed) {
                break;
            }
            
            if let IpAddr::V4(ipv4_address) = ip_address {
                arp_scan_lib::network::send_arp_request(&mut tx, selected_interface, source_ip, ipv4_address, Arc::clone(&scan_options));
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
    
    // Save response data in the database
    println!("Duration: {:?}    Packets: {:?}    Arp: {:?}", response_summary.duration_ms, response_summary.packet_count, response_summary.arp_count);
    for t in target_details {
        println!("IP: {:?} Mac: {:?}", t.ipv4, t.mac);
    }
    
    Ok("Ok".to_string())
}