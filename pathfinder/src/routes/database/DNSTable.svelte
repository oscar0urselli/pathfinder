<script lang="ts">
    import type { DnsQuery } from "$lib/schema";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    
    function viewDnsQueryDetails(event: any) {
        selectedQuery = dnsQueries.find((v) => v.id === event.currentTarget.value);
    }
    
    let selectedQuery: DnsQuery | undefined = $state(undefined);
    let dnsQueries: DnsQuery[] = $state([]);
    onMount(async () => {
        dnsQueries = await invoke("get_dns_queries");
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
                <th scope="col">Domain</th>
                <th></th>
            </tr>
        </thead>
        <tbody class="table-group-divider">
            {#each dnsQueries as q}
                <tr>
                    <th scope="row">{q.id}</th>
                    <td>{q.report}</td>
                    <td>{q.protocol}</td>
                    <td></td>
                    <td>{q.domain}</td>
                    <td><button onclick={viewDnsQueryDetails} value={q.id} class="btn btn-sm" data-bs-toggle="modal" data-bs-target="#dns-query-details" aria-label="Details"><i class="bi bi-chevron-right"></i></button></td>
                </tr>
            {/each}
        </tbody>
    </table>
</div>

<!-- ARP scan details modal -->
<div class="modal fade" id="dns-query-details" data-bs-backdrop="static" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
    <div class="modal-dialog modal-xl">
        <div class="modal-content">
            <div class="modal-header">
                <h1 class="modal-title fs-5" id="exampleModalLabel">DNS</h1>
                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
            </div>
            <div class="modal-body">
                <div class="card bg-tertiary">
                    <div class="card-body">
                        <p class="card-text">ID: {selectedQuery?.id}</p>
                        <p class="card-text">Report: {selectedQuery?.report}</p>
                        <p class="card-text">Host: {selectedQuery?.host}</p>
                        <p class="card-text">Port: {selectedQuery?.port}</p>
                        <p class="card-text">Protocol: {selectedQuery?.protocol}</p>
                        <p class="card-text">Domain: {selectedQuery?.domain}</p>
                    </div>
                </div>
                <table class="table mt-4">
                    <thead>
                        <tr>
                            <th scope="col">Name</th>
                            <th scope="col">Type</th>
                            <th scope="col">Class</th>
                            <th scope="col">TTL</th>
                            <th scope="col">Data</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each selectedQuery?.records as rr}
                            <tr>
                                <th scope="row">{rr.name}</th>
                                <td>{rr.rtype}</td>
                                <td>{rr.class}</td>
                                <td>{rr.ttl}</td>
                                <td>{rr.data}</td>
                            </tr>
                        {/each}
                    </tbody>
                </table>
            </div>
        </div>
    </div>
</div>