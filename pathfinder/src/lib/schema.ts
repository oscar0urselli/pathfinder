export type NetGraph = {
    nodes: NetNode[],
    node_holes: any[],
    edge_propery: "undirected",
    edges: any[]
};

export type NetNode = {
    name: string,
    type: "Unknown" | "Switch" | "Router" | "Server" | "Pc",
    interfaces: NetNodeInterface[],
    services: NetNodeService[]
};

export type NetNodeInterface = {
    mac: string,
    ips: string[]
};

export type NetNodeService = {
    ip: string,
    name: string,
    port: number,
    transport_protocol: string
};

export type ActivePlugins = {
    [key: string]: "Running" | "WaitingForm" | "Exiting"
};

export type PluginFormData = {
    name: string,
    config: PluginFormConfig
};

export type PluginFormConfig = PluginFormField[][];

export type PluginFormField = {
    name: string,
    title: string,
    type: "str" | "ipv4" | "ipv6" | "ipv4_cidr" | "ipv6_cidr" | "mac" | "float" | "int" | "bool",
    options: string[] | null,
    min: string | null,
    max: string | null,
    step: string | null,
    regex: string | null,
    default: string | null
};

export type Plugin = {
    path: string,
    folder: string,
    config: PluginConfig
};

export type PluginConfig = {
    name: string,
    author: string,
    license: string,
    repository: string,
    version: string,
    language: string,
    params: any
}

export type Toast = {
    alert_type: "success" | "info" | "warning" | "danger" | "none",
    text: string
};

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
    vlan_id: number | null,
    scans: {
        ipv4: string,
        mac: string,
        hostname: string,
        vendor: string,
        scan: string
    }[]
};

export type DnsQuery = {
    id: string,
    report: string,
    host: string,
    port: number,
    protocol: string,
    domain: string,
    records: {
        name: string,
        rtype: string,
        class: string,
        ttl: number,
        data: string
    }[]
};