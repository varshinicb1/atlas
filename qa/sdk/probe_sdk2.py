"""Targeted SDK confirmations: encoding, empty/help query, long query, decide unicode ctx."""
import os, sys, json, tempfile, subprocess
REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO, "python"))
import atlas_sdk
from atlas_sdk.client import _run, _BINARY, _find_binary

BUNDLE = os.path.join(REPO, "packages", "riverpod.atlas")

# A) Does the REAL CLI ever emit non-ASCII on stdout? (proves encoding relevance)
raw_bytes = subprocess.run([_find_binary(), "--json", "solve", "--bundle", BUNDLE, "AsyncNotifier"],
                           capture_output=True).stdout
nonascii = [b for b in raw_bytes if b > 127]
print(f"[A] real CLI solve stdout non-ascii bytes: {len(nonascii)}  sample={nonascii[:5]}")

# B) Demonstrate the SDK decode path is unsafe for UTF-8 on Windows (cp1252 default).
fake = os.path.join(tempfile.gettempdir(), "fake_atlas_u.py")
with open(fake, "w", encoding="utf-8") as f:
    f.write("import sys\nsys.stdout.buffer.write('{\"answer\":\"café ☕ 日本語\",\"query\":\"q\"}'.encode('utf-8'))\nsys.exit(0)\n")
orig = atlas_sdk.client._BINARY
atlas_sdk.client._BINARY = sys.executable
_orig = subprocess.run
def _fake(cmd, **kw):
    return _orig([sys.executable, fake] + cmd[1:], **kw)
subprocess.run = _fake
try:
    out = _run("reason", "--bundle", BUNDLE, "q")  # returns raw last line
    print(f"[B] _run with UTF-8 output -> OK: {out!r}")
except UnicodeDecodeError as e:
    print(f"[B] _run with UTF-8 output -> UnicodeDecodeError LEAKS: {e}")
except Exception as e:
    print(f"[B] _run with UTF-8 output -> {type(e).__name__}: {e}")
finally:
    subprocess.run = _orig
    atlas_sdk.client._BINARY = orig

# C) Empty query: what does the SDK return?
r = atlas_sdk.solve(BUNDLE, "")
print(f"[C] empty query -> query={r.query!r} total_matches={r.total_matches} nodes={len(r.nodes)} (silent, no error)")

# D) Help query -> exception type
for q in ["--help", "-h"]:
    try:
        atlas_sdk.solve(BUNDLE, q)
        print(f"[D] query {q!r} -> returned (no error)")
    except json.JSONDecodeError as e:
        print(f"[D] query {q!r} -> JSONDecodeError (raw, uncaught): {e}")
    except RuntimeError as e:
        print(f"[D] query {q!r} -> RuntimeError: {str(e)[:50]}")
    except Exception as e:
        print(f"[D] query {q!r} -> {type(e).__name__}: {e}")

# E) Long query -> exception type
try:
    atlas_sdk.solve(BUNDLE, "x" * 200000)
    print("[E] long query -> returned")
except Exception as e:
    print(f"[E] long query -> {type(e).__name__}: {e}")

# F) decide with unicode context value
try:
    res = atlas_sdk.decide(BUNDLE, "AsyncNotifier", {"complexity": "moyen é"})
    print(f"[F] decide unicode ctx -> {type(res).__name__}")
except Exception as e:
    print(f"[F] decide unicode ctx -> {type(e).__name__}: {e}")
