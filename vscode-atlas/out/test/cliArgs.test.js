"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const node_test_1 = require("node:test");
const strict_1 = __importDefault(require("node:assert/strict"));
const cliArgs_1 = require("../cliArgs");
(0, node_test_1.test)("resolveCliPath falls back to 'atlas' when empty/undefined", () => {
    strict_1.default.equal((0, cliArgs_1.resolveCliPath)(undefined), "atlas");
    strict_1.default.equal((0, cliArgs_1.resolveCliPath)(""), "atlas");
    strict_1.default.equal((0, cliArgs_1.resolveCliPath)("   "), "atlas");
    strict_1.default.equal((0, cliArgs_1.resolveCliPath)(null), "atlas");
});
(0, node_test_1.test)("resolveCliPath honors a custom path (trimmed)", () => {
    strict_1.default.equal((0, cliArgs_1.resolveCliPath)("/usr/local/bin/atlas"), "/usr/local/bin/atlas");
    strict_1.default.equal((0, cliArgs_1.resolveCliPath)("  C:/bin/atlas.exe  "), "C:/bin/atlas.exe");
});
(0, node_test_1.test)("resolveBundlePath falls back to default bundle", () => {
    strict_1.default.equal((0, cliArgs_1.resolveBundlePath)(undefined), "knowledge.atlas");
    strict_1.default.equal((0, cliArgs_1.resolveBundlePath)(""), "knowledge.atlas");
    strict_1.default.equal((0, cliArgs_1.resolveBundlePath)("custom.atlas"), "custom.atlas");
});
(0, node_test_1.test)("outputPathFor swaps supported extensions for .atlas", () => {
    strict_1.default.equal((0, cliArgs_1.outputPathFor)("docs/guide.md"), "docs/guide.atlas");
    strict_1.default.equal((0, cliArgs_1.outputPathFor)("spec.yaml"), "spec.atlas");
    strict_1.default.equal((0, cliArgs_1.outputPathFor)("spec.yml"), "spec.atlas");
    strict_1.default.equal((0, cliArgs_1.outputPathFor)("UPPER.MD"), "UPPER.atlas");
});
(0, node_test_1.test)("outputPathFor only replaces the trailing extension", () => {
    strict_1.default.equal((0, cliArgs_1.outputPathFor)("my.md.notes/file.md"), "my.md.notes/file.atlas");
});
(0, node_test_1.test)("outputPathFor appends .atlas when no known extension", () => {
    strict_1.default.equal((0, cliArgs_1.outputPathFor)("README"), "README.atlas");
});
(0, node_test_1.test)("isCompilable recognizes md/yaml/yml only", () => {
    strict_1.default.equal((0, cliArgs_1.isCompilable)("a.md"), true);
    strict_1.default.equal((0, cliArgs_1.isCompilable)("a.yaml"), true);
    strict_1.default.equal((0, cliArgs_1.isCompilable)("a.yml"), true);
    strict_1.default.equal((0, cliArgs_1.isCompilable)("a.txt"), false);
    strict_1.default.equal((0, cliArgs_1.isCompilable)("a.atlas"), false);
});
(0, node_test_1.test)("parseContextArgs splits key=value pairs into -c flags", () => {
    strict_1.default.deepEqual((0, cliArgs_1.parseContextArgs)("answer=true framework=flutter"), [
        "-c",
        "answer=true",
        "-c",
        "framework=flutter",
    ]);
});
(0, node_test_1.test)("parseContextArgs handles empty/extra whitespace", () => {
    strict_1.default.deepEqual((0, cliArgs_1.parseContextArgs)(""), []);
    strict_1.default.deepEqual((0, cliArgs_1.parseContextArgs)(undefined), []);
    strict_1.default.deepEqual((0, cliArgs_1.parseContextArgs)("  a=1   b=2  "), ["-c", "a=1", "-c", "b=2"]);
});
(0, node_test_1.test)("buildSolveArgs produces the expected CLI invocation", () => {
    strict_1.default.deepEqual((0, cliArgs_1.buildSolveArgs)("k.atlas", "how to X"), [
        "solve",
        "--bundle",
        "k.atlas",
        "how to X",
        "--json",
    ]);
});
(0, node_test_1.test)("buildSolveArgs applies the default bundle when blank", () => {
    strict_1.default.deepEqual((0, cliArgs_1.buildSolveArgs)("", "q"), [
        "solve",
        "--bundle",
        "knowledge.atlas",
        "q",
        "--json",
    ]);
});
(0, node_test_1.test)("buildDecideArgs interleaves context args before --json", () => {
    strict_1.default.deepEqual((0, cliArgs_1.buildDecideArgs)("k.atlas", "widget", ["-c", "a=1"]), [
        "decide",
        "--bundle",
        "k.atlas",
        "widget",
        "-c",
        "a=1",
        "--json",
    ]);
});
(0, node_test_1.test)("buildCompileArgs builds compile invocation", () => {
    strict_1.default.deepEqual((0, cliArgs_1.buildCompileArgs)("in.md", "in.atlas"), [
        "compile",
        "in.md",
        "--out",
        "in.atlas",
        "--json",
    ]);
});
(0, node_test_1.test)("buildDumpArgs / buildSearchArgs / buildInitArgs", () => {
    strict_1.default.deepEqual((0, cliArgs_1.buildDumpArgs)("k.atlas"), ["dump", "--bundle", "k.atlas"]);
    strict_1.default.deepEqual((0, cliArgs_1.buildSearchArgs)("flutter"), ["search", "flutter", "--json"]);
    strict_1.default.deepEqual((0, cliArgs_1.buildInitArgs)("pkg", "rust"), [
        "init",
        "pkg",
        "--template",
        "rust",
    ]);
});
(0, node_test_1.test)("parseCliJson parses the last non-empty line of output", () => {
    const out = 'info: compiling\nwarning: something\n{"nodes":3,"edges":2}\n';
    strict_1.default.deepEqual((0, cliArgs_1.parseCliJson)(out), { nodes: 3, edges: 2 });
});
(0, node_test_1.test)("parseCliJson throws on empty output", () => {
    strict_1.default.throws(() => (0, cliArgs_1.parseCliJson)("   "), /no output/);
});
//# sourceMappingURL=cliArgs.test.js.map