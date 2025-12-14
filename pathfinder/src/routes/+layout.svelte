<script lang="ts">
	// Bootrstrap CSS
    import "bootstrap/dist/css/bootstrap.min.css";
    // Bottstrap Icons
    import "bootstrap-icons/font/bootstrap-icons.min.css";
    // Bootrstrap JS
    import scriptSrc from "bootstrap/dist/js/bootstrap.bundle.min.js?url";
    
	import { onMount } from "svelte";

	let { children } = $props();

	onMount(() => {
		const tooltipTriggerList = document.querySelectorAll('[data-bs-toggle="tooltip"]')
		const tooltipList = [...tooltipTriggerList].map(tooltipTriggerEl => new bootstrap.Tooltip(tooltipTriggerEl))
	});
</script>

<svelte:head>
	<script src={scriptSrc}></script>
</svelte:head>

<div class="z-1 position-absolute m-2">
	<div class="card border-0 shadow-lg p-2 vstack gap-2">
		<a href="/" role="button" class="btn btn-primary" data-bs-toggle="tooltip" data-bs-placement="right" data-bs-custom-class="custom-tooltip" data-bs-title="Home" aria-label="Home"><i class="bi bi-house-fill"></i></a>
		<a href="/map" role="button" class="btn btn-primary" data-bs-toggle="tooltip" data-bs-placement="right" data-bs-custom-class="custom-tooltip" data-bs-title="Map" aria-label="Map"><i class="bi bi-diagram-3-fill"></i></a>
		<a href="/database" role="button" class="btn btn-primary" data-bs-toggle="tooltip" data-bs-placement="right" data-bs-custom-class="custom-tooltip" data-bs-title="Database" aria-label="Database"><i class="bi bi-database-fill"></i></a>
		<a href="/plugins" role="button" class="btn btn-primary" data-bs-toggle="tooltip" data-bs-placement="right" data-bs-custom-class="custom-tooltip" data-bs-title="Plugins" aria-label="Plugins"><i class="bi bi-box-seam-fill"></i></a>
		<a href="/settings" role="button" class="btn btn-primary" data-bs-toggle="tooltip" data-bs-placement="right" data-bs-custom-class="custom-tooltip" data-bs-title="Settings" aria-label="Settings"><i class="bi bi-gear-fill"></i></a>
	</div>
</div>

<div class="vh-100 w-100" style="background-color: #eeeeee;">
	{@render children()}
</div>