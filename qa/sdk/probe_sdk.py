"""
QA probe for the Atlas Python SDK (atlas_sdk).
Run from repo root:  python qa/sdk/probe_sdk.py
Imports the SDK by adding ../python to sys.path (no source edits).
"""
import os
import sys
import json
import tempfile
import threading

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO, "python"))

import atlas_sdk
from atlas_sdk.client import _BINARY, _find_binary, solve, decide, verify, reason, compile, install, _run

PASS = "PASS"
FAIL = "FAIL"
results = []

def check(name, cond, detail=""):
    results.append((PASS if cond else FAIL, name, detail))
    print(f"[{PASS if cond else FAIL}] {name}" + (f" -- {detail}" if detail else ""))

BUNDLE = os.path.join(REPO, "packages", "riverpod.atlas")

# 1. Public API
print("\n== 1. Import / public API ==")
expected_api = ["solve", "decide", "verify", "compile", "install", "load", "reason",
                "Node", "SolveResult", "DecideResult", "Recommendation",
                "VerificationCheck", "VerificationReport"]
missing = [n for n in expected_api if not hasattr(atlas_sdk, n)]
check("public API complete", not missing, f"missing={missing}")
check("binary located", bool(_BINARY), f"_BINARY={_BINARY}")

# 2. Normal usage + type correctness
print("\n== 2. Normal usage + types ==")
r = solve(BUNDLE, "AsyncNotifier")
check("solve returns SolveResult", isinstance(r, atlas_sdk.SolveResult))
check("confidence is float", isinstance(r.confidence, float), f"type={type(r.confidence).__name__} val={r.confidence}")
check("total_matches int", isinstance(r.total_matches, int))
check("nodes typed list[Node]", all(isinstance(n, atlas_sdk.Node) for n in r.nodes))
check("node.version str|None", all((n.version is None or isinstance(n.version, str)) for n in r.nodes),
      f"versions={[type(n.version).__name__ for n in r.nodes]}")
check("node fields match raw", r.nodes[0].name == "AsyncNotifier")

# Compare to raw CLI json
raw = _run("solve", "--bundle", BUNDLE, "AsyncNotifier")
rawj = json.loads(raw.splitlines()[-1])
check("no fields dropped vs raw",
      set(rawj.keys()) - {"nodes"} <= set(r.__dict__.keys()),
      f"raw={sorted(rawj.keys())} sdk={sorted(r.__dict__.keys())}")

vr = verify(BUNDLE)
check("verify returns VerificationReport", isinstance(vr, atlas_sdk.VerificationReport))
check("checks typed", all(isinstance(c, atlas_sdk.VerificationCheck) for c in vr.checks))
check("check.severity present", all(c.severity in ("error", "warn", "info") for c in vr.checks),
      f"severities={[c.severity for c in vr.checks]}")

# 3. Error handling
print("\n== 3. Error handling ==")
try:
    solve(os.path.join(REPO, "packages", "does_not_exist.atlas"), "x")
    check("missing bundle raises", False, "no exception")
except RuntimeError as e:
    check("missing bundle -> RuntimeError", True, str(e)[:80])
except Exception as e:
    check("missing bundle raises wrong type", False, f"{type(e).__name__}: {e}")

# empty query
try:
    solve(BUNDLE, "")
    check("empty query raises", False, "no exception")
except RuntimeError as e:
    check("empty query -> RuntimeError", True, str(e)[:80])
except Exception as e:
    check("empty query wrong type", False, f"{type(e).__name__}: {e}")

# 4. Argument injection: query starting with '-'
print("\n== 4. Argument-injection-style robustness ==")
for q in ["--version", "--help", "-h", "--bundle"]:
    try:
        out = solve(BUNDLE, q)
        check(f"query '{q}' safe", False, f"returned unexpectedly: {str(out)[:60]}")
    except RuntimeError as e:
        check(f"query '{q}' -> RuntimeError (CLI rejected)", True, str(e)[:70])
    except json.JSONDecodeError as e:
        # CLI printed help to stdout (exit 0) -> last line not JSON -> raw exception leaks
        check(f"query '{q}' -> JSONDecodeError LEAKS (not RuntimeError)", False, f"{type(e).__name__}: {e}")
    except Exception as e:
        check(f"query '{q}' -> unexpected {type(e).__name__}", False, f"{type(e).__name__}: {e}")

