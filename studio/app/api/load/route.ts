import { NextRequest, NextResponse } from "next/server";
import path from "path";
import { findBinary, dumpBundle } from "../../lib/cli";

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
    const workspaceRoot = workspace.endsWith(path.sep) ? workspace : workspace + path.sep;
    if (resolvedPath !== workspace && !resolvedPath.startsWith(workspaceRoot)) {
      return NextResponse.json({ error: "Path traversal detected" }, { status: 400 });
    }
    if (!/\.atlas$/i.test(resolvedPath)) {
      return NextResponse.json({ error: "Only .atlas bundles are allowed" }, { status: 400 });
    }
    const ir = dumpBundle(bin, resolvedPath);
    return NextResponse.json(ir);
  } catch (err: unknown) {
    const message = err instanceof Error ? err.message : String(err);
    return NextResponse.json({ error: message }, { status: 500 });
  }
}
