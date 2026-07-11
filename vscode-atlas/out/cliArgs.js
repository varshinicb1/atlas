"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.resolveCliPath = resolveCliPath;
exports.resolveBundlePath = resolveBundlePath;
exports.outputPathFor = outputPathFor;
exports.isCompilable = isCompilable;
exports.parseContextArgs = parseContextArgs;
exports.buildSolveArgs = buildSolveArgs;
exports.buildDecideArgs = buildDecideArgs;
exports.buildCompileArgs = buildCompileArgs;
exports.buildDumpArgs = buildDumpArgs;
exports.buildSearchArgs = buildSearchArgs;
exports.buildInitArgs = buildInitArgs;
exports.parseCliJson = parseCliJson;
const DEFAULT_CLI = "atlas";
const DEFAULT_BUNDLE = "knowledge.atlas";
function resolveCliPath(custom) {
    const trimmed = (custom ?? "").trim();
    return trimmed.length > 0 ? trimmed : DEFAULT_CLI;
}
function resolveBundlePath(custom) {
    const trimmed = (custom ?? "").trim();
    return trimmed.length > 0 ? trimmed : DEFAULT_BUNDLE;
}
function outputPathFor(filePath) {
    const match = /\.(md|yaml|yml)$/i.exec(filePath);
    if (match) {
        return filePath.slice(0, filePath.length - match[0].length) + ".atlas";
    }
    return filePath + ".atlas";
}
function isCompilable(filePath) {
    return /\.(md|yaml|yml)$/i.test(filePath);
}
function parseContextArgs(ctx) {
    if (!ctx)
        return [];
    return ctx
        .split(/\s+/)
        .map((s) => s.trim())
        .filter((s) => s.length > 0)
        .flatMap((s) => ["-c", s]);
}
function buildSolveArgs(bundlePath, query) {
    return ["solve", "--bundle", resolveBundlePath(bundlePath), query, "--json"];
}
function buildDecideArgs(bundlePath, query, ctxArgs) {
    return ["decide", "--bundle", resolveBundlePath(bundlePath), query, ...ctxArgs, "--json"];
}
function buildCompileArgs(filePath, outPath) {
    return ["compile", filePath, "--out", outPath, "--json"];
}
function buildDumpArgs(bundlePath) {
    return ["dump", "--bundle", resolveBundlePath(bundlePath)];
}
function buildSearchArgs(query) {
    return ["search", query, "--json"];
}
function buildInitArgs(name, template) {
    return ["init", name, "--template", template];
}
function parseCliJson(stdout) {
    const trimmed = (stdout ?? "").trim();
    if (trimmed.length === 0) {
        throw new Error("Atlas CLI returned no output");
    }
    const lines = trimmed.split(/\r?\n/);
    const last = lines[lines.length - 1];
    return JSON.parse(last);
}
//# sourceMappingURL=cliArgs.js.map