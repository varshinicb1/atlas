#!/usr/bin/env python3
"""
Full CI pipeline: compile -> test -> validate
Usage: python scripts/ci.py [--skip-tests] [--skip-lint]

Runs every step sequentially. Any failure stops the pipeline.
Returns exit code 0 only if everything passes.
"""
import sys, subprocess, time
from pathlib import Path


def step(name, cmd, cwd=None, timeout=120):
    """Run a CI step and return True if it passed."""
    print(f"\n{'=' * 60}")
    print(f"  STEP: {name}")
    print(f"  CMD: {' '.join(str(c) for c in cmd)}")
    print(f"{'=' * 60}")
    start = time.time()
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, cwd=cwd, timeout=timeout)
        elapsed = time.time() - start
        stdout = (result.stdout or "").strip()
        stderr = (result.stderr or "").strip()
        if stdout:
            # Show last 30 lines (tail)
            lines = stdout.splitlines()
            tail = lines[-min(len(lines), 30):]
            for line in tail:
                print(f"  {line}")
        if stderr:
            for line in stderr.splitlines()[:20]:
                print(f"  ERR: {line}")
        if result.returncode == 0:
            print(f"  PASSED ({elapsed:.1f}s)")
            return True
        print(f"  FAILED (exit {result.returncode}, {elapsed:.1f}s)")
        return False
    except subprocess.TimeoutExpired:
        print(f"  FAILED (TIMEOUT after {timeout}s)")
        return False
    except FileNotFoundError as e:
        print(f"  FAILED (command not found: {e})")
        return False


def main():
    import argparse
    parser = argparse.ArgumentParser(description="Run full CI pipeline for Atlas")
    parser.add_argument("--skip-tests", action="store_true", help="Skip cargo test")
    parser.add_argument("--skip-lint", action="store_true", help="Skip clippy lint")
    parser.add_argument("--fast", action="store_true", help="Skip lint and tests for quick validation")
    args = parser.parse_args()

    if args.fast:
        args.skip_tests = True
        args.skip_lint = True

    root = Path(__file__).resolve().parent.parent

    if not (root / "Cargo.toml").exists():
        print("ERROR: No Cargo.toml found. Run this from the Atlas project root.")
        return 1

    python = sys.executable

    all_passed = True

    # Step 1: Build Rust CLI
    all_passed &= step("Build Rust CLI", ["cargo", "build", "--release"], cwd=root, timeout=300)

    if not all_passed:
        print("\n  BUILD FAILED - aborting pipeline")
        return 1

    # Step 2: Run tests
    if not args.skip_tests:
        all_passed &= step("Run tests", ["cargo", "test"], cwd=root, timeout=300)
        if not all_passed:
            print("\n  TESTS FAILED - aborting pipeline")
            return 1

    # Step 3: Compile all packages
    compile_script = root / "scripts" / "compile-all.py"
    if compile_script.exists():
        all_passed &= step(
            "Compile all packages",
            [python, str(compile_script)],
            cwd=root,
            timeout=120,
        )
    else:
        print(f"\n  SKIP compile-all.py (not found at {compile_script})")

    # Step 4: Validate all packages
    validate_script = root / "scripts" / "validate-all.py"
    if validate_script.exists():
        all_passed &= step(
            "Validate all packages",
            [python, str(validate_script)],
            cwd=root,
            timeout=120,
        )
    else:
        print(f"\n  SKIP validate-all.py (not found at {validate_script})")

    # Step 5: Clippy lint
    if not args.skip_lint:
        all_passed &= step(
            "Clippy lint",
            ["cargo", "clippy", "--", "-D", "warnings"],
            cwd=root,
            timeout=300,
        )

    print(f"\n{'=' * 60}")
    print(f"  CI {'PASSED' if all_passed else 'FAILED'}")
    print(f"{'=' * 60}")
    return 0 if all_passed else 1


if __name__ == "__main__":
    sys.exit(main())
