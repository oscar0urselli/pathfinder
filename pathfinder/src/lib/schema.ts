export type NetGraph = {
    nodes: NetNode[],
    node_holes: any[],
    edge_propery: "undirected",
    edges: NetEdge[]
};

export type NetEdge = {
    a_node: string,
    a_interface: string | null,
    a_service: string | null,
    a_ip: string | null,
    a_port: number,
    b_node: string,
    b_interface: string | null,
    b_service: string | null,
    b_ip: string | null,
    b_port: number
};

export type NetNode = {
    name: string,
    type: "Unknown" | "Switch" | "Router" | "Server" | "Database" | "Pc",
    interfaces: { [key: string]: NetNodeInterface },
    services: NetNodeService[]
};

export type NetNodeInterface = {
    mac: string,
    ips: IpCidr[]
};

export type NetNodeServiceBinding = {
    ip: string,
    port: number,
    protocol: "Tcp" | "Udp"
};

export type NetNodeService = {
    name: string,
    bindings: NetNodeServiceBinding[]
};

export type IpCidr = {
    ip: string,
    mask: number
};

export type ActivePlugins = {
    [key: string]: "Running" | "WaitingForm" | "Exiting"
};

export type Modal = {
    title: string,
    type: string | "PluginForm" | "NewReport",
    data: any,
    plugin?: string
};

export type PluginFormType = {
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

export type ToastType = {
    type: "Success" | "Info" | "Warning" | "Danger" | "None",
    text: string
};

export type LogType = {
    id: number,
    ts: number,
    type: "Error" | "Warn" | "Info" | "Debug" | "Trace",
    message: string
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