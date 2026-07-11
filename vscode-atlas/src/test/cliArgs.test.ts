import { test } from "node:test";
import assert from "node:assert/strict";
import {
  resolveCliPath,
  resolveBundlePath,
  outputPathFor,
  isCompilable,
  parseContextArgs,
  buildSolveArgs,
  buildDecideArgs,
  buildCompileArgs,
  buildDumpArgs,
  buildSearchArgs,
  buildInitArgs,
  parseCliJson,
} from "../cliArgs";

test("resolveCliPath falls back to 'atlas' when empty/undefined", () => {
  assert.equal(resolveCliPath(undefined), "atlas");
  assert.equal(resolveCliPath(""), "atlas");
  assert.equal(resolveCliPath("   "), "atlas");
  assert.equal(resolveCliPath(null), "atlas");
});

test("resolveCliPath honors a custom path (trimmed)", () => {
  assert.equal(resolveCliPath("/usr/local/bin/atlas"), "/usr/local/bin/atlas");
  assert.equal(resolveCliPath("  C:/bin/atlas.exe  "), "C:/bin/atlas.exe");
});

test("resolveBundlePath falls back to default bundle", () => {
  assert.equal(resolveBundlePath(undefined), "knowledge.atlas");
  assert.equal(resolveBundlePath(""), "knowledge.atlas");
  assert.equal(resolveBundlePath("custom.atlas"), "custom.atlas");
});

test("outputPathFor swaps supported extensions for .atlas", () => {
  assert.equal(outputPathFor("docs/guide.md"), "docs/guide.atlas");
  assert.equal(outputPathFor("spec.yaml"), "spec.atlas");
  assert.equal(outputPathFor("spec.yml"), "spec.atlas");
  assert.equal(outputPathFor("UPPER.MD"), "UPPER.atlas");
});

test("outputPathFor only replaces the trailing extension", () => {
  assert.equal(outputPathFor("my.md.notes/file.md"), "my.md.notes/file.atlas");
});

test("outputPathFor appends .atlas when no known extension", () => {
  assert.equal(outputPathFor("README"), "README.atlas");
});

test("isCompilable recognizes md/yaml/yml only", () => {
  assert.equal(isCompilable("a.md"), true);
  assert.equal(isCompilable("a.yaml"), true);
  assert.equal(isCompilable("a.yml"), true);
  assert.equal(isCompilable("a.txt"), false);
  assert.equal(isCompilable("a.atlas"), false);
});

test("parseContextArgs splits key=value pairs into -c flags", () => {
  assert.deepEqual(parseContextArgs("answer=true framework=flutter"), [
    "-c",
    "answer=true",
    "-c",
    "framework=flutter",
  ]);
});

test("parseContextArgs handles empty/extra whitespace", () => {
  assert.deepEqual(parseContextArgs(""), []);
  assert.deepEqual(parseContextArgs(undefined), []);
  assert.deepEqual(parseContextArgs("  a=1   b=2  "), ["-c", "a=1", "-c", "b=2"]);
});

test("buildSolveArgs produces the expected CLI invocation", () => {
  assert.deepEqual(buildSolveArgs("k.atlas", "how to X"), [
    "solve",
    "--bundle",
    "k.atlas",
    "how to X",
    "--json",
  ]);
});

test("buildSolveArgs applies the default bundle when blank", () => {
  assert.deepEqual(buildSolveArgs("", "q"), [
    "solve",
    "--bundle",
    "knowledge.atlas",
    "q",
    "--json",
  ]);
});

test("buildDecideArgs interleaves context args before --json", () => {
  assert.deepEqual(buildDecideArgs("k.atlas", "widget", ["-c", "a=1"]), [
    "decide",
    "--bundle",
    "k.atlas",
    "widget",
    "-c",
    "a=1",
    "--json",
  ]);
});

test("buildCompileArgs builds compile invocation", () => {
  assert.deepEqual(buildCompileArgs("in.md", "in.atlas"), [
    "compile",
    "in.md",
    "--out",
    "in.atlas",
    "--json",
  ]);
});

test("buildDumpArgs / buildSearchArgs / buildInitArgs", () => {
  assert.deepEqual(buildDumpArgs("k.atlas"), ["dump", "--bundle", "k.atlas"]);
  assert.deepEqual(buildSearchArgs("flutter"), ["search", "flutter", "--json"]);
  assert.deepEqual(buildInitArgs("pkg", "rust"), [
    "init",
    "pkg",
    "--template",
    "rust",
  ]);
});

test("parseCliJson parses the last non-empty line of output", () => {
  const out = 'info: compiling\nwarning: something\n{"nodes":3,"edges":2}\n';
  assert.deepEqual(parseCliJson(out), { nodes: 3, edges: 2 });
});

test("parseCliJson throws on empty output", () => {
  assert.throws(() => parseCliJson("   "), /no output/);
});
