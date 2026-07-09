interface PackageMetadata {
  name: string;
  version: string;
  description: string;
  author?: string;
  tags: string[];
  published_at: string;
  updated_at: string;
  download_count: number;
  compiler_version: string;
  nodes?: number;
  edges?: number;
  org?: string;
}

interface PackageEntry {
  metadata: PackageMetadata;
  files: Record<string, string>;
}

interface SearchResult {
  name: string;
  version: string;
  description: string;
  tags: string[];
  download_count: number;
  published_at: string;
  nodes?: number;
  edges?: number;
}

interface PublishInput {
  name: string;
  version: string;
  description: string;
  author?: string;
  tags: string[];
  files: Record<string, string>;
  nodes?: number;
  edges?: number;
}

interface AuditEntry {
  action: "publish" | "delete" | "download";
  package_name: string;
  version: string;
  api_key: string;
  org: string;
  timestamp: string;
  ip: string;
}

interface RateLimitState {
  count: number;
  reset_at: number;
}

import { DurableObject } from "cloudflare:workers";

const RATE_LIMIT_WINDOW = 60_000;
const RATE_LIMIT_MAX = 60;

export class RegistryAgent extends DurableObject<Env> {
  private rateLimits: Map<string, RateLimitState>;

  constructor(ctx: DurableObjectState, env: Env) {
    super(ctx, env);
    this.rateLimits = new Map();
  }

  private checkRateLimit(apiKey: string): void {
    const now = Date.now();
    const state = this.rateLimits.get(apiKey);
    if (!state || now > state.reset_at) {
      this.rateLimits.set(apiKey, { count: 1, reset_at: now + RATE_LIMIT_WINDOW });
      return;
    }
    if (state.count >= RATE_LIMIT_MAX) {
      throw new RateLimitError("Rate limit exceeded. Max 60 requests/minute per API key.");
    }
    state.count++;
  }

  private async validateApiKey(apiKey: string): Promise<{ org: string; role: string }> {
    const raw = await this.env.PACKAGES.get(`apikey:${apiKey}`);
    if (!raw) {
      throw new AuthError("Invalid or missing API key.");
    }
    return JSON.parse(raw);
  }

  private async appendAudit(entry: AuditEntry): Promise<void> {
    const key = `audit:${entry.timestamp}`;
    await this.env.PACKAGES.put(key, JSON.stringify(entry));
  }

  private async trackUsage(org: string, metric: string, packageName?: string): Promise<void> {
    const today = new Date().toISOString().slice(0, 10);
    const key = `usage:${org}:${metric}:${today}`;
    const raw = await this.env.PACKAGES.get(key);
    const count = (raw ? parseInt(raw, 10) : 0) + 1;
    await this.env.PACKAGES.put(key, count.toString());
    if (packageName) {
      const pkgKey = `usage:${org}:${metric}:${packageName}:${today}`;
      const pkgRaw = await this.env.PACKAGES.get(pkgKey);
      await this.env.PACKAGES.put(pkgKey, ((pkgRaw ? parseInt(pkgRaw, 10) : 0) + 1).toString());
    }
  }

  async listPackages(org?: string): Promise<PackageMetadata[]> {
    const names = (await this.ctx.storage.get<string[]>("packages_list")) || [];
    const packages: PackageMetadata[] = [];
    for (const name of names) {
      const raw = await this.ctx.storage.get<string>(`pkg:${name}:meta`);
      if (raw) {
        const meta: PackageMetadata = JSON.parse(raw);
        if (!org || meta.org === org) {
          packages.push(meta);
        }
      }
    }
    packages.sort((a, b) => b.download_count - a.download_count);
    return packages;
  }

  async getPackage(name: string): Promise<PackageEntry | null> {
    const raw = await this.ctx.storage.get<string>(`pkg:${name}:meta`);
    if (!raw) return null;
    const filesRaw = (await this.ctx.storage.get<string>(`pkg:${name}:files`)) || "{}";
    return {
      metadata: JSON.parse(raw),
      files: JSON.parse(filesRaw),
    };
  }

