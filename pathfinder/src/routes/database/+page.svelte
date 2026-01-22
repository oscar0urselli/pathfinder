<script lang="ts">
    import { loadedReport } from "$lib/state.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import type { ReportType } from "$lib/schema";
    
    
    function changeSelectedColumns(event: any) {
        let formData = new FormData(selectedColumnsForm);
        
        selectedColumns = [];
        for (const i of formData.keys()) {
            selectedColumns.push(i);
        }
    }
    
    
    let selectedRecord: any | undefined = $state(undefined);
    
    let selectedTable: string = $state("reports");
    let selectedReport: string = $state(loadedReport.report?.id || "");
    let tables: {
        [key: string]: {
            table: string,
            name: string,
            type: string
        }[]
    } = $state({});
    let selectedColumns = $derived(tables[selectedTable].map((v) => v.name));
    
    let reports: ReportType[] = $state([]);
    
    let selectedColumnsForm: HTMLFormElement;
    
    onMount(async () => {
        tables = await invoke("get_tables_list");
        console.log(tables[selectedTable].map((v) => v.name).includes("report"));
        reports = await invoke("list_reports");
    });
</script>

<div class="container p-2">
    <div class="vstack gap-2">
        <div class="hstack gap-2">
            <div class="dropdown">
                <button class="btn btn-light border dropdown-toggle" type="button" data-bs-toggle="dropdown" aria-expanded="false">Selected columns</button>
                <div class="dropdown-menu p-2">
                    <form bind:this={selectedColumnsForm}>
                    {#each tables[selectedTable] as c}
                        <div class="form-check">
                            <input onchange={changeSelectedColumns} name={c.name} class="form-check-input" type="checkbox" value="" id="checkDefault" checked={selectedColumns.includes(c.name)}>
                            <label class="form-check-label" for="checkDefault">{c.name}</label>
                        </div>
                    {/each}
                    </form>
                </div>
            </div>
            <select bind:value={selectedTable} class="form-select" aria-label="Default select example">
                {#each Object.keys(tables) as t}
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
            <div class="spinner-grow" role="status">
                <span class="visually-hidden">Loading...</span>
            </div>
            {:then records}
            <table class="table">
                <thead>
                    <tr>
                        {#each tables[selectedTable] as c}
                            {#if selectedColumns.includes(c.name)}
                                <th scope="col">{c.name}</th>
                            {/if}
                        {/each}
                        <th></th>
                    </tr>
                </thead>
                <tbody class="table-group-divider">
                    {#each (records as any[]) as r}
                        {#if r["report"] === selectedReport || selectedReport === "" || selectedTable === "reports"}
                            <tr>
                                {#each tables[selectedTable] as c}
                                    {#if selectedColumns.includes(c.name)}
                                        <td>{String(r[c.name]).slice(0, 36)}{String(r[c.name]).length > 36 ? "..." : ""}</td>
                                    {/if}
                                {/each}
                                <td><button onclick={() => { selectedRecord = r; }} class="btn btn-sm" data-bs-toggle="modal" data-bs-target="#record-details" aria-label="Details"><i class="bi bi-chevron-right"></i></button></td>
                            </tr>
                        {/if}
                    {/each}
                </tbody>
            </table>
            {/await}
        </div>
    </div>
</div>

<!-- Modal -->
<div class="modal fade" id="record-details" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
    <div class="modal-dialog modal-lg">
        <div class="modal-content">
            <div class="modal-header">
                <h1 class="modal-title fs-5" id="exampleModalLabel">Details</h1>
                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
            </div>
            <div class="modal-body">
                {#if selectedRecord}
                    {#each Object.entries(selectedRecord) as [name, r]}
                        <p><strong>{name}</strong>: {r}</p>
                    {/each}
                {/if}
            </div>
        </div>
    </div>
</div>