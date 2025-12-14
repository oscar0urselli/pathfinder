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