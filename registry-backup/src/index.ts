interface Env {
  BACKUPS: R2Bucket;
  CONFIG: KVNamespace;
  REGISTRY: Fetcher;
}

export default {
  async scheduled(event: ScheduledEvent, env: Env, ctx: ExecutionContext): Promise<void> {
    console.log("Scheduled backup started");
    const apiKey = await env.CONFIG.get("registry_admin_key");
    if (!apiKey) {
      console.error("No registry_admin_key configured in KV");
      return;
    }

    console.log("Fetching export from registry via service binding");
    try {
      const response = await env.REGISTRY.fetch("https://registry/api/v1/admin/export", {
        headers: { "X-API-Key": apiKey }
      });

      if (!response.ok) {
        console.error(`Export failed: ${response.status} ${await response.text()}`);
        return;
      }

      const data = await response.text();
      console.log(`Export received: ${data.length} bytes`);
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
      console.log(`Found ${listed.objects.length} existing backups`);
      for (const obj of listed.objects) {
        const dateStr = obj.key.replace("registry-backup-", "").replace(".json", "");
        if (dateStr < cutoffStr) {
          await env.BACKUPS.delete(obj.key);
          console.log(`Deleted old backup: ${obj.key}`);
        }
      }
    } catch (err) {
      console.error(`Scheduled backup error: ${err instanceof Error ? err.message : String(err)}`);
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
      try {
        const apiKey = await env.CONFIG.get("registry_admin_key");
        if (!apiKey) return new Response("No API key in config", { status: 500 });
        const resp = await env.REGISTRY.fetch("https://registry/api/v1/admin/export", {
          headers: { "X-API-Key": apiKey }
        });
        if (!resp.ok) {
          const body = await resp.text();
          return new Response(`Export failed: ${resp.status} ${body.substring(0, 500)}`, { status: 500 });
        }
        const text = await resp.text();
        if (!text) return new Response("Empty export data", { status: 500 });
        const key = `registry-backup-${new Date().toISOString().split('T')[0]}.json`;
        await env.BACKUPS.put(key, text, {
          httpMetadata: { contentType: "application/json" }
        });
        return new Response(`Backup stored: ${key} (${(text.length / 1024).toFixed(1)} KB)`, { status: 200 });
      } catch (err) {
        const msg = err instanceof Error ? `${err.name}: ${err.message}\n${err.stack}` : String(err);
        return new Response(`Error: ${msg}`, { status: 500 });
      }
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
