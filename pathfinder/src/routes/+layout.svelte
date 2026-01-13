<script lang="ts">
	// Bootrstrap CSS
    import "bootstrap/dist/css/bootstrap.min.css";
    // Bottstrap Icons
    import "bootstrap-icons/font/bootstrap-icons.min.css";
    // Bootrstrap JS
    import scriptSrc from "bootstrap/dist/js/bootstrap.bundle.min.js?url";
    
	import { mount, onMount, unmount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { activePlugins, loadedReport, plugins, settings } from "$lib/state.svelte";
    import { listen } from "@tauri-apps/api/event";
    import type { PluginFormData, PluginFormConfig, Toast, ActivePlugins } from "$lib/schema";
    import { Toaster, toast } from "svelte-sonner";
    import PluginForm from "$lib/components/PluginForm.svelte";
    import { afterNavigate } from "$app/navigation";

	let { children } = $props();

	onMount(async () => {
		loadedReport.report = await invoke("get_loaded_report");
		settings.s = await invoke("get_settings");
		plugins.p = await invoke("get_plugins");
	});
	
	listen<Toast>("toast", async (event) => {
        switch (event.payload.alert_type) {
            case "success":
                toast.success(event.payload.text);
                break;
            case "info":
                toast.info(event.payload.text);
                break;
            case "warning":
                toast.warning(event.payload.text);
                break;
            case "danger":
                toast.error(event.payload.text);
                break;
            case "none":
                toast(event.payload.text);
                break;
        }
        activePlugins.p = await invoke("get_active_plugins");
	});
	
	function unmountForm() {
        unmount(form);
	}
	
	let form;
	listen<PluginFormData>("form", (event) => {
	    form = mount(PluginForm, {
			target: document.body,
			props: {
                config: event.payload.config,
                plugin: event.payload.name,
                destroyForm: unmountForm
			}
		});
	});
	
	listen<ActivePlugins>("active_plugins", (event) => {
	    activePlugins.p = event.payload;
	});
	
	afterNavigate(async (navigation) => {
	    loadedReport.report = await invoke("get_loaded_report");
					
		activePlugins.p = await invoke("get_active_plugins");
	});
	
	async function updateActivePlugins(event: any) {
	    activePlugins.p = await invoke("get_active_plugins");
	}
	
	async function terminatePlugin(event: any) {
	    await invoke("terminate_plugin", { plugin: event.currentTarget.value });
	}
</script>

<svelte:head>
	<script src={scriptSrc}></script>
</svelte:head>

<div class="z-1 position-absolute m-2 vstack gap-2">
	<div class="card border-0 shadow-lg p-2 vstack gap-2">
		<a href="/" role="button" class="btn btn-primary" aria-label="Home"><i class="bi bi-house-fill"></i></a>
		<a href="/map" role="button" class="btn btn-primary" aria-label="Map"><i class="bi bi-diagram-3-fill"></i></a>
		<a href="/database" role="button" class="btn btn-primary" aria-label="Database"><i class="bi bi-database-fill"></i></a>
		<a href="/plugins" role="button" class="btn btn-primary" aria-label="Plugins"><i class="bi bi-box-seam-fill"></i></a>
		<a href="/settings" role="button" class="btn btn-primary" aria-label="Settings"><i class="bi bi-gear-fill"></i></a>
	</div>
	{#if loadedReport.report}
	<div class="card border-0 shadow-lg p-2 vstack gap-2">
		<button type="button" class="btn btn-outline-success" data-bs-toggle="modal" data-bs-target="#report-info" aria-label="Report"><i class="bi bi-file-earmark-fill"></i></button>
	</div>
	{/if}
</div>

<div class="z-1 position-absolute bottom-0 start-0 m-2 vstack gap-2">
   	<div class="card border-0 shadow-lg p-2 vstack gap-2">
		<button onclick={updateActivePlugins} type="button" class="btn btn-secondary" data-bs-toggle="modal" data-bs-target="#plugins-manager" aria-label="Report"><i class="bi bi-cpu-fill"></i></button>
        <button type="button" class="btn btn-secondary" data-bs-toggle="modal" data-bs-target="#logs-console" aria-label="Report"><i class="bi bi-terminal-fill"></i></button>
	</div>
</div>

<Toaster richColors closeButton position={settings.s.notification_pos} expand={true} />

<div class="vh-100 w-100 overflow-scroll" style="background-color: #eeeeee;">
	{@render children()}
</div>

<!-- Plugins manager modal -->
<div class="modal fade" id="plugins-manager" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
    <div class="modal-dialog">
        <div class="modal-content">
            <div class="modal-header">
                <h1 class="modal-title fs-5" id="exampleModalLabel">Plugins Manager</h1>
                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
            </div>
            <div class="modal-body">
                <ul class="list-group">
                    {#each Object.keys(plugins.p) as p_name}
                        <li class="list-group-item">
                            <div class="hstack gap-2">
                                <p class="my-0">{p_name}</p>
                                {#if activePlugins.p[p_name] !== undefined}
                                    <span class="badge text-bg-secondary">{activePlugins.p[p_name]}</span>
                                    {#if activePlugins.p[p_name] === "Running" || activePlugins.p[p_name] === "WaitingForm"}
                                        <button onclick={terminatePlugin} value={p_name} class="ms-auto btn btn-sm btn-danger" aria-label="Stop plugin"><i class="bi bi-stop-fill"></i></button>
                                    {:else if activePlugins.p[p_name] === "Exiting"}
                                        <button class="ms-auto btn btn-sm btn-warning" aria-label="Stop plugin" disabled><i class="bi bi-hourglass-split"></i></button>
                                    {/if}
                                {:else}
                                    <span class="badge text-bg-secondary">Not started</span>
                                {/if}
                            </div>
                        </li>
                    {/each}
                </ul>
            </div>
        </div>
    </div>
</div>

<!-- Report info modal -->
<div class="modal fade" id="report-info" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
    <div class="modal-dialog">
        <div class="modal-content">
            <div class="modal-header">
                <h1 class="modal-title fs-5" id="exampleModalLabel">Report info</h1>
                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
            </div>
            <div class="modal-body">
                <p>ID: {loadedReport.report?.id}</p>
                <p>Last access: {loadedReport.report ? new Date(loadedReport.report?.last_access_tsz / 1000).toLocaleString() : undefined}</p>
                <p>Title: {loadedReport.report?.title}</p>
                <p>Author: {loadedReport.report?.author}</p>
                <p>Device: {loadedReport.report?.device}</p>
                <p>Place: {loadedReport.report?.place}</p>
            </div>
            <div class="modal-footer">
                <!--<button type="button" class="btn btn-primary">Save changes</button>-->
            </div>
        </div>
    </div>
</div>