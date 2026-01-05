import type { ReportType } from "./schema";

export const loadedReport = $state<{
    report?: ReportType
}>({});

export const settings: {
    s: {
        notification_pos: "top-left" | "top-center" | "top-right" | "bottom-left" | "bottom-center" | "bottom-right",
        plugins_server_port: number,
        python: string,
        node_js: string,
        lua: string
    }
} = $state({
    s: {
        notification_pos: "top-center",
        plugins_server_port: 5555,
        python: "",
        node_js: "",
        lua: ""
    }
});