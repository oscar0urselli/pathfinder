<script lang="ts">
    import cytoscape from "cytoscape";
    import { onMount } from "svelte";
    import { z } from "zod/v4";
    import { invoke } from "@tauri-apps/api/core";
    import { loadedReport } from "$lib/state.svelte";
    import type { ArpScanInfo, Plugin } from "$lib/schema";
    import { toast } from "svelte-sonner";
    
    
    const arpSettingsSchema = z.object({
        interface: z.string(),
        network: z.cidrv4(),
        timeout: z.int().default(2000),
        interval: z.int().default(10),
        retry: z.int().default(1),
        src_ip: z.ipv4(),
        src_mac: z.mac(),
        dst_mac: z.mac().default("00:00:00:00:00:00"),
        vlan_id: z.int().nullable().default(null)
    });

    const bi_pc_display_horizontal = encodeURI("data:image/svg+xml;utf-8," + '<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-pc-display-horizontal" viewBox="0 0 16 16"><path d="M1.5 0A1.5 1.5 0 0 0 0 1.5v7A1.5 1.5 0 0 0 1.5 10H6v1H1a1 1 0 0 0-1 1v3a1 1 0 0 0 1 1h14a1 1 0 0 0 1-1v-3a1 1 0 0 0-1-1h-5v-1h4.5A1.5 1.5 0 0 0 16 8.5v-7A1.5 1.5 0 0 0 14.5 0zm0 1h13a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-.5.5h-13a.5.5 0 0 1-.5-.5v-7a.5.5 0 0 1 .5-.5M12 12.5a.5.5 0 1 1 1 0 .5.5 0 0 1-1 0m2 0a.5.5 0 1 1 1 0 .5.5 0 0 1-1 0M1.5 12h5a.5.5 0 0 1 0 1h-5a.5.5 0 0 1 0-1M1 14.25a.25.25 0 0 1 .25-.25h5.5a.25.25 0 1 1 0 .5h-5.5a.25.25 0 0 1-.25-.25"/></svg>');

    function changeInterface(event: any) {
        const i = arpScanInfo.interfaces.find((v) => v.name === arpScanInterface);
        if (i) {
            arpScanNetwork = i.ips[0].network + `/${i.ips[0].prefix}`;
            arpScanSrcIp = i.ips[0].ip;
            arpScanSrcMac = i.mac;
        }
    }
    
    async function arpScan(event: any) {
        const result = arpSettingsSchema.safeParse({
            interface: arpScanInterface,
            network: arpScanNetwork,
            timeout: arpScanTimeout,
            interval: arpScanInterval,
            retry: arpScanRetry,
            src_ip: arpScanSrcIp,
            src_mac: arpScanSrcMac,
            dst_mac: arpScanDstMac,
            vlan_id: arpScanVlanId
        });
        if (result.success) {
            try {
                await invoke("arp_scan", { settings: result.data, reportId: loadedReport.report?.id });
                toast.success("ARP scan terminated and the results are available.");
            }
            catch (error) {
                toast.error(`ARP scan did not complete successfully: ${error}`);
            }
        }
        else {
            console.log(result.error);
        }
    }
    
    function runPlugin(event: any) {
        invoke("run_plugin", { pluginName: event.currentTarget.value });
    }

    let arpScanInfo: ArpScanInfo = $state({ interfaces: [] });
    let arpScanInterface = $state("lo")
    let arpScanNetwork = $state("127.0.0.0/8");
    let arpScanVlanId = $state(null);
    let arpScanRetry = $state(1);
    let arpScanTimeout = $state(2000);
    let arpScanInterval = $state(10);
    let arpScanSrcIp = $state("127.0.0.1");
    let arpScanSrcMac = $state("00:00:00:00:00:00");
    let arpScanDstMac = $state("00:00:00:00:00:00");
    
    let plugins: { [key: string]: Plugin } = $state({});
    onMount(async () => {
        plugins = await invoke("get_plugins");
        arpScanInfo = await invoke("arp_scan_info");
      
        let cy = cytoscape({
            container: document.getElementById("cy"),
            elements: {
                nodes: [
                    {
                        data: { id: "a", name: "192.168.1.130" },
                        classes: "bi-pc-display-horizontal"
                    },
                    {
                        data: { id: "b" }
                    }
                ],
                edges: [
                    {
                        data: { id: "ab", source: "a", target: "b" }
                    }
                ]
            },
            style: [
                {
                    selector: "node.bi-pc-display-horizontal",
                    style: {
                        "background-image": bi_pc_display_horizontal
                    }
                }
            ],
            layout: {
                name: "grid"
            }
        });
    });