  async searchPackages(query: string, limit = 20, org?: string): Promise<SearchResult[]> {
    const terms = query.toLowerCase().split(/\s+/).filter(Boolean);
    if (terms.length === 0) return [];

    const names = (await this.ctx.storage.get<string[]>("packages_list")) || [];
    const scored: { pkg: PackageMetadata; score: number }[] = [];

    for (const name of names) {
      const raw = await this.ctx.storage.get<string>(`pkg:${name}:meta`);
      if (!raw) continue;
      const meta: PackageMetadata = JSON.parse(raw);
      if (org && meta.org !== org) continue;

      let score = 0;
      const nameLower = meta.name.toLowerCase();
      const descLower = meta.description.toLowerCase();

      for (const term of terms) {
        if (nameLower.includes(term)) score += 3;
        if (descLower.includes(term)) score += 2;
        for (const tag of meta.tags) {
          if (tag.toLowerCase().includes(term)) score += 1;
        }
      }

      if (score > 0) {
        scored.push({ pkg: meta, score });
      }
    }

    scored.sort((a, b) => b.score - a.score || b.pkg.download_count - a.pkg.download_count);
    return scored.slice(0, limit).map((s) => ({
      name: s.pkg.name,
      version: s.pkg.version,
      description: s.pkg.description,
      tags: s.pkg.tags,
      download_count: s.pkg.download_count,
      published_at: s.pkg.published_at,
      nodes: s.pkg.nodes,
      edges: s.pkg.edges,
    }));
  }

  async publishPackage(data: PublishInput, auth: { org: string; role: string }, ip: string): Promise<{ success: boolean; name: string; version: string; replaced: boolean }> {
    if (!data.name || !data.name.match(/^[a-z0-9_-]+$/)) {
      throw new ValidationError("Package name must be lowercase alphanumeric with underscores/hyphens.");
    }
    if (!data.version) {
      throw new ValidationError("Version is required.");
    }
    if (!data.files || Object.keys(data.files).length === 0) {
      throw new ValidationError("At least one file is required.");
    }

    const names = (await this.ctx.storage.get<string[]>("packages_list")) || [];

    let download_count = 0;
    let replaced = false;
    const existingRaw = await this.ctx.storage.get<string>(`pkg:${data.name}:meta`);
    if (existingRaw) {
      const existing = JSON.parse(existingRaw);
      if (existing.org && existing.org !== auth.org) {
        throw new AuthError("Package belongs to a different organization.");
      }
      download_count = existing.download_count;
      replaced = true;
    }

    if (!names.includes(data.name)) {
      names.push(data.name);
      await this.ctx.storage.put("packages_list", names);
    }

    const metadata: PackageMetadata = {
      name: data.name,
      version: data.version,
      description: data.description || "",
      author: data.author,
      tags: data.tags || [],
      published_at: existingRaw ? JSON.parse(existingRaw).published_at : new Date().toISOString(),
      updated_at: new Date().toISOString(),
      download_count,
      compiler_version: "0.1.0",
      nodes: data.nodes,
      edges: data.edges,
      org: auth.org,
    };

    await this.ctx.storage.put(`pkg:${data.name}:meta`, JSON.stringify(metadata));
    await this.ctx.storage.put(`pkg:${data.name}:files`, JSON.stringify(data.files));

    await this.appendAudit({
      action: "publish",
      package_name: data.name,
      version: data.version,
      api_key: "",
      org: auth.org,
      timestamp: new Date().toISOString(),
      ip,
    });

    await this.trackUsage(auth.org, "publishes", data.name);

    return { success: true, name: data.name, version: data.version, replaced };
  }

  async deletePackage(name: string, auth: { org: string; role: string }): Promise<{ success: boolean }> {
    if (auth.role !== "admin") {
      throw new AuthError("Only admins can delete packages.");
    }

    const raw = await this.ctx.storage.get<string>(`pkg:${name}:meta`);
    if (!raw) {
      throw new ValidationError("Package not found.");
    }
    const meta: PackageMetadata = JSON.parse(raw);
    if (meta.org && meta.org !== auth.org) {
      throw new AuthError("Package belongs to a different organization.");
    }

    const names = (await this.ctx.storage.get<string[]>("packages_list")) || [];
    const filtered = names.filter((n) => n !== name);
    await this.ctx.storage.put("packages_list", filtered);
    await this.ctx.storage.delete(`pkg:${name}:meta`);
    await this.ctx.storage.delete(`pkg:${name}:files`);

    return { success: true };
  }

