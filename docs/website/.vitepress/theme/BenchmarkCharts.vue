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
const loading = ref(true);
const error = ref<string>();

interface ChartData {
  timestamps: number[];
  commitShas: string[];
  throughputs: number[];
  throughputCILo: number[];
  throughputCIHi: number[];
  timings: number[];
  timingCILo: number[];
  timingCIHi: number[];
}

let data: ChartData | null = null;
let throughputChart: uPlot | null = null;
let timingChart: uPlot | null = null;
let resizeObserver: ResizeObserver | null = null;

function axisColor(dark: boolean) {
  return dark ? "rgba(255,255,255,0.35)" : "rgba(0,0,0,0.25)";
}

function gridColor(dark: boolean) {
  return dark ? "rgba(255,255,255,0.08)" : "rgba(0,0,0,0.06)";
}

function tooltipValueFormatter(suffix: string) {
  return (
    _self: uPlot,
    rawValue: number,
    _seriesIdx: number,
    idx: number | null,
  ) => {
    if (idx == null || !data) return "--";
    const sha = data.commitShas[idx]?.slice(0, 7) ?? "?";
    const date = new Date(data.timestamps[idx] * 1000).toLocaleDateString();
    return `${rawValue.toFixed(2)} ${suffix}  (${sha} \u2014 ${date})`;
  };
}

/** Compute aggregate CIs from fixture-level median CIs. */
function parseEntries(raw: any[]): ChartData {
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

  return {
    timestamps,
    commitShas,
    throughputs,
    throughputCILo,
    throughputCIHi,
    timings,
    timingCILo,
    timingCIHi,
  };
}

function makeOpts(
  title: string,
  yLabel: string,
  suffix: string,
  strokeColor: string,
  bandFill: string,
  width: number,
  dark: boolean,
): uPlot.Options {
  return {
    width,
    height: 320,
    title,
    cursor: { drag: { x: true, y: false } },
    scales: {
      x: { time: false },
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
    series: [
      {
        value: (_u: uPlot, _v: number, _si: number, idx: number | null) => {
          if (idx == null || !data) return "--";
          const sha = data.commitShas[idx]?.slice(0, 7) ?? "?";
          const date = new Date(data.timestamps[idx] * 1000)
            .toLocaleDateString();
          return `${sha} \u2014 ${date}`;
        },
      },
      {
        label: yLabel,
        stroke: strokeColor,
        width: 2,
        value: tooltipValueFormatter(suffix),
        paths: uPlot.paths.stepped!({ align: 1 }),
      },
      // CI lower bound \u2014 hidden line, used only as band edge.
      {
        show: true,
        stroke: "transparent",
        width: 0,
        points: { show: false },
        paths: uPlot.paths.stepped!({ align: 1 }),
      },
      // CI upper bound \u2014 hidden line, used only as band edge.
      {
        show: true,
        stroke: "transparent",
        width: 0,
        points: { show: false },
        paths: uPlot.paths.stepped!({ align: 1 }),
      },
    ],
    bands: [
      { series: [3, 2], fill: bandFill },
    ],
  };
}

function createCharts(dark: boolean) {
  destroyCharts();

  if (!data || data.timestamps.length === 0) return;

  const indices = data.timestamps.map((_: number, i: number) => i);
  const tWidth = throughputContainer.value?.clientWidth ?? 600;
  const mWidth = timingContainer.value?.clientWidth ?? 600;

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
      [indices, data.throughputs, data.throughputCILo, data.throughputCIHi],
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
      [indices, data.timings, data.timingCILo, data.timingCIHi],
      timingContainer.value,
    );
  }
}

function destroyCharts() {
  throughputChart?.destroy();
  timingChart?.destroy();
  throughputChart = null;
  timingChart = null;
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
}

watch(isDark, (dark) => createCharts(dark));

onMounted(async () => {
  try {
    const res = await fetch(DATA_URL);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const json = await res.json();

    const entries: any[] = json.entries ?? [];
    entries.sort(
      (a: any, b: any) =>
        new Date(a.commit_timestamp).getTime()
        - new Date(b.commit_timestamp).getTime(),
    );

    data = parseEntries(entries);

    loading.value = false;
    await nextTick();

    createCharts(isDark.value);

    resizeObserver = new ResizeObserver(handleResize);
    if (throughputContainer.value) {
      resizeObserver.observe(throughputContainer.value);
    }
    if (timingContainer.value) resizeObserver.observe(timingContainer.value);
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
</style>
