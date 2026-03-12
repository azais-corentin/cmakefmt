use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const SUMMARY_SCHEMA_VERSION: &str = "cmakefmt.trace.summary.v1";
const EVENT_FORMAT_INVOCATION: &str = "cmakefmt.format.invocation";

const STAGE_EVENT_NAMES: &[(&str, &str)] = &[
    ("cmakefmt.format.bypass_check", "bypassCheck"),
    ("cmakefmt.format.strip_bom", "stripBom"),
    ("cmakefmt.format.pipeline", "pipeline"),
    ("cmakefmt.format.normalize_bare_cr", "normalizeBareCr"),
    ("cmakefmt.format.parse", "parse"),
    (
        "cmakefmt.format.resolve_print_options",
        "resolvePrintOptions",
    ),
    ("cmakefmt.format.generate_ir", "generateIr"),
    ("cmakefmt.format.print", "print"),
    ("cmakefmt.format.post_process", "postProcess"),
    ("cmakefmt.format.finalize_whitespace", "finalizeWhitespace"),
    ("cmakefmt.format.final_newline", "finalNewline"),
    ("cmakefmt.format.restore_bare_cr", "restoreBareCr"),
    ("cmakefmt.parser.file", "parserFile"),
    ("cmakefmt.parser.command", "parserCommand"),
    ("cmakefmt.gen_file", "genFile"),
    ("cmakefmt.gen_file.command", "genFileCommand"),
    ("cmakefmt.gen_command", "genCommand"),
    ("cmakefmt.printer.format", "printerFormat"),
    ("cmakefmt.post_process", "postProcessAll"),
    ("cmakefmt.post_process.align_block", "postProcessAlignBlock"),
    (
        "cmakefmt.post_process.reflow_comment",
        "postProcessReflowComment",
    ),
];

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TraceModeFlags {
    pub check: bool,
    pub diff: bool,
    pub write: bool,
    pub stdin: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TraceFileRecord {
    pub path: String,
    pub input_bytes: u64,
    pub changed: bool,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TraceSummaryInput<'a> {
    pub tool_version: &'a str,
    pub mode: TraceModeFlags,
    pub file_count: usize,
    pub input_bytes_total: u64,
    pub changed_files: usize,
    pub error_count: usize,
    pub total_wall_ms: f64,
    pub file_records: Vec<TraceFileRecord>,
}

#[derive(Debug, Deserialize)]
struct ChromeTraceEvent {
    name: String,
    ph: String,
    #[serde(default)]
    ts: Option<f64>,
    #[serde(default)]
    dur: Option<f64>,
    #[serde(default)]
    tid: Option<u64>,
    #[serde(default)]
    pid: Option<u64>,
    #[serde(default)]
    args: Option<Value>,
}

#[derive(Debug, Clone)]
struct NormalizedEvent {
    name: String,
    start_us: f64,
    end_us: f64,
    duration_us: f64,
    tid: u64,
    args: Option<Value>,
    child_duration_us: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SummaryDocument {
    schema_version: &'static str,
    tool_version: String,
    generated_at: String,
    generated_at_unix_ms: u128,
    invocation: InvocationSection,
    timing: TimingSection,
    files: Vec<FileSummary>,
    notes: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct InvocationSection {
    mode: TraceModeFlags,
    file_count: usize,
    input_bytes_total: u64,
    changed_files: usize,
    error_count: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TimingSection {
    total_wall_ms: f64,
    stages: Vec<AggregateEntry>,
    hotspots: Vec<HotspotEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AggregateEntry {
    name: String,
    total_ms: f64,
    pct_total: f64,
    calls: usize,
    avg_ms: f64,
    p95_ms: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HotspotEntry {
    name: String,
    total_ms: f64,
    self_ms: f64,
    calls: usize,
    avg_ms: f64,
    p95_ms: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct FileSummary {
    path: String,
    input_bytes: u64,
    changed: bool,
    stage_durations_ms: BTreeMap<String, f64>,
    status: String,
}

#[derive(Default)]
struct AggregateAccumulator {
    durations_ms: Vec<f64>,
    total_ms: f64,
    self_ms: f64,
}

type BeginStackKey = (u64, u64, String);
type BeginStackValue = Vec<(f64, Option<Value>)>;

pub(crate) fn write_summary_from_trace(
    trace_path: &Path,
    summary_path: &Path,
    summary_input: &TraceSummaryInput<'_>,
) -> Result<()> {
    let trace_contents = std::fs::read_to_string(trace_path)
        .with_context(|| format!("failed to read trace file {}", trace_path.display()))?;
    let raw_trace_events = parse_trace_events(&trace_contents)
        .with_context(|| format!("failed to parse trace file {}", trace_path.display()))?;

    let mut events = normalize_events(raw_trace_events);
    events.sort_by(|a, b| a.start_us.total_cmp(&b.start_us));
    derive_self_time(&mut events);

    let stage_name_map: HashMap<&'static str, &'static str> =
        STAGE_EVENT_NAMES.iter().copied().collect();
    let stages = build_stage_aggregates(&events, &stage_name_map, summary_input.total_wall_ms);
    let hotspots = build_hotspot_aggregates(&events);
    let files = build_file_summaries(&events, &stage_name_map, &summary_input.file_records);

    let generated_at_unix_ms = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    let summary = SummaryDocument {
        schema_version: SUMMARY_SCHEMA_VERSION,
        tool_version: summary_input.tool_version.to_string(),
        generated_at: generated_at_unix_ms.to_string(),
        generated_at_unix_ms,
        invocation: InvocationSection {
            mode: summary_input.mode,
            file_count: summary_input.file_count,
            input_bytes_total: summary_input.input_bytes_total,
            changed_files: summary_input.changed_files,
            error_count: summary_input.error_count,
        },
        timing: TimingSection {
            total_wall_ms: round_ms(summary_input.total_wall_ms),
            stages,
            hotspots,
        },
        files,
        notes: vec![
            "Summary excludes CMake source snippets and argument text by design.".to_string(),
        ],
    };

    std::fs::write(summary_path, serde_json::to_vec_pretty(&summary)?)
        .with_context(|| format!("failed to write summary file {}", summary_path.display()))?;
    Ok(())
}

fn parse_trace_events(trace_contents: &str) -> Result<Vec<ChromeTraceEvent>> {
    let parsed: Value = serde_json::from_str(trace_contents)?;
    match parsed {
        Value::Array(events) => {
            let events = serde_json::from_value::<Vec<ChromeTraceEvent>>(Value::Array(events))?;
            Ok(events)
        }
        Value::Object(object) => {
            let Some(events) = object.get("traceEvents") else {
                return Err(anyhow!("missing traceEvents in trace JSON object"));
            };
            let events = serde_json::from_value::<Vec<ChromeTraceEvent>>(events.clone())?;
            Ok(events)
        }
        _ => Err(anyhow!("unexpected trace JSON shape")),
    }
}

fn normalize_events(events: Vec<ChromeTraceEvent>) -> Vec<NormalizedEvent> {
    let mut normalized = Vec::new();
    let mut begin_stack: HashMap<BeginStackKey, BeginStackValue> = HashMap::new();

    for event in events {
        let Some(start_us) = event.ts else {
            continue;
        };
        let tid = event.tid.unwrap_or(0);
        let pid = event.pid.unwrap_or(0);

        match event.ph.as_str() {
            "X" => {
                let duration_us = event.dur.unwrap_or(0.0).max(0.0);
                normalized.push(NormalizedEvent {
                    name: event.name,
                    start_us,
                    end_us: start_us + duration_us,
                    duration_us,
                    tid,
                    args: event.args,
                    child_duration_us: 0.0,
                });
            }
            "B" => {
                begin_stack
                    .entry((pid, tid, event.name))
                    .or_default()
                    .push((start_us, event.args));
            }
            "E" => {
                let key = (pid, tid, event.name.clone());
                let Some(stack) = begin_stack.get_mut(&key) else {
                    continue;
                };
                let Some((begin_us, begin_args)) = stack.pop() else {
                    continue;
                };
                let duration_us = (start_us - begin_us).max(0.0);
                normalized.push(NormalizedEvent {
                    name: event.name,
                    start_us: begin_us,
                    end_us: begin_us + duration_us,
                    duration_us,
                    tid,
                    args: begin_args,
                    child_duration_us: 0.0,
                });
                if stack.is_empty() {
                    begin_stack.remove(&key);
                }
            }
            _ => {}
        }
    }

    normalized
}

fn derive_self_time(events: &mut [NormalizedEvent]) {
    let mut indices_by_tid: HashMap<u64, Vec<usize>> = HashMap::new();
    for (idx, event) in events.iter().enumerate() {
        indices_by_tid.entry(event.tid).or_default().push(idx);
    }

    for indices in indices_by_tid.values_mut() {
        indices.sort_by(|a, b| events[*a].start_us.total_cmp(&events[*b].start_us));
        let mut stack: Vec<usize> = Vec::new();

        for idx in indices.iter().copied() {
            while let Some(parent_idx) = stack.last().copied() {
                if events[parent_idx].end_us <= events[idx].start_us {
                    stack.pop();
                } else {
                    break;
                }
            }

            if let Some(parent_idx) = stack.last().copied()
                && events[idx].end_us <= events[parent_idx].end_us
            {
                events[parent_idx].child_duration_us += events[idx].duration_us;
            }

            stack.push(idx);
        }
    }
}

fn build_stage_aggregates(
    events: &[NormalizedEvent],
    stage_name_map: &HashMap<&'static str, &'static str>,
    total_wall_ms: f64,
) -> Vec<AggregateEntry> {
    let mut acc: BTreeMap<String, AggregateAccumulator> = BTreeMap::new();

    for event in events {
        let Some(stage_name) = stage_name_map.get(event.name.as_str()) else {
            continue;
        };
        let duration_ms = us_to_ms(event.duration_us);
        let entry = acc.entry((*stage_name).to_string()).or_default();
        entry.total_ms += duration_ms;
        entry.durations_ms.push(duration_ms);
    }

    acc.into_iter()
        .map(|(name, aggregate)| {
            let calls = aggregate.durations_ms.len();
            let avg = if calls == 0 {
                0.0
            } else {
                aggregate.total_ms / calls as f64
            };
            AggregateEntry {
                name,
                total_ms: round_ms(aggregate.total_ms),
                pct_total: pct(aggregate.total_ms, total_wall_ms),
                calls,
                avg_ms: round_ms(avg),
                p95_ms: round_ms(percentile95_ms(&aggregate.durations_ms)),
            }
        })
        .collect()
}

fn build_hotspot_aggregates(events: &[NormalizedEvent]) -> Vec<HotspotEntry> {
    let mut acc: HashMap<String, AggregateAccumulator> = HashMap::new();

    for event in events {
        if !event.name.starts_with("cmakefmt.") || event.name == EVENT_FORMAT_INVOCATION {
            continue;
        }
        let duration_ms = us_to_ms(event.duration_us);
        let self_ms = us_to_ms((event.duration_us - event.child_duration_us).max(0.0));
        let entry = acc.entry(event.name.clone()).or_default();
        entry.total_ms += duration_ms;
        entry.self_ms += self_ms;
        entry.durations_ms.push(duration_ms);
    }

    let mut hotspots: Vec<HotspotEntry> = acc
        .into_iter()
        .map(|(name, aggregate)| {
            let calls = aggregate.durations_ms.len();
            let avg = if calls == 0 {
                0.0
            } else {
                aggregate.total_ms / calls as f64
            };
            HotspotEntry {
                name,
                total_ms: round_ms(aggregate.total_ms),
                self_ms: round_ms(aggregate.self_ms),
                calls,
                avg_ms: round_ms(avg),
                p95_ms: round_ms(percentile95_ms(&aggregate.durations_ms)),
            }
        })
        .collect();

    hotspots.sort_by(|a, b| b.total_ms.total_cmp(&a.total_ms));
    hotspots.truncate(30);
    hotspots
}

fn build_file_summaries(
    events: &[NormalizedEvent],
    stage_name_map: &HashMap<&'static str, &'static str>,
    file_records: &[TraceFileRecord],
) -> Vec<FileSummary> {
    let mut files = Vec::new();

    let invocations: Vec<&NormalizedEvent> = events
        .iter()
        .filter(|event| event.name == EVENT_FORMAT_INVOCATION)
        .collect();

    if invocations.is_empty() {
        for record in file_records {
            files.push(FileSummary {
                path: record.path.clone(),
                input_bytes: record.input_bytes,
                changed: record.changed,
                stage_durations_ms: BTreeMap::new(),
                status: record.status.clone(),
            });
        }
        return files;
    }

    for (index, invocation) in invocations.iter().enumerate() {
        let record = file_records.get(index);
        let mut stage_durations_ms: BTreeMap<String, f64> = BTreeMap::new();

        for event in events {
            if event.tid != invocation.tid {
                continue;
            }
            if event.start_us < invocation.start_us || event.end_us > invocation.end_us {
                continue;
            }
            let Some(stage_name) = stage_name_map.get(event.name.as_str()) else {
                continue;
            };
            let value = stage_durations_ms
                .entry((*stage_name).to_string())
                .or_insert(0.0);
            *value += us_to_ms(event.duration_us);
        }

        for duration in stage_durations_ms.values_mut() {
            *duration = round_ms(*duration);
        }

        files.push(FileSummary {
            path: record
                .map(|f| f.path.clone())
                .or_else(|| {
                    invocation
                        .args
                        .as_ref()
                        .and_then(|a| extract_string_field(a, "path"))
                })
                .unwrap_or_else(|| format!("<invocation-{index}>")),
            input_bytes: record
                .map(|f| f.input_bytes)
                .or_else(|| {
                    invocation
                        .args
                        .as_ref()
                        .and_then(|a| extract_u64_field(a, "input_bytes"))
                })
                .unwrap_or_default(),
            changed: record
                .map(|f| f.changed)
                .or_else(|| {
                    invocation
                        .args
                        .as_ref()
                        .and_then(|a| extract_bool_field(a, "changed"))
                })
                .unwrap_or(false),
            stage_durations_ms,
            status: record
                .map(|f| f.status.clone())
                .unwrap_or_else(|| "ok".to_string()),
        });
    }

    if file_records.len() > files.len() {
        for extra in file_records.iter().skip(files.len()) {
            files.push(FileSummary {
                path: extra.path.clone(),
                input_bytes: extra.input_bytes,
                changed: extra.changed,
                stage_durations_ms: BTreeMap::new(),
                status: extra.status.clone(),
            });
        }
    }

    files
}

fn extract_string_field(value: &Value, key: &str) -> Option<String> {
    let field = value.get(key)?;
    match field {
        Value::String(text) => Some(text.clone()),
        Value::Bool(v) => Some(v.to_string()),
        Value::Number(number) => Some(number.to_string()),
        _ => None,
    }
}

fn extract_u64_field(value: &Value, key: &str) -> Option<u64> {
    let field = value.get(key)?;
    match field {
        Value::Number(number) => number.as_u64(),
        Value::String(text) => text.parse::<u64>().ok(),
        _ => None,
    }
}

fn extract_bool_field(value: &Value, key: &str) -> Option<bool> {
    let field = value.get(key)?;
    match field {
        Value::Bool(v) => Some(*v),
        Value::String(text) => match text.as_str() {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        },
        _ => None,
    }
}

fn percentile95_ms(samples: &[f64]) -> f64 {
    if samples.is_empty() {
        return 0.0;
    }

    let mut sorted = samples.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let idx = ((sorted.len() as f64) * 0.95).ceil() as usize;
    let idx = idx.saturating_sub(1).min(sorted.len() - 1);
    sorted[idx]
}

fn us_to_ms(value_us: f64) -> f64 {
    value_us / 1_000.0
}

fn pct(value: f64, total: f64) -> f64 {
    if total <= f64::EPSILON {
        0.0
    } else {
        round_ms((value / total) * 100.0)
    }
}

fn round_ms(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}
