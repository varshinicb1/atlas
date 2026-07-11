# Atlas Benchmark Suite

Real, reproducible benchmarks for the Atlas Knowledge Operating System. No hand-written estimates — every Atlas number here was **measured on a real machine** by running the prebuilt `atlas.exe` binary.

## What's in this directory

| File | Purpose |
|------|---------|
| `run_benchmarks.py` | The harness. Compiles a package, runs N `atlas solve` / `atlas decide` queries, measures latency, verifies determinism, writes `results.json`. **Measured numbers only.** |
| `results.json` | The latest measured Atlas results (latency percentiles, bundle sizes, compile time, determinism). |
| `smolagents_comparison.py` | Honest cost/latency comparison vs smolagents. Modeled from published pricing by default; can do live measurement if an API key is supplied. |
| `smolagents_results.json` | Output of the smolagents comparison run. |
| `bench_solve.atlas` / `bench_decide.atlas` | The compiled bundles produced during the run (artifacts, regenerated each run). |

## Prerequisites

- Windows (this suite was built/run on Windows 11). The harness uses `target/release/atlas.exe`.
- The prebuilt binary at `target/release/atlas.exe` (do **not** rebuild cargo — use what's already built).
- Python 3 available as `py` (or `python3`). Tested with Python 3.14.

> The harness auto-detects `target/release/atlas.exe` relative to the repo root. Override with `--atlas <path>` if needed.

## Run the Atlas benchmarks

```powershell
# 1000 measured iterations (default)
py benchmarks\run_benchmarks.py

# smaller / faster run
py benchmarks\run_benchmarks.py --n 200 --warmup 5

# explicit binary path
py benchmarks\run_benchmarks.py --atlas target\release\atlas.exe
```

This will:
1. Compile `packages/rust_patterns.md` → `bench_solve.atlas` and measure compile time + bundle size.
2. Compile `packages/flutter_core.md` + `packages/decisions/flutter_core.yaml` → `bench_decide.atlas` (includes 2 decision trees) and measure compile time + bundle size.
3. Run N `atlas solve` queries (10 varied real queries, cycled) and record per-call latency.
4. Run N `atlas decide` queries (5 real queries with `-c answer=false` to reach a terminal node) and record per-call latency.
5. Run the **same** `solve` query 100× and check the `--json` output is byte-for-byte identical (determinism).
6. Print a summary table and write everything to `benchmarks/results.json`.

## Run the smolagents comparison

```powershell
# Modeled from published pricing + typical latency (no API key, no network)
py benchmarks\smolagents_comparison.py --n 1000

# Live measurement (requires `pip install openai` + an OpenAI API key; costs real money)
$env:OPENAI_API_KEY = "sk-..."
py benchmarks\smolagents_comparison.py --measure --model gpt-4o --n 50
```

**Important:** smolagents numbers are **modeled** unless you run `--measure`. They are clearly flagged `measured: false` in the JSON. Do not present modeled numbers as if they were measured on this machine.

## How to read the numbers (honesty notes)

- **Latency includes process spawn.** Each measurement spawns a fresh `atlas.exe` via `subprocess.run`. The reported milliseconds therefore include OS process creation, Rust binary startup, `.atlas` file read + deserialize, **and** the solve/decide work. This is the honest cost of one CLI query. It is *not* the in-process library latency (which would be lower). The exact `timing_includes` list and `timing_caveat` are stored in `results.json`.
- **Warmup is excluded.** 10 uncounted warmup calls run first so OS file caches stabilize; only the N measured calls are reported.
- **Sample size is N=1000** (configurable). We report min / mean / median (p50) / p95 / p99 / max in milliseconds.
- **Determinism is real.** Same query 100×, byte-identical `--json` stdout. Atlas is deterministic by construction.
- **Hardware is recorded** in `results.json` (`measured_on_machine`) so results are attributable to a specific environment.

## Known limitations

- `decide` with the *published* flutter decision trees only reaches a terminal node when the CLI-provided `-c answer=<value>` context is consistent along the chosen path. The trees reuse the single key `answer` at every branch, so paths requiring `answer=true` then `answer=external` cannot be expressed via the current CLI (one key, one value). The harness uses `answer=false`, which reliably reaches the `StatelessWidget` terminal. This is a real property of the current package/CLI, not a benchmark artifact.
- smolagents modeled figures depend on published pricing/latency assumptions stored in `MODELS` inside `smolagents_comparison.py`. They are estimates; run `--measure` for ground truth.
- Only single-threaded, single-process latency is measured. No concurrency, no server-mode, no network registry calls.
