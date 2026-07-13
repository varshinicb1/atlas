---
kind: Package
id: package:vercel
name: Vercel Deployment
version: "2"
purpose: Document Vercel deployment patterns — project configuration, serverless functions, preview deployments, edge functions, and performance optimization for frontend applications.
problem_solved: Provides a global platform for deploying frontend applications with automatic SSL, CDN, serverless functions, preview deployments for every git branch, and seamless integration with major frameworks, eliminating infrastructure management for frontend teams.
install: npm i -g vercel
dependencies:
  - concept:deployment
  - concept:cdns
  - concept:next-js
concepts:
  - name: Project Configuration (vercel.json)
    id: concept:vercel/vercel-json
    description: "Root configuration file for Vercel projects. Defines build command, output directory (dist, .next), install command, environment variables, rewrites/redirects/headers, cron jobs, and function regions. Frameworks auto-detect configuration; vercel.json overrides defaults."
  - name: Serverless Functions
    id: concept:vercel/serverless-functions
    description: "API endpoints deployed as serverless functions in /api directory. Supported runtimes: Node.js (18, 20, 22), Python, Go, Ruby, and custom Docker. Functions use the /api/route pattern or file-based routing. Max execution time: 10s (Hobby), 60s (Pro), 900s (Enterprise)."
  - name: Edge Functions
    id: concept:vercel/edge-functions
    description: "JavaScript/TypeScript functions running at the edge (V8 isolates) in 18 regions worldwide. Sub-millisecond cold starts, 50ms CPU limit (Pro: 5s). Use for A/B testing, geolocation redirects, authentication, headers, and rewrites. Export a default function handling request → response."
  - name: Preview Deployments
    id: concept:vercel/preview-deployments
    description: "Automatic deployments for every git branch and PR. Each preview gets a unique URL (project-name-git-hash.vercel.app). Environment variables can differ from production. Preview comments on PRs with the deployment URL. Auto-deletes when the branch is merged or deleted."
  - name: Incremental Static Regeneration (ISR)
    id: concept:vercel/isr
    description: "Next.js ISR on Vercel caches statically-generated pages and re-renders them on demand when traffic arrives after a stale time. Vercel persists ISR cache globally. on-demand revalidation via res.revalidate(path) or next.revalidate tag-based revalidation."
  - name: Analytics & Speed Insights
    id: concept:vercel/analytics
    description: "Built-in real-user monitoring: Analytics (page views, top pages, referrals), Web Analytics (privacy-first, no cookies), Speed Insights (LCP, CLS, INP from real users). Enable via the Vercel dashboard or @vercel/analytics and @vercel/speed-insights npm packages."
  - name: Environment Variables
    id: concept:vercel/env-vars
    description: "Environment variables configured per-environment (development, preview, production). Sensitive values are encrypted at rest. Pull to local via vercel env pull .env.local. System env vars: VERCEL_URL, VERCEL_ENV, VERCEL_REGION, VERCEL_GIT_COMMIT_SHA, NEXT_PUBLIC_VERCEL_URL."
  - name: Cron Jobs
    id: concept:vercel/cron
    description: "Scheduled serverless function invocations via cron expressions. Configured in vercel.json: { crons: [{ path: '/api/cleanup', schedule: '0 0 * * *' }] }. Minimum interval: 1 minute (Pro). Used for database cleanup, cache warming, report generation, and external API sync."
  - name: Monorepo Support
    id: concept:vercel/monorepo
    description: "Vercel natively supports monorepos. Set rootDirectory in vercel.json to point at the app's directory. Each app in the monorepo has its own project. TurboRepo integration optimizes build caching. Monorepos require pnpm or npm workspaces."
  - name: Logs & Observability
    id: concept:vercel/logs
    description: "Real-time and historical logs via the Vercel dashboard and CLI (vercel logs). Logs include function invocations, build output, and runtime errors. Log drain to external providers (Datadog, Logtail, Axiom) via the integrations page. Log retention: 3 days (Hobby), 14 days (Pro)."
  - name: Webhooks
    id: concept:vercel/webhooks
    description: "Outbound HTTP notifications for deployment events: deployment.created, deployment.ready, deployment.error, project.created. Configure in project settings. Payload includes deployment URL, state, and metadata. Used for Slack notifications, status pages, and deployment dashboards."
