#!/usr/bin/env python3
"""
Score knowledge packages on quality metrics.
Usage: python scripts/quality-score.py [--json] [packages/*.atlas]

Scoring dimensions (each 0-100):
- Completeness: ratio of required fields filled (name, description, purpose, version)
- Graph Density: edges / (nodes * max_edges_possible)
- Concept Coverage: diversity of node kinds (package, concept, api, failure, decision tree)
- Decision Tree Quality: trees that have no orphan nodes, all branches reachable
- Documentation: body text length and structure (has headings, examples)
- Verification: atlas verify passes cleanly
"""
import sys, json, subprocess
from pathlib import Path


WEIGHTS = {
    "completeness": 0.20,
    "graph_density": 0.15,
    "concept_coverage": 0.20,
    "decision_tree_quality": 0.20,
    "documentation": 0.15,
    "verification": 0.10,
}


def get_cli():
    """Find atlas CLI binary."""
    root = Path(__file__).resolve().parent.parent
    candidates = [
        root / "target" / "release" / "atlas-cli.exe",
        root / "target" / "release" / "atlas-cli",
    ]
    for c in candidates:
        if c.exists():
            return str(c)
    import shutil
    which = shutil.which("atlas-cli") or shutil.which("atlas")
    if which:
        return which
    return "atlas-cli"


def dump_ir(cli, path):
    """Get full IR JSON from an atlas binary."""
    try:
        result = subprocess.run(
            [cli, "dump", "--bundle", str(path)],
            capture_output=True, text=True, timeout=30
        )
        if result.returncode != 0:
            return None
        return json.loads(result.stdout.strip())
    except (json.JSONDecodeError, subprocess.TimeoutExpired):
        return None


def score_completeness(ir):
    """Score how complete the package metadata is."""
    meta = ir.get("meta", {})
    score = 0
    if meta.get("schema_version"):
        score += 20
    if meta.get("generator"):
        score += 10
    if meta.get("created_at"):
        score += 10

    nodes = ir.get("nodes", [])
    if not nodes:
        return 0

    main_node = nodes[0]
    if main_node.get("id"):
        score += 15
    if main_node.get("name"):
        score += 10
    if main_node.get("version"):
        score += 15
    desc = main_node.get("description", "") or ""
    if len(desc) > 50:
        score += 10
    attrs = main_node.get("attributes", {}) or {}
    if attrs.get("purpose"):
        score += 5
    if attrs.get("problem_solved"):
        score += 5

    return min(score, 100)


def score_graph_density(ir):
    """Score edge/node ratio."""
    nodes = len(ir.get("nodes", []))
    edges = len(ir.get("edges", []))
    if nodes <= 1:
        return 0
    ratio = edges / nodes
    if 1.5 <= ratio <= 3.0:
        return 100
    elif ratio > 0.5:
        return int(70 + 30 * (1 - abs(ratio - 2.0) / 2.0))
    else:
        return int(50 * ratio / 0.5)


def score_concept_coverage(ir):
    """Score diversity of node kinds."""
    nodes = ir.get("nodes", [])
    if not nodes:
        return 0

    kinds = set()
    for n in nodes:
        k = n.get("kind", "")
        if k:
            kinds.add(k)

    desired = {"Package", "Concept", "API", "FailureMode"}
    found = kinds & desired
    score = int((len(found) / len(desired)) * 60)

    trees = ir.get("decision_trees", [])
    if trees:
        score += 20 + min(len(trees) * 10, 20)

    examples = ir.get("examples", [])
    if examples:
        score += 10

    return min(score, 100)


def score_decision_tree_quality(ir):
    """Score decision tree quality (reachability, no orphans)."""
    trees = ir.get("decision_trees", [])
    if not trees:
        return 0

    scores = []
    for tree in trees:
        nodes_list = tree.get("nodes", [])
        if not nodes_list:
            scores.append(0)
            continue

        non_terminal = [n for n in nodes_list if not n.get("terminal")]
        branchless = [n for n in non_terminal if not n.get("branches")]

        referenced = set()
        defined = {n.get("id") for n in nodes_list}
        for n in non_terminal:
            for b in n.get("branches", []):
                target = b.get("next")
                if target:
                    referenced.add(target)
        orphans = referenced - defined

        s = 100
        if branchless:
            s -= 20 * len(branchless)
        if orphans:
            s -= 15 * len(orphans)

        tags = tree.get("trigger", {}).get("tags")
        if tags:
            s += 10

        terminal_nodes = [n for n in nodes_list if n.get("terminal")]
        with_instructions = sum(1 for n in terminal_nodes if n["terminal"].get("agent_instructions"))
        if terminal_nodes and (with_instructions / len(terminal_nodes)) > 0.5:
            s += 10

        scores.append(max(0, min(s, 100)))

    return int(sum(scores) / len(scores)) if scores else 0


def score_documentation(ir):
    """Score body documentation quality."""
    nodes = ir.get("nodes", [])
    if not nodes:
        return 0
    main = nodes[0]
    desc = main.get("description", "") or ""

    score = 0
    if len(desc) > 100:
        score += 20
    if len(desc) > 500:
        score += 15
    if len(desc) > 2000:
        score += 15

    if "# " in desc:
        score += 10
    if "## " in desc:
        score += 10
    if "|" in desc:
        score += 5
    if "```" in desc:
        score += 10

    attrs = main.get("attributes", {}) or {}
    if attrs.get("purpose"):
        score += 5
    if attrs.get("problem_solved"):
        score += 5
    if attrs.get("install"):
        score += 5

    return min(score, 100)


