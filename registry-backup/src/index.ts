interface Env {
  BACKUPS: R2Bucket;
  CONFIG: KVNamespace;
}

export default {
  async scheduled(event: ScheduledEvent, env: Env, ctx: ExecutionContext): Promise<void> {
    const apiKey = await env.CONFIG.get("registry_admin_key");
    if (!apiKey) {
      console.error("No registry_admin_key configured in KV");
      return;
    }

    const url = "https://atlas-hub-registry.cbvarshini1.workers.dev/api/v1/admin/export";
    const response = await fetch(url, {
      headers: { "X-API-Key": apiKey }
    });

    if (!response.ok) {
      console.error(`Export failed: ${response.status} ${await response.text()}`);
      return;
    }

    const data = await response.text();
    const date = new Date().toISOString().split('T')[0];
    const key = `registry-backup-${date}.json`;

    await env.BACKUPS.put(key, data, {
      httpMetadata: { contentType: "application/json" }
    });

    console.log(`Backup stored: ${key} (${(data.length / 1024).toFixed(1)} KB)`);

    const cutoff = new Date();
    cutoff.setDate(cutoff.getDate() - 30);
    const cutoffStr = cutoff.toISOString().split('T')[0];

    const listed = await env.BACKUPS.list({ prefix: "registry-backup-" });
    for (const obj of listed.objects) {
      const dateStr = obj.key.replace("registry-backup-", "").replace(".json", "");
      if (dateStr < cutoffStr) {
        await env.BACKUPS.delete(obj.key);
        console.log(`Deleted old backup: ${obj.key}`);
      }
    }
  },

  async fetch(request: Request, env: Env, ctx: ExecutionContext): Promise<Response> {
    const url = new URL(request.url);
    if (request.method === "POST" && url.pathname === "/trigger") {
      const authKey = request.headers.get("X-API-Key");
      const configuredKey = await env.CONFIG.get("registry_admin_key");
      if (!authKey || authKey !== configuredKey) {
        return new Response("Unauthorized", { status: 401 });
      }
      await this.scheduled!({ type: "scheduled", scheduledTime: Date.now() } as any, env, ctx);
      return new Response("Backup triggered", { status: 200 });
    }

    if (request.method === "GET" && url.pathname === "/") {
      const listed = await env.BACKUPS.list({ prefix: "registry-backup-" });
      return new Response(JSON.stringify({
        backups: listed.objects.map(o => ({
          key: o.key,
          size: o.size,
          uploaded: o.uploaded
        }))
      }), { headers: { "Content-Type": "application/json" } });
    }

    return new Response("Not found", { status: 404 });
  }
};
