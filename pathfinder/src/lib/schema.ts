export type NetGraph = {
    nodes: NetNode[],
    node_holes: any[],
    edge_propery: "undirected",
    edges: any[]
};

export type NetNode = {
    name: string,
    type: "Unknown" | "Switch" | "Router" | "Server" | "Pc",
    interfaces: { [key: string]: NetNodeInterface },
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