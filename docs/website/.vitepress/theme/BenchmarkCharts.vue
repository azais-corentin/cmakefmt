<script setup lang="ts">
import uPlot from "uplot";
import { useData } from "vitepress";
import { nextTick, onMounted, onUnmounted, ref, watch } from "vue";
import "uplot/dist/uPlot.min.css";

const DATA_URL =
  "https://raw.githubusercontent.com/azais-corentin/cmakefmt/refs/heads/benchmark-history/benchmarks/cmakefmt-fixtures-history.json";

const { isDark } = useData();

const throughputContainer = ref<HTMLDivElement>();
const timingContainer = ref<HTMLDivElement>();
const throughputBarContainer = ref<HTMLDivElement>();
const timingBarContainer = ref<HTMLDivElement>();
const loading = ref(true);
const error = ref<string>();
const hasBaselines = ref(false);

interface Baseline {
  label: string;
  throughputMBps: number;
  timingMs: number;
  color: string;
}

interface ChartData {
  timestamps: number[];
  commitShas: string[];
  throughputs: number[];
  throughputCILo: number[];
  throughputCIHi: number[];
  timings: number[];
  timingCILo: number[];
  timingCIHi: number[];
  baselines: Baseline[];
}

let data: ChartData | null = null;
let throughputChart: uPlot | null = null;
let timingChart: uPlot | null = null;
let throughputBarChart: uPlot | null = null;
let timingBarChart: uPlot | null = null;
let resizeObserver: ResizeObserver | null = null;

// -- Formatting helpers --

const msFormatter = new Intl.NumberFormat("en-US", {
  maximumFractionDigits: 2,
  minimumFractionDigits: 0,
});

function formatValue(value: number, unit: "ms" | "MB/s"): string {
  if (unit === "ms") {
    return `${msFormatter.format(value)} ms`;
  }
  // MB/s: use enough precision for small values
  if (value >= 10) return `${value.toFixed(1)} MB/s`;
  if (value >= 1) return `${value.toFixed(2)} MB/s`;
  return `${value.toFixed(3)} MB/s`;
}

// -- Theme helpers --

function axisColor(dark: boolean) {
  return dark ? "rgba(255,255,255,0.35)" : "rgba(0,0,0,0.25)";
}

function gridColor(dark: boolean) {
  return dark ? "rgba(255,255,255,0.08)" : "rgba(0,0,0,0.06)";
}

function textColor(dark: boolean) {
  return dark ? "rgba(255,255,255,0.85)" : "rgba(0,0,0,0.78)";
}

// -- Tooltip formatter for line charts --

function tooltipValueFormatter(suffix: "ms" | "MB/s") {
  return (
    _self: uPlot,
    rawValue: number,
    _seriesIdx: number,
    idx: number | null,
  ) => {
    if (idx == null || !data) return "--";
    const sha = data.commitShas[idx]?.slice(0, 7) ?? "?";
    const date = new Date(data.timestamps[idx] * 1000).toLocaleDateString();
    return `${formatValue(rawValue, suffix)}  (${sha} — ${date})`;
  };
}

// -- Data parsing --

