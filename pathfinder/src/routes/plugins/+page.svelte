<script lang="ts">
    import type { Plugin } from "$lib/schema";
    import { plugins } from "$lib/state.svelte";
    import { path } from "@tauri-apps/api";
    import { invoke } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";
    import { BaseDirectory, readTextFile } from "@tauri-apps/plugin-fs";
    import { openPath } from "@tauri-apps/plugin-opener";
    import { onMount } from "svelte";

    async function viewPluginDetails(event: any) {
        selectedPlugin = plugins.p[event.currentTarget.value];
        pluginDetailsView = "readme";
        if (selectedPlugin !== undefined) {
            const readmeFile = await path.join("plugins", await path.join(selectedPlugin.folder, "README.md"));
            selectedReadme = await readTextFile(readmeFile, {
                baseDir: BaseDirectory.AppLocalData
            });
            
            const licenseFile = await path.join("plugins", await path.join(selectedPlugin.folder, "LICENSE"));
            selectedLicense = await readTextFile(licenseFile, {
                baseDir: BaseDirectory.AppLocalData
            });
        }
    }
    
    async function openPluginFolder(event: any) {
        if (selectedPlugin?.path !== undefined) {
            await openPath(selectedPlugin.path);
        }
    }
    
    async function importPluginFromFolder(event: any) {
        const folder = await open({
            directory: true
        });
        await invoke("import_plugin_from_folder", { path: folder });
        
        plugins.p = await invoke("get_plugins");
    }
    
    let selectedReadme: string = $state("");
    let selectedLicense: string = $state("");
    let selectedPlugin: Plugin | undefined = $state(undefined);
    let pluginDetailsView: "readme" | "license" = $state("readme");
</script>

<div class="container p-2">
    <div class="hstack mb-4">
        <h3>Plugins</h3>
        <button onclick={importPluginFromFolder} class="ms-auto btn btn-lg btn-success shadow-lg" aria-label="Add plugin"><i class="bi bi-plus-lg"></i></button>
    </div>
    <div class="card p-2">
        <table class="table">
            <thead>
                <tr>
                    <th scope="col">Name</th>
                    <th scope="col">Version</th>
                    <th scope="col">License</th>
                    <th scope="col">Author</th>
                    <th scope="col">Language</th>
                    <th></th>
                </tr>
            </thead>
            <tbody>
                {#each Object.values(plugins.p) as p}
                    <tr>
                        <th scope="row">{p.config.name}</th>
                        <td>{p.config.version}</td>
                        <td>{p.config.license}</td>
                        <td>{p.config.author}</td>
                        <td>{p.config.language}</td>
                        <td><button onclick={viewPluginDetails} value={p.config.name} class="btn btn-sm" data-bs-toggle="modal" data-bs-target="#plugin-details" aria-label="Details"><i class="bi bi-chevron-right"></i></button></td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
</div>

<!-- Plugin details modal -->
<div class="modal fade" id="plugin-details" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
    <div class="modal-dialog modal-xl">
        <div class="modal-content">
            <div class="modal-header">
                <h1 class="modal-title fs-5" id="exampleModalLabel">{selectedPlugin?.config.name} <span class="badge text-bg-secondary ms-2">{selectedPlugin?.config.version}</span></h1>
                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
            </div>
            <div class="modal-body">
                <ul class="nav nav-underline">
                    <li class="nav-item">
                        <button onclick={() => pluginDetailsView = "readme"} class="nav-link {pluginDetailsView === "readme" ? "active" : ""}">README</button>
                    </li>
                    <li class="nav-item">
                        <button onclick={() => pluginDetailsView = "license"} class="nav-link {pluginDetailsView === "license" ? "active" : ""}">LICENSE</button>
                    </li>
                    <li class="nav-item ms-auto">
                        <a class="btn btn-outline-primary" href={selectedPlugin?.config.repository} aria-label="Repository"><i class="bi bi-link-45deg"></i></a>
                    </li>
                    <li class="nav-item">
                        <button onclick={openPluginFolder} class="btn btn-outline-primary" aria-label="View folder"><i class="bi bi-code-slash"></i></button>
                    </li>
                </ul>
                {#if pluginDetailsView === "readme"}
                <pre class="p-2">{selectedReadme}</pre>
                {:else if pluginDetailsView === "license"}
                <pre class="p-3">{selectedLicense}</pre>
                {/if}
            </div>
        </div>
    </div>
</div>