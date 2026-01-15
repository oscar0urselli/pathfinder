<script lang="ts">
    import cytoscape from "cytoscape";
    import { onMount } from "svelte";
    import { z } from "zod/v4";
    import { invoke } from "@tauri-apps/api/core";
    import { loadedReport, plugins } from "$lib/state.svelte";
    import type { ArpScanInfo, NetNode, NetNodeInterface, NetNodeService, Plugin } from "$lib/schema";
    import { toast } from "svelte-sonner";
    
    
    const bi_pc_display_horizontal = encodeURI("data:image/svg+xml;utf-8," + '<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-pc-display-horizontal" viewBox="0 0 16 16"><path d="M1.5 0A1.5 1.5 0 0 0 0 1.5v7A1.5 1.5 0 0 0 1.5 10H6v1H1a1 1 0 0 0-1 1v3a1 1 0 0 0 1 1h14a1 1 0 0 0 1-1v-3a1 1 0 0 0-1-1h-5v-1h4.5A1.5 1.5 0 0 0 16 8.5v-7A1.5 1.5 0 0 0 14.5 0zm0 1h13a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-.5.5h-13a.5.5 0 0 1-.5-.5v-7a.5.5 0 0 1 .5-.5M12 12.5a.5.5 0 1 1 1 0 .5.5 0 0 1-1 0m2 0a.5.5 0 1 1 1 0 .5.5 0 0 1-1 0M1.5 12h5a.5.5 0 0 1 0 1h-5a.5.5 0 0 1 0-1M1 14.25a.25.25 0 0 1 .25-.25h5.5a.25.25 0 1 1 0 .5h-5.5a.25.25 0 0 1-.25-.25"/></svg>');
    
    function runPlugin(event: any) {
        invoke("run_plugin", { pluginName: event.currentTarget.value });
    }
    
    async function addNode(event: any) {
        let formData = new FormData(addNodeForm);
        
        let netNode: NetNode = {
            name: formData.get("name") || "Node",
            type: formData.get("type") || "Unknown",
            interfaces: nodeInterfaces,
            services: nodeServices
        };
        
        await invoke("add_net_node", { node: netNode });
        
        toast.success("Inserted new node to the graph.");
    }
    
    function addNodeInterface(event: any) {
        nodeInterfaces.push({
            mac: "",
            ips: []
        });
    }
    
    function removeNodeInterface(event: any) {
        nodeInterfaces.splice(Number(event.currentTarget.value), 1);
    }
    
    function changeNodeInterface(event: any) {
        nodeInterfaces[event.currentTarget.getAttribute("data-interface-index")].mac = event.currentTarget.value;
    }
    
    function addNodeInterfaceIp(event: any) {
        nodeInterfaces[event.currentTarget.value].ips.push("");
    }
    
    function removeNodeInterfaceIp(event: any) {
        nodeInterfaces[event.currentTarget.getAttribute("data-interface-index")].ips.splice(Number(event.currentTarget.getAttribute("data-ip-index")), 1);
    }
    
    function changeNodeInterfaceIp(event: any) {
        nodeInterfaces[event.currentTarget.getAttribute("data-interface-index")].ips[event.currentTarget.getAttribute("data-ip-index")] = event.currentTarget.value;
    }
    
    function addNodeService(event: any) {
        nodeServices.push({
            name: "",
            ip: "",
            port: 80,
            transport_protocol: "TCP"
        });
    }
    
    function removeNodeService(event: any) {
        nodeServices.splice(Number(event.currentTarget.value), 1);
    }
    
    function changeNodeServiceName(event: any) {
        nodeServices[event.currentTarget.getAttribute("data-service-index")].name = event.currentTarget.value;
    }
    function changeNodeServiceIp(event: any) {
        nodeServices[event.currentTarget.getAttribute("data-service-index")].ip = event.currentTarget.value;
    }
    function changeNodeServicePort(event: any) {
        nodeServices[event.currentTarget.getAttribute("data-service-index")].port = Number(event.currentTarget.value);
    }
    function changeNodeServiceTransportProtocol(event: any) {
        nodeServices[event.currentTarget.getAttribute("data-service-index")].transport_protocol = event.currentTarget.value;
    }

    let nodeInterfaces: NetNodeInterface[] = $state([]);
    let nodeServices: NetNodeService[] = $state([]);
    let addNodeForm: HTMLFormElement;
    
    onMount(async () => {
        let graph = await invoke("get_net_graph");
        console.log(graph);
      
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
        <div class="rounded p-1" style="height: 40px; background-color: #cccccc;"></div>
        <button data-bs-toggle="modal" data-bs-target="#add-node" type="button" class="btn btn-success" aria-label="Add node"><i class="bi bi-node-plus-fill"></i></button>
        <button type="button" class="btn btn-success" aria-label="Add edge"><i class="bi bi-arrows"></i></button>
    </div>
</div>

<div id="cy" class="h-100"></div>

<!-- Add node modal -->
<div class="modal fade" id="add-node" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
    <div class="modal-dialog modal-xl">
        <div class="modal-content">
            <div class="modal-header">
                <h1 class="modal-title fs-5" id="exampleModalLabel">Add new node</h1>
                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
            </div>
            <form bind:this={addNodeForm}>
            <div class="modal-body">
                <div class="row">
                    <div class="col-8 mb-3">
                        <label for="exampleInputPassword1" class="form-label">Name</label>
                        <input name="name" type="text" class="form-control">
                    </div>
                    <div class="col mb-3">
                        <label for="exampleInputPassword1" class="form-label">Node type</label>
                        <select name="type" class="form-select" aria-label="Default select example">
                            <option value="Unknown" selected>Unknow</option>
                            <option value="Switch">Switch</option>
                            <option value="Router">Router</option>
                            <option value="Server">Server</option>
                            <option value="Pc">Pc</option>
                        </select>
                    </div>
                </div>
                <hr>
                <p class="fs-5">Interfaces</p>
                <div class="d-flex align-content-start flex-wrap gap-2">
                    {#each Object.entries(nodeInterfaces) as [i_index, i]}
                        <div class="hstack">
                            <button onclick={removeNodeInterface} value={i_index} class="rounded-end-0 btn btn-danger align-self-stretch" aria-label="Close"><i class="bi bi-x-lg"></i></button>
                            <div class="rounded-start-0 card p-2">
                                <div class="mb-3">
                                    <label for="exampleInputPassword1" class="form-label">MAC</label>
                                    <input value={i.mac} onchange={changeNodeInterface} data-interface-index={i_index} type="text" class="form-control">
                                </div>
                                <p class="my-0">IPs</p>
                                <div class="d-flex align-content-start flex-wrap gap-2">
                                    {#each Object.entries(i.ips) as [ip_index, ip]}
                                        <div class="input-group w-auto">
                                            <input value={ip} onchange={changeNodeInterfaceIp} data-ip-index={ip_index} data-interface-index={i_index} type="text" class="form-control">
                                            <button onclick={removeNodeInterfaceIp} data-ip-index={ip_index} data-interface-index={i_index} class="btn btn-danger" aria-label="Remove IP"><i class="bi bi-dash-lg"></i></button>
                                        </div>
                                    {/each}
                                    <button onclick={addNodeInterfaceIp} value={i_index} class="btn btn-outline-secondary" aria-label="Add interface ip"><i class="bi bi-plus-lg"></i></button>
                                </div>
                            </div>
                        </div>
                    {/each}
                    <button onclick={addNodeInterface} class="align-items-stretch btn btn-outline-secondary" aria-label="Add interface"><i class="bi bi-plus-lg"></i></button>
                </div>
                <hr>
                <p class="fs-5">Services</p>
                <div class="d-flex align-content-start flex-wrap gap-2">
                    {#each Object.entries(nodeServices) as [s_index, s]}
                        <div class="hstack">
                            <button onclick={removeNodeService} value={s_index} class="rounded-end-0 btn btn-danger align-self-stretch" aria-label="Close"><i class="bi bi-x-lg"></i></button>
                            <div class="rounded-start-0 card p-2 vstack">
                                <div class="hstack gap-2">
                                    <div class="mb-3">
                                        <label for="exampleInputPassword1" class="form-label">Name</label>
                                        <input value={s.name} onchange={changeNodeServiceName} data-service-index={s_index} type="text" class="form-control">
                                    </div>
                                    <div class="mb-3">
                                        <label for="exampleInputPassword1" class="form-label">Transport protocol</label>
                                        <select value={s.transport_protocol} onchange={changeNodeServiceTransportProtocol} data-service-index={s_index} class="form-select" aria-label="Service transport protocol">
                                            <option value="TCP">TCP</option>
                                            <option value="UDP">UDP</option>
                                        </select>
                                    </div>
                                </div>
                                <div class="hstack gap-2">
                                    <div class="mb-3">
                                        <label for="exampleInputPassword1" class="form-label">IP</label>
                                        <input value={s.ip} onchange={changeNodeServiceIp} data-service-index={s_index} type="text" class="form-control">
                                    </div>
                                    <div class="mb-3">
                                        <label for="exampleInputPassword1" class="form-label">Port</label>
                                        <input value={s.port} onchange={changeNodeServicePort} data-service-index={s_index} type="number" class="form-control">
                                    </div>
                                </div>
                            </div>
                        </div>
                    {/each}
                    <button onclick={addNodeService} class="align-items-stretch btn btn-outline-secondary" aria-label="Add service"><i class="bi bi-plus-lg"></i></button>
                </div>
            </div>
            <div class="modal-footer">
                <button onclick={addNode} type="submit" class="btn btn-success">Create</button>
            </div>
            </form>
        </div>
    </div>
</div>