/** Compute aggregate CIs from fixture-level median CIs. */
function parseEntries(
  raw: any[],
  rawBaselines?: Record<string, any>,
): ChartData {
  const timestamps: number[] = [];
  const commitShas: string[] = [];
  const throughputs: number[] = [];
  const throughputCILo: number[] = [];
  const throughputCIHi: number[] = [];
  const timings: number[] = [];
  const timingCILo: number[] = [];
  const timingCIHi: number[] = [];

  for (const entry of raw) {
    timestamps.push(
      Math.floor(new Date(entry.commit_timestamp).getTime() / 1000),
    );
    commitShas.push(entry.commit_sha ?? "");
    throughputs.push(
      (entry.aggregate?.aggregate_throughput_gb_per_s ?? 0) * 1000,
    );
    timings.push((entry.aggregate?.weighted_total_seconds ?? 0) * 1000);

    const fixtures: any[] = entry.fixtures ?? [];
    // Sum fixture-level CI bounds. Timing is additive across fixtures.
    let sumCILoSec = 0;
    let sumCIHiSec = 0;
    let hasCI = false;
    let totalBytes = entry.aggregate?.total_input_bytes ?? 0;

    for (const f of fixtures) {
      const lo = f.mean_confidence_interval_lower_seconds;
      const hi = f.mean_confidence_interval_upper_seconds;
      if (lo != null && hi != null) {
        sumCILoSec += lo;
        sumCIHiSec += hi;
        hasCI = true;
      } else {
        // Fall back to mean for fixtures missing CI data.
        const mean = f.mean_seconds ?? 0;
        sumCILoSec += mean;
        sumCIHiSec += mean;
      }
    }

    if (hasCI && sumCILoSec > 0 && sumCIHiSec > 0) {
      timingCILo.push(sumCILoSec * 1000);
      timingCIHi.push(sumCIHiSec * 1000);
      // Invert: higher time → lower throughput.
      const GB = 1e9;
      throughputCILo.push((totalBytes / sumCIHiSec / GB) * 1000);
      throughputCIHi.push((totalBytes / sumCILoSec / GB) * 1000);
    } else {
      // No CI available — collapse band to the median value.
      const lastTiming = timings[timings.length - 1];
      const lastThroughput = throughputs[throughputs.length - 1];
      timingCILo.push(lastTiming);
      timingCIHi.push(lastTiming);
      throughputCILo.push(lastThroughput);
      throughputCIHi.push(lastThroughput);
    }
  }

  const BASELINE_COLORS: Record<string, string> = {
    cmake_format: "#f59e0b",
    gersemi: "#ef4444",
  };

  const baselines: Baseline[] = [];
  if (rawBaselines) {
    for (const [key, val] of Object.entries(rawBaselines)) {
      if (!val || typeof val !== "object") continue;
      const version: string = val.tool_version ?? key;
      // Some tool_version values already contain the tool name (e.g. "gersemi 0.26.1").
      // Only prepend the key when the version string does not start with it.
      const label = version.toLowerCase().startsWith(key.toLowerCase())
        ? version
        : `${key} ${version}`;
      baselines.push({
        label,
        throughputMBps: (val.throughput_gb_per_s ?? 0) * 1000,
        timingMs: (val.mean_seconds ?? 0) * 1000,
        color: BASELINE_COLORS[key] ?? "#888888",
      });
    }
  }

  return {
    timestamps,
    commitShas,
    throughputs,
    throughputCILo,
    throughputCIHi,
    timings,
    timingCILo,
    timingCIHi,
    baselines,
  };
}

// -- Line chart options (no baseline series) --

function makeOpts(
  title: string,
  yLabel: string,
  suffix: "ms" | "MB/s",
  strokeColor: string,
  bandFill: string,
  width: number,
  dark: boolean,
): uPlot.Options {
  const series: uPlot.Series[] = [
    {
      value: (_u: uPlot, _v: number, _si: number, idx: number | null) => {
        if (idx == null || !data) return "--";
        const sha = data.commitShas[idx]?.slice(0, 7) ?? "?";
        const date = new Date(data.timestamps[idx] * 1000)
          .toLocaleDateString();
        return `${sha} — ${date}`;
      },
    },
    {
      label: yLabel,
      stroke: strokeColor,
      width: 2,
      value: tooltipValueFormatter(suffix),
      paths: uPlot.paths.stepped!({ align: 1 }),
    },
    // CI lower bound — hidden line, used only as band edge.
    {
      show: true,
      stroke: "transparent",
      width: 0,
      points: { show: false },
      paths: uPlot.paths.stepped!({ align: 1 }),
    },
    // CI upper bound — hidden line, used only as band edge.
    {
      show: true,
      stroke: "transparent",
      width: 0,
      points: { show: false },
      paths: uPlot.paths.stepped!({ align: 1 }),
    },
  ];

  return {
    width,
    height: 320,
    title,
    cursor: { drag: { x: true, y: false } },
    scales: {
      x: { time: false },
      y: {
        range: (
          _u: uPlot,
          _min: number,
          max: number,
        ): uPlot.Range.MinMax => [0, max],
      },
    },
    axes: [
      {
        stroke: axisColor(dark),
        grid: { stroke: gridColor(dark) },
        ticks: { stroke: gridColor(dark) },
        space: 70,
        values: (_u: uPlot, splits: number[]) =>
          splits.map((i) => data?.commitShas[i]?.slice(0, 7) ?? ""),
      },
      {
        label: yLabel,
        stroke: axisColor(dark),
        grid: { stroke: gridColor(dark) },
        ticks: { stroke: gridColor(dark) },
      },
    ],
    series,
    bands: [
      { series: [3, 2], fill: bandFill },
    ],
  };
}

