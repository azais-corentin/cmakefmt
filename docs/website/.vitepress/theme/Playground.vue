<script setup lang="ts">
import { useData } from "vitepress";
import {
  onBeforeUnmount,
  onMounted,
  ref,
  shallowRef,
  watch,
  watchEffect,
} from "vue";
// Static imports are SSR-safe: CodeMirror only touches the DOM at EditorView construction time.
import { json } from "@codemirror/lang-json";
import { StreamLanguage } from "@codemirror/language";
import { cmake } from "@codemirror/legacy-modes/mode/cmake";
import { Compartment, EditorState } from "@codemirror/state";
import { oneDark } from "@codemirror/theme-one-dark";
import { basicSetup, EditorView } from "codemirror";
import { useFormatter } from "./playground/useFormatter";
// encodeState is pure (fflate + btoa) — no DOM access, safe at module level.
import {
  DEFAULT_CONFIG_JSON,
  DEFAULT_INPUT,
  WASM_URL,
} from "./playground/constants";
import { encodeState, useUrlState } from "./playground/useUrlState";

const { isDark } = useData();

// Must be called at setup time so it can register its own onMounted to read the URL hash.
const { initialInput, initialConfig, updateHash } = useUrlState();

// ── Reactive state for template (SSR-safe initial values) ────────────────────
const loading = ref(true);
const error = ref<string | null>(null);
const pluginInfo = shallowRef<{ name: string; version: string } | null>(null);
const configError = ref<string | null>(null);
const configOpen = ref(true);
const copied = ref(false);

// DOM attachment points for the three CodeMirror instances.
const inputContainerEl = ref<HTMLElement | null>(null);
const outputContainerEl = ref<HTMLElement | null>(null);
const configContainerEl = ref<HTMLElement | null>(null);

// Editor view refs — shallowRef because we never need deep reactivity on EditorView internals.
const inputView = shallowRef<EditorView | null>(null);
const outputView = shallowRef<EditorView | null>(null);
const configView = shallowRef<EditorView | null>(null);

// Private mutable state not exposed to the template.
let formatter: ReturnType<typeof useFormatter> | null = null;
let themeCompartment: Compartment | null = null;
let formatTimer: ReturnType<typeof setTimeout> | null = null;
let urlTimer: ReturnType<typeof setTimeout> | null = null;

