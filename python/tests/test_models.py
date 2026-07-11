from dataclasses import asdict

from atlas_sdk import models


def test_node_defaults():
    n = models.Node(id="a", name="A", kind="Concept")
    assert n.description is None
    assert n.version is None


def test_solve_result_roundtrip():
    data = {
        "query": "q",
        "bundle": "b",
        "confidence": 0.5,
        "total_matches": 1,
        "nodes": [models.Node(id="x", name="X", kind="API", version="1.0")],
    }
    r = models.SolveResult(**data)
    assert r.total_matches == 1
    assert r.nodes[0].kind == "API"
    d = asdict(r)
    assert d["nodes"][0]["version"] == "1.0"


def test_decide_result_recommendations():
    r = models.DecideResult(
        tree_id="t",
        path=["a", "b"],
        rationale="why",
        recommendations=[models.Recommendation(node_id="x", confidence=0.9)],
        agent_instructions="do it",
    )
    assert r.agent_instructions == "do it"
    assert r.recommendations[0].node_id == "x"
    assert r.recommendations[0].confidence == 0.9


def test_verification_report_checks():
    r = models.VerificationReport(
        passed=False,
        checks=[
            models.VerificationCheck(name="c", passed=True, message="ok", severity="warn"),
            models.VerificationCheck(name="d", passed=False, message="bad", severity="error"),
        ],
    )
    assert r.passed is False
    assert len(r.checks) == 2
    assert r.checks[1].severity == "error"


def test_empty_node_list_default():
    r = models.SolveResult(query="q", bundle="b", confidence=0.0, total_matches=0)
    assert r.nodes == []