// -- Bar chart options --

const BAR_COLORS = {
  cmakefmt: "#22c55e",
  gersemi: "#ef4444",
  cmake_format: "#f59e0b",
};

function makeBarOpts(
  title: string,
  unit: "ms" | "MB/s",
  toolNames: string[],
  values: number[],
  width: number,
  dark: boolean,
): { opts: uPlot.Options; chartData: uPlot.AlignedData } {
  const maxVal = Math.max(...values);
  const numTools = toolNames.length;

  // Build per-tool series: each tool has a value only at its own index.
  const seriesData: (number | null)[][] = toolNames.map((_, ti) =>
    toolNames.map((_, di) => (di === ti ? values[ti] : null))
  );

  const indices = toolNames.map((_, i) => i);

  // Bar path builder — size and layout only; orientation comes from scales.
  const barPaths = uPlot.paths.bars!({ size: [0.6, 100] });

  const series: uPlot.Series[] = [
    // x-axis (category index)
    {},
  ];

  for (let i = 0; i < numTools; i++) {
    const name = toolNames[i];
    const color = name.includes("cmakefmt")
      ? BAR_COLORS.cmakefmt
      : name.toLowerCase().includes("gersemi")
      ? BAR_COLORS.gersemi
      : name.toLowerCase().includes("cmake_format")
      ? BAR_COLORS.cmake_format
      : BAR_COLORS.cmake_format;

    series.push({
      label: name,
      stroke: color,
      fill: color + "66", // ~40% opacity
      width: 0,
      paths: barPaths,
      points: { show: false },
    });
  }

  // Closed over by draw hook.
  const chartSeriesData = seriesData;
  const chartUnit = unit;

  const opts: uPlot.Options = {
    width,
    height: 160,
    title,
    cursor: { show: false },
    legend: { show: false },
    scales: {
      x: {
        time: false,
        ori: 1, // categories on vertical axis
        dir: -1, // top-to-bottom
        range: (
          _u: uPlot,
          _min: number,
          _max: number,
        ): uPlot.Range.MinMax => [-0.5, numTools - 0.5],
      },
      y: {
        ori: 0, // values on horizontal axis
        dir: 1, // left-to-right
        range: [0, maxVal * 1.25], // pad for value labels
      },
    },
    axes: [
      {
        side: 3, // left side for category labels
        stroke: axisColor(dark),
        grid: { show: false },
        ticks: { show: false },
        gap: 10,
        size: 140,
        // Force splits at exact integer positions so every tool gets a label.
        splits: (_u: uPlot) => toolNames.map((_, i) => i),
        values: (_u: uPlot, splits: number[]) =>
          splits.map((i) => toolNames[i] ?? ""),
      },
      {
        side: 2, // bottom for values
        stroke: axisColor(dark),
        grid: { stroke: gridColor(dark) },
        ticks: { stroke: gridColor(dark) },
        space: 80, // minimum pixels between ticks to avoid overlap
        values: (_u: uPlot, splits: number[]) =>
          splits.map((v) => formatValue(v, unit)),
        size: 40,
      },
    ],
    series,
    hooks: {
      draw: [
        (u: uPlot) => {
          const ctx = u.ctx;
          ctx.save();
          ctx.font = "bold 12px sans-serif";

          for (let si = 0; si < chartSeriesData.length; si++) {
            const row = chartSeriesData[si];
            for (let di = 0; di < row.length; di++) {
              const val = row[di];
              if (val == null) continue;

              const label = formatValue(val, chartUnit);
              // In horizontal bar mode with ori=1 on x-scale,
              // x maps to vertical position, y maps to horizontal position.
              const cx = u.valToPos(di, "x", true);
              const cy = u.valToPos(val, "y", true);

              const labelWidth = ctx.measureText(label).width;
              const chartLeft = u.valToPos(0, "y", true);
              const barWidth = Math.abs(cy - chartLeft);

              ctx.fillStyle = textColor(dark);
              ctx.textBaseline = "middle";

              if (barWidth > labelWidth + 16) {
                // Place inside bar, near the end
                ctx.textAlign = "right";
                ctx.fillText(label, cy - 8, cx);
              } else {
                // Place outside bar, to the right
                ctx.textAlign = "left";
                ctx.fillText(label, cy + 6, cx);
              }
            }
          }

          ctx.restore();
        },
      ],
    },
  };

  return { opts, chartData: [indices, ...seriesData] as uPlot.AlignedData };
}

