<script lang="ts">
    import type { NetNode, NetNodeServiceBinding } from "$lib/schema";
    import { NodeSquareProgram } from "@sigma/node-square";
    import { invoke } from "@tauri-apps/api/core";
    import Graph from "graphology";
    import ForceSupervisor from "graphology-layout-force/worker";
    import { Sigma } from "sigma";
    import { DEFAULT_NODE_PROGRAM_CLASSES } from "sigma/settings";
    import MACInput from "./MACInput.svelte";
    

    let { index, node }: { index: number, node: NetNode } = $props();
    
    let interfaceName: string = $state("");
    let interfaceMac: string = $state("");
    let interfaceIps: string[] = $state([]);
    
    let serviceName: string = $state("");
    let serviceBindings: NetNodeServiceBinding[] = $state([]);
    
    let sigmaGraphEl: HTMLDivElement;
    $effect(() => {
        const sigmaGraph = new Graph();
        let i = 0;
        for (const [i_name, iface] of Object.entries(node.interfaces)) {
            const angle = (Number(i) * 2 * Math.PI) / (Number(i) + 1);
            sigmaGraph.addNode(i_name, {
                label: i_name,
                size: 30,
                x: 100 * Math.cos(angle),
                y: 100 * Math.sin(angle),
                type: "square",
                color: "gray",
                pictoColor: "black",
            });
            i += 1;
        }
        
        for (const service of node.services) {
            const angle = (Number(i) * 2 * Math.PI) / (Number(i) + 1);
            sigmaGraph.addNode(service.name, {
                label: service.name,
                size: 30,
                x: 100 * Math.cos(angle),
                y: 100 * Math.sin(angle),
                color: "gray",
                pictoColor: "black",
            });
            i += 1;
        }
        
        const sigmaRenderer = new Sigma(sigmaGraph, sigmaGraphEl, {
            allowInvalidContainer: true,
            nodeProgramClasses: {
                ...DEFAULT_NODE_PROGRAM_CLASSES,
                square: NodeSquareProgram
            }
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
    
    function addInterfaceIp(event: any) {
        interfaceIps.push("");
    }
    
    function changeInterfaceIp(event: any) {
        interfaceIps[event.currentTarget.getAttribute("data-ip-index")] = event.currentTarget.value;
    }
    
    function removeInterfaceIp(event: any) {
        interfaceIps.splice(Number(event.currentTarget.getAttribute("data-ip-index")), 1);
    }
    
    async function addInterface(event: any) {
        node.interfaces[interfaceName] = {
            mac: interfaceMac,
            ips: [...interfaceIps]
        };
        await invoke("edit_net_node", { index, node });
        
        interfaceName = "";
        interfaceMac = "";
        interfaceIps = [];
    }
    
    function addServiceBinding(event: any) {
        serviceBindings.push({
            ip: "",
            port: 55555,
            protocol: "Tcp"
        });
    }
    
    function changeServiceBindingIp(event: any) {
        serviceBindings[event.currentTarget.getAttribute("data-b-index")].ip = event.currentTarget.value;
    }
    
    function changeServiceBindingPort(event: any) {
        serviceBindings[event.currentTarget.getAttribute("data-b-index")].port = Number(event.currentTarget.value);
    }
    
    function changeServiceBindingProtocol(event: any) {
        serviceBindings[event.currentTarget.getAttribute("data-b-index")].protocol = event.currentTarget.value;
    }
    
    function removeServiceBinding(event: any) {
        serviceBindings.splice(Number(event.currentTarget.getAttribute("data-b-index")), 1);
    }
    
    async function addService(event: any) {
        node.services.push({
            name: serviceName,
            bindings: [...serviceBindings]
        });
        await invoke("edit_net_node", { index, node });
      
        serviceName = "";
        serviceBindings = [];
    }
</script>

<div class="modal fade" id="info-node" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
    <div class="modal-dialog modal-xl modal-dialog-scrollable">
        <div class="modal-content">
            <div class="modal-header">
                <h1 class="modal-title fs-5" id="exampleModalLabel">{node.name} #{index}</h1>
                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
            </div>
            <div class="modal-body py-0 ps-0">
                <div class="row">
                    <div class="col px-0 border-end mh-100">
                        <div bind:this={sigmaGraphEl} class="h-100"></div>
                    </div>
                    <div class="col py-1">
                        <p class="fs-5 mb-0">Interfaces</p>
                        <table class="table">
                            <thead>
                                <tr>
                                    <th scope="col">Name</th>
                                    <th scope="col">MAC</th>
                                    <th scope="col">IPs</th>
                                </tr>
                            </thead>
                            <tbody>
                                {#each Object.entries(node.interfaces) as [iface_name, iface]}
                                    <tr>
                                        <td>{iface_name}</td>
                                        <td>{iface.mac}</td>
                                        <td>{iface.ips.toString()}</td>
                                    </tr>
                                {/each}
                            </tbody>
                        </table>
                        <p class="fs-5 mb-0">Add interface</p>
                        <form class="mb-3">
                            <div class="mb-3">
                                <label for="exampleInputPassword1" class="form-label">Name</label>
                                <input bind:value={interfaceName} min="1" type="text" class="form-control">
                            </div>
                            <!--<div class="mb-3">
                                <label for="exampleInputPassword1" class="form-label">MAC</label>
                                <input bind:value={interfaceMac} type="text" class="form-control">
                                </div>-->
                            <MACInput />
                            <p class="my-0">IPs</p>
                            <div class="d-flex align-content-start flex-wrap gap-2 mb-3">
                                {#each Object.entries(interfaceIps) as [ip_index, ip]}
                                    <div class="input-group w-auto">
                                        <input value={ip} onchange={changeInterfaceIp} data-ip-index={ip_index} type="text" class="form-control">
                                        <button onclick={removeInterfaceIp} data-ip-index={ip_index} class="btn btn-danger" aria-label="Remove IP"><i class="bi bi-dash-lg"></i></button>
                                    </div>
                                {/each}
                                <button onclick={addInterfaceIp} class="btn btn-outline-secondary" aria-label="Add interface ip"><i class="bi bi-plus-lg"></i></button>
                            </div>
                            <button type="submit" onclick={addInterface} class="btn btn-success" aria-label="Add interface">Add interface</button>
                        </form>
                        <hr>
                        <p class="fs-5 mb-0">Services</p>
                        <table class="table">
                            <thead>
                                <tr>
                                    <th scope="col">Name</th>
                                    <th scope="col">Bindings</th>
                                </tr>
                            </thead>
                            <tbody>
                                {#each node.services as service}
                                    <tr>
                                        <td>{service.name}</td>
                                        <td>{service.bindings.map((v) => `${v.ip}:${v.port}/${v.protocol}`)}</td>
                                    </tr>
                                {/each}
                            </tbody>
                        </table>
                        <p class="fs-5 mb-0">Add service</p>
                        <form class="mb-3">
                            <div class="mb-3">
                                <label for="exampleInputPassword1" class="form-label">Name</label>
                                <input bind:value={serviceName} type="text" class="form-control">
                            </div>
                            <p class="my-0">Bindings</p>
                            <div class="d-flex align-content-start flex-wrap gap-2 mb-3">
                                {#each Object.entries(serviceBindings) as [b_index, b]}
                                    <div class="input-group w-auto">
                                        <input value={b.ip} onchange={changeServiceBindingIp} data-b-index={b_index} type="text" class="form-control">
                                        <input value={b.port} onchange={changeServiceBindingPort} data-b-index={b_index} type="text" class="form-control">
                                        <select value={b.protocol} onchange={changeServiceBindingProtocol} data-b-index={b_index} class="form-select" aria-label="Service binding protocol select">
                                            <option value="Tcp">TCP</option>
                                            <option value="Udp">UDP</option>
                                        </select>
                                        <button onclick={removeServiceBinding} data-b-index={b_index} class="btn btn-danger" aria-label="Remove IP"><i class="bi bi-dash-lg"></i></button>
                                    </div>
                                {/each}
                                <button onclick={addServiceBinding} class="btn btn-outline-secondary" aria-label="Add service binding"><i class="bi bi-plus-lg"></i></button>
                            </div>
                            <button onclick={addService} class="btn btn-success" aria-label="Add service">Add service</button>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>