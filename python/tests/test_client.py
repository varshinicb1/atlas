import subprocess
from dataclasses import dataclass, field
from typing import Optional

import atlas_sdk
from atlas_sdk import models
from atlas_sdk.client import (
    _BINARY,
    _find_binary,
    _run,
    _run_single,
    compile,
    decide,
    install,
    load,
    reason,
    solve,
    verify,
)


@dataclass
class _Call:
    args: list
    kwargs: dict


class FakeRun:
    """Records subprocess.run calls and returns canned stdout/stderr."""

    def __init__(self, stdout: str = "", returncode: int = 0, stderr: str = ""):
        self.stdout = stdout
        self.returncode = returncode
        self.stderr = stderr
        self.calls: list[_Call] = []

    def __call__(self, *args, **kwargs):
        self.calls.append(_Call(args=args, kwargs=kwargs))
        return subprocess.CompletedProcess(
            args=args,
            returncode=self.returncode,
            stdout=self.stdout,
            stderr=self.stderr,
        )

    def last_cmd(self) -> list:
        return self.calls[-1].args[0]


SOLVE_JSON = (
    '{"query":"AsyncNotifier","bundle":"riverpod","confidence":0.7,'
    '"total_matches":2,"nodes":['
    '{"id":"concept:flutter/async_notifier","name":"AsyncNotifier","kind":"Concept","description":null,"version":null},'
    '{"id":"api:flutter/riverpod/async_notifier_build","name":"AsyncNotifier.build()","kind":"API",'
    '"description":"Initializes the async notifier state","version":"2.5.1"}]}'
)

VERIFY_JSON = (
    '{"passed":true,"checks":['
    '{"name":"API existence","passed":true,"message":"All 24 edges reference valid nodes","severity":"error"},'
    '{"name":"Version consistency","passed":true,"message":"7 versioned nodes","severity":"warn"},'
    '{"name":"Provenance","passed":false,"message":"13 / 14 nodes have provenance","severity":"error"}]}'
)

REASON_JSON = '{"answer":"AsyncNotifier is a state holder.","query":"what is AsyncNotifier"}'

DECIDE_JSON = (
    '{"tree_id":"tree:state_mgmt","path":["root","provider"],"rationale":"use provider",'
    '"recommendations":[{"node_id":"api:flutter/riverpod","confidence":0.9}],'
    '"agent_instructions":"prefer Riverpod"}'
)

INSTALL_OUT = (
    '{"name":"riverpod","status":"installed","version":"2.5.1"}\n'
    '{"name":"flutter_core","status":"installed","version":"3.29"}'
)

COMPILE_JSON = '{"bundle":"out.atlas","nodes":3,"edges":2}'

DUMP_JSON = (
    '{"meta":{"schema_version":"0.1.0","generator":"atlas-compiler/0.1.0"},'
    '"nodes":[{"id":"api:atlas/compile","name":"compile","kind":"API","description":null,"version":"0.1.0"}],'
    '"edges":[],"decision_trees":[]}'
)


def _patch(monkeypatch, fake: FakeRun):
    monkeypatch.setattr("atlas_sdk.client._BINARY", "atlas")
    monkeypatch.setattr(subprocess, "run", fake)
    return fake


def test_find_binary_raises_when_missing(monkeypatch):
    monkeypatch.setattr("atlas_sdk.client._BINARY", None)
    monkeypatch.setattr("atlas_sdk.client.shutil.which", lambda c: None)
    # _find_binary uses pathlib.Path.exists(), not os.path.exists.
    monkeypatch.setattr("atlas_sdk.client.Path.exists", lambda self: False)
    try:
        _find_binary()
    except RuntimeError as e:
        assert "Could not find" in str(e)
    else:
        raise AssertionError("expected RuntimeError")


def test_solve_parses_nodes(monkeypatch):
    fake = _patch(monkeypatch, FakeRun(stdout=SOLVE_JSON))
    result = solve("riverpod.atlas", "AsyncNotifier")
    assert isinstance(result, models.SolveResult)
    assert result.query == "AsyncNotifier"
    assert result.bundle == "riverpod"
    assert result.confidence == 0.7
    assert result.total_matches == 2
    assert len(result.nodes) == 2
    assert isinstance(result.nodes[0], models.Node)
    assert result.nodes[0].name == "AsyncNotifier"
    assert result.nodes[1].version == "2.5.1"


def test_solve_arguments(monkeypatch):
    fake = _patch(monkeypatch, FakeRun(stdout=SOLVE_JSON))
    solve("riverpod.atlas", "AsyncNotifier")
    cmd = fake.last_cmd()
    assert cmd[0] == "atlas"
    assert cmd[1] == "--json"
    assert cmd[2:5] == ["solve", "--bundle", "riverpod.atlas"]
    assert cmd[5] == "AsyncNotifier"


