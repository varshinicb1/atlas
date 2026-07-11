# QA Findings — Atlas Python SDK & MCP Server

**Date:** 2026-07-11
**Environment:** Windows 11, PowerShell 5.1, Python 3.12.4, Node v22.19.0, `atlas.exe` (prebuilt, `target/release/atlas.exe`).
**Method:** Real binary + real compiled bundles (`packages/*.atlas`) + throwaway probes under `qa/sdk/` (`probe_sdk.py`, `probe_sdk2.py`, `probe_mcp.mjs`). Product source was not modified.

**Headline:** No P0 crashes, and the **MCP server is robust** (every error path becomes a proper `isError` response — it never crashes). The **Python SDK is the weaker component**: it lets raw Python/OS exceptions leak to the caller instead of raising meaningful `RuntimeError`s, and it passes user input to the CLI as positional args without guarding against `-`-prefixed (flag-like) queries or oversized input.

---

## Severity summary

| ID | Component | Severity | Title |
|----|-----------|----------|-------|
| SDK-01 | Python SDK | **P1** | `-`-prefixed query (`--help`, `-h`) leaks raw `json.JSONDecodeError` |
| SDK-02 | Python SDK | **P1** | Oversized query leaks `FileNotFoundError [WinError 206]` |
| SDK-03 | Python SDK | **P2** | Non-JSON CLI stdout leaks raw `json.JSONDecodeError` (no `JSONDecodeError` handling) |
| SDK-04 | Python SDK | **P2** | Empty/blank query silently returns ALL nodes (wrong data, no error) |
| SDK-05 | Python SDK | **P2** | `subprocess.run(text=True)` without `encoding=` → locale-dependent decode (CJK/emoji crash on cp1252) |
| SDK-06 | Python SDK | **P3** | `timeout` param only on `solve`, not `decide/verify/reason/compile/install` |
| SDK-07 | Python SDK | **P3** | `requires-python=">=3.9"` but code uses `X | Y` union hints (needs 3.10+) |
| SDK-08 | Python SDK | **P3** | `pyproject.toml` `readme="README.md"` points to a missing file |
| SDK-09 | Python SDK | **P3** | `packages.find where=["."]` would also ship the `tests` package |
| MCP-01 | MCP server | **P2** | `-`-prefixed query → `isError` with raw JSON-parse message (leaks internals) |
| MCP-02 | MCP server | **P2** | `bundle` resource URI template `{path}` drops subdirectories → nested/absolute paths fail |
| MCP-03 | MCP server | **P2** | Empty query in `atlas_solve` returns misleading SUCCESS (all nodes) |
| MCP-04 | MCP server | **P3** | `context_json` not validated as object → malformed `-c` args for string/array/number |
| MCP-05 | MCP server | **P3** | `resolvePath` resolves relative to server cwd; breaks if server launched elsewhere |

**Counts:** 0 × P0, 2 × P1, 6 × P2, 6 × P3 → **14 findings (9 SDK, 5 MCP)**.
Note: `pip install -e python` **succeeded** (setuptools tolerates the missing README with a warning), so packaging is not hard-broken — SDK-08 is a fragility nit, not a blocker. All 24 existing unit tests pass (`npm test`: 24 passed; `python -m pytest`: unit tests pass).

---

## Python SDK

### SDK-01 — P1 — `-`-prefixed query leaks raw `json.JSONDecodeError`
- **Repro:**
  ```python
  import atlas_sdk
  atlas_sdk.solve("packages/riverpod.atlas", "--help")   # or "-h"
  ```
  The CLI treats `--help`/`-h` as a flag, prints help text to **stdout** (exit 0). `_run_single` returns the last line, then `json.loads` raises.
- **Expected:** A meaningful `RuntimeError` (e.g. "query looks like a flag; ...") or the query passed through literally.
- **Actual:** `json.JSONDecodeError: Expecting value: line 1 column 3 (char 2)` — raw, uncaught, leaks internal detail.
- **Why it matters:** An enterprise dev searching for a term that starts with `-` (CLI flag names are common knowledge-graph queries) gets a stack-trace-grade exception. Same root cause as MCP-01.
- **Fix:** In `_run`/`_run_single`, separate positional args with `--` (`cmd = [_BINARY, "--json", *args, "--", query]` — note `query` is currently appended already; add `--` before it), and wrap `json.loads` in `try/except json.JSONDecodeError` → `RuntimeError("atlas CLI returned invalid JSON: ...")`.

