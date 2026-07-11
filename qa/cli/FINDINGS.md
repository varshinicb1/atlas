# Atlas CLI — QA / Red-Team FINDINGS

**Target:** `target\release\atlas.exe` (prebuilt, NOT rebuilt)
**Scope:** malformed inputs, CLI arg edge cases, compile/solve/decide/verify/init/install/search/publish, determinism, robustness
**Harness:** throwaway probe scripts + fixtures under `qa\cli\` (no product source touched, no `cargo`)

## Summary

| Severity | Count | IDs |
|----------|-------|-----|
| **P0** (crash / security / data-loss) | 2 | CLI-01, CLI-02 |
| **P1** (broken core behavior / integration) | 4 | CLI-03, CLI-04, CLI-05, CLI-06 |
| **P2** (bad UX / correctness) | 6 | CLI-07, CLI-08, CLI-09, CLI-10, CLI-11, CLI-12 |
| **P3** (nitpick) | 2 | CLI-13, CLI-14 |
| **Total** | **14** | |

> Environment note: a real P0 panic was found that leaks a Rust source path + line (`atlas-runtime/src/loader.rs:15:28`) to the user. The `--json` output on success is valid JSON but is encoded as **UTF-16 LE** (not UTF-8), which breaks `jq`/Python `json`/Node `JSON.parse`. On all error paths `--json` emits **empty stdout**, so the "always-valid-JSON" enterprise requirement is violated.

---

## CLI-01 — P0 — Panic / crash on corrupt or truncated `.atlas` bundle
- **Component:** `solve`, `decide`, `reason`, `verify`, `dump` (all bundle-loading commands)
- **Repro:**
  ```powershell
  cd qa\cli
  # build a valid bundle, then truncate it
  ..\..\target\release/atlas.exe compile atlas.md --out baseline.atlas
  $b=[System.IO.File]::ReadAllBytes("baseline.atlas")
  [System.IO.File]::WriteAllBytes("trunc.atlas",[byte[]]$b[0..([math]::Floor($b.Length/2))])
  ..\..\target\release/atlas.exe solve x --bundle trunc.atlas --json
  ```
  (same repro with `decide`/`reason`/`verify`/`dump` instead of `solve` — all crash)
- **Expected:** clean, non-zero exit with a structured error like `{"error":"corrupt or truncated bundle"}`.
- **Actual:** process aborts with **exit code 101** and a raw Rust backtrace leaked to stderr:
  ```
  thread 'main' panicked at atlas-runtime/src/loader.rs:15:28:
  range start index 5353 out of range for slice of length 2984
  ```
  Observed for `solve` (B02), `decide`, `reason` (B08), `verify` (C01), `dump` (Z06).
- **Why it matters:** a partially-downloaded / tampered / disk-truncated `.atlas` (a common real-world case) crashes the CLI and dumps internal source paths — unacceptable for an enterprise "deterministic/verifiable" product.
- **Suggested fix:** in `loader.rs` validate the header magic, section table and length fields *before* slicing; return `anyhow::Error` (clean message + non-zero exit) instead of indexing out of bounds.

## CLI-02 — P0 — Path traversal / arbitrary write outside CWD in `init`
- **Component:** `init`
- **Repro:**
  ```powershell
  cd qa\cli
  ..\..\target\release/atlas.exe init ../evilpkg --json
  ```
- **Expected:** reject the name (it is not a package name) or create `./evilpkg` inside CWD; never write outside the working directory.
- **Actual:** `init` happily created the scaffolded package **outside** the current directory — verified at `C:\Users\varsh\OneDrive\Documents\3Atlas\qa\evilpkg\` (containing `evilpkg.yaml`, `decisions\`, `.gitignore`). Also reproducible via `-d ..\out_evil`.
- **Why it matters:** a user/automation passing `../<name>` (or an absolute path) causes `atlas` to write files to an arbitrary location — a security/write-integrity defect.
- **Suggested fix:** treat `NAME` strictly as a single path segment; strip/reject `..`, separators, and absolute paths; only ever write under the resolved CWD (or the explicit, validated `-d`).

## CLI-03 — P1 — `--json` errors emit EMPTY stdout (no JSON envelope)
- **Component:** all commands, every error path
- **Repro:** `..\..\target\release/atlas.exe compile empty.md --out x.atlas --json` → exit **1**, stdout is **0 bytes**. Same for missing args (`solve --json` → exit 2, empty stdout), missing bundle, corrupt inputs, publish/install failures, etc. (verified across A01–A13, B01, B02, C01, E05, E06…).
- **Expected:** `{"error":"<message>","code":<n>}` on stdout so pipelines can `ConvertFrom-Json` / `JSON.parse` uniformly.
- **Actual:** nothing on stdout; the human `Error: ...` text goes to stderr. Enterprise automation that does `out = run(...); json.loads(out)` gets empty input and fails opaquely.
- **Suggested fix:** in `--json` mode, serialize a typed error object to **stdout** on every failure path (keep non-zero exit code).

## CLI-04 — P1 — `init --json` ignores `--json`, prints human text
- **Component:** `init`
- **Repro:** `..\..\target\release/atlas.exe init mypkg --json`
- **Expected:** `{"created":true,"path":"mypkg",...}` JSON on stdout.
- **Actual:** stdout contains human prose, e.g.
  ```
  Created knowledge package 'mypkg' in ./mypkg
    mypkg/mypkg.md   - knowledge document (edit this!)
    ...
  Next steps: ...
  ```
  (confirmed for every template variant D01–D12 and unknown template D11).
- **Suggested fix:** branch on `--json` in `init` and emit a structured object.

## CLI-05 — P1 — `--json` output is UTF-16 LE, not UTF-8
- **Component:** all commands (success output)
- **Repro:**
  ```powershell
  ..\..\target\release/atlas.exe solve "binary format" --bundle baseline.atlas --json > out.json
  python -c "print(open('out.json','rb').read(4).hex())"   # -> fffe7b00
  ```
  The BOM `ff fe` proves UTF-16 LE. `python -c "import json,sys; json.load(sys.stdin)"` and `jq` both fail/refuse.
- **Expected:** UTF-8 JSON (RFC 8259 recommends UTF-8; every standard parser/pipe expects it).
- **Actual:** valid JSON, but UTF-16-encoded with BOM. Breaks `jq`, `python json`, `node JSON.parse`, most SDK integration.
- **Suggested fix:** write the JSON as UTF-8 without BOM (e.g. `serde_json::to_string` + `fs::write` as bytes).

## CLI-06 — P1 — `decide` returns `null` for every input (core command non-functional)
- **Component:** `decide`
- **Repro:**
  ```powershell
  ..\..\target\release/atlas.exe compile atlas.md ..\..\packages\decisions\cloudflare_workers.yaml --out cftree.atlas
  ..\..\target\release/atlas.exe decide "store config flags at the edge" --bundle cftree.atlas --json   # -> null
  ..\..\target\release/atlas.exe decide x --bundle cftree.atlas --json -c intent=choose_solution -c domain=edge  # -> null
  ```
  Tried 4 natural phrasings + explicit context; all return JSON `null` (exit 0). The bundle genuinely contains 2 decision trees (confirmed via `dump`): one with `trigger {intent: choose_solution, domain: edge, tags: [cloudflare,storage,workers]}` and `root: data_shape_node`.
- **Expected:** a decision result with `path`/`rationale`/`recommendations`, or at minimum a structured "no match" object.
- **Actual:** always `null`, silently, with exit 0 — a "silent wrong answer" with no signal that nothing matched.
- **Suggested fix:** debug the trigger-matching + tree walker; when no tree matches, return `{"matched":false,...}` instead of bare `null`.

## CLI-07 — P2 — `solve` output internally inconsistent (`total_matches:0` but 5 nodes)
- **Component:** `solve`
- **Repro:** `..\..\target\release/atlas.exe solve "binary format" --bundle baseline.atlas --json`
- **Expected:** `total_matches` equals `len(nodes)` (or the field's meaning must be documented).
- **Actual:** `{"query":"binary format","bundle":"atlas","confidence":0.7,"total_matches":0,"nodes":[ ... 5 entries ... ]}` — contradictory.
- **Why it matters:** consumers that count results from `total_matches` will under-count.
- **Suggested fix:** set `total_matches = nodes.len()` (or rename/document the field).

## CLI-08 — P2 — `compile` with no sources "succeeds" and writes an empty bundle
- **Component:** `compile`
- **Repro:** `..\..\target\release/atlas.exe compile --json` → exit **0**, stdout `{"decision_trees":0,"edges":0,"nodes":0,"output":"bundle.atlas"}` and a 0-node `.atlas` is created.
- **Expected:** error (exit non-zero) — compiling nothing is not a valid operation.
- **Actual:** silent success producing an empty bundle.
- **Suggested fix:** require ≥1 SOURCE; error otherwise.

## CLI-09 — P2 — `search --json` output format is inconsistent (object vs NDJSON)
- **Component:** `search`
- **Repro (registry reachable):** `..\..\target\release/atlas.exe search x --json` → single JSON object `{"results":[],"total":0}`.
- **Repro (registry unreachable):** `..\..\target\release/atlas.exe search x --registry http://localhost:9999 --json` → **NDJSON / multiple concatenated objects**:
  ```
  {"error":"Registry not reachable","registry":"http://localhost:9999","detail":"io: Connection refused"}
  {"name":"g","source":"local"}
  {"name":"qa_baseline","source":"local"}
  ...
  ```
