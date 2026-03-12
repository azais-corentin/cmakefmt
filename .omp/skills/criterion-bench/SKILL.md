# Criterion Benchmark Skill

How to run Criterion 0.5 benchmarks, compare baselines, and detect performance regressions.

## Baseline Comparison Workflow

Criterion compares two runs: a saved **baseline** (before changes) and a **new sample** (after changes). The workflow is always two steps:

1. **Save a baseline** on the current (clean) code:

```bash
cargo bench -- --save-baseline before 2>/dev/null
```

2. **Make your changes**, then compare against the saved baseline:

```bash
cargo bench -- --baseline before 2>/dev/null
```

Criterion prints a summary to stdout showing whether each benchmark improved, regressed, or showed no change. Detailed statistics are written to disk.

## Output Location

After a comparison run, per-benchmark change statistics are written to:

```
target/criterion/<group>/<bench_id>/change/estimates.json
```

Find all of them with:

```bash
find target/criterion -path '*/change/estimates.json'
```

## Parsing `estimates.json`

Each file contains `mean` and `median` objects with identical structure:

```json
{
  "mean": {
    "point_estimate": -0.0014,
    "confidence_interval": {
      "confidence_level": 0.95,
      "lower_bound": -0.0057,
      "upper_bound": 0.0040
    },
    "standard_error": 0.0025
  },
  "median": { }
}
```

**Values are fractional, not percentages.** `0.05` means 5% slower. `-0.02` means 2% faster.

### Extract regressions with `jq`

Flag any benchmark where `mean.point_estimate > 0.05` (more than 5% slower):

```bash
find target/criterion -path '*/change/estimates.json' -exec sh -c '
  jq -e "select(.mean.point_estimate > 0.05)" "$1" >/dev/null 2>&1 && echo "REGRESSION: $1 ($(jq -r ".mean.point_estimate * 100 | round | tostring + \"%\"" "$1"))"
' _ {} \;
```

A stricter check flags any benchmark whose confidence interval lower bound is above zero (statistically significant slowdown at the configured confidence level):

```bash
find target/criterion -path '*/change/estimates.json' -exec sh -c '
  jq -e "select(.mean.confidence_interval.lower_bound > 0)" "$1" >/dev/null 2>&1 && echo "SIGNIFICANT REGRESSION: $1"
' _ {} \;
```

## Regression Threshold Guidance

| Condition | Meaning | When to use |
|---|---|---|
| `mean.point_estimate > 0.05` | Point estimate exceeds 5% regression | General-purpose gate |
| `mean.confidence_interval.lower_bound > 0` | Statistically significant slowdown | Strict gate: even small regressions flagged if confident |
| `mean.point_estimate > 0.10` | Point estimate exceeds 10% regression | Lenient gate for noisy environments |

Choose based on environment stability. CI with dedicated hardware can use the strict gate. Local laptops with variable load should use a wider threshold.

## Useful Flags

| Flag | Default | Purpose |
|---|---|---|
| `--save-baseline <name>` | `base` | Save results under a named baseline |
| `--baseline <name>` | `base` | Compare against a named baseline (fails if missing) |
| `--baseline-lenient <name>` | — | Compare against baseline, skip benchmarks that lack it |
| `--sample-size <N>` | 100 | Number of samples per benchmark |
| `--warm-up-time <secs>` | 3 | Warm-up duration before sampling |
| `--measurement-time <secs>` | 5 | Measurement duration per sample |
| `--noise-threshold <f>` | 0.01 | Changes below this fraction are considered noise |
| `--confidence-level <f>` | 0.95 | Confidence level for intervals |
| `--significance-level <f>` | 0.05 | Threshold for significance tests |

Increase `--sample-size` or `--measurement-time` when results are noisy. Use `--baseline-lenient` when benchmarks have been added or removed between runs.
