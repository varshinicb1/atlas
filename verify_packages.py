import json, sys, subprocess

def run(*args):
    r = subprocess.run(args, capture_output=True, text=True)
    return json.loads(r.stdout.strip().split("\n")[-1])

# Test solve
s = run("target/release/atlas-cli.exe", "solve", "--bundle", "knowledge.atlas", "rust", "--json")
print(f"solve rust: {s['total_matches']} matches, confidence={s['confidence']}")
for n in s["nodes"][:5]:
    print(f"  [{n['kind']:14s}] {n['name']}")

# Test solve for different topics
for q in ["flutter", "typescript", "atlas", "widget"]:
    s = run("target/release/atlas-cli.exe", "solve", "--bundle", "knowledge.atlas", q, "--json")
    print(f"\nsolve '{q}': {s['total_matches']} matches")

# Test decide with context
d = run("target/release/atlas-cli.exe", "decide", "--bundle", "knowledge.atlas", "widget", "-c", "stateful=true", "--json")
if d:
    print(f"\ndecide: {d['tree_id']} -> {d['path']}")
else:
    print(f"\ndecide: no matching tree (expected without context)")

# Test with context
d2 = run("target/release/atlas-cli.exe", "decide", "--bundle", "knowledge.atlas", "widget", "-c", "needs_state=true", "-c", "complex=true", "--json")
if d2:
    print(f"decide with context: {d2['tree_id']} -> {d2['path']}")
    for r in d2["recommendations"]:
        print(f"  recommend {r['node_id']} (confidence={r['confidence']})")
else:
    print("decide with context: no match")

# Summary
m = run("target/release/atlas-cli.exe", "dump", "--bundle", "knowledge.atlas", "--json")
trees = m.get("decision_trees", [])
print(f"\nTotal: {len(m['nodes'])} nodes, {len(m['edges'])} edges, {len(trees)} decision trees")
for t in trees:
    print(f"  Tree: {t['id']}  tags={t['trigger']['tags']}")

# Test decide with widget tree
for ctx in [["answer=true"], ["answer=false"], ["answer=shared"], ["answer=local"]]:
    args = ["target/release/atlas-cli.exe", "decide", "--bundle", "knowledge.atlas", "widget"]
    for c in ctx:
        args.extend(["-c", c])
    d = run(*args)
    if d:
        print(f"\ndecide(widget, {ctx}): {d['tree_id']} path={d['path']}")
        for r in d['recommendations']:
            print(f"  → {r['node_id']} (conf={r['confidence']})")
    else:
        print(f"\ndecide(widget, {ctx}): no match")

