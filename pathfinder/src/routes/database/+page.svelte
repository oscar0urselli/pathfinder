<script lang="ts">
    import { loadedReport } from "$lib/state.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import ARPTable from "./ARPTable.svelte";
    import DNSTable from "./DNSTable.svelte";
    
    
    onMount(async () => {
        loadedReport.report = await invoke("get_loaded_report");
    });
</script>

<div class="container p-2">
    <div class="vstack gap-2">
        <div class="hstack gap-2">
            <button class="btn btn-outline-primary">ARP</button>
            <button class="btn btn-outline-primary">DNS</button>
            <button class="btn btn-outline-primary">SNMP</button>
        </div>
        <ARPTable />
        <DNSTable />
    </div>
</div>