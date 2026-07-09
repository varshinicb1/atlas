#!/usr/bin/env python3
"""Publish all compiled .atlas packages to the live registry."""
import subprocess, json, sys, os
from pathlib import Path

REGISTRY = "https://atlas-hub-registry.cbvarshini1.workers.dev"
root = Path(__file__).resolve().parent.parent
cli = root / "target" / "release" / "atlas-cli.exe"

packages = sorted(root.glob("packages/*.md"))
results = []

for md_path in packages:
    name = md_path.stem
    yaml_path = root / "packages" / "decisions" / f"{name}.yaml"
    out_path = root / "packages" / f"{name}.atlas"

    sources = [str(md_path)]
    if yaml_path.exists():
        sources.append(str(yaml_path))

    cmd = [str(cli), "compile", *sources, "--out", str(out_path), "--json"]
    r = subprocess.run(cmd, capture_output=True, text=True, timeout=60)
    if r.returncode != 0:
        print(f"  [SKIP] {name}: compile failed ({r.stderr.strip()})")
        continue

    stats = json.loads(r.stdout.strip())
    print(f"  Publishing {name}... ({stats['nodes']} nodes, {stats['edges']} edges)")

    # Read frontmatter to get description and tags
    desc = name.replace("_", " ").title()
    tags = ["knowledge"]
    with open(md_path, encoding="utf-8") as f:
        content = f.read()
    import re
    m = re.search(r"^purpose:\s*(.+)$", content, re.MULTILINE)
    if m:
        desc = m.group(1).strip().strip('"').strip("'")
    m = re.search(r"^tags:\s*\[(.+)\]", content, re.MULTILINE)
    if m:
        tags = [t.strip().strip('"').strip("'") for t in m.group(1).split(",")]

    payload = {
        "name": name,
        "version": stats.get("version", "0.1.0"),
        "description": desc,
        "tags": tags,
        "nodes": stats["nodes"],
        "edges": stats["edges"],
        "files": {},
    }

    if out_path.exists():
        with open(out_path, "rb") as f:
            import base64
            payload["files"][f"{name}.atlas"] = base64.b64encode(f.read()).decode()

    # Also include source files
    for src in sources:
        with open(src, encoding="utf-8") as f:
            payload["files"][Path(src).name] = f.read()

    # publish via curl using temp file
    tmp = root / "scripts" / f"_pkg_{name}.json"
    with open(tmp, "w", encoding="utf-8") as f:
        json.dump(payload, f, ensure_ascii=False)

    curl_cmd = ["curl.exe", "-s", "-X", "POST", f"{REGISTRY}/api/publish",
                "-H", "Content-Type: application/json",
                "-d", f"@{tmp}"]
    cr = subprocess.run(curl_cmd, capture_output=True, text=True, timeout=30)
    tmp.unlink(missing_ok=True)

    if cr.returncode == 0:
        resp = json.loads(cr.stdout.strip())
        if resp.get("success"):
            print(f"    OK v{resp['version']}")
            results.append(name)
        else:
            print(f"    FAILED: {resp}")
    else:
        print(f"    CURL ERROR: {cr.stderr.strip() or cr.stdout.strip()}")

print(f"\nPublished {len(results)}/{len(packages)} packages")
