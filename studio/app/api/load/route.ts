import { NextRequest, NextResponse } from "next/server";
import { execFileSync } from "child_process";
import { accessSync } from "fs";
import path from "path";

function findBinary(): string {
  const candidates: string[] = process.platform === "win32"
    ? ["atlas-cli.exe", "atlas.exe"]
    : ["atlas-cli", "atlas"];

  const workspace = path.resolve(process.cwd(), "..");

  for (const sub of ["release", "debug"]) {
    for (const c of candidates) {
      const p = path.join(workspace, "target", sub, c);
      try {
        accessSync(p);
        return p;
      } catch {}
    }
  }

  throw new Error("atlas CLI binary not found. Build with: cargo build --release");
}

export async function GET(request: NextRequest) {
  const bundlePath = request.nextUrl.searchParams.get("path");
  if (!bundlePath) {
    return NextResponse.json({ error: "Missing 'path' query parameter" }, { status: 400 });
  }

  try {
    const bin = findBinary();
    const workspace = path.resolve(process.cwd(), "..");
    const cleanPath = bundlePath.trim();
    if (/[;&|`$]/.test(cleanPath)) {
      return NextResponse.json({ error: "Invalid path" }, { status: 400 });
    }
    const resolved = path.isAbsolute(cleanPath) ? cleanPath : path.join(workspace, cleanPath);
    const resolvedPath = path.resolve(resolved);
    if (!resolvedPath.startsWith(workspace)) {
      return NextResponse.json({ error: "Path traversal detected" }, { status: 400 });
    }
    const output = execFileSync(bin, ["dump", "--bundle", resolvedPath], {
      encoding: "utf8",
      maxBuffer: 50 * 1024 * 1024,
      timeout: 10000,
    });
    const lines = output.trim().split("\n");
    const json = lines[lines.length - 1];
    return NextResponse.json(JSON.parse(json));
  } catch (err: unknown) {
    const message = err instanceof Error ? err.message : String(err);
    return NextResponse.json({ error: message }, { status: 500 });
  }
}
