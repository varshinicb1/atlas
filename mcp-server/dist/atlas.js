import { execFileSync } from "child_process";
import { existsSync } from "fs";
import { join, isAbsolute, resolve } from "path";
export function findBinary() {
    const candidates = process.platform === "win32"
        ? ["atlas-cli.exe", "atlas.exe"]
        : ["atlas-cli", "atlas"];
    const searchDirs = [];
    if (process.env.HOME) {
        searchDirs.push(join(process.env.HOME, ".atlas", "bin"));
    }
    if (process.env.LOCALAPPDATA) {
        searchDirs.push(join(process.env.LOCALAPPDATA, "atlas", "bin"));
    }
    searchDirs.push("/usr/local/bin");
    searchDirs.push("/usr/bin");
    const pathEnv = process.env.PATH || "";
    const pathSep = process.platform === "win32" ? ";" : ":";
    for (const dir of pathEnv.split(pathSep)) {
        if (dir)
            searchDirs.push(dir);
    }
    for (const dir of searchDirs) {
        for (const c of candidates) {
            const p = join(dir, c);
            if (existsSync(p))
                return p;
        }
    }
    throw new Error("atlas CLI binary not found. Install with: cargo install --path atlas-cli");
}
export function resolvePath(p) {
    if (isAbsolute(p))
        return p;
    const cwd = process.cwd();
    // Try cwd first, then parent
    for (const base of [cwd, resolve(cwd, "..")]) {
        const candidate = join(base, p);
        if (existsSync(candidate))
            return candidate;
    }
    return join(cwd, p);
}
export function runCLI(...args) {
    const bin = findBinary();
    const output = execFileSync(bin, ["--json", ...args], {
        encoding: "utf8",
        maxBuffer: 50 * 1024 * 1024,
        timeout: 30000,
    });
    const lines = output.trim().split("\n");
    return lines[lines.length - 1];
}
export function solve(bundlePath, query) {
    const resolved = resolvePath(bundlePath);
    const raw = runCLI("solve", "--bundle", resolved, query);
    return JSON.parse(raw);
}
export function decide(bundlePath, query, context) {
    const resolved = resolvePath(bundlePath);
    const args = ["decide", "--bundle", resolved, query];
    if (context) {
        for (const [k, v] of Object.entries(context)) {
            args.push("-c", `${k}=${v}`);
        }
    }
    const raw = runCLI(...args);
    if (raw === "null")
        return null;
    return JSON.parse(raw);
}
export function verify(bundlePath, artifact) {
    const resolved = resolvePath(bundlePath);
    const args = ["verify", "--bundle", resolved];
    if (artifact)
        args.push("--artifact", artifact);
    const raw = runCLI(...args);
    return JSON.parse(raw);
}
export function dump(bundlePath) {
    const resolved = resolvePath(bundlePath);
    const raw = runCLI("dump", "--bundle", resolved);
    return JSON.parse(raw);
}
export function reason(bundlePath, query) {
    const resolved = resolvePath(bundlePath);
    const raw = runCLI("reason", "--bundle", resolved, query);
    const data = JSON.parse(raw);
    return data.answer || "";
}
