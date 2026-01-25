<script lang="ts">
    import type { Plugin, PluginFormConfig } from "$lib/schema";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let { title, config, plugin, closeFormCallback, hideFormCallback }: { title: string, config: PluginFormConfig, plugin: string, closeFormCallback: any, hideFormCallback: any } = $props();
    
    const patterns = {
        str: undefined,
        float: undefined,
        int: undefined,
        ipv4: "(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)",
        ipv4_cidr: "(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\/(?:3[0-2]|2[0-9]|1[0-9]|[0]?[0-9])",
        ipv6: undefined,
        ipv6_cidr: undefined,
        mac: "(([0-9A-Fa-f]{2}[-:]){5}[0-9A-Fa-f]{2})|(([0-9A-Fa-f]{4}\.){2}[0-9A-Fa-f]{4})"
    };
    
    async function submitForm(event: any) {
        let formData = new FormData(form);
        
        let fields: {
            [key: string]: boolean | number | string | null | any
        } = {};
        for (let i = 0; i < config.length; i++) {
            for (let j = 0; j < config[i].length; j++) {
                if (config[i][j].type === "bool") {
                    fields[config[i][j].name] = formData.get(config[i][j].name) !== null;
                }
                else if (config[i][j].type === "int" || config[i][j].type === "float") {
                    fields[config[i][j].name] = Number(formData.get(config[i][j].name));
                }
                else {
                    fields[config[i][j].name] = formData.get(config[i][j].name);
                }
            }
        }
        
        await invoke("send_plugin_form_res", { plugin, params: JSON.stringify(fields) });
        
        modal.hide();
        closeFormCallback();
    }
    
    async function closeForm(event: any) {
        await invoke("terminate_plugin", { plugin });
      
        modal.hide();
        closeFormCallback();
    }
    
    async function hideForm(event: any) {
        modal.hide();
        hideFormCallback(
            title,
            "PluginForm",
            {
                "PluginForm": {
                    config
                }
            },
            plugin
        );
    }
    
    let form: HTMLFormElement;
    let modalEl: HTMLDivElement;
    let modal: any;
    onMount(() => {
        modal = new bootstrap.Modal(modalEl);
        modal.show();
    });
</script>

<div bind:this={modalEl} class="modal fade" data-bs-backdrop="static" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
    <div class="modal-dialog modal-lg">
        <div class="modal-content">
            <div class="modal-header">
                <h1 class="modal-title fs-5" id="exampleModalLabel">{title} - {plugin}</h1>
                <button onclick={closeForm} type="button" class="btn-close" aria-label="Close"></button>
            </div>
            <form bind:this={form}>
            <div class="modal-body">
                {#each config as i}
                    <div class="row">
                        {#each i as f}
                            <div class="col mb-3">
                                {#if f.type === "bool"}
                                    <div class="form-check">
                                        <input class="form-check-input" type="checkbox" value="" name={f.name} checked={f.default === "True" ? true : false}>
                                        <label class="form-check-label" for="formFieldLabel">{f.title}</label>
                                    </div>
                                {:else}
                                    {#if f.options === null}
                                        {#if ["str", "ipv4", "ipv4_cidr", "ipv6", "ipv6_cidr", "mac"].includes(f.type)}
                                        <label for="formFieldLabel" class="form-label">{f.title}</label>
                                        <input value={f.default} name={f.name} pattern={patterns[f.type] ? patterns[f.type] : f.regex} type="text" class="form-control">
                                        {:else if f.type === "float"}
                                        <label for="formFieldLabel" class="form-label">{f.title}</label>
                                        <input value={f.default} name={f.name} min={f.min} max={f.max} step={f.step} type="number" class="form-control">
                                        {:else if f.type === "int"}
                                        <label for="formFieldLabel" class="form-label">{f.title}</label>
                                        <input value={f.default} name={f.name} min={f.min} max={f.max} step="1" type="number" class="form-control">
                                        {/if}
                                    {:else}
                                        <label for="formFieldLabel" class="form-label">{f.title}</label>
                                        <select name={f.name} class="form-select">
                                            {#each f.options as v}
                                                <option value={v} selected={v === f.default}>{v}</option>
                                            {/each}
                                        </select>
                                    {/if}
                                {/if}
                            </div>
                        {/each}
                    </div>
                    
                {/each}
            </div>
            <div class="modal-footer">
                <button onclick={hideForm} type="submit" class="btn btn-secondary">Hide</button>
                <button onclick={submitForm} type="submit" class="btn btn-success">Send</button>
            </div>
            </form>
        </div>
    </div>
</div>