onMounted(() => {
  // useFormatter() creates a Web Worker — must be browser-only (inside onMounted).
  formatter = useFormatter();

  // Sync formatter reactive state into the template-accessible refs.
  // watchEffect inside a lifecycle hook is properly scoped to this component.
  watchEffect(() => {
    loading.value = formatter!.loading.value;
    error.value = formatter!.error.value;
    pluginInfo.value = formatter!.pluginInfo.value;
  });

  themeCompartment = new Compartment();

  // Base theme that maps CodeMirror chrome to VitePress CSS variables.
  // oneDark is layered on top in dark mode via the compartment.
  const vpBase = EditorView.theme({
    "&": {
      height: "100%",
      fontSize: "13px",
      backgroundColor: "var(--vp-c-bg-soft)",
      color: "var(--vp-c-text-1)",
    },
    ".cm-scroller": {
      overflow: "auto",
      fontFamily: "var(--vp-font-family-mono, ui-monospace, monospace)",
    },
    ".cm-gutters": {
      backgroundColor: "var(--vp-c-bg-soft)",
      color: "var(--vp-c-text-3)",
      border: "none",
      borderRight: "1px solid var(--vp-c-border)",
    },
    ".cm-cursor, .cm-dropCursor": { borderLeftColor: "var(--vp-c-text-1)" },
    // Suppress the active-line highlight so the background CSS variable shows through.
    ".cm-activeLine": { backgroundColor: "transparent" },
    ".cm-activeLineGutter": { backgroundColor: "transparent" },
    ".cm-selectionBackground, ::selection": {
      backgroundColor: "var(--vp-c-brand-soft) !important",
    },
  });

  const getThemeExt = () => (isDark.value ? [oneDark] : []);

  // Build the shared extension list for an editor. readOnly=true for the output pane.
  const makeExtensions = (readOnly = false) => [
    basicSetup,
    vpBase,
    themeCompartment!.of(getThemeExt()),
    ...(readOnly
      ? [EditorState.readOnly.of(true), EditorView.editable.of(false)]
      : []),
  ];

  const cmakeLang = StreamLanguage.define(cmake);

  // Input editor
  inputView.value = new EditorView({
    parent: inputContainerEl.value!,
    state: EditorState.create({
      doc: initialInput.value ?? DEFAULT_INPUT,
      extensions: [
        ...makeExtensions(),
        cmakeLang,
        EditorView.updateListener.of(update => {
          if (!update.docChanged) return;
          scheduleFormat();
          scheduleUrlUpdate();
        }),
      ],
    }),
  });

  // Output editor — read-only
  outputView.value = new EditorView({
    parent: outputContainerEl.value!,
    state: EditorState.create({
      doc: "",
      extensions: [...makeExtensions(true), cmakeLang],
    }),
  });

  // Config editor — JSON mode
  configView.value = new EditorView({
    parent: configContainerEl.value!,
    state: EditorState.create({
      doc: initialConfig.value ?? DEFAULT_CONFIG_JSON,
      extensions: [
        ...makeExtensions(),
        json(),
        EditorView.updateListener.of(update => {
          if (!update.docChanged) return;
          scheduleFormat();
          scheduleUrlUpdate();
        }),
      ],
    }),
  });

  // Reconfigure all three editors when the VitePress dark mode changes.
  watch(isDark, () => {
    const effect = themeCompartment!.reconfigure(getThemeExt());
    inputView.value?.dispatch({ effects: effect });
    outputView.value?.dispatch({ effects: effect });
    configView.value?.dispatch({ effects: effect });
  });

  // Trigger the first format as soon as the WASM plugin finishes loading.
  watch(
    () => formatter!.loading.value,
    isLoading => {
      if (!isLoading && !formatter!.error.value) {
        triggerFormat();
      }
    },
  );

  formatter!.loadPlugin(WASM_URL);
});

onBeforeUnmount(() => {
  if (formatTimer) clearTimeout(formatTimer);
  if (urlTimer) clearTimeout(urlTimer);
  inputView.value?.destroy();
  outputView.value?.destroy();
  configView.value?.destroy();
  formatter?.dispose();
});

// ── Internal helpers ─────────────────────────────────────────────────────────

function getInputText(): string {
  return inputView.value?.state.doc.toString() ?? "";
}

function getConfigText(): string {
  return configView.value?.state.doc.toString() ?? "";
}

function setOutputText(text: string): void {
  const view = outputView.value;
  if (!view) return;
  view.dispatch({
    changes: { from: 0, to: view.state.doc.length, insert: text },
  });
}

function scheduleFormat(): void {
  if (formatTimer) clearTimeout(formatTimer);
  formatTimer = setTimeout(triggerFormat, 10);
}

function scheduleUrlUpdate(): void {
  if (urlTimer) clearTimeout(urlTimer);
  urlTimer = setTimeout(() => {
    updateHash(getInputText(), getConfigText());
  }, 500);
}

async function triggerFormat(): Promise<void> {
  if (!formatter || formatter.loading.value) return;

  const configText = getConfigText();
  let config: Record<string, unknown>;
  try {
    config = JSON.parse(configText);
    configError.value = null;
  } catch (e) {
    configError.value = `Invalid JSON: ${(e as Error).message}`;
    return;
  }

  try {
    formatter.setConfig(config);
    const result = await formatter.format(getInputText());
    setOutputText(result);
  } catch (e) {
    const msg = (e as Error).message;
    // "Superseded" rejections are expected when the user types faster than 300 ms.
    // They are not errors — just discard them silently.
    if (!msg.includes("Superseded")) {
      error.value = msg;
    }
  }
}

// ── Public actions ───────────────────────────────────────────────────────────

