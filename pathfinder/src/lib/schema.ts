type ArpScanIpInfo = {
    ip: string,
    network: string,
    prefix: number
};

type ArpScanInterface = {
    name: string,
    mac: string,
    ips: ArpScanIpInfo[]
};

type ArpScanInfo = {
    interfaces: ArpScanInterface[]
};

type ReportType = {
    id: string,
    last_access_tsz: number,
    title: string,
    author: string,
    device: string,
    place: string,
    version: string
};