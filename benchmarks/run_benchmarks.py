#!/usr/bin/env python3
"""
Atlas benchmark harness (REAL, MEASURED numbers only).

What this does:
  1. Compiles a knowledge package into a .atlas binary bundle.
  2. Runs N `atlas solve` queries and measures end-to-end latency
     (this includes process spawn + binary load + solve).
  3. Runs N `atlas decide` queries and measures latency similarly.
  4. Measures the .atlas file size and the compile time.
  5. Verifies determinism: runs the same query 100x and checks the
     output is byte-for-byte identical.
  6. Writes everything to benchmarks/results.json and prints a summary.

IMPORTANT HONESTY NOTE ON TIMING:
  Each measurement spawns a fresh `atlas.exe` subprocess
  (subprocess.run). The numbers below therefore include:
    - OS process creation / spawn overhead
    - Rust binary startup + std init
    - .atlas file read + deserialize
    - the actual solve/decide work
  This is the *honest real-world* cost of one CLI query. It is NOT the
  "in-memory library call" latency. If you want the latter, call the
  atlas_runtime crate directly (out of scope here). We report the CLI
  number because that is what a user pays per `atlas solve` invocation.

Run:
  py benchmarks/run_benchmarks.py            # N=1000 default
  py benchmarks/run_benchmarks.py --n 500
  py benchmarks/run_benchmarks.py --atlas target/release/atlas.exe

Requires: the prebuilt binary at target/release/atlas.exe (no cargo build).
"""

import argparse
import json
import os
import platform
import statistics
import subprocess
import sys
import time
import ctypes


def resolve_atlas(explicit):
    here = os.path.dirname(os.path.abspath(__file__))
    repo = os.path.dirname(here)
    if explicit:
        p = explicit
    else:
        p = os.path.join(repo, "target", "release", "atlas.exe")
    if not os.path.exists(p):
        sys.exit(f"FATAL: atlas binary not found at {p}")
    return os.path.abspath(p)


def run_capture(atlas, *args):
    """Run atlas and return (returncode, stdout_bytes, stderr_text)."""
    proc = subprocess.run(
        [atlas, *args],
        capture_output=True,
    )
    return proc.returncode, proc.stdout, proc.stderr.decode("utf-8", "replace")


def measure_calls(atlas, args_list, n, warmup=10):
    """Run each arg-set once (cycling) n times, return list of ms per call."""
    times = []
    # warmup (not counted) to let OS file cache stabilize
    for w in range(warmup):
        run_capture(atlas, *args_list[w % len(args_list)])
    for i in range(n):
        args = args_list[i % len(args_list)]
        t0 = time.perf_counter()
        proc = subprocess.run([atlas, *args], capture_output=True)
        t1 = time.perf_counter()
        if proc.returncode != 0:
            sys.stderr.write(
                f"WARN: non-zero exit for {args}: {proc.stderr.decode('utf-8','replace')[:200]}\n"
            )
        times.append((t1 - t0) * 1000.0)
    return times


def stats(ms_list):
    s = sorted(ms_list)
    n = len(s)

    def pct(p):
        if n == 1:
            return s[0]
        k = (n - 1) * (p / 100.0)
        f = int(k)
        c = min(f + 1, n - 1)
        if f == c:
            return s[f]
        return s[f] + (s[c] - s[f]) * (k - f)

    return {
        "n": n,
        "min_ms": round(s[0], 4),
        "mean_ms": round(statistics.fmean(s), 4),
        "median_ms": round(statistics.median(s), 4),
        "p95_ms": round(pct(95), 4),
        "p99_ms": round(pct(99), 4),
        "max_ms": round(s[-1], 4),
        "stdev_ms": round(statistics.pstdev(s), 4),
    }


