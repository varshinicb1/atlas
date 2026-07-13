import { execFileSync } from "child_process";
import { accessSync } from "fs";
import path from "path";

export function findBinary(): string {
  const ext = process.platform === "win32" ? ".exe" : "";
  const workspace = path.resolve(process.cwd(), "..");
  for (const sub of ["release", "debug"]) {
    for (const name of ["atlas", "atlas-cli"]) {
      const p = path.join(workspace, "target", sub, name + ext);
      try {
        accessSync(p);
        return p;
      } catch {}
    }
  }
  throw new Error("atlas CLI binary not found. Build with: cargo build --release");
}

export function dumpBundle(bin: string, bundlePath: string): unknown {
  const output = execFileSync(bin, ["dump", "--bundle", bundlePath], {
    encoding: "utf8",
    maxBuffer: 50 * 1024 * 1024,
    timeout: 10000,
  });
  const lines = output.trim().split("\n");
  return JSON.parse(lines[lines.length - 1]);
}

const REGISTRY_BASE = "https://atlas-hub-registry.cbvarshini1.workers.dev";

export function registryURL(path: string): string {
  return `${REGISTRY_BASE}${path}`;
}

export interface RegistryPackageSummary {
  name: string;
  version: string;
  description?: string;
  tags?: string[];
}

export interface RegistryPackageFile {
  path: string;
  content: string;
}

export interface RegistryPackageDetail {
  name: string;
  version: string;
  description?: string;
  tags?: string[];
  files: RegistryPackageFile[];
}
