<script lang="ts">
    import { loadedReport, settings } from "$lib/state.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";
    import { onMount } from "svelte";
    import { toast } from "svelte-sonner";

    function testNotificationPos(event: any) {
        toast.info("Test toast position");
    }
    
    function changeNotificationPos(event: any) {
        settings.s.notification_pos = notificationPos;
        invoke("set_notifications_pos", { pos: notificationPos });
    }
    
    function changePluginsServerPort(event: any) {
        invoke("set_plugins_server_port", { port: pluginServerPort });
    }
    
    async function changePythonPath(event: any) {
        const file = await open({
            multiple: false,
            directory: false
        });
        pythonPath = file || undefined;
        pythonPath = await invoke("set_python_interpreter", { path: pythonPath });
    }
    
    async function changeNodeJsPath(event: any) {
        const file = await open({
            multiple: false,
            directory: false
        });
        nodeJsPath = file || undefined;
        nodeJsPath = await invoke("set_node_js_interpreter", { path: nodeJsPath });
    }
    
    async function changeLuaPath(event: any) {
        const file = await open({
            multiple: false,
            directory: false
        });
        luaPath = file || undefined;
        luaPath = await invoke("set_lua_interpreter", { path: luaPath });
    }
    
    let notificationPos: "top-left" | "top-center" | "top-right" | "bottom-left" | "bottom-center" | "bottom-right" = $state(settings.s.notification_pos);
    let pluginServerPort: number = $state(settings.s.plugins_server_port);
    let pythonPath: string | undefined = $state(settings.s.python);
    let nodeJsPath: string | undefined = $state(settings.s.node_js);
    let luaPath: string | undefined = $state(settings.s.lua);
    
    onMount(async () => {
        settings.s = await invoke("get_settings");
    });
</script>

<div class="container p-4 vh-100">
    <div class="row h-100">
        <div class="col-2">
            <nav id="navbar-settings" class="h-100 flex-column align-items-stretch pe-4 border-end">
                <nav class="nav nav-pills flex-column">
                    <a class="nav-link" href="#notifications">Notifications</a>
                    <a class="nav-link" href="#plugins">Plugins</a>
                    <nav class="nav nav-pills flex-column">
                        <a class="nav-link ms-3" href="#plugins-server">Plugins server</a>
                        <a class="nav-link ms-3" href="#plugins-py">Python</a>
                        <!--<a class="nav-link ms-3" href="#plugins-js">Node.js</a>
                        <a class="nav-link ms-3" href="#plugins-lua">Lua</a>-->
                    </nav>
                </nav>
            </nav>
        </div>
        <div class="col-10 p-2">
            <div data-bs-spy="scroll" data-bs-target="#navbar-settings" data-bs-smooth-scroll="true" tabindex="-1">
                <div id="notifications">
                    <h4>Notifications</h4>
                    <p>Notifications placement:</p>
                    <div class="input-group mb-3">
                        <select onchange={changeNotificationPos} bind:value={notificationPos} class="form-select" id="inputGroupSelect02">
                            {#each ["top-left", "top-center", "top-right", "bottom-left", "bottom-center", "bottom-right"] as i}
                                <option value={i} selected={settings.s.notification_pos === i}>{i.replace("-", " ")}</option>
                            {/each}
                        </select>
                        <button onclick={testNotificationPos} class="btn btn-info" for="inputGroupSelect02">Test choice</button>
                    </div>
                </div>
                <div id="plugins">
                    <h4>Plugins</h4>
                    <p>Settings related to how Pathfinder interacts with the plugins.</p>
                </div>
                <div id="plugins-server">
                    <h5>Plugins server</h5>
                    <p>Port used by the 0MQ server to communicate with plugins. In order to apply this configuration, all the running plugins will be terminated and the app restarted.</p>
                    <div class="input-group mb-3">
                        <input bind:value={pluginServerPort} type="number" step="1" min="1" max="65535" class="form-control" placeholder="5555" aria-label="Recipient’s username" aria-describedby="basic-addon2">
                        <button onclick={changePluginsServerPort} class="btn btn-success" id="basic-addon2">Apply and restart</button>
                    </div>
                </div>
                <div id="plugins-py">
                    <h5>Python</h5>
                    <p>Path to the Python interpreter to use:</p>
                    <div class="input-group mb-3">
                        <button onclick={changePythonPath} class="btn btn-light border" id="basic-addon2">Choose File</button>
                        <input bind:value={pythonPath} type="text" class="form-control" placeholder="no file selected" aria-label="Recipient’s username" aria-describedby="basic-addon2" disabled>
                    </div>
                </div>
                <!--
                <div id="plugins-js">
                    <h5>Node.js</h5>
                    <p>Path to the Node.js interpreter to use:</p>
                    <div class="input-group mb-3">
                        <button onclick={changeNodeJsPath} class="btn btn-light border" id="basic-addon2">Choose File</button>
                        <input bind:value={nodeJsPath} type="text" class="form-control" placeholder="no file selected" aria-label="Recipient’s username" aria-describedby="basic-addon2" disabled>
                    </div>
                </div>
                <div id="plugins-lua">
                    <h5>Lua</h5>
                    <p>Path to the Lua interpreter to use:</p>
                    <div class="input-group mb-3">
                        <button onclick={changeLuaPath} class="btn btn-light border" id="basic-addon2">Choose File</button>
                        <input bind:value={luaPath} type="text" class="form-control" placeholder="no file selected" aria-label="Recipient’s username" aria-describedby="basic-addon2" disabled>
                    </div>
                </div>
                -->
            </div>
        </div>
    </div>
</div>