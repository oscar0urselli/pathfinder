<script lang="ts">
    import type { Plugin, PluginFormConfig } from "$lib/schema";
    import { onMount } from "svelte";

    let { config, plugin }: { config: PluginFormConfig, plugin: string } = $props();
    
    async function submitForm(event: any) {
        let formData = new FormData(form);
        
        let fields = {};
        for (let f of Object.keys(config)) {
            console.log(f);
            //formData.get(f) !== null
        }
        modal.dispose();
    }
    
    async function closeForm(event: any) {
        
    }
    
    let form: HTMLFormElement;
    let modalEl: HTMLDivElement;
    let modal;
    onMount(() => {
        modal = new bootstrap.Modal(modalEl);
        modal.show();
    });
</script>

<div bind:this={modalEl} class="modal fade" data-bs-backdrop="static" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
    <div class="modal-dialog modal-lg">
        <div class="modal-content">
            <div class="modal-header">
                <h1 class="modal-title fs-5" id="exampleModalLabel">{plugin}</h1>
                <button onclick={closeForm} type="button" class="btn-close" aria-label="Close"></button>
            </div>
            <form bind:this={form}>
            <div class="modal-body">
                {#each Object.entries(config) as [fieldName, fieldConfig]}
                    <div class="mb-3">
                        {#if fieldConfig.type === "str"}
                        <label for={fieldName} class="form-label">{fieldConfig.title}</label>
                        <input id={fieldName} value={fieldConfig.default} name={fieldName} pattern={fieldConfig.regex} type="text" class="form-control">
                        {:else if fieldConfig.type === "ipv4"}
                        <label for={fieldName} class="form-label">{fieldConfig.title}</label>
                        <input id={fieldName} value={fieldConfig.default} name={fieldName} pattern="(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)" type="text" class="form-control">
                        {:else if fieldConfig.type === "ipv4_cidr"}
                        <label for={fieldName} class="form-label">{fieldConfig.title}</label>
                        <input id={fieldName} value={fieldConfig.default} name={fieldName} pattern="(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\/(?:3[0-2]|2[0-9]|1[0-9]|[0]?[0-9])" type="text" class="form-control">
                        {:else if fieldConfig.type === "ipv6"}
                        <label for={fieldName} class="form-label">{fieldConfig.title}</label>
                        <input id={fieldName} value={fieldConfig.default} name={fieldName} type="text" class="form-control">
                        {:else if fieldConfig.type === "ipv6_cidr"}
                        <label for={fieldName} class="form-label">{fieldConfig.title}</label>
                        <input id={fieldName} value={fieldConfig.default} name={fieldName} type="text" class="form-control">
                        {:else if fieldConfig.type === "mac"}
                        <label for={fieldName} class="form-label">{fieldConfig.title}</label>
                        <input id={fieldName} value={fieldConfig.default} name={fieldName} pattern="(([0-9A-Fa-f]{2}[-:]){5}[0-9A-Fa-f]{2})|(([0-9A-Fa-f]{4}\.){2}[0-9A-Fa-f]{4})" type="text" class="form-control">
                        {:else if fieldConfig.type === "float"}
                        <label for={fieldName} class="form-label">{fieldConfig.title}</label>
                        <input id={fieldName} value={fieldConfig.default} name={fieldName} min={fieldConfig.min} max={fieldConfig.max} step={fieldConfig.step} type="number" class="form-control">
                        {:else if fieldConfig.type === "int"}
                        <label for={fieldName} class="form-label">{fieldConfig.title}</label>
                        <input id={fieldName} value={fieldConfig.default} name={fieldName} min={fieldConfig.min} max={fieldConfig.max} step="1" type="number" class="form-control">
                        {:else if fieldConfig.type === "bool"}
                        <div class="form-check">
                            <input class="form-check-input" type="checkbox" value="" id={fieldName} name={fieldName} checked={fieldConfig.default}>
                            <label class="form-check-label" for={fieldName}>{fieldConfig.title}</label>
                        </div>
                        {/if}
                    </div>
                {/each}
            </div>
            <div class="modal-footer">
                <button onclick={submitForm} type="submit" class="btn btn-success">Run</button>
            </div>
            </form>
        </div>
    </div>
</div>