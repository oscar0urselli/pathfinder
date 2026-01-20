<script lang="ts">
    import { onMount } from "svelte";
    import { z } from "zod/v4";
    import { invoke } from "@tauri-apps/api/core";
    import { loadedReport, plugins } from "$lib/state.svelte";
    import type { ArpScanInfo, NetGraph, NetNode, NetNodeInterface, NetNodeService, Plugin } from "$lib/schema";
    import { toast } from "svelte-sonner";
    import Graph from "graphology";
    import Sigma from "sigma";
    import { NodeCircleProgram, createNodeCompoundProgram } from "sigma/rendering";
    import { NodeImageProgram, NodePictogramProgram, createNodeImageProgram } from "@sigma/node-image";
    import ForceSupervisor from "graphology-layout-force/worker";
    import NetNodeForm from "$lib/components/NetNodeForm.svelte";
    import { listen } from "@tauri-apps/api/event";
    
    
    const bi_pc_display_horizontal = '<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-pc-display-horizontal" viewBox="0 0 16 16"><path d="M1.5 0A1.5 1.5 0 0 0 0 1.5v7A1.5 1.5 0 0 0 1.5 10H6v1H1a1 1 0 0 0-1 1v3a1 1 0 0 0 1 1h14a1 1 0 0 0 1-1v-3a1 1 0 0 0-1-1h-5v-1h4.5A1.5 1.5 0 0 0 16 8.5v-7A1.5 1.5 0 0 0 14.5 0zm0 1h13a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-.5.5h-13a.5.5 0 0 1-.5-.5v-7a.5.5 0 0 1 .5-.5M12 12.5a.5.5 0 1 1 1 0 .5.5 0 0 1-1 0m2 0a.5.5 0 1 1 1 0 .5.5 0 0 1-1 0M1.5 12h5a.5.5 0 0 1 0 1h-5a.5.5 0 0 1 0-1M1 14.25a.25.25 0 0 1 .25-.25h5.5a.25.25 0 1 1 0 .5h-5.5a.25.25 0 0 1-.25-.25"/></svg>';
    const bi_router_fill = '<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-router-fill" viewBox="0 0 16 16"><path d="M5.525 3.025a3.5 3.5 0 0 1 4.95 0 .5.5 0 1 0 .707-.707 4.5 4.5 0 0 0-6.364 0 .5.5 0 0 0 .707.707"/><path d="M6.94 4.44a1.5 1.5 0 0 1 2.12 0 .5.5 0 0 0 .708-.708 2.5 2.5 0 0 0-3.536 0 .5.5 0 0 0 .707.707Z"/><path d="M2.974 2.342a.5.5 0 1 0-.948.316L3.806 8H1.5A1.5 1.5 0 0 0 0 9.5v2A1.5 1.5 0 0 0 1.5 13H2a.5.5 0 0 0 .5.5h2A.5.5 0 0 0 5 13h6a.5.5 0 0 0 .5.5h2a.5.5 0 0 0 .5-.5h.5a1.5 1.5 0 0 0 1.5-1.5v-2A1.5 1.5 0 0 0 14.5 8h-2.306l1.78-5.342a.5.5 0 1 0-.948-.316L11.14 8H4.86zM2.5 11a.5.5 0 1 1 0-1 .5.5 0 0 1 0 1m4.5-.5a.5.5 0 1 1 1 0 .5.5 0 0 1-1 0m2.5.5a.5.5 0 1 1 0-1 .5.5 0 0 1 0 1m1.5-.5a.5.5 0 1 1 1 0 .5.5 0 0 1-1 0m2 0a.5.5 0 1 1 1 0 .5.5 0 0 1-1 0"/><path d="M8.5 5.5a.5.5 0 1 1-1 0 .5.5 0 0 1 1 0"/></svg>';
    const bi_hdd_stack_fill = '<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-hdd-stack-fill" viewBox="0 0 16 16"><path d="M2 9a2 2 0 0 0-2 2v1a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-1a2 2 0 0 0-2-2zm.5 3a.5.5 0 1 1 0-1 .5.5 0 0 1 0 1m2 0a.5.5 0 1 1 0-1 .5.5 0 0 1 0 1M2 2a2 2 0 0 0-2 2v1a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2zm.5 3a.5.5 0 1 1 0-1 .5.5 0 0 1 0 1m2 0a.5.5 0 1 1 0-1 .5.5 0 0 1 0 1"/></svg>';
    const bi_database_fill = '<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-database-fill" viewBox="0 0 16 16"><path d="M3.904 1.777C4.978 1.289 6.427 1 8 1s3.022.289 4.096.777C13.125 2.245 14 2.993 14 4s-.875 1.755-1.904 2.223C11.022 6.711 9.573 7 8 7s-3.022-.289-4.096-.777C2.875 5.755 2 5.007 2 4s.875-1.755 1.904-2.223"/><path d="M2 6.161V7c0 1.007.875 1.755 1.904 2.223C4.978 9.71 6.427 10 8 10s3.022-.289 4.096-.777C13.125 8.755 14 8.007 14 7v-.839c-.457.432-1.004.751-1.49.972C11.278 7.693 9.682 8 8 8s-3.278-.307-4.51-.867c-.486-.22-1.033-.54-1.49-.972"/><path d="M2 9.161V10c0 1.007.875 1.755 1.904 2.223C4.978 12.711 6.427 13 8 13s3.022-.289 4.096-.777C13.125 11.755 14 11.007 14 10v-.839c-.457.432-1.004.751-1.49.972-1.232.56-2.828.867-4.51.867s-3.278-.307-4.51-.867c-.486-.22-1.033-.54-1.49-.972"/><path d="M2 12.161V13c0 1.007.875 1.755 1.904 2.223C4.978 15.711 6.427 16 8 16s3.022-.289 4.096-.777C13.125 14.755 14 14.007 14 13v-.839c-.457.432-1.004.751-1.49.972-1.232.56-2.828.867-4.51.867s-3.278-.307-4.51-.867c-.486-.22-1.033-.54-1.49-.972"/></svg>';
    
    
    function runPlugin(event: any) {
        invoke("run_plugin", { pluginName: event.currentTarget.value });
    }
    
    async function removeNode(event: any) {
        await invoke("remove_net_node", { node: Number(selectedNode) });
        
        netGraph = await invoke("get_net_graph");
        
        toast.warning("Node removed from the graph");
    }
    
    async function addEdge(event: any) {
        let formData = new FormData(addEdgeForm);
        
        await invoke("add_net_edge", {
            src: Number(formData.get("src")),
            dst: Number(formData.get("dst")) 
        })
        
        netGraph = await invoke("get_net_graph");
        
        toast.success("Inserted new edge to the graph.");
    }
    
    async function removeEdge(event: any) {
        await invoke("remove_net_edge", { edge: Number(selectedEdge) });
        
        netGraph = await invoke("get_net_graph");
        
        toast.warning("Edge removed from the graph");
    }

    let addEdgeForm: HTMLFormElement;
    
    let selectedNode: string | null = $state(null);
    let selectedEdge: string | null = $state(null);
    
    let netGraph: NetGraph = $state({ nodes: [], node_holes: [], edge_propery: "undirected", edges: [] });
    
    let sigmaGraphEl: HTMLDivElement;
    onMount(async () => {
        netGraph = await invoke("get_net_graph");
        console.log(netGraph);
    });
    
    listen<NetGraph>("updateNetGraph", async (event) => {
        netGraph = event.payload; 
    });
    
    $effect(() => {
        const sigmaGraph = new Graph();
        for (const [i, n] of Object.entries(netGraph.nodes)) {
            const angle = (Number(i) * 2 * Math.PI) / (Number(i) + 1);
            sigmaGraph.addNode(i, {
                label: n.name,
                size: 30,
                x: 100 * Math.cos(angle),
                y: 100 * Math.sin(angle),
                image: URL.createObjectURL(new Blob([bi_hdd_stack_fill], { type: "image/svg+xml" })),
                color: "gray",
                pictoColor: "black",
            });
        }
        
        for (const [i, e] of Object.entries(netGraph.edges)) {
            sigmaGraph.addEdgeWithKey(i, String(e[0]), String(e[1]), { size: 10, color: "#AA00AA" });
        }
        
        const NodePictogramCustomProgram = createNodeImageProgram({
            padding: 0.3,
            size: { mode: "force", value: 256 },
            drawingMode: "color",
            colorAttribute: "pictoColor",
        });
        const NodeProgram = createNodeCompoundProgram([NodeCircleProgram, NodePictogramCustomProgram]);
  
        let clickedNode: null | string = null;
        let hoveredEdge: null | string = null;
        let clickedEdge: null | string = null;
        const sigmaRenderer = new Sigma(sigmaGraph, sigmaGraphEl, {
            defaultNodeType: "pictogram",
                nodeProgramClasses: {
                  pictogram: NodeProgram,
            },
            nodeReducer(node, data) {
                const res = { ...data };
                
                if (node === clickedNode) res.color = "#FF3350";
                
                return res;
            },
            enableEdgeEvents: true,
            edgeReducer(edge, data) {
                const res = { ...data };
                
                if (edge === hoveredEdge || edge === clickedEdge) res.color = "#FF8830";
                
                return res;
            }
        });
        
        sigmaRenderer.on("clickNode", ({ node }) => {
            selectedNode = node;
            clickedNode = node;
            selectedEdge = null;
            clickedEdge = null;
            sigmaRenderer.refresh();
        });
        sigmaRenderer.on("clickEdge", ({ edge }) => {
            selectedNode = null;
            clickedNode = null;
            selectedEdge = edge;
            clickedEdge = edge;
            sigmaRenderer.refresh();
        });
        sigmaRenderer.on("clickStage", ({}) => {
            selectedNode = null;
            clickedNode = null;
            selectedEdge = null;
            clickedEdge = null;
            sigmaRenderer.refresh();
        });
        sigmaRenderer.on("enterEdge", ({ edge }) => {
            hoveredEdge = edge;
            sigmaRenderer.refresh();
        });
        sigmaRenderer.on("leaveEdge", ({ edge }) => {
            hoveredEdge = null;
            sigmaRenderer.refresh();
        });
        
        const layout = new ForceSupervisor(sigmaGraph, {
            settings: {
                gravity: 0.01
            },
            isNodeFixed: (_, attr) => attr.highlighted
        });
        
        layout.start();
        
        return () => {
            sigmaRenderer.kill();
            layout.kill();
        };
    });