### SDK-02 — P1 — Oversized query leaks `FileNotFoundError [WinError 206]`
- **Repro:** `atlas_sdk.solve("packages/riverpod.atlas", "x" * 200000)`
- **Expected:** Bounded error (`RuntimeError` / clear "query too long") or the query passed via stdin.
- **Actual:** `FileNotFoundError: [WinError 206] The filename or extension is too long` — a confusing OS error leaks straight through.
- **Fix:** Validate input length (and/or send the query via stdin with `--query -`); convert `subprocess`/`OSError` into `RuntimeError`.

### SDK-03 — P2 — Non-JSON CLI stdout leaks raw `json.JSONDecodeError`
- **Repro (simulate a CLI that prints a banner then exits 0):**
  ```python
  # Point _BINARY at a script that prints non-JSON and exits 0
  atlas_sdk.client._BINARY = sys.executable
  # monkeypatch subprocess.run to call: [sys.executable, fake_script]
  atlas_sdk.solve("packages/riverpod.atlas", "x")
  ```
- **Expected:** `RuntimeError` ("invalid JSON from atlas CLI").
- **Actual:** `json.JSONDecodeError` (only non-zero exits are caught today).
- **Fix:** Catch `json.JSONDecodeError` in `_run`/`_run_single` and re-raise as `RuntimeError`.

### SDK-04 — P2 — Empty query silently returns ALL nodes
- **Repro:** `r = atlas_sdk.solve("packages/riverpod.atlas", "")` → `r.total_matches == 14`, `len(r.nodes) == 14`, no exception.
- **Expected:** `RuntimeError("query must not be empty")` (the bare CLI rejects empty query with a usage error, but the SDK passes `""` as a positional string which the CLI accepts as "match everything").
- **Actual:** A successful-looking result containing every node — a silent data-correctness bug that an LLM/agent would treat as a confident match.
- **Fix:** Reject empty/whitespace queries in `solve`/`decide`/`reason` before calling the CLI. (Mirrors MCP-03.)

### SDK-05 — P2 — `subprocess.run(text=True)` without explicit encoding
- **Repro / reasoning:** `client.py:_run` calls `subprocess.run(cmd, capture_output=True, text=True, ...)` with no `encoding=`. On this box the locale decoded UTF-8 fine, but `locale.getpreferredencoding()` reports `cp1252`. A bundle/node containing CJK or emoji (extremely common in enterprise knowledge packs with code/comments/names) would raise `UnicodeDecodeError` on a default-Windows (cp1252) machine. Confirmed the decode path is unpinned by code inspection; the SDK solved a Unicode bundle here only because this environment happens to decode UTF-8.
- **Expected:** Deterministic UTF-8 decoding regardless of machine locale.
- **Actual:** Locale-dependent; crashes on cp1252 + non-Latin1 content.
- **Fix:** `subprocess.run(cmd, capture_output=True, text=True, encoding="utf-8", errors="replace", timeout=timeout)`.

### SDK-06 — P3 — `timeout` not exposed on most functions
- **Repro:** `inspect.signature(atlas_sdk.decide)` → `('bundle', 'query', 'context')`; same for `verify`/`reason`/`compile`/`install`. Only `solve` has `timeout`.
- **Impact:** Callers cannot bound long-running `decide`/`verify`/`reason`/`compile`; they inherit the fixed 30 s default.
- **Fix:** Add `timeout: int = 30` to the other functions and forward to `_run_single`/`_run`.

### SDK-07 — P3 — `requires-python` too low for union syntax
- **Repro / reasoning:** `pyproject.toml` declares `requires-python = ">=3.9"`, but `client.py` uses runtime-evaluated union hints like `sources: list[str | Path]` and `context: Optional[dict[str, str]]`. The `X | Y` form is only valid at runtime on **3.10+**; on 3.9 it raises `TypeError: unsupported operand type(s) for |: 'type' and 'type'` at import time.
- **Fix:** Bump `requires-python = ">=3.10"`, or add `from __future__ import annotations` at the top of `client.py` (and `models.py`).

### SDK-08 — P3 — dangling `readme` reference
- **Repro:** `python -m pip install -e python` **succeeds** but emits a warning; `python/README.md` does not exist while `pyproject.toml` has `readme = "README.md"`.
- **Impact:** Fragile — stricter build backends / `python -m build` can fail; PyPI long-description is empty.
- **Fix:** Add `python/README.md` or remove the `readme` key.

### SDK-09 — P3 — `tests` package gets shipped
- **Repro:** `[tool.setuptools.packages.find] where = ["."]` will discover `atlas_sdk` **and** `tests` (which has `__init__.py`).
- **Fix:** `where = ["."]`, `exclude = ["tests", "tests.*"]` (or put `tests` outside the package root).

