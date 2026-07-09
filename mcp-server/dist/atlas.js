import { execSync } from "child_process";
import { accessSync } from "fs";
import { join, isAbsolute, resolve } from "path";
function findBinary() {
    const candidates = process.platform === "win32"
        ? ["atlas-cli.exe", "atlas.exe"]
        : ["atlas-cli", "atlas"];
    const searchDirs = [];
    const cwd = process.cwd();
    // Check cwd/target/release, cwd/target/debug, ../target/release, ../target/debug
    for (const base of [cwd, resolve(cwd, "..")]) {
        searchDirs.push(join(base, "target", "release"));
        searchDirs.push(join(base, "target", "debug"));
    }
    for (const dir of searchDirs) {
        for (const c of candidates) {
            const p = join(dir, c);
            try {
                accessSync(p);
                return p;
            }
            catch { }
        }
    }
    // Look in PATH
    try {
        const which = execSync(process.platform === "win32" ? "where atlas-cli 2>nul" : "which atlas-cli 2>/dev/null", { encoding: "utf8" }).trim();
        if (which)
            return which;
    }
    catch { }
    throw new Error("atlas CLI binary not found. Build with: cargo build --release");
}
function resolvePath(p) {
    if (isAbsolute(p))
        return p;
    const cwd = process.cwd();
    // Try cwd first, then parent
    for (const base of [cwd, resolve(cwd, "..")]) {
        const candidate = join(base, p);
        try {
            accessSync(candidate);
            return candidate;
        }
        catch { }
    }
    return join(cwd, p);
}
function runCLI(...args) {
    const bin = findBinary();
    const output = execSync(`"${bin}" --json ${args.join(" ")}`, {
        encoding: "utf8",
        maxBuffer: 50 * 1024 * 1024,
    });
    const lines = output.trim().split("\n");
    return lines[lines.length - 1];
}
export function solve(bundlePath, query) {
    const resolved = resolvePath(bundlePath);
    const raw = runCLI("solve", "--bundle", `"${resolved}"`, `"${query}"`);
    return JSON.parse(raw);
}
export function decide(bundlePath, query, context) {
    const resolved = resolvePath(bundlePath);
    const args = ["decide", "--bundle", `"${resolved}"`, `"${query}"`];
    if (context) {
        for (const [k, v] of Object.entries(context)) {
            args.push("-c", `"${k}=${v}"`);
        }
    }
    const raw = runCLI(...args);
    if (raw === "null")
        return null;
    return JSON.parse(raw);
}
export function verify(bundlePath, artifact) {
    const resolved = resolvePath(bundlePath);
    const args = ["verify", "--bundle", `"${resolved}"`];
    if (artifact)
        args.push("--artifact", `"${artifact}"`);
    const raw = runCLI(...args);
    return JSON.parse(raw);
}
export function dump(bundlePath) {
    const resolved = resolvePath(bundlePath);
    const raw = runCLI("dump", "--bundle", `"${resolved}"`);
    return JSON.parse(raw);
}
export function reason(bundlePath, query) {
    const resolved = resolvePath(bundlePath);
    const raw = runCLI("reason", "--bundle", `"${resolved}"`, `"${query}"`);
    const data = JSON.parse(raw);
    return data.answer || "";
}
