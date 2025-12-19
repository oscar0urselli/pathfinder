<script lang="ts">
    import type { Arp, ArpScan } from "$lib/schema";
    import { loadedReport } from "$lib/state.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    
    async function viewArpScanDetails(event: any) {
        const arpScan = arpScans.find((v) => v.id === event.currentTarget.value);
        if (arpScan) {
            selectedArpScan = {
                scan: arpScan,
                arp: await invoke("get_arps", { scanId: arpScan.id })
            }
        }
    }
    
    let selectedArpScan: {
        scan: ArpScan,
        arp: Arp[]
    } | undefined = $state(undefined);
    
    let selectedProtocol = $state("all");
    let arpScans: ArpScan[] = $state([]);
    onMount(async () => {
        loadedReport.report = await invoke("get_loaded_report");
        arpScans = await invoke("get_arp_scans");
    });
</script>

<div class="container p-2">
    <div class="vstack gap-2">
        <div class="row">
            <div class="col-4 form-floating">
                <select bind:value={selectedProtocol} class="form-select" id="floatingSelect" aria-label="Floating label select example">
                    <option value="all">All</option>
                    <option value="arp">ARP</option>
                    <option value="snmp">SNMP</option>
                    <option value="dns">DNS</option>
                </select>
                <label for="floatingSelect">Protocol</label>
            </div>
            <div class="col-4 form-floating">
                <select class="form-select" id="floatingSelect" aria-label="Floating label select example">
                    <option value="all">All</option>
                    <option value="lo">lo</option>
                    <option value="wlo1">wlo1</option>
                    <option value="tap0">tap0</option>
                </select>
                <label for="floatingSelect">Interface</label>
            </div>
        </div>
        <div class="card p-2">
            <table class="table">
                <thead>
                    <tr>
                        <th scope="col">ID</th>
                        <th scope="col">Report</th>
                        <th scope="col">Protocol</th>
                        <th scope="col">Datetime</th>
                        <th scope="col">Interface</th>
                        <th></th>
                    </tr>
                </thead>
                <tbody class="table-group-divider">
                    {#each arpScans as s}
                        <tr>
                            <th scope="row">{s.id}</th>
                            <td>{s.report}</td>
                            <td>ARP</td>
                            <td></td>
                            <td>{s.interface}</td>
                            <td><button onclick={viewArpScanDetails} value={s.id} class="btn btn-sm" data-bs-toggle="modal" data-bs-target="#arp-scan-details" aria-label="Details"><i class="bi bi-chevron-right"></i></button></td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    </div>
</div>


<!-- ARP scan details modal -->
<div class="modal fade" id="arp-scan-details" data-bs-backdrop="static" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
    <div class="modal-dialog modal-xl">
        <div class="modal-content">
            <div class="modal-header">
                <h1 class="modal-title fs-5" id="exampleModalLabel">ARP</h1>
                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
            </div>
            <div class="modal-body">
                <div class="card bg-tertiary">
                    <div class="card-body">
                        <div class="row">
                            <div class="col">
                                <p class="card-text">ID: {selectedArpScan?.scan.id}</p>
                                <p class="card-text">Report: {selectedArpScan?.scan.report}</p>
                                <p class="card-text">ARP count: {selectedArpScan?.scan.arp_count}</p>
                                <p class="card-text">Duration: {selectedArpScan?.scan.duration_ms} ms</p>
                                <p class="card-text">Packet count: {selectedArpScan?.scan.packet_count}</p>
                                <p class="card-text">Interface: {selectedArpScan?.scan.interface}</p>
                                <p class="card-text">Network: {selectedArpScan?.scan.network}</p>
                            </div>
                            <div class="col">
                                <p class="card-text">Timeout: {selectedArpScan?.scan.timeout} ms</p>
                                <p class="card-text">Interval: {selectedArpScan?.scan.interval} ms</p>
                                <p class="card-text">Retry: {selectedArpScan?.scan.retry}</p>
                                <p class="card-text">Source IP: {selectedArpScan?.scan.src_ip}</p>
                                <p class="card-text">Source MAC: {selectedArpScan?.scan.src_mac}</p>
                                <p class="card-text">Destination MAC: {selectedArpScan?.scan.dst_mac}</p>
                                <p class="card-text">VLAN ID: {selectedArpScan?.scan.vlan_id}</p>
                            </div>
                        </div>
                    </div>
                </div>
                <table class="table mt-4">
                    <thead>
                        <tr>
                            <th scope="col">#</th>
                            <th scope="col">IPv4</th>
                            <th scope="col">MAC</th>
                            <th scope="col">Hostname</th>
                            <th scope="col">Vendor</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each selectedArpScan?.arp as a}
                            <tr>
                                <th scope="row">{a.id}</th>
                                <td>{a.ipv4}</td>
                                <td>{a.mac}</td>
                                <td>{a.hostname}</td>
                                <td>{a.vendor}</td>
                            </tr>
                        {/each}
                    </tbody>
                </table>
            </div>
        </div>
    </div>
</div>