async function shareLink(): Promise<void> {
  const url = window.location.origin
    + window.location.pathname
    + "#"
    + encodeState(getInputText(), getConfigText());
  await navigator.clipboard.writeText(url);
  copied.value = true;
  setTimeout(() => {
    copied.value = false;
  }, 2000);
}

async function reset(): Promise<void> {
  const iv = inputView.value;
  const cv = configView.value;
  if (!iv || !cv) return;

  iv.dispatch({
    changes: { from: 0, to: iv.state.doc.length, insert: DEFAULT_INPUT },
  });
  cv.dispatch({
    changes: { from: 0, to: cv.state.doc.length, insert: DEFAULT_CONFIG_JSON },
  });
  configError.value = null;

  // Cancel any pending debounced format and run immediately.
  if (formatTimer) clearTimeout(formatTimer);
  await triggerFormat();
}
</script>

<template>
  <div class="playground">
    <!-- Header bar -->
    <div class="pg-header">
      <span class="pg-title">Playground</span>
      <span class="pg-version">
        <template v-if="loading && !error">Loading…</template>
        <template v-else-if="error">Plugin error</template>
        <template v-else-if="pluginInfo">cmakefmt {{
            pluginInfo.version
          }}</template>
      </span>
    </div>

    <!-- Error banner (plugin load errors and format errors) -->
    <div v-if="error" class="pg-error" role="alert">
      <strong>Error:</strong> {{ error }}
    </div>

    <!-- Split panes: input (left) / output (right) -->
    <div class="pg-editors">
      <div class="pg-pane">
        <div class="pg-pane-label">Input</div>
        <div
          ref="inputContainerEl"
          class="pg-editor-wrap"
          :class="{ 'is-loading': loading }"
        >
        </div>
      </div>
      <div class="pg-pane">
        <div class="pg-pane-label">Output</div>
        <div
          ref="outputContainerEl"
          class="pg-editor-wrap"
          :class="{ 'is-loading': loading }"
        >
        </div>
      </div>
    </div>

    <!-- Configuration (collapsible) -->
    <div class="pg-config">
      <button
        class="pg-config-toggle"
        :aria-expanded="configOpen"
        @click="configOpen = !configOpen"
      >
        <span>Configuration</span>
        <svg
          class="pg-chevron"
          :class="{ 'is-open': configOpen }"
          viewBox="0 0 16 16"
          width="16"
          height="16"
          aria-hidden="true"
        >
          <path
            d="M4 6l4 4 4-4"
            stroke="currentColor"
            stroke-width="1.5"
            fill="none"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
      </button>
      <div v-show="configOpen" class="pg-config-body">
        <div ref="configContainerEl" class="pg-config-editor"></div>
        <div v-if="configError" class="pg-config-error" role="alert">
          {{ configError }}
        </div>
      </div>
    </div>

    <!-- Action bar -->
    <div class="pg-actions">
      <button class="pg-btn pg-btn-primary" @click="shareLink">
        {{ copied ? "Copied!" : "Share Link" }}
      </button>
      <button class="pg-btn" :disabled="loading" @click="reset">Reset</button>
    </div>
  </div>
</template>

<style scoped>
.playground {
  display: flex;
  flex-direction: column;
  height: calc(100dvh - var(--vp-nav-height));
  border: 1px solid var(--vp-c-border);
  overflow: hidden;
  background: var(--vp-c-bg);
  font-family: var(--vp-font-family-base);
}

/* ── Header ─────────────────────────────────────────────────────────────────── */
.pg-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 16px;
  background: var(--vp-c-bg-soft);
  border-bottom: 1px solid var(--vp-c-border);
}

.pg-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--vp-c-text-1);
}

.pg-version {
  font-size: 12px;
  color: var(--vp-c-text-2);
  font-family: var(--vp-font-family-mono, ui-monospace, monospace);
}

