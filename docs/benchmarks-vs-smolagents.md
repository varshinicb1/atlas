# Atlas vs smolagents: Benchmarks

> **Credibility notice.** The previous version of this document contained
> hand-written estimates ("0.3ms", "2.1s", "~$0.10/query", "997/1000 correct").
> Those were guesses and have been **removed**. Every Atlas figure below was
> **measured on a real machine** by running the prebuilt `atlas.exe` binary
> (see `benchmarks/run_benchmarks.py`). Every smolagents figure is **modeled
> from published pricing/latency** and is explicitly labeled as not measured
> on this machine (see `benchmarks/smolagents_comparison.py`). We do not mix
> the two as if both were measured.

## Architecture Comparison (factual)

| Dimension | Atlas | smolagents |
|-----------|-------|------------|
| Knowledge storage | Deterministic `.atlas` binary | LLM context window / RAG |
| Decision engine | Decision trees (zero LLM calls) | `CodeAgent` writes & runs Python via LLM |
| Verification | Built-in `verify` (API/version/provenance) | None built in |
| Audit trail | Cryptographic, replayable | None |
| Per-query API cost | **$0** (local compute only) | Per-token LLM cost (see below) |
| Offline capable | Yes — single binary, air-gapped | No — requires LLM API |
| Deterministic | Yes (verified, see below) | No — LLM is non-deterministic |
| Compliance (EU AI Act) | Built-in `verify --policy eu-ai-act` | None built in |

## Methodology (how to reproduce)

1. `py benchmarks/run_benchmarks.py --n 1000` compiles
   `packages/rust_patterns.md` (solve) and
   `packages/flutter_core.md` + `packages/decisions/flutter_core.yaml`
   (decide), then runs 1,000 `atlas solve` and 1,000 `atlas decide`
   invocations, timing each with `time.perf_counter()` around
   `subprocess.run`.
2. **Timing is end-to-end CLI latency**: it includes OS process spawn, Rust
   binary startup, `.atlas` read + deserialize, *and* the solve/decide work.
   It is **not** in-process library latency. Process-spawn overhead is the
   dominant term and is intentionally included for honesty.
3. 10 uncounted warmup calls run first so OS file caches stabilize.
4. Determinism: the same `solve` query is run 100× and the `--json` stdout is
   compared byte-for-byte.

**Machine the numbers below were measured on:**
- Date: 2026-07-11
- OS: Windows 11 (AMD64)
- CPU: Intel64 Family 6 Model 186 (16 logical cores)
- RAM: 16 GB
- Binary: `atlas 0.1.0` (prebuilt `target/release/atlas.exe`)
- Python: 3.14.3 (harness only; Atlas itself is the Rust binary)
- Sample size: N = 1,000 per command (+ 10 warmup), determinism N = 100

## Atlas — MEASURED results

Source of truth: `benchmarks/results.json`.

### Bundle build

| Metric | Solve bundle | Decide bundle |
|--------|--------------|---------------|
| Source | `packages/rust_patterns.md` | `flutter_core.md` + `decisions/flutter_core.yaml` |
| `.atlas` size | **7,509 bytes** | **7,951 bytes** |
| Compile time | **17.3 ms** | **17.8 ms** |
| Nodes / edges / decision trees | 20 / 39 / 0 | 18 / 27 / **2** |

### Query latency (N = 1,000, milliseconds, full CLI invocation)

| Percentile | `atlas solve` | `atlas decide` |
|------------|---------------|----------------|
| min | 10.81 | 12.76 |
| mean | **14.24** | **17.68** |
| median (p50) | **12.84** | **16.48** |
| p95 | 20.93 | 24.54 |
| p99 | 28.13 | 32.76 |
| max | 113.08 | 56.90 |

Decide latency is marginally higher because it additionally loads the
decision trees and walks the chosen tree. Both are in the ~12–18 ms mean
range for a cold CLI process.

### Determinism

| Check | Result |
|-------|--------|
| Same `solve` query, 100 runs, byte-identical `--json` output | **YES** |

Atlas is deterministic by construction: identical input → identical output,
no LLM, no sampling.

## smolagents — MODELED comparison (not measured here)

Run `py benchmarks/smolagents_comparison.py` to regenerate. These figures use
**published per-token pricing + typical agentic latency** and are flagged
`measured: false`. To get ground-truth numbers, run with `--measure`
(requires an OpenAI API key and spends real money).

Assumptions per agentic query: ~1,500 input + ~600 output tokens (gpt-4o),
~900 output tokens (deepseek-r1); typical round-trip latency per query
(gpt-4o ≈ 2.1 s, deepseek-r1 ≈ 3.8 s) reflecting multi-step CodeAgent
(plan → code → execute → observe).

| Model | Cost / query | Cost / 1,000 queries | Mean latency | p95 latency | p99 latency |
|-------|--------------|----------------------|--------------|-------------|-------------|
| gpt-4o (modeled) | $0.00975 (~1.0¢) | $9.75 | 2,090 ms | 3,015 ms | 3,299 ms |
| deepseek-r1 (modeled) | $0.00280 (~0.3¢) | $2.80 | 3,783 ms | 5,456 ms | 5,970 ms |

## Head-to-head (measured Atlas vs modeled smolagents)

| Metric | Atlas (measured) | smolagents gpt-4o (modeled) | smolagents deepseek-r1 (modeled) |
|--------|------------------|-----------------------------|----------------------------------|
| Mean query latency | **~14 ms** | ~2,090 ms | ~3,783 ms |
| Latency speedup | 1× | **~147× faster** | **~266× faster** |
| p99 latency | **~28 ms** | ~3,299 ms | ~5,970 ms |
| Per-query API cost | **$0** | ~$0.0098 | ~$0.0028 |
| Deterministic | **Yes (verified)** | No | No |
| Offline | **Yes** | No | No |
| Cost for 10,000 queries (API) | **$0** | ~$97.50 | ~$28.00 |

> The "speedup" is an **order-of-magnitude comparison**: a measured local
> binary (≈14 ms, including process spawn) vs a modeled cloud LLM round-trip
> (≈2–4 s). The Atlas number is a ceiling on CLI cost; an in-process library
> call would be lower. The smolagents numbers are estimates — treat the
> ratio as "roughly two-to-three orders of magnitude", not a precise figure.

## What we did NOT measure (honest gaps)

- **Answer correctness / accuracy.** We did not run a labeled Q&A benchmark,
  so we make no claim like "99.7% correct". Atlas returns the knowledge-graph
  match deterministically; whether that matches a human's intent is a
  separate evaluation we have not performed.
- **In-process library latency.** All Atlas numbers are full CLI invocations.
  Embedding Atlas as a library would remove spawn overhead.
- **Concurrency / throughput.** Single-threaded, single-process only.
- **Electricity / carbon.** Atlas incurs negligible local compute; we did not
  meter it. smolagents carbon depends on the model provider's datacenter.

## How to reproduce

```powershell
# Measure Atlas (writes benchmarks/results.json)
py benchmarks\run_benchmarks.py --n 1000

# Model smolagents from published pricing (writes benchmarks/smolagents_results.json)
py benchmarks\smolagents_comparison.py --n 1000

# Optional: real smolagents measurement (costs money, needs `pip install openai`)
$env:OPENAI_API_KEY = "sk-..."
py benchmarks\smolagents_comparison.py --measure --model gpt-4o --n 50
```

See `benchmarks/README.md` for full details and limitations.