apis:
  - name: vercel.json
    id: api:vercel/vercel-json
    signature: "{ functions: { 'api/**/*.ts': { maxDuration: 30, runtime: 'nodejs@20' } }, rewrites: [{ source: '/blog/:path', destination: '/blog/[...slug]' }], headers: [{ source: '/(.*)', headers: [{ key: 'X-Frame-Options', value: 'DENY' }] }], crons: [{ path: '/api/cron', schedule: '0 */6 * * *' }] }"
    returns: Project configuration object.
    description: "The vercel.json file configures builds, routes, headers, redirects, rewrites, functions, and environment variables. Function-level config sets maxDuration, memory (128MB-3008MB), and regions. Rewrites proxy to another path without URL change."
  - name: "@vercel/analytics"
    id: api:vercel/analytics-package
    signature: "import { Analytics } from '@vercel/analytics/react'; <Analytics mode='production' debug={false} />"
    returns: A React component that tracks page views and events.
    description: "Vercel Web Analytics integration. Place in the root layout. Tracks page views, custom events, and web vitals. Privacy-first: no cookies, no personal data. mode: 'auto' enables in production only. debug logs to console."
  - name: vercel deploy
    id: api:vercel/deploy-cli
    signature: "vercel [project-path] [--prod] [--prebuilt] [--archive=tgz] [--yes] [--env KEY=value]"
    returns: Deployment URL.
    description: "CLI command to deploy a project. --prod promotes to production. --prebuilt skips the build step (use with CI-built artifacts). --yes skips confirmation prompts. vercel --cwd ./apps/web for monorepo subdirectory deploys."
  - name: "@vercel/og (OG Images)"
    id: api:vercel/og
    signature: "import { ImageResponse } from '@vercel/og'; export const config = { runtime: 'edge' }; return new ImageResponse(<div style={{ fontSize: 60 }}>Hello</div>, { width: 1200, height: 630 })"
    returns: An edge function response with a PNG image.
    description: "Dynamic Open Graph image generation using JSX and CSS. Runs at the edge for near-instant generation. Supports custom fonts (load from URL). Width/height default 1200x630. Use in /api/og route with query params for dynamic text."
sections:
  - title: Project Setup
    id: section:vercel/setup
    content: |
      vercel.json for a Next.js app with API routes and edge functions:

      ```json
      {
          "installCommand": "npm ci",
          "buildCommand": "next build",
          "outputDirectory": ".next",
          "framework": "nextjs",
          "functions": {
              "api/**/*.ts": { "maxDuration": 30 },
              "api/webhook/**/*.ts": { "maxDuration": 300, "memory": 1024 },
              "middleware.ts": { "runtime": "edge" }
          },
          "crons": [
              { "path": "/api/cleanup", "schedule": "0 6 * * *" }
          ],
          "rewrites": [
              { "source": "/blog/:path*", "destination": "/blog/[...slug]" }
          ],
          "headers": [
              {
                  "source": "/(.*)",
                  "headers": [
                      { "key": "X-Content-Type-Options", "value": "nosniff" },
                      { "key": "Referrer-Policy", "value": "origin-when-cross-origin" }
                  ]
              }
          ]
      }
      ```
  - title: Preview Deployments in PRs
    id: section:vercel/preview-deployments
    content: |
      Every PR automatically gets a preview URL. Use environment variables per scope:

      ```
      # Production
      NEXT_PUBLIC_API_URL = https://api.example.com
      DATABASE_URL = postgres://prod...

      # Preview (set in Vercel dashboard)
      NEXT_PUBLIC_API_URL = https://staging-api.example.com
      DATABASE_URL = postgres://staging...
      ```

      Add a comment to PRs automatically using the Vercel GitHub App. The bot posts the preview URL when the deployment is ready:

      ```
      ✅ Preview deployed to https://my-app-git-feature-branch.vercel.app
      🔍 Inspect: https://vercel.com/my-team/my-app/HASH
      ```

      Preview environment variables can differ from production — configure them per git branch pattern in project settings.
---
