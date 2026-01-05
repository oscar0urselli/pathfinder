<script lang="ts">
    import type { ReportType } from "$lib/schema";
    import { loadedReport, settings } from "$lib/state.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { toast } from "svelte-sonner";
    import { z } from "zod/v4";
    
    
    const newReportSchema = z.object({
        title: z.string().nonempty(),
        author: z.string(),
        device: z.string(),
        place: z.string()
    });
    
    async function newReport(event: any) {
        const result = newReportSchema.safeParse({
            title: newReportTitle,
            author: newReportAuthor,
            device: newReportDevice,
            place: newReportPlace
        });
        
        if (result.success) {
            const data: ReportType = await invoke("new_report", { report: result.data });
            toast.success(`New report '${newReportTitle}' has been created and loaded.`);
            loadedReport.report = data;
        }
        else {
            toast.error(`${result.error}`);
        }
    }
    
    async function listReports(event: any) {
        const data: ReportType[] = await invoke("list_reports");
        reports = data;
    }
    
    function selectReport(event: any) {
        selectedReport = event.currentTarget.value;
    }
    
    async function loadReport(event: any) {
        try {
            const data: ReportType = await invoke("load_report", { id: selectedReport });
            loadedReport.report = data;
            toast.success(`Report with ID '${selectedReport}' has been loaded.`);
        }
        catch (error) {
            toast.error(`Couldn't load report with ID '${selectedReport}'. ${error}`);
        }
    }
    
    let selectedReport: string | null = $state(null);
    
    let reports: ReportType[] = $state([]);
    
    let newReportTitle = $state("");
    let newReportAuthor = $state("");
    let newReportDevice = $state("");
    let newReportPlace = $state("");
    
    onMount(async () => {
        loadedReport.report = await invoke("get_loaded_report");
        settings.s = await invoke("get_settings");
    });
</script>

<div class="position-absolute top-50 start-50 translate-middle">
    <div class="hstack gap-3 vh-100">
        <button type="button" class="btn btn-light btn-lg p-5 h-50 w-50" data-bs-toggle="modal" data-bs-target="#new-report">
            <div class="vstack">
                <p class="fs-3 fw-bold">NEW REPORT</p>
                <i class="bi bi-plus-circle"></i>
            </div>
        </button>
        <button onclick={listReports} type="button" class="btn btn-light btn-lg p-5 h-50 w-50" data-bs-toggle="modal" data-bs-target="#load-report">
            <div class="vstack">
                <p class="fs-3 fw-bold">LOAD REPORT</p>
                <i class="bi bi-arrow-clockwise"></i>
            </div>
        </button>
        <button type="button" class="btn btn-light btn-lg p-5 h-50 w-50">
            <div class="vstack">
                <p class="fs-3 fw-bold">HELP</p>
                <i class="bi bi-question-circle"></i>
            </div>
        </button>
    </div>
</div>

<!-- New report modal -->
<div class="modal fade" id="new-report" data-bs-backdrop="static" data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticBackdropLabel" aria-hidden="true">
    <div class="modal-dialog">
        <div class="modal-content">
            <div class="modal-header">
                <h1 class="modal-title fs-5" id="staticBackdropLabel">New report</h1>
                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
            </div>
            <div class="modal-body">
                <div class="mb-3">
                    <label for="report-new-title" class="form-label">Title</label>
                    <input name="title" bind:value={newReportTitle} type="text" class="form-control">
                </div>
                <div class="mb-3">
                    <label for="report-new-title" class="form-label">Author</label>
                    <input name="author" bind:value={newReportAuthor} type="text" class="form-control">
                </div>
                <div class="mb-3">
                    <label for="report-new-title" class="form-label">Device</label>
                    <input name="device" bind:value={newReportDevice} type="text" class="form-control">
                </div>
                <div class="mb-3">
                    <label for="report-new-title" class="form-label">Place</label>
                    <input name="place" bind:value={newReportPlace} type="text" class="form-control">
                </div>
            </div>
            <div class="modal-footer">
                <button onclick={newReport} type="button" class="btn btn-primary">Create</button>
            </div>
        </div>
    </div>
</div>

<!-- Load report modal -->
<div class="modal fade" id="load-report" data-bs-backdrop="static" data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticBackdropLabel" aria-hidden="true">
    <div class="modal-dialog modal-md">
        <div class="modal-content">
            <div class="modal-header">
                <h1 class="modal-title fs-5" id="staticBackdropLabel">Load report</h1>
                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
            </div>
            <div class="modal-body">
                <div class="list-group">
                    {#each reports as r}
                        <button onclick={selectReport} value={r.id} type="button" class="list-group-item list-group-item-action {selectedReport === r.id || (selectedReport === null && loadedReport.report?.id === r.id) ? "active" : ""}">
                            <div class="d-flex w-100 justify-content-between">
                                <h5 class="mb-1">{r.title}</h5>
                                <small>{new Date(r.last_access_tsz / 1000).toLocaleString()}</small>
                            </div>
                            <p class="mb-1">{r.place}</p>
                            <small>{r.author} - {r.device}</small>
                        </button>
                    {/each}
                </div>
            </div>
            <div class="modal-footer">
                <button onclick={loadReport} type="button" class="btn btn-primary" disabled={selectedReport === null}>Load</button>
            </div>
        </div>
    </div>
</div>