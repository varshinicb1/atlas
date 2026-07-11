import app from "../index";
import { RegistryAgent } from "../registry";

function makeMockKV() {
  const s = new Map<string, string>();
  return { get: async (k: string) => s.get(k) ?? null, put: async (k: string, v: string) => { s.set(k, String(v)); }, delete: async (k: string) => { s.delete(k); }, list: async (o?: any) => ({ keys: Array.from(s.keys()).filter(k => !o?.prefix || k.startsWith(o.prefix)).map(n => ({ name: n })) }) };
}
const kv = makeMockKV();
const agent = new RegistryAgent({} as any, { PACKAGES: kv } as any);
const env = { PACKAGES: kv, RegistryAgent: { idFromName: (_: string) => "default", get: () => agent } } as any;
await kv.put("apikey:admin-acme", JSON.stringify({ org: "acme", role: "admin" }));

it("probe invalid name status", async () => {
  let status = -1, body = "";
  try {
    const r = await app.fetch(new Request("https://t/api/v1/publish", {
      method: "POST", headers: { "X-API-Key": "admin-acme", "Content-Type": "application/json" },
      body: JSON.stringify({ name: "MyPkg", version: "1", description: "d", tags: [], files: { a: "b" } }),
    }), env);
    status = r.status; body = await r.text();
  } catch (e: any) {
    status = -999; body = "THREW: " + (e?.message ?? e);
  }
  console.log("PROBE_STATUS=" + status);
  console.log("PROBE_BODY=" + body);
  expect(true).toBe(true);
});
