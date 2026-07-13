"""
QA Agent — Registry-backed test user that exercises the full Atlas pipeline.

Acts as a test user: discovers packages, compiles bundles, runs solve/decide/
verify/reason/search on every available package, and reports defects.

Usage:
    python python/examples/qa_agent.py                     # quick smoke test
    python python/examples/qa_agent.py --full               # full audit (every package)
    python python/examples/qa_agent.py --report             # generate JSON report
"""

from __future__ import annotations

import json
import sys
import time
import traceback
import urllib.error
from dataclasses import dataclass, field
from pathlib import Path
from typing import Optional

import atlas_sdk as atlas
from atlas_sdk import Agent, RegistryClient

REPORT_DIR = Path("qa-reports")
REPORT_DIR.mkdir(exist_ok=True)


@dataclass
class QAResult:
    passed: int = 0
    failed: int = 0
    skipped: int = 0
    defects: list[dict] = field(default_factory=list)
    start_time: float = 0.0
    end_time: float = 0.0

    def add_pass(self, test: str):
        self.passed += 1
        _log(f"PASS {test}")

    def add_fail(self, test: str, detail: str):
        self.failed += 1
        self.defects.append({"test": test, "detail": detail, "time": time.time()})
        _log(f"FAIL {test}: {detail[:200]}")

    def add_skip(self, test: str, reason: str):
        self.skipped += 1
        _log(f"SKIP {test}: {reason}")

    @property
    def total(self) -> int:
        return self.passed + self.failed + self.skipped

    def to_dict(self) -> dict:
        return {
            "passed": self.passed,
            "failed": self.failed,
            "skipped": self.skipped,
            "total": self.total,
            "duration_seconds": round(self.end_time - self.start_time, 2),
            "defects": self.defects,
        }

    def print_summary(self):
        _log("=" * 50)
        _log(f"QA Agent Summary: {self.passed}/{self.total} passed, {self.failed} failed, {self.skipped} skipped")
        _log(f"Duration: {self.end_time - self.start_time:.2f}s")
        if self.defects:
            _log(f"Defects: {len(self.defects)}")
            for d in self.defects:
                _log(f"  - [{d['test']}] {d['detail'][:120]}")
        _log("=" * 50)


def _log(msg: str):
    ts = time.strftime("%H:%M:%S")
    print(f"[{ts}] {msg}")


def _banner(title: str):
    _log("")
    _log(f"### {title} ###")
    _log("")


# ---------------------------------------------------------------------------
# Test suites
# ---------------------------------------------------------------------------


def test_cli_ping(result: QAResult):
    """Verify the atlas CLI binary is reachable."""
    try:
        import subprocess
        from atlas_sdk.client import _find_binary, _BINARY
        if _BINARY is None:
            bin_path = _find_binary()
        else:
            bin_path = _BINARY
        out = subprocess.run([bin_path, "--version"], capture_output=True, text=True, timeout=10)
        assert out.returncode == 0, f"exit code {out.returncode}"
        assert "atlas" in out.stdout.lower(), f"unexpected output: {out.stdout}"
        result.add_pass("cli_ping")
    except Exception as e:
        result.add_fail("cli_ping", str(e))


def test_sdk_solve(result: QAResult):
    """Solve with a known package."""
    for name, query in [
        ("zod", "How to parse a string with Zod?"),
        ("prisma", "How to define a model in Prisma?"),
        ("riverpod", "What is a StateNotifier?"),
    ]:
        agent = Agent(f"qa-solve-{name}", packages=[name])
        try:
            r = agent.solve(query)
            assert r.nodes, f"no nodes for {name}"
            assert any(n.confidence > 0 for n in r.nodes), f"all zero confidence for {name}"
            result.add_pass(f"solve_{name}")
        except Exception as e:
            result.add_fail(f"solve_{name}", str(e))
        finally:
            agent.cleanup()


def test_sdk_decide(result: QAResult):
    """Walk decision trees."""
    root = Path(__file__).resolve().parent.parent
    tree = str(root / "examples" / "tech_stack.tree.yaml")
    pkg = str(root / ".." / "packages" / "tech_stack_decisions.md")
    if not Path(tree).exists() or not Path(pkg).exists():
        result.add_skip("decide_tech_stack", "tree files not found")
        return
    agent = Agent("qa-decide", sources=[tree, pkg])
    try:
        d = agent.decide("I need a database for user data", context={"type": "saas"})
        assert d is not None, "decide returned None"
        assert d.tree_id, "missing tree_id"
        assert d.recommendations, "no recommendations"
        result.add_pass("decide_tech_stack")
    except Exception as e:
        result.add_fail("decide_tech_stack", str(e))
    finally:
        agent.cleanup()


def test_sdk_verify(result: QAResult):
    """Verify compiled bundles pass checks."""
    for name in ["zod", "prisma"]:
        agent = Agent(f"qa-verify-{name}", packages=[name])
        try:
            agent.build_bundle(name)
            report = agent.verify(name)
            if not report.passed:
                msgs = [c.message for c in report.checks if not c.passed]
                result.add_fail(f"verify_{name}", f"failed checks: {msgs}")
            else:
                result.add_pass(f"verify_{name}")
        except Exception as e:
            result.add_fail(f"verify_{name}", str(e))
        finally:
            agent.cleanup()