- **Expected:** always a single, parseable JSON document (e.g. `{"results":[...],"error":null|"..."}`).
- **Actual:** single object when reachable, NDJSON-with-inline-error when not — breaks standard single-document JSON parsers and mixes error + result objects.
- **Suggested fix:** always emit one JSON object; include the error/message inside it rather than interleaving.

## CLI-10 — P2 — Unknown `--policy` / unknown `-t` template silently accepted or mis-reported
- **Component:** `verify`, `init`
- **Repro:**
  - `..\..\target\release/atlas.exe verify --bundle baseline.atlas --policy totally_unknown_policy_xyz --json` → exit **0**, `{"passed":false,"checks":[{"name":"Unknown policy","passed":false,"message":"Unknown compliance policy: ... Supported: eu-ai-act, soc2, hipaa",...}]}`.
  - `..\..\target\release/atlas.exe init x -t does_not_exist --json` → exit **0**, scaffolds a **generic** package (unknown template silently falls back) with no warning.
- **Expected:** unknown policy/template → non-zero exit + clear error; do not silently fall back.
- **Actual:** unknown policy returns exit 0 with the failure buried in a check; unknown template silently becomes generic.
- **Suggested fix:** validate `--policy`/`--template` against the known set; reject with non-zero exit and explicit message.

