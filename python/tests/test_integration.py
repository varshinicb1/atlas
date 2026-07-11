"""
Integration tests that exercise the REAL `atlas` CLI binary.

These are skipped automatically when the binary (target/release/atlas.exe)
or a usable .atlas bundle is not present, so the suite still passes in CI
environments without a compiled binary.
"""

import os
import shutil
from pathlib import Path

import pytest

import atlas_sdk
from atlas_sdk.client import _find_binary

REPO_ROOT = Path(__file__).resolve().parent.parent.parent


def _bundle(name: str) -> Path:
    return REPO_ROOT / name


def _have_binary() -> bool:
    try:
        _find_binary()
        return True
    except RuntimeError:
        return False


pytestmark = pytest.mark.skipif(
    not _have_binary(),
    reason="atlas CLI binary not found; skipping integration tests",
)


def test_integration_solve_real_binary():
    bundle = _bundle("packages/riverpod.atlas")
    if not bundle.exists():
        pytest.skip(f"bundle not found: {bundle}")
    result = atlas_sdk.solve(str(bundle), "AsyncNotifier")
    assert isinstance(result, atlas_sdk.SolveResult)
    assert result.query == "AsyncNotifier"
    assert result.confidence >= 0.0


def test_integration_decide_null():
    bundle = _bundle("packages/atlas.atlas")
    if not bundle.exists():
        pytest.skip(f"bundle not found: {bundle}")
    # This bundle has no matching decision tree -> returns None
    assert atlas_sdk.decide(str(bundle), "compile") is None


def test_integration_verify_real_binary():
    bundle = _bundle("packages/riverpod.atlas")
    if not bundle.exists():
        pytest.skip(f"bundle not found: {bundle}")
    report = atlas_sdk.verify(str(bundle))
    assert isinstance(report, atlas_sdk.VerificationReport)
    assert report.passed in (True, False)
    assert len(report.checks) >= 1


def test_integration_reason_real_binary():
    bundle = _bundle("packages/riverpod.atlas")
    if not bundle.exists():
        pytest.skip(f"bundle not found: {bundle}")
    answer = atlas_sdk.reason(str(bundle), "what is AsyncNotifier")
    assert isinstance(answer, str)
    assert len(answer) > 0
