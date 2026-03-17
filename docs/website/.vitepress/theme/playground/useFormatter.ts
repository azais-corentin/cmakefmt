import { ref, shallowRef } from "vue";
// Vite resolves `?worker` imports into a Worker constructor at build time.
// This must be a static top-level import so Vite can analyse the URL.
import FormatterWorker from "./worker.ts?worker";

// ---------------------------------------------------------------------------
// Worker protocol
// ---------------------------------------------------------------------------

type ToWorker =
  | { type: "load"; url: string }
  | { type: "config"; config: Record<string, unknown> }
  | { type: "format"; fileText: string };

type FromWorker =
  | { type: "loaded"; pluginInfo: { name: string; version: string }; resolvedConfig: Record<string, unknown> }
  | { type: "formatted"; text: string }
  | { type: "error"; message: string };

// ---------------------------------------------------------------------------

interface PendingFormat {
  resolve: (text: string) => void;
  reject: (err: Error) => void;
}

/**
 * Composable that owns a formatting Web Worker.
 *
 * Callers are responsible for browser-only instantiation (call inside
 * `onMounted` or guard with `import.meta.env.SSR`).
 *
 * Lifecycle: call `dispose()` in `onUnmounted` to terminate the worker.
 */
export function useFormatter() {
  const worker = new FormatterWorker();

  const loading = ref(true);
  const error = ref<string | null>(null);
  const pluginInfo = shallowRef<{ name: string; version: string } | null>(null);
  const resolvedConfig = shallowRef<Record<string, unknown> | null>(null);

  // Sequential formatting: only one request in-flight at a time.
  // A newer call supersedes the previous one.
  let pending: PendingFormat | null = null;

  worker.onmessage = (event: MessageEvent<FromWorker>) => {
    const msg = event.data;
    switch (msg.type) {
      case "loaded":
        loading.value = false;
        error.value = null;
        pluginInfo.value = msg.pluginInfo;
        resolvedConfig.value = msg.resolvedConfig;
        break;

      case "formatted":
        if (pending !== null) {
          const { resolve } = pending;
          pending = null;
          resolve(msg.text);
        }
        break;

      case "error":
        loading.value = false;
        error.value = msg.message;
        if (pending !== null) {
          const { reject } = pending;
          pending = null;
          reject(new Error(msg.message));
        }
        break;
    }
  };

  // Surface uncaught worker errors (e.g. script load failure, syntax error).
  worker.onerror = (event: ErrorEvent) => {
    loading.value = false;
    error.value = event.message ?? "Unknown worker error";
    if (pending !== null) {
      const { reject } = pending;
      pending = null;
      reject(new Error(error.value!));
    }
  };

  // ---------------------------------------------------------------------------
  // Public API
  // ---------------------------------------------------------------------------

  /** Kick off WASM plugin loading inside the worker. */
  function loadPlugin(url: string): void {
    loading.value = true;
    error.value = null;
    worker.postMessage({ type: "load", url } satisfies ToWorker);
  }

  /** Push a new config snapshot; the worker applies it synchronously. */
  function setConfig(config: Record<string, unknown>): void {
    worker.postMessage({ type: "config", config } satisfies ToWorker);
  }

  /**
   * Request formatting of `fileText`.
   *
   * If a previous format request is still pending its promise is rejected with
   * "Superseded" so callers can ignore stale results.
   */
  function format(fileText: string): Promise<string> {
    if (pending !== null) {
      pending.reject(new Error("Superseded by newer format request"));
      pending = null;
    }

    return new Promise<string>((resolve, reject) => {
      pending = { resolve, reject };
      worker.postMessage({ type: "format", fileText } satisfies ToWorker);
    });
  }

  /** Terminate the worker. Must be called in `onUnmounted`. */
  function dispose(): void {
    if (pending !== null) {
      pending.reject(new Error("Worker disposed"));
      pending = null;
    }
    worker.terminate();
  }

  return {
    loading,
    error,
    pluginInfo,
    resolvedConfig,
    loadPlugin,
    setConfig,
    format,
    dispose,
  };
}