## CLI-11 — P2 — `compile --out` does not create parent directories
- **Component:** `compile`
- **Repro:** `..\..\target\release/atlas.exe compile atlas.md --out nope/sub/out.atlas --json`
- **Expected:** create `nope/sub/` and write the file (like `mkdir -p`).
- **Actual:** fails with opaque OS error `Error: The system cannot find the path specified. (os error 3)` (exit 1).
- **Suggested fix:** `create_dir_all` for the parent of `--out` before writing.

## CLI-12 — P2 — `decide -c` accepts malformed context (empty key, duplicate keys) silently
- **Component:** `decide`
- **Repro:**
  - `..\..\target\release/atlas.exe decide x --bundle baseline.atlas -c =value --json` → exit **0** (empty key accepted).
  - `..\..\target\release/atlas.exe decide x --c k1=v1 -c k1=v2 --json` → exit **0** (duplicate keys accepted, silently last-wins).
- **Expected:** reject empty key (`=value`) with a clear error; define duplicate-key behavior.
- **Actual:** both silently accepted (exit 0). (Note: `-c novalue` with no `=` *is* correctly rejected — good.)
- **Suggested fix:** validate `key=value` context tokens; error on empty key; document duplicate handling.

## CLI-13 — P3 — `compile` with the same source listed twice errors confusingly
- **Component:** `compile`
- **Repro:** `..\..\target\release/atlas.exe compile atlas.md atlas.md --out dup.atlas --json`
- **Expected:** either dedupe identical sources, or a clear "duplicate source" message.
- **Actual:** `Error: Node with id 'package:atlas' already exists` (the dedup is detected at node-merge time, not at source level) — misleading for a user error of listing a file twice.
- **Suggested fix:** normalize/dedupe SOURCES before compiling; or detect duplicate paths and warn.

## CLI-14 — P3 — Error text uses `Error:` prefix on stderr, inconsistent across commands
- **Component:** all (ergonomics)
- **Repro:** many of the above (e.g. `compile empty.md` → `Error: No YAML frontmatter found in empty.md`).
- **Expected:** consistent, machine-friendly error shape, especially in `--json` mode (see CLI-03).
- **Actual:** human `Error: ...` strings on stderr, no structured form. Low severity but compounds the integration pain for CLI-03.
- **Suggested fix:** unify error rendering; in `--json` mode always emit the structured object from CLI-03.

---

## Positives (no defect found)
- Valid `compile`/`solve`/`decide`/`reason`/`verify`/`dump` on a good bundle produce **deterministic** output (solve ×10 identical, decide ×5 identical — `G01`/`G02`), and re-compiling the same source is byte-identical (`F09` SAME).
- Long queries (5000 chars), Unicode/space paths, special-char searches, and unknown CLI flags are handled gracefully (clap usage errors, exit 2).
- `reason` with an unsupported `--model` errors cleanly (exit 1) rather than panicking.
- `install`/`search` gracefully fall back to local bundles when the registry is unreachable (functional, though the JSON shape issue in CLI-09 remains).
