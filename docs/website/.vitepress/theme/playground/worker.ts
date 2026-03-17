import { createFromBuffer } from "@dprint/formatter";
import type { Formatter } from "@dprint/formatter";

// Messages to worker
type ToWorker =
  | { type: "load"; url: string }
  | { type: "config"; config: Record<string, unknown> }
  | { type: "format"; fileText: string };

// Messages from worker
type FromWorker =
  | { type: "loaded"; pluginInfo: { name: string; version: string }; resolvedConfig: Record<string, unknown> }
  | { type: "formatted"; text: string }
  | { type: "error"; message: string };

function post(msg: FromWorker): void {
  self.postMessage(msg);
}

let formatter: Formatter | null = null;

self.addEventListener("message", async (event: MessageEvent<ToWorker>) => {
  const msg = event.data;

  try {
    switch (msg.type) {
      case "load": {
        let response: Response;
        try {
          response = await fetch(msg.url);
        } catch (err) {
          post({
            type: "error",
            message: "Failed to load cmakefmt WASM plugin.",
          });
          return;
        }

        if (!response.ok) {
          const body = await response.text().catch(() => "");
          const detail = body || `HTTP ${response.status} ${response.statusText}`;
          post({
            type: "error",
            message: "Failed to load cmakefmt WASM plugin.",
          });
          return;
        }

        const contentType = response.headers.get("content-type") ?? "";
        if (contentType.includes("text/html")) {
          post({
            type: "error",
            message: `Could not load the formatter plugin \u2014 the WASM file was not found at "${msg.url}".`,
          });
          return;
        }

        const buffer = await response.arrayBuffer();

        try {
          formatter = createFromBuffer(buffer);
        } catch (err) {
          const isDev = msg.url.startsWith("/");
          const hint = isDev
            ? " Try rebuilding with `mise run build`."
            : ` The file at "${msg.url}" may be corrupt or incompatible.`;
          post({
            type: "error",
            message: `Failed to initialize the formatter plugin.${hint}`,
          });
          return;
        }

        post({
          type: "loaded",
          pluginInfo: formatter.getPluginInfo(),
          resolvedConfig: formatter.getResolvedConfig(),
        });
        break;
      }

      case "config": {
        if (formatter === null) {
          post({ type: "error", message: "Formatter not loaded. Send a 'load' message first." });
          return;
        }

        // GlobalConfiguration fields live at top-level; everything else is plugin config.
        // The worker protocol passes a flat config object — forward it entirely as plugin
        // config and leave global config empty so dprint's own defaults apply.
        formatter.setConfig({}, msg.config);

        post({
          type: "loaded",
          pluginInfo: formatter.getPluginInfo(),
          resolvedConfig: formatter.getResolvedConfig(),
        });
        break;
      }

      case "format": {
        if (formatter === null) {
          post({ type: "error", message: "Formatter not loaded. Send a 'load' message first." });
          return;
        }

        let text: string;
        try {
          text = formatter.formatText({ filePath: "CMakeLists.txt", fileText: msg.fileText });
        } catch (err) {
          post({ type: "error", message: `Format error: ${err instanceof Error ? err.message : String(err)}` });
          return;
        }

        post({ type: "formatted", text });
        break;
      }

      default: {
        // Exhaustiveness guard: narrow to `never` so TypeScript warns on new
        // message types added to ToWorker without a corresponding case here.
        const _exhaustive: never = msg;
        post({ type: "error", message: `Unknown message type: ${(_exhaustive as ToWorker).type}` });
      }
    }
  } catch (err) {
    post({ type: "error", message: `Unexpected worker error: ${err instanceof Error ? err.message : String(err)}` });
  }
});
