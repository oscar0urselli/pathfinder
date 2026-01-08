<script lang="ts">
    import { loadedReport } from "$lib/state.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import ARPTable from "./ARPTable.svelte";
    import type { ReportType } from "$lib/schema";
    
    
    let selectedTable: string = $state("reports");
    let selectedReport: string = $state(loadedReport.report?.id || "");
    let tables: string[] = $state([]);
    let reports: ReportType[] = $state([]);
    onMount(async () => {
        tables = await invoke("get_tables_list");
        
        reports = await invoke("list_reports");
    });
</script>

<div class="container p-2">
    <div class="vstack gap-2">
        <div class="hstack gap-2">
            <select bind:value={selectedTable} class="form-select" aria-label="Default select example">
                {#each tables as t}
                    <option value={t} selected={t === "reports"}>{t}</option>
                {/each}
            </select>
            {#if selectedTable !== "reports"}
                <select bind:value={selectedReport} class="form-select" aria-label="Default select example">
                    <option value="" selected={loadedReport.report === undefined}>All</option>
                    {#each reports as r}
                    <option value={r.id} selected={loadedReport.report?.id === r.id}>{r.title}</option>
                    {/each}
                </select>
            {/if}
        </div>
        <div class="card p-2 overflow-x-scroll">
            {#await invoke("get_table", { table: selectedTable })}
	
            {:then records}
            {@const fields = Object.keys(records[0])}
            <table class="table">
                <thead>
                    <tr>
                        {#each fields as f}
                            <th scope="col">{f}</th>
                        {/each}
                        <th></th>
                    </tr>
                </thead>
                <tbody class="table-group-divider">
                    {#each (records as any[]) as r}
                        {#if fields.includes("report") || selectedTable === "reports"}
                            {#if r["report"] === selectedReport || selectedReport === "" || selectedTable === "reports"}
                                <tr>
                                    {#each Object.values(r) as v}
                                        <td>{v}</td>
                                    {/each}
                                    <td><button class="btn btn-sm" data-bs-toggle="modal" data-bs-target="#arp-scan-details" aria-label="Details"><i class="bi bi-chevron-right"></i></button></td>
                                </tr>
                            {/if}
                        {/if}
                    {/each}
                </tbody>
            </table>
            {/await}
        </div>
    </div>
</div>