// In-memory stand-in for `cloudflare:workers` used by vitest under Node.
// The real module only exists in the Workers runtime; this lets the
// RegistryAgent Durable Object be unit-tested with a hand-rolled storage
// backend while preserving the public `DurableObject` base class shape.

class MemoryStorage {
  private data = new Map<string, string>();

  async get(key: string): Promise<string | undefined> {
    return this.data.get(key);
  }

  async put(key: string, value: string): Promise<void> {
    this.data.set(key, value);
  }

  async delete(key: string): Promise<void> {
    this.data.delete(key);
  }

  async list(opts?: {
    prefix?: string;
    limit?: number;
  }): Promise<{ keys: { name: string }[] }> {
    const keys = Array.from(this.data.keys())
      .filter((k) => !opts?.prefix || k.startsWith(opts.prefix))
      .slice(0, opts?.limit ?? 100)
      .map((name) => ({ name }));
    return { keys };
  }
}

export class DurableObject<T = unknown> {
  ctx: { storage: MemoryStorage };
  env: T;

  constructor(_ctx: unknown, env: T) {
    this.ctx = { storage: new MemoryStorage() };
    this.env = env;
  }
}
