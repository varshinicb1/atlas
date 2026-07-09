#!/usr/bin/env python3
"""
Compile all knowledge packages in the packages/ directory.
Usage: python scripts/compile-all.py [--release] [--cli-path PATH]

Scans packages/*.md, finds matching decisions/<name>.yaml pairs,
and compiles each knowledge package.
"""
import sys, os, subprocess, glob, json, time
from pathlib import Path


def find_cli(release=True):
    """Find atlas CLI binary."""
    root = Path(__file__).resolve().parent.parent
    candidates = [
        root / "target" / ("release" if release else "debug") / "atlas-cli.exe",
        root / "target" / ("release" if release else "debug") / "atlas-cli",
    ]
    for c in candidates:
        if c.exists():
            return str(c)
    import shutil
    which = shutil.which("atlas-cli") or shutil.which("atlas")
    if which:
        return which
    print("ERROR: Cannot find atlas CLI. Build it first with: cargo build --release")
    sys.exit(1)


def compile_package(cli, md_path, yaml_path, out_path):
    """Compile a single knowledge package, return stats dict."""
    sources = [str(md_path)]
    if yaml_path and yaml_path.exists():
        sources.append(str(yaml_path))
    cmd = [cli, "compile", *sources, "--out", str(out_path), "--json"]
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=60)
        if result.returncode == 0 and result.stdout.strip():
            stats = json.loads(result.stdout.strip())
            stats["name"] = md_path.stem
            stats["success"] = True
            stats["output"] = str(out_path)
            return stats
        return {"name": md_path.stem, "success": False, "error": result.stderr.strip() or result.stdout.strip()}
    except subprocess.TimeoutExpired:
        return {"name": md_path.stem, "success": False, "error": "TIMEOUT"}
    except json.JSONDecodeError:
        return {"name": md_path.stem, "success": False, "error": f"JSON parse error: {result.stdout}"}


def watch_loop(root, md_files, cli, args):
    """Watch for file changes and recompile on modification."""
    mtimes = {p: p.stat().st_mtime for p in md_files}
    decision_dir = root / "packages" / "decisions"
    if decision_dir.exists():
        for y in decision_dir.glob("*.yaml"):
            mtimes[y] = y.stat().st_mtime
    print("\nWatching for changes... (Ctrl+C to stop)")
    try:
        while True:
            time.sleep(2)
            for p, old_mtime in list(mtimes.items()):
                if not p.exists():
                    continue
                new_mtime = p.stat().st_mtime
                if new_mtime > old_mtime:
                    mtimes[p] = new_mtime
                    md_path = p if p.suffix == ".md" else next(
                        (m for m in md_files if m.stem == p.stem), None
                    )
                    if md_path is None:
                        continue
                    name = md_path.stem
                    yaml_path = decision_dir / f"{name}.yaml"
                    out_path = root / "packages" / f"{name}.atlas"
                    print(f"\n  [{time.strftime('%H:%M:%S')}] Changed: {p.name}")
                    print(f"  Recompiling {name}...", end=" ")
                    sys.stdout.flush()
                    stats = compile_package(cli, md_path, yaml_path, out_path)
                    if stats.get("success"):
                        print(f"OK ({stats.get('nodes', 0)} nodes, {stats.get('edges', 0)} edges)")
                    else:
                        print(f"FAILED: {stats.get('error', 'unknown')}")
    except KeyboardInterrupt:
        print("\nStopped.")


def main():
    import argparse
    parser = argparse.ArgumentParser(description="Compile all Atlas knowledge packages")
    parser.add_argument("--release", action="store_true", default=True, help="Use release build")
    parser.add_argument("--cli-path", help="Explicit path to atlas CLI")
    parser.add_argument("--watch", action="store_true", help="Watch for file changes and recompile")
    args = parser.parse_args()

    root = Path(__file__).resolve().parent.parent
    cli = args.cli_path or find_cli(args.release)

    print(f"Using CLI: {cli}")
    print()

    md_files = sorted(root.glob("packages/*.md"))
    if not md_files:
        print("No .md files found in packages/ directory.")
        return 1

    results = []

    for md_path in md_files:
        name = md_path.stem
        yaml_path = root / "packages" / "decisions" / f"{name}.yaml"
        out_path = root / "packages" / f"{name}.atlas"

        print(f"  Compiling {name}...", end=" ")
        sys.stdout.flush()
        stats = compile_package(cli, md_path, yaml_path, out_path)
        results.append(stats)
        if stats.get("success"):
            nodes = stats.get("nodes", 0)
            edges = stats.get("edges", 0)
            trees = stats.get("decision_trees", 0)
            print(f"OK ({nodes} nodes, {edges} edges, {trees} trees)")
        else:
            print(f"FAILED: {stats.get('error', 'unknown')}")

    print()
    print("=" * 60)
    success_count = sum(1 for r in results if r.get("success"))
    print(f"Total: {len(results)} packages, {success_count} succeeded, {len(results) - success_count} failed")

    if args.watch:
        watch_loop(root, md_files, cli, args)

    return 0 if success_count == len(results) else 1


if __name__ == "__main__":
    sys.exit(main())
