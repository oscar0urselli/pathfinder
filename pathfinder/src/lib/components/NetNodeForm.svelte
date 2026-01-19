<script lang="ts">
    import type { NetNode, NetNodeInterface, NetNodeService } from "$lib/schema";
    import { invoke } from "@tauri-apps/api/core";
    import { toast } from "svelte-sonner";

    let { formMode, submitForm, nodeIndex, node }: { formMode: "edit" | "add", submitForm: any, nodeIndex: number | undefined, node: NetNode | undefined } = $props();
    
    async function submit(event: any) {
        let formData = new FormData(nodeForm);
        
        let netNode: NetNode = {
            name: formData.get("name") || "Node",
            type: formData.get("type") || "Unknown",
            interfaces: nodeInterfaces,
            services: nodeServices
        };
        
        if (formMode === "edit") {
            await invoke("edit_net_node", { index: nodeIndex, node: netNode });
        }
        else {
            await invoke("add_net_node", { node: netNode });
        }
        
        submitForm();
        
        
        if (formMode === "edit") {
            toast.success("Updated node of the graph.");
        }
        else {
            toast.success("Inserted new node in the graph.");
        }
    }
    
    function addNodeInterface(event: any) {
        const minCeiled = Object.keys(nodeInterfaces).length;
        const maxFloored = Math.pow(2, 32);

        nodeInterfaces["interface " + String(Math.floor(Math.random() * (maxFloored - minCeiled) + minCeiled))] = {
            mac: "ff:ff:ff:ff:ff:ff",
            ips: []
        };
    }
    
    function removeNodeInterface(event: any) {
        delete nodeInterfaces[event.currentTarget.value];
    }
    
    function changeNodeInterfaceName(event: any) {
        let old_i_index = event.currentTarget.getAttribute("data-interface-index");
        nodeInterfaces[event.currentTarget.value] = nodeInterfaces[old_i_index];
        delete nodeInterfaces[old_i_index];
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
    
    let nodeInterfaces: { [key: string]: NetNodeInterface } = $derived(node !== undefined ? node.interfaces : {});
    let nodeServices: NetNodeService[] = $derived(node !== undefined ? node.services : []);
    let nodeForm: HTMLFormElement;
</script>

<!-- Node modal with form -->
<div class="modal fade" id="{formMode}-node" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
    <div class="modal-dialog modal-xl">
        <div class="modal-content">
            <div class="modal-header">
                <h1 class="modal-title fs-5" id="exampleModalLabel">
                {#if formMode === "edit"}
                Edit node
                {:else}
                Add new node
                {/if}
                </h1>
                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
            </div>
            <form bind:this={nodeForm}>
            <div class="modal-body">
                <div class="row">
                    <div class="col-8 mb-3">
                        <label for="exampleInputPassword1" class="form-label">Name</label>
                        <input value={node?.name} name="name" type="text" class="form-control">
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
                                    <label for="exampleInputPassword1" class="form-label">Name</label>
                                    <input value={i_index} onchange={changeNodeInterfaceName} data-interface-index={i_index} type="text" class="form-control">
                                </div>
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
                <button onclick={submit} type="submit" class="btn btn-success">
                {#if  formMode === "edit"}
                Edit
                {:else}
                Create
                {/if}
                </button>
            </div>
            </form>
        </div>
    </div>
</div>