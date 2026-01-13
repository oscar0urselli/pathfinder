<script lang="ts">
    import type { Plugin, PluginFormConfig } from "$lib/schema";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let { config, plugin, destroyForm }: { config: PluginFormConfig, plugin: string, destroyForm: any } = $props();
    
    const patterns = {
        ipv4: "(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)",
        ipv4_cidr: "(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\/(?:3[0-2]|2[0-9]|1[0-9]|[0]?[0-9])",
        ipv6: undefined,
        ipv6_cidr: undefined,
        mac: "(([0-9A-Fa-f]{2}[-:]){5}[0-9A-Fa-f]{2})|(([0-9A-Fa-f]{4}\.){2}[0-9A-Fa-f]{4})"
    };
    
    async function submitForm(event: any) {
        let formData = new FormData(form);
        
        let fields: any = {};
        for (let f of Object.keys(config)) {
            if (config[f].type === "bool") {
                fields[f] = formData.get(f) !== null;
            }
            else if (config[f].type === "int" || config[f].type === "float") {
                fields[f] = Number(formData.get(f));
            }
            else {
                fields[f] = formData.get(f);
            }
        }
        
        await invoke("send_plugin_form_res", { plugin, params: JSON.stringify(fields) });
        
        modal.hide();
        destroyForm();
    }
    
    async function closeForm(event: any) {
        await invoke("terminate_plugin", { plugin });
      
        modal.hide();
        destroyForm();
    }
    
    let form: HTMLFormElement;
    let modalEl: HTMLDivElement;
    let modal;
    onMount(() => {
        modal = new bootstrap.Modal(modalEl);
        modal.show();
        console.log(config);
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
                        {#if fieldConfig.type === "bool"}
                            <div class="form-check">
                                <input class="form-check-input" type="checkbox" value="" id={fieldName} name={fieldName} checked={fieldConfig.default === "true" ? true : false}>
                                <label class="form-check-label" for={fieldName}>{fieldConfig.title}</label>
                            </div>
                        {:else}
                            {#if fieldConfig.options === null}
                                {#if ["str", "ipv4", "ipv4_cidr", "ipv6", "ipv6_cidr", "mac"].includes(fieldConfig.type)}
                                <label for={fieldName} class="form-label">{fieldConfig.title}</label>
                                <input id={fieldName} value={fieldConfig.default} name={fieldName} pattern={patterns[fieldConfig.type] ? patterns[fieldConfig.type] : fieldConfig.regex} type="text" class="form-control">
                                {:else if fieldConfig.type === "float"}
                                <label for={fieldName} class="form-label">{fieldConfig.title}</label>
                                <input id={fieldName} value={fieldConfig.default} name={fieldName} min={fieldConfig.min} max={fieldConfig.max} step={fieldConfig.step} type="number" class="form-control">
                                {:else if fieldConfig.type === "int"}
                                <label for={fieldName} class="form-label">{fieldConfig.title}</label>
                                <input id={fieldName} value={fieldConfig.default} name={fieldName} min={fieldConfig.min} max={fieldConfig.max} step="1" type="number" class="form-control">
                                {/if}
                            {:else}
                                <label for={fieldName} class="form-label">{fieldConfig.title}</label>
                                <select id={fieldName} name={fieldName} class="form-select">
                                    {#each fieldConfig.options as i}
                                        <option value={i} selected={i === fieldConfig.default}>{i}</option>
                                    {/each}
                                </select>
                            {/if}
                        {/if}
                    </div>
                {/each}
            </div>
            <div class="modal-footer">
                <button onclick={submitForm} type="submit" class="btn btn-success">Send</button>
            </div>
            </form>
        </div>
    </div>
</div>