# 5. Unicode + very long query
print("\n== 5. Unicode / long query ==")
try:
    ru = solve(BUNDLE, "状態管理 🚀 日本語")
    check("unicode query handled", isinstance(ru, atlas_sdk.SolveResult))
except Exception as e:
    check("unicode query handled", False, f"{type(e).__name__}: {e}")

longq = "x" * 100000
try:
    rl = solve(BUNDLE, longq)
    check("very long query handled", isinstance(rl, atlas_sdk.SolveResult))
except Exception as e:
    check("very long query handled", False, f"{type(e).__name__}: {e}")

# 6. Invalid JSON from CLI (simulate with a fake binary)
print("\n== 6. Invalid JSON handling ==")
fake = os.path.join(tempfile.gettempdir(), "fake_atlas.py")
with open(fake, "w") as f:
    f.write("import sys\nprint('some non-json banner line')\nprint('also not json')\nsys.exit(0)\n")
# point SDK at the fake python interpreter via BINARY override
import subprocess
orig = _BINARY if _BINARY else _find_binary()
atlas_sdk.client._BINARY = sys.executable
atlas_sdk.client._run.__globals__  # noop
# Monkeypatch _run to use fake script: easiest is to wrap binary call.
# We patch the subprocess.run used by _run via a small shim module.
import types
shim = types.ModuleType("atlas_sdk._shim")
import atlas_sdk.client as _cl
_orig_run = subprocess.run
def _fake_run(cmd, **kw):
    # Replace the atlas binary with our fake script
    newcmd = [sys.executable, fake] + cmd[1:]
    return _orig_run(newcmd, **kw)
subprocess.run = _fake_run
try:
    solve(BUNDLE, "x")
    check("invalid json -> handled", False, "no exception")
except RuntimeError as e:
    check("invalid json -> RuntimeError", True, str(e)[:80])
except json.JSONDecodeError as e:
    check("invalid json -> JSONDecodeError LEAKS (not RuntimeError)", False, f"{type(e).__name__}")
except Exception as e:
    check("invalid json -> unexpected", False, f"{type(e).__name__}: {e}")
finally:
    subprocess.run = _orig_run
    atlas_sdk.client._BINARY = orig

# 7. Timeout not propagated to decide/verify/reason/compile
print("\n== 7. Timeout propagation ==")
import inspect
sig = inspect.signature(decide)
check("decide accepts timeout param", "timeout" in sig.parameters,
      f"decide params={list(sig.parameters)}")
sig2 = inspect.signature(verify)
check("verify accepts timeout param", "timeout" in sig2.parameters,
      f"verify params={list(sig2.parameters)}")

# 8. Thread safety
print("\n== 8. Thread safety ==")
errs = []
def worker():
    try:
        solve(BUNDLE, "AsyncNotifier")
    except Exception as e:
        errs.append(e)
ths = [threading.Thread(target=worker) for _ in range(10)]
[t.start() for t in ths]; [t.join() for t in ths]
check("concurrent calls no error", not errs, f"errs={[str(e) for e in errs][:2]}")

# 9. CLI not on PATH / binary missing
print("\n== 9. Binary missing ==")
atlas_sdk.client._BINARY = None
real_which = atlas_sdk.client.shutil.which
atlas_sdk.client.shutil.which = lambda c: None
real_exists = atlas_sdk.client.Path.exists
atlas_sdk.client.Path.exists = lambda self: False
try:
    _find_binary()
    check("missing binary raises", False)
except RuntimeError as e:
    check("missing binary -> RuntimeError", True, str(e)[:60])
finally:
    atlas_sdk.client.shutil.which = real_which
    atlas_sdk.client.Path.exists = real_exists
    atlas_sdk.client._BINARY = orig

# Summary
fails = [r for r in results if r[0] == FAIL]
print(f"\n=== SDK probe summary: {len(results)-len(fails)} pass / {len(fails)} fail ===")
sys.exit(1 if fails else 0)