</script>

<div class="z-1 position-absolute start-50 translate-middle-x m-2">
    <div class="card border-0 shadow-lg p-2 hstack gap-2">
        {#each Object.keys(plugins.p) as p}
            <button onclick={runPlugin} value={p} type="button" class="btn btn-secondary">{p}</button>
        {/each}
        <div class="rounded p-1" style="height: 40px; background-color: #cccccc;"></div>
        <button data-bs-toggle="modal" data-bs-target="#add-node" type="button" class="btn btn-success" aria-label="Add node"><i class="bi bi-node-plus-fill"></i></button>
        <button data-bs-toggle="modal" data-bs-target="#add-edge" type="button" class="btn btn-success" aria-label="Add edge"><i class="bi bi-arrows"></i></button>
    </div>
</div>

<!-- Node toolbar -->
<div class="z-1 position-absolute top-50 end-0 m-2 vstack gap-2" hidden={selectedNode === null}>
   	<div class="card border-0 shadow-lg p-2 vstack gap-2">
        <button type="button" class="btn btn-primary" data-bs-toggle="modal" data-bs-target="#edit-node" aria-label="Edit node"><i class="bi bi-pencil-fill"></i></button>
		<button onclick={removeNode} type="button" class="btn btn-danger" aria-label="Remove node"><i class="bi bi-trash-fill"></i></button>
	</div>
</div>

<!-- Edge toolbar -->
<div class="z-1 position-absolute top-50 end-0 m-2 vstack gap-2" hidden={selectedEdge === null}>
   	<div class="card border-0 shadow-lg p-2 vstack gap-2">
		<button onclick={removeEdge} type="button" class="btn btn-danger" aria-label="Remove edge"><i class="bi bi-trash-fill"></i></button>
	</div>
</div>

<div bind:this={sigmaGraphEl} class="h-100"></div>

<!-- Add node modal -->
<NetNodeForm formMode="add" submitForm={async () => { netGraph = await invoke("get_net_graph"); }} nodeIndex={undefined} node={undefined} />

<!-- Edit node modal -->
<NetNodeForm formMode="edit" submitForm={async () => { netGraph = await invoke("get_net_graph"); }} nodeIndex={Number(selectedNode)} node={netGraph.nodes[selectedNode]} />

<!-- Add edge modal -->
<div class="modal fade" id="add-edge" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
    <div class="modal-dialog modal-xl">
        <div class="modal-content">
            <div class="modal-header">
                <h1 class="modal-title fs-5" id="exampleModalLabel">Add new edge</h1>
                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
            </div>
            <form bind:this={addEdgeForm}>
            <div class="modal-body">
                <div class="row">
                    <div class="col mb-3">
                        <label for="exampleInputPassword1" class="form-label">Source</label>
                        <select name="src" class="form-select" aria-label="Default select example">
                            {#each Object.entries(netGraph.nodes) as [i, n]}
                                <option value={i}>{n.name}</option>
                            {/each}
                        </select>
                    </div>
                    <div class="col mb-3">
                        <label for="exampleInputPassword1" class="form-label">Destination</label>
                        <select name="dst" class="form-select" aria-label="Default select example">
                            {#each Object.entries(netGraph.nodes) as [i, n]}
                                <option value={i}>{n.name}</option>
                            {/each}
                        </select>
                    </div>
                </div>
            </div>
            <div class="modal-footer">
                <button onclick={addEdge} type="submit" class="btn btn-success">Create</button>
            </div>
            </form>
        </div>
    </div>
</div>