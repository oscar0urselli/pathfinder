<script lang="ts">
	// Bootrstrap CSS
    import "bootstrap/dist/css/bootstrap.min.css";
    // Bottstrap Icons
    import "bootstrap-icons/font/bootstrap-icons.min.css";
    // Bootrstrap JS
    import scriptSrc from "bootstrap/dist/js/bootstrap.bundle.min.js?url";
    
	import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { loadedReport } from "$lib/state.svelte";

	let { children } = $props();

	onMount(async () => {
		const tooltipTriggerList = document.querySelectorAll('[data-bs-toggle="tooltip"]')
		const tooltipList = [...tooltipTriggerList].map(tooltipTriggerEl => new bootstrap.Tooltip(tooltipTriggerEl));
		
		loadedReport.report = await invoke("get_loaded_report");
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

<div class="vh-100 w-100" style="background-color: #eeeeee;">
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
                <p>Last access: {new Date(loadedReport.report?.last_access_tsz / 1000).toLocaleString()}</p>
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