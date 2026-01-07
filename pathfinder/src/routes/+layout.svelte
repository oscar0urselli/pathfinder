<script lang="ts">
	// Bootrstrap CSS
    import "bootstrap/dist/css/bootstrap.min.css";
    // Bottstrap Icons
    import "bootstrap-icons/font/bootstrap-icons.min.css";
    // Bootrstrap JS
    import scriptSrc from "bootstrap/dist/js/bootstrap.bundle.min.js?url";
    
	import { mount, onMount, unmount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { loadedReport, settings } from "$lib/state.svelte";
    import { listen } from "@tauri-apps/api/event";
    import type { PluginFormData, PluginFormConfig, Toast } from "$lib/schema";
    import { Toaster, toast } from "svelte-sonner";
    import PluginForm from "$lib/components/PluginForm.svelte";

	let { children } = $props();

	onMount(async () => {
		const tooltipTriggerList = document.querySelectorAll('[data-bs-toggle="tooltip"]')
		const tooltipList = [...tooltipTriggerList].map(tooltipTriggerEl => new bootstrap.Tooltip(tooltipTriggerEl));
		
		loadedReport.report = await invoke("get_loaded_report");
		settings.s = await invoke("get_settings");
	});
	
	listen<Toast>("toast", (event) => {
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
                closeForm: unmountForm
			}		
		});
	});
</script>

<svelte:head>
	<script src={scriptSrc}></script>
</svelte:head>

<div class="z-1 position-absolute m-2 vstack gap-2">
	<div class="card border-0 shadow-lg p-2 vstack gap-2">
		<a href="/" role="button" class="btn btn-primary" data-bs-toggle="tooltip" data-bs-placement="right" data-bs-custom-class="custom-tooltip" data-bs-title="Home" aria-label="Home"><i class="bi bi-house-fill"></i></a>
		<a href="/map" role="button" class="btn btn-primary" data-bs-toggle="tooltip" data-bs-placement="right" data-bs-custom-class="custom-tooltip" data-bs-title="Map" aria-label="Map"><i class="bi bi-diagram-3-fill"></i></a>
		<a href="/database" role="button" class="btn btn-primary" data-bs-toggle="tooltip" data-bs-placement="right" data-bs-custom-class="custom-tooltip" data-bs-title="Database" aria-label="Database"><i class="bi bi-database-fill"></i></a>
		<a href="/plugins" role="button" class="btn btn-primary" data-bs-toggle="tooltip" data-bs-placement="right" data-bs-custom-class="custom-tooltip" data-bs-title="Plugins" aria-label="Plugins"><i class="bi bi-box-seam-fill"></i></a>
		<a href="/settings" role="button" class="btn btn-primary" data-bs-toggle="tooltip" data-bs-placement="right" data-bs-custom-class="custom-tooltip" data-bs-title="Settings" aria-label="Settings"><i class="bi bi-gear-fill"></i></a>
	</div>
	{#if loadedReport.report}
	<div class="card border-0 shadow-lg p-2 vstack gap-2">
		<button type="button" class="btn btn-outline-success" data-bs-toggle="modal" data-bs-target="#report-info" aria-label="Report"><i class="bi bi-file-earmark-fill"></i></button>
	</div>
	{/if}
</div>

<Toaster richColors closeButton position={settings.s.notification_pos} expand={true} />

<div class="vh-100 w-100 overflow-scroll" style="background-color: #eeeeee;">
	{@render children()}
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