/* ── Error banner ───────────────────────────────────────────────────────────── */
.pg-error {
  padding: 10px 16px;
  font-size: 13px;
  color: var(--vp-custom-block-danger-text, #b00020);
  background: var(--vp-custom-block-danger-bg, #fff0f0);
  border-bottom: 1px solid var(--vp-c-border);
}

/* ── Split panes ────────────────────────────────────────────────────────────── */
.pg-editors {
  flex: 1;
  min-height: 0;
  display: grid;
  grid-template-columns: 1fr 1fr;
  border-bottom: 1px solid var(--vp-c-border);
}

@media (max-width: 768px) {
  .pg-editors {
    grid-template-columns: 1fr;
  }
}

.pg-pane {
  display: flex;
  flex-direction: column;
  min-height: 0;
  border-right: 1px solid var(--vp-c-border);
  min-width: 0; /* prevent CodeMirror long lines from blowing out the grid */
}

.pg-pane:last-child {
  border-right: none;
}

.pg-pane-label {
  flex-shrink: 0;
  padding: 5px 12px;
  font-size: 11px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--vp-c-text-2);
  background: var(--vp-c-bg-soft);
  border-bottom: 1px solid var(--vp-c-border);
}

.pg-editor-wrap {
  flex: 1;
  min-height: 0;
  overflow: hidden;
  transition: opacity 0.2s;
}

.pg-editor-wrap.is-loading {
  opacity: 0.45;
  pointer-events: none;
}

/* ── Config section ─────────────────────────────────────────────────────────── */
.pg-config {
  border-bottom: 1px solid var(--vp-c-border);
}

.pg-config-toggle {
  display: flex;
  align-items: center;
  gap: 6px;
  width: 100%;
  padding: 8px 16px;
  font-size: 13px;
  font-weight: 500;
  color: var(--vp-c-text-1);
  background: var(--vp-c-bg-soft);
  border: none;
  cursor: pointer;
  text-align: left;
  transition: background 0.15s;
}

.pg-config-toggle:hover {
  background: var(--vp-c-bg-mute, var(--vp-c-bg-soft));
}

.pg-chevron {
  margin-left: auto;
  flex-shrink: 0;
  transition: transform 0.2s;
}

.pg-chevron.is-open {
  transform: rotate(180deg);
}

.pg-config-body {
  border-top: 1px solid var(--vp-c-border);
}

.pg-config-editor {
  height: 180px;
  overflow: hidden;
}

.pg-config-error {
  padding: 6px 12px;
  font-size: 12px;
  color: var(--vp-custom-block-danger-text, #b00020);
  background: var(--vp-custom-block-danger-bg, #fff0f0);
  border-top: 1px solid var(--vp-c-border);
}

/* ── Action bar ─────────────────────────────────────────────────────────────── */
.pg-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 12px 16px;
  background: var(--vp-c-bg-soft);
}

/* ── Buttons ────────────────────────────────────────────────────────────────── */
.pg-btn {
  padding: 6px 14px;
  font-size: 13px;
  font-weight: 500;
  border-radius: 6px;
  border: 1px solid var(--vp-c-border);
  background: var(--vp-c-bg);
  color: var(--vp-c-text-1);
  cursor: pointer;
  transition: background 0.15s, border-color 0.15s;
}

.pg-btn:hover:not(:disabled) {
  background: var(--vp-c-bg-soft);
  border-color: var(--vp-c-brand-1);
  color: var(--vp-c-brand-1);
}

.pg-btn:disabled {
  opacity: 0.45;
  cursor: not-allowed;
}

.pg-btn-primary {
  background: var(--vp-c-brand-1);
  color: #fff;
  border-color: var(--vp-c-brand-1);
}

.pg-btn-primary:hover:not(:disabled) {
  background: var(--vp-c-brand-2, var(--vp-c-brand-1));
  border-color: var(--vp-c-brand-2, var(--vp-c-brand-1));
  color: #fff;
}

/* ── CodeMirror integration ─────────────────────────────────────────────────── */
/* :deep pierces the scoped boundary to reach CodeMirror's injected DOM nodes.  */
:deep(.cm-editor) {
  height: 100%;
}

:deep(.cm-scroller) {
  overflow: auto;
}
</style>
