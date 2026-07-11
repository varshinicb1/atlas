const DEFAULT_CLI = "atlas";
const DEFAULT_BUNDLE = "knowledge.atlas";

export function resolveCliPath(custom?: string | null): string {
  const trimmed = (custom ?? "").trim();
  return trimmed.length > 0 ? trimmed : DEFAULT_CLI;
}

export function resolveBundlePath(custom?: string | null): string {
  const trimmed = (custom ?? "").trim();
  return trimmed.length > 0 ? trimmed : DEFAULT_BUNDLE;
}

export function outputPathFor(filePath: string): string {
  const match = /\.(md|yaml|yml)$/i.exec(filePath);
  if (match) {
    return filePath.slice(0, filePath.length - match[0].length) + ".atlas";
  }
  return filePath + ".atlas";
}

export function isCompilable(filePath: string): boolean {
  return /\.(md|yaml|yml)$/i.test(filePath);
}

export function parseContextArgs(ctx?: string | null): string[] {
  if (!ctx) return [];
  return ctx
    .split(/\s+/)
    .map((s) => s.trim())
    .filter((s) => s.length > 0)
    .flatMap((s) => ["-c", s]);
}

export function buildSolveArgs(bundlePath: string, query: string): string[] {
  return ["solve", "--bundle", resolveBundlePath(bundlePath), query, "--json"];
}

export function buildDecideArgs(
  bundlePath: string,
  query: string,
  ctxArgs: string[]
): string[] {
  return ["decide", "--bundle", resolveBundlePath(bundlePath), query, ...ctxArgs, "--json"];
}

export function buildCompileArgs(filePath: string, outPath: string): string[] {
  return ["compile", filePath, "--out", outPath, "--json"];
}

export function buildDumpArgs(bundlePath: string): string[] {
  return ["dump", "--bundle", resolveBundlePath(bundlePath)];
}

export function buildSearchArgs(query: string): string[] {
  return ["search", query, "--json"];
}

export function buildInitArgs(name: string, template: string): string[] {
  return ["init", name, "--template", template];
}

export function parseCliJson<T = unknown>(stdout: string): T {
  const trimmed = (stdout ?? "").trim();
  if (trimmed.length === 0) {
    throw new Error("Atlas CLI returned no output");
  }
  const lines = trimmed.split(/\r?\n/);
  const last = lines[lines.length - 1];
  return JSON.parse(last) as T;
}