---

## MCP server

> Robustness is good: in every error scenario the server returned an `isError: true` tool response (or an error resource) — it never crashed. Findings below are about correctness/UX, not crashes.

### MCP-01 — P2 — `-`-prefixed query leaks raw JSON-parse message
- **Repro:** `atlas_solve` with `query: "--help"` →
  `isError: true, text: "Error: No number after minus sign in JSON at position 3 ..."`
- **Expected:** A clean, user-facing error; the query should not be interpreted as a CLI flag.
- **Actual:** Works (no crash) but the message exposes JSON internals — same root cause as SDK-01 (`atlas.ts` doesn't pass `--` before the positional query, and `JSON.parse` errors aren't wrapped).
- **Fix:** In `atlas.ts`, append `--` before the query arg (e.g. `runCLI("solve", "--bundle", resolved, "--", query)`); wrap `JSON.parse` failures in a friendly error.

### MCP-02 — P2 — `bundle` resource URI template drops subdirectories
- **Repro:** The resource is registered as `atlas://bundle/{path}`. MCP URI-template variables stop at the first `/`, so a request for `atlas://bundle/packages/riverpod.atlas` extracts `path = "packages"` (not the full path). Probe confirmed: passing the full path in `variables.path` works (`application/json`); passing only `packages` fails (`text/plain` error: "Access is denied"/not found).
- **Expected:** Absolute or nested relative bundle paths should resolve.
- **Actual:** Only single-segment paths work; any real bundle under a directory fails.
- **Fix:** Use a wildcard segment: `atlas://bundle/{path+}`.

### MCP-03 — P2 — Empty query returns misleading SUCCESS
- **Repro:** `atlas_solve` with `query: ""` → success response `Query: \nConfidence: 0.95\nFound 14 nodes:` (all nodes).
- **Expected:** An error (`isError: true`) for empty query.
- **Actual:** A confident-looking "match everything" result — silent wrong data for the calling agent (same root cause as SDK-04).
- **Fix:** Validate non-empty query in the `atlas_solve`/`atlas_decide`/`atlas_reason` handlers before calling the CLI.

### MCP-04 — P3 — `context_json` not validated as object
- **Repro:** `atlas_decide` with `context_json: '"hello"'` (a JSON string) → `JSON.parse` succeeds, then `Object.entries("hello")` yields per-character pairs → malformed `-c 0=h -c 1=e ...` args to the CLI (no crash, but wrong semantics). Arrays/numbers behave similarly.
- **Expected:** Reject non-object `context_json` with a clear error.
- **Fix:** After `JSON.parse`, assert `ctx && typeof ctx === "object" && !Array.isArray(ctx)`; otherwise return `isError`.

### MCP-05 — P3 — Bundle path resolved relative to server cwd
- **Repro reasoning:** `atlas.ts:resolvePath` resolves a relative `bundle` against `process.cwd()` (and its parent). If the MCP server is launched from a different directory than the client expects (typical in enterprise deployments), relative bundle paths supplied by the LLM won't resolve; absolute paths work. Every failure is caught as `isError`, so severity is low.
- **Fix:** Document that `bundle` must be an absolute path (or resolve relative to a configured `ATLAS_BUNDLE_ROOT`), and validate existence before invoking the CLI for a clearer error.

---

## What was verified to be CORRECT (no defect)
- Public API surface (`atlas_sdk.solve/decide/verify/compile/install/load/reason` + dataclasses) imports and is complete.
- `SolveResult.confidence` is a `float`; `nodes` typed as `list[Node]`; `VerificationReport.checks` typed; field set matches raw `--json` (no fields dropped).
- No **shell injection**: both SDK (`subprocess.run(list)`) and MCP (`execFileSync(array)`) use argument lists, never `shell=True` — a malicious query cannot inject shell commands. (The real risk is *flag* injection — SDK-01/MCP-01 — not shell injection.)
- Concurrent `solve()` calls are safe (no errors across 10–20 threads).
- `pip install -e python` works; `npm install` + `npm run build` + `npm test` (24 tests) all pass.
- MCP handlers never crash on missing bundle, bad query, unicode, oversized input, or invalid `context_json` — all become `isError`.
- `atlas_navigate` correctly reports missing nodes and lists neighbors for existing nodes.
- `decide` returns `None` when no decision tree matches (correct per design) — note: **no published bundle in this repo contains a decision tree**, so the `DecideResult`/`Recommendation` population path is currently unexercised by real data (test-coverage gap, not a defect).