</script>

<div class="z-1 position-absolute start-50 translate-middle-x m-2">
    <div class="card border-0 shadow-lg p-2 hstack gap-2">
        <button type="button" class="btn btn-secondary" data-bs-toggle="modal" data-bs-target="#arp-settings">ARP</button>
        {#each Object.keys(plugins) as p}
            <button onclick={runPlugin} value={p} type="button" class="btn btn-secondary">{p}</button>
        {/each}
    </div>
</div>

<div id="cy" class="h-100"></div>


<!-- ARP Settings -->
<div class="modal fade" id="arp-settings" data-bs-backdrop="static" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
    <div class="modal-dialog modal-lg">
        <div class="modal-content">
            <div class="modal-header">
                <h1 class="modal-title fs-5" id="exampleModalLabel">ARP</h1>
                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
            </div>
            <div class="modal-body">
                <div class="mb-3">
                    <label for="arp-interface" class="form-label">Interface</label>
                    <select onchange={changeInterface} name="interface" bind:value={arpScanInterface} class="form-select" id="arp-interface" aria-label="Default select example">
                        {#each arpScanInfo.interfaces as i}
                            <option value={i.name}>{i.name}</option>
                        {/each}
                    </select>
                </div>
                <div class="mb-3">
                    <label for="arp-network-range" class="form-label">IPv4 range</label>
                    <input name="network" pattern="(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\/(?:3[0-2]|2[0-9]|1[0-9]|[0]?[0-9])" bind:value={arpScanNetwork} type="text" class="form-control" id="arp-network-range">
                </div>
                <div class="row">
                    <div class="col mb-3">
                        <label for="arp-vlan-id" class="form-label">VLAN ID</label>
                        <input name="vlad_id" min="1" max="4094" bind:value={arpScanVlanId} type="number" class="form-control" id="arp-vlan-id">
                    </div>
                    <div class="col mb-3">
                        <label for="arp-retry" class="form-label">Retry</label>
                        <input name="retry" min="0" bind:value={arpScanRetry} type="number" class="form-control" id="arp-retry">
                    </div>
                </div>
                <div class="row">
                    <div class="col mb-3">
                        <label for="arp-timeout" class="form-label">Timeout [ms]</label>
                        <input name="timeout" min="0" bind:value={arpScanTimeout} type="number" class="form-control" id="arp-timeout">
                    </div>
                    <div class="col mb-3">
                        <label for="arp-req-interval" class="form-label">Request interval [ms]</label>
                        <input name="interval" min="0" bind:value={arpScanInterval} type="number" class="form-control" id="arp-req-interval">
                    </div>
                </div>
                <div class="row">
                    <div class="col mb-3">
                        <label for="arp-src-ip" class="form-label">Source IPv4</label>
                        <input name="src_ip" pattern="(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)" bind:value={arpScanSrcIp} type="text" class="form-control" id="arp-src-ip">
                    </div>
                    <div class="col mb-3">
                        <label for="arp-src-mac" class="form-label">Source MAC</label>
                        <input name="src_mac" pattern="(([0-9A-Fa-f]{2}[-:]){5}[0-9A-Fa-f]{2})|(([0-9A-Fa-f]{4}\.){2}[0-9A-Fa-f]{4})" bind:value={arpScanSrcMac} type="text" class="form-control" id="arp-src-mac">
                    </div>    
                </div>
                <div class="mb-3">
                    <label for="arp-dst-mac" class="form-label">Destination MAC</label>
                    <input name="dst_mac" pattern="(([0-9A-Fa-f]{2}[-:]){5}[0-9A-Fa-f]{2})|(([0-9A-Fa-f]{4}\.){2}[0-9A-Fa-f]{4})" bind:value={arpScanDstMac} type="text" class="form-control" id="arp-dst-mac">
                </div>
            </div>
            <div class="modal-footer">
                <button onclick={arpScan} type="submit" class="btn btn-success">Scan</button>
            </div>
        </div>
    </div>
</div>