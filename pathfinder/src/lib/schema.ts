export type ArpScanIpInfo = {
    ip: string,
    network: string,
    prefix: number
};

export type ArpScanInterface = {
    name: string,
    mac: string,
    ips: ArpScanIpInfo[]
};

export type ArpScanInfo = {
    interfaces: ArpScanInterface[]
};

export type ReportType = {
    id: string,
    last_access_tsz: number,
    title: string,
    author: string,
    device: string,
    place: string,
    version: string
};

export type ArpScan = {
    id: string,
    report: string,
    arp_count: number,
    duration_ms: number,
    packet_count: number,
    interface: string,
    network: string,
    timeout: number,
    interval: number,
    retry: number,
    src_ip: string,
    src_mac: string,
    dst_mac: string,
    vlan_id: number | null
};

export type Arp = {
    id: number,
    ipv4: string,
    mac: string,
    hostname: string,
    vendor: string,
    scan: string
};