  async getOrgStats(org: string): Promise<{
    total_packages: number;
    total_downloads: number;
    publishes_today: number;
    queries_today: number;
  }> {
    const names = (await this.ctx.storage.get<string[]>("packages_list")) || [];
    let total_packages = 0;
    let total_downloads = 0;

    for (const name of names) {
      const raw = await this.ctx.storage.get<string>(`pkg:${name}:meta`);
      if (raw) {
        const meta: PackageMetadata = JSON.parse(raw);
        if (meta.org === org) {
          total_packages++;
          total_downloads += meta.download_count;
        }
      }
    }

    const today = new Date().toISOString().slice(0, 10);
    const publishesRaw = await this.env.PACKAGES.get(`usage:${org}:publishes:${today}`);
    const queriesRaw = await this.env.PACKAGES.get(`usage:${org}:downloads:${today}`);

    return {
      total_packages,
      total_downloads,
      publishes_today: publishesRaw ? parseInt(publishesRaw, 10) : 0,
      queries_today: queriesRaw ? parseInt(queriesRaw, 10) : 0,
    };
  }

  async migrateOrg(fromOrg: string | null, toOrg: string): Promise<{ migrated: number }> {
    const names = (await this.ctx.storage.get<string[]>("packages_list")) || [];
    let migrated = 0;
    for (const name of names) {
      const raw = await this.ctx.storage.get<string>(`pkg:${name}:meta`);
      if (raw) {
        const meta: PackageMetadata = JSON.parse(raw);
        if (fromOrg === null && (meta.org === undefined || meta.org === null || meta.org === "")) {
          meta.org = toOrg;
          await this.ctx.storage.put(`pkg:${name}:meta`, JSON.stringify(meta));
          migrated++;
        } else if (fromOrg !== null && meta.org === fromOrg) {
          meta.org = toOrg;
          await this.ctx.storage.put(`pkg:${name}:meta`, JSON.stringify(meta));
          migrated++;
        }
      }
    }
    return { migrated };
  }

  async getDetailedHealth(): Promise<Record<string, unknown>> {
    const names = (await this.ctx.storage.get<string[]>("packages_list")) || [];
    const uniqueOrgs = new Set<string>();
    let totalFiles = 0;

    for (const name of names) {
      const raw = await this.ctx.storage.get<string>(`pkg:${name}:meta`);
      if (raw) {
        const meta: PackageMetadata = JSON.parse(raw);
        if (meta.org) uniqueOrgs.add(meta.org);
      }
      const filesRaw = await this.ctx.storage.get<string>(`pkg:${name}:files`);
      if (filesRaw) {
        totalFiles += Object.keys(JSON.parse(filesRaw)).length;
      }
    }

    return {
      status: "ok",
      total_packages: names.length,
      total_files: totalFiles,
      organizations: uniqueOrgs.size,
      storage_driver: "durable_object_sqlite",
      kv_driver: "cloudflare_kv",
      timestamp: new Date().toISOString(),
    };
  }

  async incrementDownload(name: string): Promise<void> {
    const raw = await this.ctx.storage.get<string>(`pkg:${name}:meta`);
    if (!raw) return;
    const meta: PackageMetadata = JSON.parse(raw);
    meta.download_count++;
    await this.ctx.storage.put(`pkg:${name}:meta`, JSON.stringify(meta));
    if (meta.org) {
      await this.trackUsage(meta.org, "downloads", name);
    }
  }
}

export class RateLimitError extends Error {
  constructor(msg: string) {
    super(msg);
    this.name = "RateLimitError";
  }
}

export class AuthError extends Error {
  constructor(msg: string) {
    super(msg);
    this.name = "AuthError";
  }
}

export class ValidationError extends Error {
  constructor(msg: string) {
    super(msg);
    this.name = "ValidationError";
  }
}
