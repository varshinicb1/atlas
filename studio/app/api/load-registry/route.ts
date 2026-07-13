import { NextRequest, NextResponse } from "next/server";
import { execFileSync } from "child_process";
import { writeFileSync, mkdtempSync } from "fs";
import path from "path";
import os from "os";
import { findBinary, registryURL } from "../../lib/cli";

export async function GET(request: NextRequest) {
  const name = request.nextUrl.searchParams.get("name");
  if (!name) {
    return NextResponse.json({ error: "Missing 'name' query parameter" }, { status: 400 });
  }

  const sanitized = name.replace(/[^a-zA-Z0-9_-]/g, "");
  if (sanitized !== name) {
    return NextResponse.json({ error: "Invalid package name" }, { status: 400 });
  }

  let tmpDir = "";
  try {
    const bin = findBinary();

    const res = await fetch(registryURL(`/api/v1/packages/${sanitized}`), {
      headers: { "User-Agent": "AtlasStudio/0.1" },
    });
    if (!res.ok) {
      return NextResponse.json({ error: `Package '${sanitized}' not found` }, { status: 404 });
    }
    const pkg = await res.json();
    const files: Array<{ path: string; content: string }> = pkg.files || [];
    const mdFile = files.find((f) => f.path.endsWith(".md"));
    if (!mdFile) {
      return NextResponse.json({ error: "No .md file in package" }, { status: 404 });
    }

    tmpDir = mkdtempSync(path.join(os.tmpdir(), "atlas-"));
    const mdPath = path.join(tmpDir, `${sanitized}.md`);
    writeFileSync(mdPath, mdFile.content, "utf8");

    const yamlFiles = files.filter(
      (f) => f.path.endsWith(".yaml") || f.path.endsWith(".yml")
    );
    const yamlPaths: string[] = [];
    for (const yf of yamlFiles) {
      const yamlName = yf.path.split("/").pop() || `${sanitized}.yaml`;
      const yamlPath = path.join(tmpDir, yamlName);
      writeFileSync(yamlPath, yf.content, "utf8");
      yamlPaths.push(yamlPath);
    }

    const atlasPath = path.join(tmpDir, `${sanitized}.atlas`);

    const compileArgs = [
      "compile",
      mdPath,
      ...yamlPaths,
      "--out",
      atlasPath,
    ];
    execFileSync(bin, compileArgs, {
      encoding: "utf8",
      maxBuffer: 50 * 1024 * 1024,
      timeout: 30000,
    });

    const dumpOutput = execFileSync(bin, ["dump", "--bundle", atlasPath], {
      encoding: "utf8",
      maxBuffer: 50 * 1024 * 1024,
      timeout: 10000,
    });
    const lines = dumpOutput.trim().split("\n");
    const ir = JSON.parse(lines[lines.length - 1]);

    return NextResponse.json(ir);
  } catch (err: unknown) {
    const message = err instanceof Error ? err.message : String(err);
    return NextResponse.json({ error: message }, { status: 500 });
  } finally {
    if (tmpDir) {
      try {
        const fs = await import("fs/promises");
        await fs.rm(tmpDir, { recursive: true, force: true });
      } catch {}
    }
  }
}