// -- Chart lifecycle --

async function createCharts(dark: boolean) {
  destroyCharts();

  if (!data || data.timestamps.length === 0) return;

  const indices = data.timestamps.map((_: number, i: number) => i);
  const tWidth = throughputContainer.value?.clientWidth ?? 600;
  const mWidth = timingContainer.value?.clientWidth ?? 600;

  const throughputData: uPlot.AlignedData = [
    indices,
    data.throughputs,
    data.throughputCILo,
    data.throughputCIHi,
  ];

  const timingData: uPlot.AlignedData = [
    indices,
    data.timings,
    data.timingCILo,
    data.timingCIHi,
  ];

  if (throughputContainer.value) {
    throughputChart = new uPlot(
      makeOpts(
        "Aggregate Throughput",
        "MB/s",
        "MB/s",
        "#22c55e",
        "rgba(34,197,94,0.12)",
        tWidth,
        dark,
      ),
      throughputData,
      throughputContainer.value,
    );
  }

  if (timingContainer.value) {
    timingChart = new uPlot(
      makeOpts(
        "Aggregate Timing",
        "ms",
        "ms",
        "#6366f1",
        "rgba(99,102,241,0.12)",
        mWidth,
        dark,
      ),
      timingData,
      timingContainer.value,
    );
  }

  // -- Bar comparison charts (only when baselines exist) --
  if (data.baselines.length > 0) {
    hasBaselines.value = true;
    // Wait for Vue to render the bar chart containers before creating uPlot instances.
    await nextTick();

    const lastIdx = data.timestamps.length - 1;
    const cmakefmtThroughput = data.throughputs[lastIdx];
    const cmakefmtTiming = data.timings[lastIdx];

    const toolNames = ["cmakefmt", ...data.baselines.map((b) => b.label)];
    const throughputValues = [
      cmakefmtThroughput,
      ...data.baselines.map((b) => b.throughputMBps),
    ];
    const timingValues = [
      cmakefmtTiming,
      ...data.baselines.map((b) => b.timingMs),
    ];

    const barWidth = throughputBarContainer.value?.clientWidth ?? 600;

    if (throughputBarContainer.value) {
      const { opts, chartData } = makeBarOpts(
        "Throughput Comparison",
        "MB/s",
        toolNames,
        throughputValues,
        barWidth,
        dark,
      );
      throughputBarChart = new uPlot(
        opts,
        chartData,
        throughputBarContainer.value,
      );
    }

    if (timingBarContainer.value) {
      const { opts, chartData } = makeBarOpts(
        "Timing Comparison",
        "ms",
        toolNames,
        timingValues,
        barWidth,
        dark,
      );
      timingBarChart = new uPlot(opts, chartData, timingBarContainer.value);
    }
  } else {
    hasBaselines.value = false;
  }
}

