---
kind: Package
id: package:deployment-patterns
name: Deployment & Infrastructure Patterns
version: "1.0"
purpose: Document deployment, CI/CD, containerization, and infrastructure patterns for modern web applications — from Docker multi-stage builds to edge deployment, blue-green releases, and observability stacks.
problem_solved: Provides battle-tested deployment patterns that balance cost, performance, reliability, and developer experience — eliminating the guesswork of Dockerfile optimization, CI/CD pipeline design, observability setup, and multi-environment management.
install: atlas solve deployment-patterns.atlas "deploy a Next.js app to Cloudflare"
dependencies:
  - concept:devops
  - concept:cloud-computing
  - concept:ci-cd
concepts:
  - name: Docker Multi-stage Build
    id: concept:deployment/docker-multistage
    description: Dockerfile with multiple FROM stages — build stage (full SDK + deps for compilation), deps stage (production-only node_modules), runtime stage (distroless or Alpine, only artifacts + production deps). Reduces image size from 1.5GB+ to under 200MB for Node.js apps.
  - name: Edge Deployment
    id: concept:deployment/edge
    description: Deploy code to 330+ locations worldwide — Cloudflare Workers, Vercel Edge Functions, Fastly Compute. Near-zero cold starts (Workers use isolate reuse). Sub-50ms response times globally. Best for API gateways, authentication, redirects, A/B testing, and static asset delivery.
  - name: Serverless Deployment
    id: concept:deployment/serverless
    description: AWS Lambda, Vercel Serverless, Netlify Functions — auto-scaling, pay-per-invocation. Cold starts for infrequent requests (200ms-1s for Node.js, 1-3s for Python). Best for event-driven workloads, webhooks, API endpoints with variable traffic. Use provisioned concurrency for latency-sensitive paths.
  - name: Container Orchestration
    id: concept:deployment/kubernetes
    description: "K8s for complex microservice architectures — auto-scaling, rolling updates, service mesh, canary deployments, secrets management. Overkill for <5 services. Alternatives: Nomad (simpler), Docker Compose + Swarm (for small teams), or serverless containers (Cloud Run, Fargate)."
  - name: CI/CD Pipeline
    id: concept:deployment/ci-cd
    description: "GitHub Actions (most common), GitLab CI, CircleCI. Pipeline stages: lint → typecheck → test → build → deploy. Environment promotion: preview (PR) → staging (main) → production (tag/release). Cache node_modules and build artifacts between runs. Matrix testing for multi-version compatibility."
  - name: Blue-Green Deployment
    id: concept:deployment/blue-green
    description: "Two identical production environments — route traffic from blue to green after green passes health checks. Instant rollback by switching back. Zero-downtime deployments. Requires load balancer with traffic switching. Alternatives: canary (gradual traffic shift), rolling (instance-by-instance replacement)."
  - name: Observability Stack
    id: concept:deployment/observability
    description: Logs (structured JSON via pino/winston → CloudWatch/Logtail), metrics (request rate, error rate, latency p50/p95/p99 → Prometheus/Grafana), traces (distributed tracing via OpenTelemetry → Honeycomb/Datadog), alerts (PagerDuty/OnCall for paging). Start with logs, add metrics, add traces when debugging distributed systems.
  - name: Environment Management
    id: concept:deployment/environments
    description: Development (local), Preview (per-PR deploy), Staging (mirrors production, uses same services), Production (real traffic). Environment parity — same infrastructure, config, and data patterns. Use environment variables (validated by Zod) for differences. Feature flags (Flagship/LaunchDarkly) for gradual rollouts.
  - name: Secret Management
    id: concept:deployment/secrets
    description: Never in code, never in env files committed to git. Use platform secret managers — GitHub Actions secrets, AWS Secrets Manager, Vercel Environment Variables, Cloudflare Workers Secrets. For local dev, .env.local (gitignored). For teams, 1Password CLI, Doppler, or Infisical.
  - name: Database Migrations in CI/CD
    id: concept:deployment/migrations
    description: "Run migrations as part of deployment, not separately. For serverless: run migration before traffic switch (baking into deployment script). For containers: migration init container runs before app starts. Always have backward-compatible migrations (add columns before removing, no backfills in deploy). Rollback migrations exist but rarely used."
  - name: Health Checks
    id: concept:deployment/health-checks
    description: "/healthz (liveness — process is alive), /readyz (readiness — can accept traffic: DB connected, cache warmed), /startupz (startup — slow initialization complete). Load balancers and orchestrators use these to route traffic and restart unhealthy instances."
  - name: Content Delivery Network
    id: concept:deployment/cdn
    description: Global edge cache for static assets (images, JS, CSS, fonts). Cloudflare CDN (fastest, most PoPs), Fastly (programmable VCL), CloudFront (tightest AWS integration). Cache headers (Cache-Control, ETag, Last-Modified) determine cache behavior. Purge cache on deployment for updated assets.
