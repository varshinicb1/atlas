"""
Atlas Python SDK — wraps the `atlas` CLI binary via subprocess.

Usage:
    import atlas_sdk as atlas
    result = atlas.solve("riverpod", "AsyncNotifier")
    for node in result.nodes:
        print(node.name, node.kind)
"""

import json
import os
import platform
import subprocess
import sys
import shutil
from pathlib import Path
from typing import Optional

from .models import (
    Node,
    SolveResult,
    DecideResult,
    Recommendation,
    VerificationCheck,
    VerificationReport,
)


def _find_binary() -> str:
    """Locate the `atlas` CLI binary."""
    if platform.system() == "Windows":
        candidates = ["atlas.exe", "atlas-cli.exe"]
    else:
        candidates = ["atlas", "atlas-cli"]

    # 1. Check PATH
    for c in candidates:
        found = shutil.which(c)
        if found:
            return found

    # 2. Check common install locations
    home = Path.home()
    common_dirs = [
        home / ".atlas" / "bin",
        Path("/usr/local/bin"),
        Path("/usr/bin"),
    ]
    if platform.system() == "Windows" and os.environ.get("LOCALAPPDATA"):
        common_dirs.append(Path(os.environ["LOCALAPPDATA"]) / "atlas" / "bin")
    for d in common_dirs:
        for c in candidates:
            p = d / c
            if p.exists():
                return str(p.resolve())

    # 3. Check relative to package install location
    pkg_dir = Path(__file__).resolve().parent
    for c in candidates:
        p = pkg_dir / c
        if p.exists():
            return str(p.resolve())

    # 4. Check relative to repo (development layout)
    workspace = Path(__file__).resolve().parent.parent.parent
    for sub in ["target/release", "target/debug"]:
        for c in candidates:
            p = workspace / sub / c
            if p.exists():
                return str(p.resolve())

    raise RuntimeError(
        "Could not find the `atlas` CLI binary. "
        "Install it with: cargo install --path atlas-cli"
    )


_BINARY: Optional[str] = None


def _run(*args: str, timeout: int = 30) -> str:
    global _BINARY
    if _BINARY is None:
        _BINARY = _find_binary()
    cmd = [_BINARY, "--json", *args]
    result = subprocess.run(
        cmd,
        capture_output=True,
        text=True,
        timeout=timeout,
    )
    if result.returncode != 0:
        err = result.stderr.strip()
        raise RuntimeError(f"atlas CLI error: {err}")
    return result.stdout.strip()


def _run_single(*args: str, timeout: int = 30) -> str:
    """Run and return the last JSON line (for single-result commands)."""
    out = _run(*args, timeout=timeout)
    if not out:
        return out
    return out.splitlines()[-1]


def compile(sources: list[str | Path], out: str | Path = "bundle.atlas") -> dict:
    """Compile Markdown/YAML sources into an .atlas binary."""
    parts = ["compile"]
    for s in sources:
        parts.append(str(s))
    parts.extend(["--out", str(out)])
    raw = _run_single(*parts)
    return json.loads(raw)


def solve(
    bundle: str | Path,
    query: str,
    timeout: int = 30,
) -> SolveResult:
    """Search the knowledge graph for nodes matching `query`."""
    raw = _run_single("solve", "--bundle", str(bundle), query, timeout=timeout)
    data = json.loads(raw)
    nodes = [Node(**n) for n in data.pop("nodes", [])]
    return SolveResult(**data, nodes=nodes)


def decide(
    bundle: str | Path,
    query: str,
    context: Optional[dict[str, str]] = None,
) -> Optional[DecideResult]:
    """Walk a decision tree to get a recommendation (LLM-free)."""
    parts = ["decide", "--bundle", str(bundle), query]
    if context:
        for k, v in context.items():
            parts.extend(["-c", f"{k}={v}"])
    raw = _run_single(*parts)
    if raw == "null":
        return None
    data = json.loads(raw)
    recs = [Recommendation(**r) for r in data.pop("recommendations", [])]
    return DecideResult(**data, recommendations=recs)


def verify(
    bundle: str | Path,
    artifact: Optional[str] = None,
) -> VerificationReport:
    """Run verification checks against the knowledge graph."""
    parts = ["verify", "--bundle", str(bundle)]
    if artifact:
        parts.extend(["--artifact", artifact])
    raw = _run_single(*parts)
    data = json.loads(raw)
    checks = [VerificationCheck(**c) for c in data.pop("checks", [])]
    return VerificationReport(**data, checks=checks)


def reason(
    bundle: str | Path,
    query: str,
    model: Optional[str] = None,
) -> str:
    """Reason over the knowledge graph and return an answer."""
    parts = ["reason", "--bundle", str(bundle), query]
    if model:
        parts.extend(["--model", model])
    raw = _run_single(*parts)
    data = json.loads(raw)
    return data.get("answer", "")


def install(
    sources: list[str | Path],
    name: Optional[str] = None,
) -> list[dict]:
    """Install a compiled .atlas bundle (or compile + install from source)."""
    parts = ["install"]
    if name:
        parts.extend(["-n", name])
    for s in sources:
        parts.append(str(s))
    raw = _run(*parts)
    results = []
    for line in raw.strip().splitlines():
        results.append(json.loads(line))
    return results


def load(bundle: str | Path) -> dict:
    """Alias: compile a single source with emit-ir and return the IR."""
    from tempfile import TemporaryDirectory
    with TemporaryDirectory() as tmp:
        out = Path(tmp) / "bundle.atlas"
        res = compile([str(bundle)], out=out)
        return res