def score_verification(ir):
    """Score based on verification results inferred from IR."""
    nodes = ir.get("nodes", [])
    edges = ir.get("edges", [])
    failures = ir.get("failure_modes", [])

    if not nodes:
        return 0

    score = 100

    node_ids = {n.get("id") for n in nodes}
    failure_ids = {f.get("id") for f in failures}
    valid_targets = node_ids | failure_ids

    for e in edges:
        if e.get("edge_type") == "PartOf" and e.get("dst") not in valid_targets:
            score -= 10

    no_prov = [n for n in nodes if not n.get("provenance")]
    if no_prov:
        score -= 10 * len(no_prov)

    return max(0, score)


def score_package(cli, path):
    """Score a single package."""
    ir = dump_ir(cli, path)
    if not ir:
        return {
            "overall": 0,
            "error": "Failed to read binary",
            "dimensions": {},
            "issues": ["Cannot load binary"],
            "strengths": [],
        }

    dimensions = {
        "completeness": score_completeness(ir),
        "graph_density": score_graph_density(ir),
        "concept_coverage": score_concept_coverage(ir),
        "decision_tree_quality": score_decision_tree_quality(ir),
        "documentation": score_documentation(ir),
        "verification": score_verification(ir),
    }

    overall = int(sum(dimensions[k] * WEIGHTS[k] for k in dimensions))

    issues = []
    strengths = []

    if dimensions["completeness"] < 60:
        issues.append("Low metadata completeness - missing version, purpose, or description")
    else:
        strengths.append("Good metadata completeness")

    if dimensions["graph_density"] < 40:
        issues.append("Sparse graph - add more relationships between concepts")
    elif dimensions["graph_density"] > 80:
        strengths.append("Well-connected knowledge graph")

    if dimensions["concept_coverage"] < 40:
        issues.append("Limited node diversity - add APIs, failure modes, and decision trees")
    elif dimensions["concept_coverage"] > 70:
        nodes = ir.get("nodes", [])
        kind = nodes[0].get("kind", "unknown") if nodes else "unknown"
        strengths.append(f"Good concept diversity ({kind} + APIs + failures)")

    if dimensions["decision_tree_quality"] > 0:
        trees = ir.get("decision_trees", [])
        strengths.append(f"{len(trees)} decision trees for guided recommendations")

    failure_count = len(ir.get("failure_modes", []))
    if failure_count > 0:
        strengths.append(f"{failure_count} failure modes documented")

    if dimensions["documentation"] < 50:
        issues.append("Thin documentation - expand the body with examples and structured content")

    node_ids = {n.get("id") for n in ir.get("nodes", [])}
    failure_ids = {f.get("id") for f in ir.get("failure_modes", [])}
    valid = node_ids | failure_ids
    for e in ir.get("edges", []):
        if e.get("edge_type") == "PartOf" and e.get("dst") not in valid:
            issues.append(f"PartOf edge references non-existent node: {e.get('dst')}")

    return {
        "overall": overall,
        "dimensions": dimensions,
        "issues": issues[:5],
        "strengths": strengths[:5],
    }


def main():
    import argparse
    parser = argparse.ArgumentParser(description="Score knowledge packages")
    parser.add_argument("paths", nargs="*", help="Package .atlas files to score")
    parser.add_argument("--json", action="store_true", help="Output JSON")
    parser.add_argument("--all", action="store_true", help="Score all packages in packages/")
    args = parser.parse_args()

    cli = get_cli()
    root = Path(__file__).resolve().parent.parent

    if args.all:
        paths = sorted(root.glob("packages/*.atlas"))
    elif args.paths:
        paths = [Path(p) for p in args.paths]
    else:
        paths = sorted(root.glob("packages/*.atlas"))

    if not paths:
        print("No .atlas files found.")
        return 1

    scores = {}
    for p in paths:
        scores[p.stem] = score_package(cli, p)

    if args.json:
        result = {"packages": scores}
        all_scores = [s["overall"] for s in scores.values()]
        if all_scores:
            result["summary"] = {
                "count": len(all_scores),
                "average": int(sum(all_scores) / len(all_scores)),
                "min": min(all_scores),
                "max": max(all_scores),
            }
        print(json.dumps(result, indent=2))
    else:
        header = f"{'Package':40} {'Overall':>8}  {'Comp':>4} {'Graph':>4} {'Concepts':>8} {'Trees':>4} {'Docs':>4} {'Vrfy':>4}"
        print(header)
        print("-" * len(header))
        for name, s in sorted(scores.items()):
            d = s.get("dimensions", {})
            print(f"{name:40} {s.get('overall', 0):>8}  {d.get('completeness', 0):>4} {d.get('graph_density', 0):>4} {d.get('concept_coverage', 0):>8} {d.get('decision_tree_quality', 0):>4} {d.get('documentation', 0):>4} {d.get('verification', 0):>4}")

        print()
        for name, s in sorted(scores.items()):
            if s.get("issues"):
                for issue in s["issues"]:
                    print(f"  {name}: ! {issue}")
            if s.get("strengths"):
                for strength in s["strengths"]:
                    print(f"  {name}: + {strength}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