function destroyCharts() {
  throughputChart?.destroy();
  timingChart?.destroy();
  throughputBarChart?.destroy();
  timingBarChart?.destroy();
  throughputChart = null;
  timingChart = null;
  throughputBarChart = null;
  timingBarChart = null;
}

function handleResize() {
  if (throughputChart && throughputContainer.value) {
    throughputChart.setSize({
      width: throughputContainer.value.clientWidth,
      height: 320,
    });
  }
  if (timingChart && timingContainer.value) {
    timingChart.setSize({
      width: timingContainer.value.clientWidth,
      height: 320,
    });
  }
  if (throughputBarChart && throughputBarContainer.value) {
    throughputBarChart.setSize({
      width: throughputBarContainer.value.clientWidth,
      height: 160,
    });
  }
  if (timingBarChart && timingBarContainer.value) {
    timingBarChart.setSize({
      width: timingBarContainer.value.clientWidth,
      height: 160,
    });
  }
}

watch(isDark, (dark) => createCharts(dark));

onMounted(async () => {
  try {
    const res = await fetch(`${DATA_URL}?_t=${Date.now()}`);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const json = await res.json();

    const entries: any[] = json.entries ?? [];
    entries.sort(
      (a: any, b: any) =>
        new Date(a.commit_timestamp).getTime()
        - new Date(b.commit_timestamp).getTime(),
    );

    data = parseEntries(entries, json.baselines);

    loading.value = false;
    await nextTick();

    createCharts(isDark.value);

    resizeObserver = new ResizeObserver(handleResize);
    if (throughputContainer.value) {
      resizeObserver.observe(throughputContainer.value);
    }
    if (timingContainer.value) resizeObserver.observe(timingContainer.value);
    if (throughputBarContainer.value) {
      resizeObserver.observe(throughputBarContainer.value);
    }
    if (timingBarContainer.value) {
      resizeObserver.observe(timingBarContainer.value);
    }
  } catch (e: any) {
    error.value = e?.message ?? "Failed to load benchmark data";
    loading.value = false;
  }
});

onUnmounted(() => {
  destroyCharts();
  resizeObserver?.disconnect();
  resizeObserver = null;
});
</script>

<template>
  <div class="benchmark-charts">
    <div v-if="loading" class="state-msg">Loading benchmark data…</div>
    <div v-else-if="error" class="state-msg error">Error: {{ error }}</div>
    <template v-else>
      <div ref="throughputContainer" class="chart-container" />
      <div ref="timingContainer" class="chart-container" />
      <template v-if="hasBaselines">
        <h3 class="comparison-heading">Tool Comparison</h3>
        <div ref="throughputBarContainer" class="chart-container bar-chart" />
        <div ref="timingBarContainer" class="chart-container bar-chart" />
      </template>
    </template>
  </div>
</template>

<style scoped>
.benchmark-charts {
  margin-top: 1.5rem;
}

.chart-container {
  width: 100%;
  margin-bottom: 2rem;
}

.bar-chart {
  margin-bottom: 1.5rem;
}

.comparison-heading {
  margin-top: 1rem;
  margin-bottom: 1rem;
  font-size: 1.15rem;
  font-weight: 600;
  color: var(--vp-c-text-1);
}

.state-msg {
  padding: 2rem;
  text-align: center;
  color: var(--vp-c-text-2);
}

.state-msg.error {
  color: var(--vp-c-danger-1);
}

/* Hide CI band series from the uPlot legend (3rd and 4th entries). */
.chart-container :deep(.u-legend .u-series:nth-child(n+3)) {
  display: none;
}

/* Legend styling for line charts. */
.chart-container :deep(.u-legend) {
  font-size: 0.85rem;
  text-align: center;
}

.chart-container :deep(.u-legend .u-label) {
  font-weight: normal;
}
</style>