def test_decide_with_context_builds_args(monkeypatch):
    fake = _patch(monkeypatch, FakeRun(stdout=DECIDE_JSON))
    result = decide("atlas.atlas", "compile", {"complexity": "medium", "lang": "dart"})
    assert isinstance(result, models.DecideResult)
    cmd = fake.last_cmd()
    assert "decide" in cmd
    assert "-c" in cmd
    # context pairs present (order may vary)
    ctx = " ".join(cmd)
    assert "complexity=medium" in ctx
    assert "lang=dart" in ctx
    assert isinstance(result.recommendations[0], models.Recommendation)


def test_decide_null_returns_none(monkeypatch):
    fake = _patch(monkeypatch, FakeRun(stdout="null"))
    assert decide("atlas.atlas", "compile") is None


def test_verify_parses_checks(monkeypatch):
    fake = _patch(monkeypatch, FakeRun(stdout=VERIFY_JSON))
    report = verify("riverpod.atlas")
    assert isinstance(report, models.VerificationReport)
    assert report.passed is True
    assert len(report.checks) == 3
    assert isinstance(report.checks[0], models.VerificationCheck)
    assert report.checks[0].severity == "error"
    assert report.checks[2].passed is False


def test_verify_with_artifact(monkeypatch):
    fake = _patch(monkeypatch, FakeRun(stdout=VERIFY_JSON))
    verify("riverpod.atlas", "api:flutter/riverpod")
    cmd = fake.last_cmd()
    assert "--artifact" in cmd
    assert "api:flutter/riverpod" in cmd


def test_reason_returns_answer(monkeypatch):
    fake = _patch(monkeypatch, FakeRun(stdout=REASON_JSON))
    answer = reason("atlas.atlas", "what is AsyncNotifier")
    assert answer == "AsyncNotifier is a state holder."
    cmd = fake.last_cmd()
    assert "reason" in cmd
    assert "what is AsyncNotifier" in cmd


def test_reason_with_model_arg(monkeypatch):
    fake = _patch(monkeypatch, FakeRun(stdout=REASON_JSON))
    reason("atlas.atlas", "q", model="gpt-4")
    cmd = fake.last_cmd()
    assert "--model" in cmd
    assert "gpt-4" in cmd


def test_install_multi_line(monkeypatch):
    fake = _patch(monkeypatch, FakeRun(stdout=INSTALL_OUT))
    results = install(["riverpod.md", "flutter_core.md"], name="myorg")
    assert isinstance(results, list)
    assert len(results) == 2
    assert results[0]["name"] == "riverpod"
    cmd = fake.last_cmd()
    assert "install" in cmd
    assert "-n" in cmd
    assert "myorg" in cmd


def test_compile_single(monkeypatch):
    fake = _patch(monkeypatch, FakeRun(stdout=COMPILE_JSON))
    out = compile(["src.md"], out="out.atlas")
    assert out["bundle"] == "out.atlas"
    cmd = fake.last_cmd()
    assert "compile" in cmd
    assert "--out" in cmd
    assert "out.atlas" in cmd


def test_load_returns_dict(monkeypatch, tmp_path):
    # load() calls compile(); patch the public function so we don't need a temp binary.
    monkeypatch.setattr(atlas_sdk.client, "compile", lambda sources, out: {"bundle": "tmp.atlas"})
    res = load("src.md")
    assert res == {"bundle": "tmp.atlas"}


def test_error_raises_runtime(monkeypatch):
    fake = _patch(monkeypatch, FakeRun(stdout="", returncode=1, stderr="boom"))
    try:
        solve("riverpod.atlas", "x")
    except RuntimeError as e:
        assert "atlas CLI error: boom" in str(e)
    else:
        raise AssertionError("expected RuntimeError")


def test_run_single_returns_last_line(monkeypatch):
    fake = _patch(monkeypatch, FakeRun(stdout="line1\n" + SOLVE_JSON))
    out = _run_single("solve", "--bundle", "riverpod.atlas", "AsyncNotifier")
    assert out == SOLVE_JSON


def test_run_passes_timeout(monkeypatch):
    fake = _patch(monkeypatch, FakeRun(stdout=SOLVE_JSON))
    solve("riverpod.atlas", "AsyncNotifier", timeout=12)
    assert fake.calls[-1].kwargs.get("timeout") == 12


def test_module_exports():
    assert callable(atlas_sdk.solve)
    assert callable(atlas_sdk.decide)
    assert callable(atlas_sdk.verify)
    assert callable(atlas_sdk.compile)
    assert callable(atlas_sdk.install)
    assert callable(atlas_sdk.load)
    assert callable(atlas_sdk.reason)
