"""
End-to-end test: registry-backed Atlas Agents used as a test user.

Exercises the full pipeline a real consumer hits:
  discover (registry search) -> fetch package source -> compile -> solve/decide.

Run with: pytest python/tests/test_agents_e2e.py -s
"""

import atlas_sdk as atlas
from atlas_sdk import Agent
from pathlib import Path


def test_frontend_agent_solves_form_validation():
    agent = Agent("frontend-test", discover_limit=3)
    try:
        result = agent.solve("How do I validate a TypeScript form safely at runtime?")
        assert isinstance(result, atlas.SolveResult)
        # The agent should have pulled in a validation-related package and returned nodes.
        assert result.nodes, f"no knowledge returned for query; names={agent.discover('form validation')}"
        # At least one returned node should be about validation / schemas.
        kinds = {n.kind for n in result.nodes}
        assert kinds, "expected at least one node kind"
    finally:
        agent.cleanup()


def test_backend_agent_solves_zod_question():
    agent = Agent("backend-test", packages=["zod"], discover_limit=2)
    try:
        result = agent.solve("How to parse user input and return typed data in Zod?")
        assert result.nodes
        # The zod package defines the parse API node.
        ids = {n.id for n in result.nodes}
        assert any("parse" in i for i in ids), f"expected a parse API node, got {ids}"
    finally:
        agent.cleanup()


def test_data_agent_solves_prisma_question():
    agent = Agent("data-test", packages=["prisma"], discover_limit=2)
    try:
        result = agent.solve("How do I define a one-to-many relation in Prisma schema?")
        assert result.nodes
    finally:
        agent.cleanup()


def test_stack_agent_decides():
    # Decision trees use the dedicated YAML format and reference concept nodes
    # that must exist in the same bundle, so the agent compiles the tree
    # together with the package that defines those concepts.
    root = Path(__file__).resolve().parent.parent
    tree = str(root / "examples" / "tech_stack.tree.yaml")
    pkg = str(root.parent / "packages" / "tech_stack_decisions.md")
    agent = Agent("stack-test", sources=[tree, pkg])
    try:
        decision = agent.decide("I need to build a SaaS web app", context={"type": "saas"})
        assert decision is not None, "decide returned None for a walkable decision tree"
        assert decision.tree_id, "decision missing tree_id"
        assert decision.recommendations, "decision produced no recommendations"
    finally:
        agent.cleanup()


def test_agent_verify_passes():
    agent = Agent("verify-test", packages=["zod"], discover_limit=2)
    try:
        report = agent.verify("validate input")
        assert isinstance(report, atlas.VerificationReport)
        assert report.passed, f"verification failed: {[c.message for c in report.checks if not c.passed]}"
    finally:
        agent.cleanup()


if __name__ == "__main__":
    for fn in [
        test_frontend_agent_solves_form_validation,
        test_backend_agent_solves_zod_question,
        test_data_agent_solves_prisma_question,
        test_stack_agent_decides,
        test_agent_verify_passes,
    ]:
        print(f"RUN {fn.__name__} ...", end=" ")
        fn()
        print("OK")
    print("ALL AGENT TESTS PASSED")