def test_registry_search(result: QAResult):
    """Registry search returns results."""
    client = RegistryClient()
    queries = ["zod", "prisma", "flutter", "react", "python"]
    for q in queries:
        try:
            results = client.search(q, limit=3)
            assert len(results) > 0, f"no results for {q}"
            assert any(r.get("name") for r in results), "results missing name"
            result.add_pass(f"registry_search_{q}")
        except Exception as e:
            result.add_fail(f"registry_search_{q}", str(e))


def test_registry_get_package(result: QAResult):
    """Fetch individual package from registry."""
    client = RegistryClient()
    for name in ["zod", "prisma", "flutter_core", "riverpod"]:
        try:
            pkg = client.get_package(name)
            assert pkg is not None, f"package {name} not found"
            assert "name" in pkg, "missing name"
            result.add_pass(f"registry_get_{name}")
        except Exception as e:
            result.add_fail(f"registry_get_{name}", str(e))


def test_agent_discover(result: QAResult):
    """Agent discover finds relevant packages."""
    agent = Agent("qa-discover")
    try:
        names = agent.discover("form validation typescript")
        assert len(names) > 0, f"no packages discovered"
        result.add_pass("agent_discover")
    except Exception as e:
        result.add_fail("agent_discover", str(e))
    finally:
        agent.cleanup()


def test_full_pipeline(result: QAResult):
    """End-to-end: discover -> fetch -> compile -> solve -> decide -> verify."""
    agent = Agent("qa-full-pipeline", packages=["zod", "prisma"])
    try:
        agent.build_bundle("zod")
        bundle = agent.bundle
        assert bundle.exists(), "bundle not created"

        check = atlas.verify(bundle)
        assert check.passed, f"bundle failed verification: {[c.message for c in check.checks if not c.passed]}"

        sr = agent.solve("parse schema")
        assert sr.nodes, "solve returned no nodes"

        result.add_pass("full_pipeline")
    except Exception as e:
        result.add_fail("full_pipeline", str(e))
    finally:
        agent.cleanup()


def test_all_registry_packages(result: QAResult):
    """Fetch, compile, and verify every package in the registry (full audit)."""
    client = RegistryClient()
    agent = Agent("qa-full-audit")
    try:
        raw = client._get("/api/v1/packages?limit=100")
        pkgs = raw if isinstance(raw, list) else raw.get("packages", []) if isinstance(raw, dict) else []
        if not pkgs:
            result.add_skip("full_audit", "no packages returned from registry")
            return

        _log(f"Auditing {len(pkgs)} packages ...")
        fails = 0
        for pkg in pkgs:
            name = pkg.get("name", pkg.get("id", "unknown"))
            for attempt in range(3):
                try:
                    time.sleep(1.0)
                    agent._ws.mkdir(parents=True, exist_ok=True)
                    agent._bundle = None
                    agent.packages = [name]
                    r = agent.solve(name)
                    if not r.nodes:
                        fails += 1
                        result.add_fail(f"audit_{name}", "solve returned 0 nodes")
                    else:
                        result.add_pass(f"audit_{name}")
                    break
                except urllib.error.HTTPError as e:
                    if e.code == 429 and attempt < 2:
                        _log(f"  rate limited on {name}, retrying ({attempt+1}/2)...")
                        time.sleep(3.0)
                        continue
                    fails += 1
                    result.add_fail(f"audit_{name}", str(e)[:150])
                    break
                except Exception as e:
                    fails += 1
                    result.add_fail(f"audit_{name}", str(e)[:150])
                    break

        if fails == 0:
            _log(f"All {len(pkgs)} packages compiled and solved without errors")
    except Exception as e:
        result.add_fail("full_audit", str(e))
    finally:
        agent.packages = []
        agent.cleanup()


# ---------------------------------------------------------------------------
# Runner
# ---------------------------------------------------------------------------


def run_smoke(result: QAResult):
    _banner("Smoke Tests")
    test_cli_ping(result)
    test_registry_search(result)
    test_registry_get_package(result)
    test_sdk_solve(result)
    test_sdk_decide(result)
    test_sdk_verify(result)
    test_agent_discover(result)
    test_full_pipeline(result)


def run_full(result: QAResult):
    _banner("Full Audit")
    test_all_registry_packages(result)


def main():
    flags = set(sys.argv[1:])
    result = QAResult()
    result.start_time = time.time()

    if "--full" in flags:
        run_smoke(result)
        run_full(result)
    else:
        run_smoke(result)

    result.end_time = time.time()
    result.print_summary()

    if "--report" in flags:
        report = result.to_dict()
        timestamp = time.strftime("%Y%m%d_%H%M%S")
        path = REPORT_DIR / f"qa-report-{timestamp}.json"
        path.write_text(json.dumps(report, indent=2))
        _log(f"Report saved to {path}")

    sys.exit(1 if result.failed > 0 else 0)


if __name__ == "__main__":
    main()
