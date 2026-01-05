<script lang="ts">
    import type { ArpScan } from "$lib/schema";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    
    function viewArpScanDetails(event: any) {
        selectedArpScan = arpScans.find((v) => v.id === event.currentTarget.value);
    }
    
    let selectedArpScan: ArpScan | undefined = $state(undefined);
    
    let arpScans: ArpScan[] = $state([]);
    onMount(async () => {
        arpScans = await invoke("get_arp_scans");
    });
</script>

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
                                <p class="card-text">ID: {selectedArpScan?.id}</p>
                                <p class="card-text">Report: {selectedArpScan?.report}</p>
                                <p class="card-text">ARP count: {selectedArpScan?.arp_count}</p>
                                <p class="card-text">Duration: {selectedArpScan?.duration_ms} ms</p>
                                <p class="card-text">Packet count: {selectedArpScan?.packet_count}</p>
                                <p class="card-text">Interface: {selectedArpScan?.interface}</p>
                                <p class="card-text">Network: {selectedArpScan?.network}</p>
                            </div>
                            <div class="col">
                                <p class="card-text">Timeout: {selectedArpScan?.timeout} ms</p>
                                <p class="card-text">Interval: {selectedArpScan?.interval} ms</p>
                                <p class="card-text">Retry: {selectedArpScan?.retry}</p>
                                <p class="card-text">Source IP: {selectedArpScan?.src_ip}</p>
                                <p class="card-text">Source MAC: {selectedArpScan?.src_mac}</p>
                                <p class="card-text">Destination MAC: {selectedArpScan?.dst_mac}</p>
                                <p class="card-text">VLAN ID: {selectedArpScan?.vlan_id}</p>
                            </div>
                        </div>
                    </div>
                </div>
                <table class="table mt-4">
                    <thead>
                        <tr>
                            <th scope="col">IPv4</th>
                            <th scope="col">MAC</th>
                            <th scope="col">Hostname</th>
                            <th scope="col">Vendor</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each selectedArpScan?.scans as a}
                            <tr>
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