<script lang="ts">
    import cytoscape from "cytoscape";
    import { onMount } from "svelte";
    import { z } from "zod/v4";
    import { invoke } from "@tauri-apps/api/core";
    import { loadedReport, plugins } from "$lib/state.svelte";
    import type { ArpScanInfo, Plugin } from "$lib/schema";
    import { toast } from "svelte-sonner";
    
    
    const bi_pc_display_horizontal = encodeURI("data:image/svg+xml;utf-8," + '<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-pc-display-horizontal" viewBox="0 0 16 16"><path d="M1.5 0A1.5 1.5 0 0 0 0 1.5v7A1.5 1.5 0 0 0 1.5 10H6v1H1a1 1 0 0 0-1 1v3a1 1 0 0 0 1 1h14a1 1 0 0 0 1-1v-3a1 1 0 0 0-1-1h-5v-1h4.5A1.5 1.5 0 0 0 16 8.5v-7A1.5 1.5 0 0 0 14.5 0zm0 1h13a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-.5.5h-13a.5.5 0 0 1-.5-.5v-7a.5.5 0 0 1 .5-.5M12 12.5a.5.5 0 1 1 1 0 .5.5 0 0 1-1 0m2 0a.5.5 0 1 1 1 0 .5.5 0 0 1-1 0M1.5 12h5a.5.5 0 0 1 0 1h-5a.5.5 0 0 1 0-1M1 14.25a.25.25 0 0 1 .25-.25h5.5a.25.25 0 1 1 0 .5h-5.5a.25.25 0 0 1-.25-.25"/></svg>');
    
    function runPlugin(event: any) {
        invoke("run_plugin", { pluginName: event.currentTarget.value });
    }
    
    onMount(async () => {
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
        {#each Object.keys(plugins.p) as p}
            <button onclick={runPlugin} value={p} type="button" class="btn btn-secondary">{p}</button>
        {/each}
    </div>
</div>

<div id="cy" class="h-100"></div>