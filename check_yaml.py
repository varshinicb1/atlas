import yaml
import sys

path = sys.argv[1] if len(sys.argv) > 1 else "packages/decisions/flutter_core.yaml"

with open(path) as f:
    content = f.read()

docs = content.split("---")
for i, doc in enumerate(docs):
    if not doc.strip():
        continue
    d = yaml.safe_load(doc)
    ids = {n["id"] for n in d["nodes"]}
    targets = set()
    for n in d["nodes"]:
        if "branches" in n:
            for b in n["branches"]:
                targets.add(b["next"])
    missing = targets - ids
    if missing:
        print(f"Doc {i} ({d['id']}): MISSING TARGETS: {missing}")
    else:
        print(f"Doc {i} ({d['id']}): all targets valid ({len(ids)} nodes, {len(targets)} refs)")