def hardware():
    u = platform.uname()
    ram_bytes = None
    try:
        c = ctypes.c_ulonglong(0)
        ctypes.windll.kernel32.GetPhysicallyInstalledSystemMemory(ctypes.byref(c))
        ram_bytes = c.value * 1024  # returns kilobytes
    except Exception:
        pass
    if not ram_bytes or ram_bytes < 1_000_000:
        try:
            import shutil

            ram_bytes = shutil.virtual_memory().total
        except Exception:
            ram_bytes = None
    return {
        "os": f"{u.system} {u.release}",
        "machine": u.machine,
        "processor": u.processor or "unknown",
        "cpu_logical_count": os.cpu_count(),
        "ram_gb": round(ram_bytes / (1024**3), 1) if ram_bytes else None,
        "python": platform.python_version(),
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--n", type=int, default=1000, help="measured iterations")
    ap.add_argument("--warmup", type=int, default=10)
    ap.add_argument("--atlas", default=None)
    ap.add_argument(
        "--out", default=os.path.join(os.path.dirname(os.path.abspath(__file__)), "results.json")
    )
    args = ap.parse_args()

    atlas = resolve_atlas(args.atlas)
    here = os.path.dirname(os.path.abspath(__file__))
    repo = os.path.dirname(here)

    solve_src = os.path.join(repo, "packages", "rust_patterns.md")
    decide_md = os.path.join(repo, "packages", "flutter_core.md")
    decide_yaml = os.path.join(repo, "packages", "decisions", "flutter_core.yaml")

    solve_bundle = os.path.join(here, "bench_solve.atlas")
    decide_bundle = os.path.join(here, "bench_decide.atlas")

    print(f"Atlas binary : {atlas}")
    print(f"Iterations  : {args.n} (+ {args.warmup} warmup, uncounted)")
    print("=" * 64)

    # ---- Compile solve bundle ----
    t0 = time.perf_counter()
    rc, out, err = run_capture(atlas, "compile", solve_src, "-o", solve_bundle, "--json")
    t1 = time.perf_counter()
    if rc != 0:
        sys.exit(f"FATAL compile solve: {err}")
    compile_solve_ms = (t1 - t0) * 1000.0
    solve_bundle_bytes = os.path.getsize(solve_bundle)
    compile_solve_meta = json.loads(out.decode("utf-8"))

    # ---- Compile decide bundle (md + decision yaml) ----
    t0 = time.perf_counter()
    rc, out, err = run_capture(
        atlas, "compile", decide_md, decide_yaml, "-o", decide_bundle, "--json"
    )
    t1 = time.perf_counter()
    if rc != 0:
        sys.exit(f"FATAL compile decide: {err}")
    compile_decide_ms = (t1 - t0) * 1000.0
    decide_bundle_bytes = os.path.getsize(decide_bundle)
    compile_decide_meta = json.loads(out.decode("utf-8"))

    print(
        f"Compiled solve bundle  : {compile_solve_ms:.1f} ms  "
        f"({solve_bundle_bytes} bytes, {compile_solve_meta.get('nodes')} nodes)"
    )
    print(
        f"Compiled decide bundle : {compile_decide_ms:.1f} ms  "
        f"({decide_bundle_bytes} bytes, {compile_decide_meta.get('decision_trees')} decision trees)"
    )

    # ---- Solve queries (real, varied) ----
    solve_queries = [
        "error handling",
        "Result type in Rust",
        "async programming with tokio",
        "serde serialization",
        "ownership and borrowing",
        "lifetimes in Rust",
        "traits and generics",
        "pattern matching",
        "smart pointers Box Arc",
        "Option and the ? operator",
    ]
    solve_args = [
        ["solve", "--bundle", solve_bundle, q, "--json"] for q in solve_queries
    ]
    print(f"\nRunning {args.n} `atlas solve` invocations ...")
    solve_times = measure_calls(atlas, solve_args, args.n, warmup=args.warmup)
    solve_stats = stats(solve_times)
    print(
        f"  mean={solve_stats['mean_ms']:.2f}ms  "
        f"median={solve_stats['median_ms']:.2f}ms  "
        f"p95={solve_stats['p95_ms']:.2f}ms  "
        f"p99={solve_stats['p99_ms']:.2f}ms  "
        f"min={solve_stats['min_ms']:.2f}ms  max={solve_stats['max_ms']:.2f}ms"
    )

    # ---- Decide queries (real, must match a tree + consistent path) ----
    # The published flutter decision trees require a single consistent
    # `answer` context down the chosen path to reach a terminal node.
    # `answer=false` reliably reaches a terminal (StatelessWidget path).
    decide_queries = [
        "flutter widget for a list",
        "widget choice for a button",
        "ui for a settings screen",
        "flutter state management",
        "flutter ui for a form",
    ]
    decide_args = [
        ["decide", "--bundle", decide_bundle, q, "-c", "answer=false", "--json"]
        for q in decide_queries
    ]
    print(f"Running {args.n} `atlas decide` invocations ...")
    decide_times = measure_calls(atlas, decide_args, args.n, warmup=args.warmup)
    decide_stats = stats(decide_times)
    print(
        f"  mean={decide_stats['mean_ms']:.2f}ms  "
        f"median={decide_stats['median_ms']:.2f}ms  "
        f"p95={decide_stats['p95_ms']:.2f}ms  "
        f"p99={decide_stats['p99_ms']:.2f}ms  "
        f"min={decide_stats['min_ms']:.2f}ms  max={decide_stats['max_ms']:.2f}ms"
    )

    # ---- Determinism check (same query 100x, byte-identical?) ----
    det_query = "error handling"
    det_args = ["solve", "--bundle", solve_bundle, det_query, "--json"]
    baseline = None
    all_same = True
    for i in range(100):
        rc, out, _ = run_capture(atlas, *det_args)
        if rc != 0 or not out:
            all_same = False
            break
        if baseline is None:
            baseline = out
        elif out != baseline:
            all_same = False
            break
    print(
        f"\nDeterminism: 100x '{det_query}' -> identical output: {'YES' if all_same else 'NO'}"
    )

    # ---- Assemble results ----
    rc, ver_out, _ = run_capture(atlas, "--version")
    version = ver_out.decode("utf-8", "replace").strip() if rc == 0 else "unknown"

    results = {
        "generated_at": time.strftime("%Y-%m-%dT%H:%M:%S%z"),
        "atlas_version": version,
        "measured_on_machine": hardware(),
        "sample_size": args.n,
        "warmup_uncounted": args.warmup,
        "timing_includes": [
            "OS process spawn",
            "Rust binary startup + std init",
            ".atlas file read + deserialize",
            "solve/decide work",
        ],
        "timing_caveat": (
            "End-to-end CLI latency per invocation (subprocess.run). "
            "NOT in-process library latency. Process-spawn overhead dominates "
            "and is intentionally included for honesty."
        ),
        "compile": {
            "solve_bundle_source": os.path.relpath(solve_src, repo),
            "solve_bundle_bytes": solve_bundle_bytes,
            "solve_compile_ms": round(compile_solve_ms, 3),
            "solve_compile_meta": compile_solve_meta,
            "decide_bundle_sources": [
                os.path.relpath(decide_md, repo),
                os.path.relpath(decide_yaml, repo),
            ],
            "decide_bundle_bytes": decide_bundle_bytes,
            "decide_compile_ms": round(compile_decide_ms, 3),
            "decide_compile_meta": compile_decide_meta,
        },
        "solve": {
            "command": "atlas solve --bundle bench_solve.atlas <query> --json",
            "query_set": solve_queries,
            "latency_ms": solve_stats,
        },
        "decide": {
            "command": "atlas decide --bundle bench_decide.atlas <query> -c answer=false --json",
            "query_set": decide_queries,
            "latency_ms": decide_stats,
            "note": (
                "Published flutter decision trees require a single consistent "
                "`answer` context along the chosen path to reach a terminal. "
                "answer=false reaches the StatelessWidget terminal reliably."
            ),
        },
        "determinism": {
            "query": det_query,
            "runs": 100,
            "identical_output": all_same,
            "method": "byte-for-byte comparison of --json stdout",
        },
    }

    with open(args.out, "w", encoding="utf-8") as f:
        json.dump(results, f, indent=2)
    print(f"\nWrote results -> {args.out}")

    # ---- Summary table ----
    print("\n" + "=" * 64)
    print("SUMMARY TABLE (measured on this machine)")
    print("=" * 64)
    print(f"{'Metric':<22}{'solve (ms)':>14}{'decide (ms)':>14}")
    for k in ["min_ms", "mean_ms", "median_ms", "p95_ms", "p99_ms", "max_ms"]:
        print(f"{k:<22}{solve_stats[k]:>14}{decide_stats[k]:>14}")
    print("-" * 50)
    print(f"solve bundle size : {solve_bundle_bytes} bytes")
    print(f"decide bundle size: {decide_bundle_bytes} bytes")
    print(f"compile solve     : {compile_solve_ms:.1f} ms")
    print(f"compile decide    : {compile_decide_ms:.1f} ms")
    print(f"deterministic     : {'YES' if all_same else 'NO'}")
    print("=" * 64)


if __name__ == "__main__":
    main()
