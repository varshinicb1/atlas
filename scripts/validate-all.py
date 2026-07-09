#!/usr/bin/env python3
"""
Run atlas verify on all compiled knowledge packages.
Usage: python scripts/validate-all.py [--cli-path PATH] [--json]

Scans packages/*.atlas and runs atlas verify on each,
reporting pass/fail per package with detailed check results.
"""
import sys, subprocess, json
from pathlib import Path


def find_cli():
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
    print("ERROR: Cannot find atlas CLI.")
    sys.exit(1)


def verify_package(cli, atlas_path):
    """Run atlas verify on a single package, return parsed result."""
    cmd = [str(cli), "verify", "--bundle", str(atlas_path), "--json"]
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
        if result.returncode != 0 and not result.stdout.strip():
            return {"passed": False, "checks": [], "error": result.stderr.strip()}
        report = json.loads(result.stdout.strip())
        return report
    except subprocess.TimeoutExpired:
        return {"passed": False, "checks": [], "error": "TIMEOUT"}
    except json.JSONDecodeError:
        return {"passed": False, "checks": [], "error": f"JSON parse error: {result.stdout[:200]}"}


def main():
    import argparse
    parser = argparse.ArgumentParser(description="Validate all Atlas knowledge packages")
    parser.add_argument("--cli-path", help="Explicit path to atlas CLI")
    parser.add_argument("--json", action="store_true", help="Output results as JSON")
    args = parser.parse_args()

    cli = args.cli_path or find_cli()
    root = Path(__file__).resolve().parent.parent

    atlas_files = sorted(root.glob("packages/*.atlas"))
    if not atlas_files:
        print("No .atlas files found in packages/ directory.")
        return 0

    all_passed = True
    results = {}

    print(f"Validating {len(atlas_files)} packages...\n")

    for atlas_path in atlas_files:
        name = atlas_path.stem
        report = verify_package(cli, atlas_path)
        passed = report.get("passed", False)
        checks = report.get("checks", [])
        error = report.get("error")

        results[name] = {
            "passed": passed,
            "path": str(atlas_path),
            "checks": checks,
            "error": error,
        }

        status = "PASS" if passed else "FAIL"
        print(f"  [{status}] {name}")
        if error:
            print(f"          ERROR: {error}")
            all_passed = False
        elif not passed:
            for check in checks:
                if not check.get("passed"):
                    msg = check.get("message", "no message")
                    cname = check.get("name", "unnamed check")
                    print(f"          x {cname}: {msg}")
            all_passed = False

    print(f"\n{'=' * 60}")
    print(f"Overall: {'ALL PASSED' if all_passed else 'SOME FAILED'}")

    if args.json:
        summary = {
            "total": len(atlas_files),
            "passed": sum(1 for r in results.values() if r["passed"]),
            "failed": sum(1 for r in results.values() if not r["passed"]),
            "packages": results,
        }
        print(json.dumps(summary, indent=2))

    return 0 if all_passed else 1


if __name__ == "__main__":
    sys.exit(main())