apis:
  - name: Dockerfile multi-stage
    id: api:deployment/dockerfile
    signature: "FROM node:20-alpine AS builder\nWORKDIR /app\nCOPY package*.json ./\nRUN npm ci\nCOPY . .\nRUN npm run build\n\nFROM node:20-alpine\nWORKDIR /app\nCOPY --from=builder /app/dist ./dist\nCOPY --from=builder /app/node_modules ./node_modules\nEXPOSE 3000\nCMD [\"node\", \"dist/index.js\"]"
    returns: Production container image.
    description: Multi-stage Dockerfile. Builder stage installs dev dependencies and compiles. Runtime stage copies only artifacts and production deps. Use node:20-alpine for minimal size. Distroless base (gcr.io/distroless/nodejs) for security-sensitive deployments.
  - name: GitHub Actions CI/CD
    id: api:deployment/github-actions
    signature: "name: Deploy\non: push: branches: [main]\njobs:\n  test:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v4\n      - uses: actions/setup-node@v4\n      - run: npm ci\n      - run: npm run typecheck\n      - run: npm run lint\n      - run: npm test\n  deploy:\n    needs: [test]\n    steps:\n      - run: npx wrangler deploy"
    returns: CI/CD pipeline configuration.
    description: GitHub Actions workflow. test job runs in parallel for speed. deploy job depends on test. Secrets injected from GitHub Actions secrets. Matrix strategy for multi-version testing. Environment protection rules for production.
  - name: Health Check Middleware
    id: api:deployment/health-check
    signature: "app.get('/healthz', (c) => c.text('ok'))\napp.get('/readyz', async (c) => {\n  try {\n    await db.$queryRaw`SELECT 1`\n    return c.text('ready')\n  } catch { return c.text('not ready', 503) }\n})"
    returns: Health check endpoints.
    description: Simple health check routes. /healthz returns immediately (process alive). /readyz checks database connectivity and other dependencies. /startupz for slow initializations. Return 503 for unhealthy states. Platform orchestration uses these to manage traffic.
  - name: Zod Env Validation
    id: api:deployment/env-validation
    signature: "const envSchema = z.object({ DATABASE_URL: z.string().url(), REDIS_URL: z.string().optional(), NODE_ENV: z.enum(['development', 'production', 'test']) })\nconst env = envSchema.parse(process.env)\nexport default env"
    returns: Validated typed environment config.
    description: Runtime environment variable validation. Fail fast on startup if required variables are missing. Optional vars with .default(). Typed env access throughout the app. Document all env vars in the schema for onboarding.
failures:
  - id: failure:deployment/cold-start-p95
    symptom: API responses occasionally take 2-5 seconds, usually after periods of inactivity.
    cause: Serverless cold starts — platform spins up a new instance after idle timeout (typically 5-15 minutes).
    fix: Use provisioned concurrency (AWS) or keep-warm pings (Vercel, Netlify). Switch to edge runtime (Workers) which has near-zero cold starts. Reduce deployment size and dependency count. Use snapshot-based startups (GraalVM, Node.js snapshot).
  - id: failure:deployment/migration-race
    symptom: New code runs against old database schema during rolling deployment.
    cause: Migration applied simultaneously with code deployment — some instances have new code, some have old, against a partially migrated database.
    fix: Separate migration from deployment — apply backward-compatible migrations first, then deploy new code. Remove deprecated columns in a separate deployment cycle. Use expand-contract pattern (add column → deploy read from both → deploy write to new → remove old).
  - id: failure:deployment/env-config-drift
    symptom: "\"Works on my machine\" — app works locally but fails in production."
    cause: Environment variable differences or missing service connections between environments.
    fix: Use environment parity — same database version, same service versions, same OS. Validate env vars at startup with Zod. Use Docker for local development to match production. Maintain a single docker-compose.yml for local services.
  - id: failure:deployment/no-monitoring
    symptom: Team discovers production outage from user complaints instead of alerts.
    cause: No monitoring or alerting configured. Health checks not running. Error tracking not connected.
    fix: "Implement at minimum: health check endpoints, error tracking (Sentry), uptime monitoring (Betterstack/Pingdom), and a simple synthetic check that verifies core user flow every 5 minutes. Add alerting when p95 latency exceeds 500ms or error rate exceeds 1%."
extends: []
implements: []
uses:
  - concept:devops
  - concept:cloud-computing
  - concept:ci-cd
part_of: concept:software-infrastructure
solves:
  - problem:deployment-strategy-selection
  - problem:production-infrastructure-setup
  - problem:ci-cd-pipeline-design
alternatives: []
---
The most important deployment decision is the compute model: edge, serverless, or containers. Edge (Workers, Edge Functions) offers the lowest latency and highest scale with zero cold starts, but has platform constraints (CPU time limit, memory limit, no local filesystem). Serverless (Lambda, Vercel Functions) auto-scales and bills per request, but cold starts affect p95 latency. Containers (ECS, Cloud Run, K8s) offer full control and no platform limits, but require more DevOps overhead and cost base capacity regardless of traffic.

The deployment pipeline should enforce quality gates in sequence: lint (seconds), typecheck (seconds), test (minutes), build (minutes), deploy (seconds). Each gate blocks the next if it fails. The test gate is the most common bottleneck — optimize by running unit tests in parallel (Vitest workers), integration tests against a test database, and E2E tests (Playwright) on a preview deployment. Cache node_modules and .next/.nuxt build cache between CI runs to keep pipeline times under 5 minutes.

Observability should follow the logs → metrics → traces progression. Start with structured JSON logs shipped to a log aggregation service (CloudWatch, Logtail, Axiom). Add business metrics (users created, orders placed, error types) and system metrics (request latency, memory, CPU). Add distributed tracing (OpenTelemetry) when debugging cross-service latency issues. Never try to implement all three at once — it leads to alert fatigue and incomplete implementations.
