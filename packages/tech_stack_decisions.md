---
kind: Package
id: package:tech-stack-decisions
name: Tech Stack Decision Framework
version: "1.0"
purpose: Guide teams through selecting the optimal tech stack for any project — from solo dev MVPs to enterprise monorepos — by evaluating project requirements against framework, database, and deployment trade-offs with structured decision trees.
problem_solved: Eliminates analysis paralysis in technology selection by providing a decision-tree-driven evaluation framework that maps project characteristics (team size, performance needs, deployment targets, data complexity) to specific technology recommendations with rationales and alternatives.
install: atlas solve tech-stack-decisions.atlas "I need to build a real-time dashboard"
dependencies:
  - concept:software-architecture
  - concept:project-planning
concepts:
  - name: Project Profiling
    id: concept:tech-stack/project-profile
    description: A 5-dimension evaluation — team size (solo, small, large), performance requirement (static, dynamic, real-time), deployment target (edge, server, mobile), data complexity (simple, relational, multi-model), and timeline (hours, days, weeks, months). Each dimension maps to technology choices.
  - name: Framework Decision
    id: concept:tech-stack/framework-decision
    description: Decision tree evaluating framework choice based on rendering needs (SSR vs SSG vs CSR vs real-time), team expertise (React, Vue, Svelte), deployment platform, and interactivity level. Outputs specific framework + rendering mode combination.
  - name: Database Decision
    id: concept:tech-stack/database-decision
    description: Decision tree for database selection — relational (PostgreSQL via Prisma/Drizzle), document (MongoDB/Firebase), edge (D1/CockroachDB), KV (Redis/Cloudflare KV/Upstash), vector (pgvector/LanceDB), or combination. Evaluates query patterns, scale, consistency needs, and budget.
  - name: Auth Strategy
    id: concept:tech-stack/auth-decision
    description: Decision tree for authentication approach — managed (Clerk/Auth0), framework-native (NextAuth/Lucia), self-hosted (SuperTokens/Kinde), or passwordless (Magic Link/WebAuthn). Evaluates team auth expertise, multi-tenancy needs, OAuth provider requirements, and compliance (SOC2, GDPR).
  - name: Deployment Decision
    id: concept:tech-stack/deployment-decision
    description: Decision tree for deployment — edge (Cloudflare Workers/Pages), serverless (Vercel/Netlify), container (Docker/K8s on AWS/GCP), or platform (Railway/Fly.io). Evaluates traffic patterns, geographic distribution, cold-start tolerance, team DevOps expertise, and budget.
  - name: State Management Decision
    id: concept:tech-stack/state-decision
    description: Decision tree for frontend state — server state (TanStack Query/SWR), client state (Zustand/Jotai/Redux), URL state (Nuxt/Next.js router), form state (React Hook Form/Zod). Evaluates state complexity, persistence needs, and sync requirements.
  - name: Rendering Strategy Decision
    id: concept:tech-stack/rendering-decision
    description: Decision tree for page rendering — static generation (SSG), server-side render (SSR), client-side render (CSR), incremental static regeneration (ISR), or streaming SSR. Evaluates content freshness, SEO needs, time-to-interactive targets, and data source.
  - name: UI Component Decision
    id: concept:tech-stack/ui-decision
    description: Decision tree for UI framework — copy-paste (shadcn/ui), headless (Radix/Ariakit), design-system (MUI/Chakra/Ant), or zero-runtime (Tailwind only). Evaluates customization needs, accessibility requirements, design team involvement, and bundle size constraints.
  - name: Monorepo Decision
    id: concept:tech-stack/monorepo-decision
    description: Decision tree for monorepo tooling — Turborepo (Next.js/Vite), Nx (Angular/React), PNPM workspace (minimal), or Bazel (large-scale). Evaluates team size, number of apps, build speed needs, and CI infrastructure.
decision_trees:
  - id: decision:framework
    trigger:
      intent: select-framework
      domain: frontend
      tags: [architecture, setup]
    root: d:framework-start
    nodes:
      - id: d:framework-start
        question: Is your app primarily content-focused (blog, docs, marketing) or app-focused (dashboard, SaaS, tool)?
        branches:
          - condition: content
            next: d:framework-content
          - condition: app
            next: d:framework-app
      - id: d:framework-content
        question: What is your team's primary frontend framework experience?
        branches:
          - condition: react
            next: d:framework-content-react
          - condition: vue
            next: d:content-nuxt
          - condition: svelte
            next: d:content-sveltekit
        terminal:
          recommendation:
            - item: "Next.js SSG + MDX"
            - item: "Astro (multi-framework)"
            - item: "Nuxt Content v3"
          rationale: Content sites benefit from static generation. Next.js and Astro provide the best content-authoring experience with MDX, image optimization, and incremental static regeneration.
      - id: d:content-nuxt
        question: Do you need real-time or user-specific content?
        branches:
          - condition: "yes"
            next: d:framework-app-with-nuxt
          - condition: "no"
            next: d:content-deploy
        terminal:
          recommendation:
            - item: "Nuxt 3 + Nuxt Content"
            - item: "Prerender with routeRules"
          rationale: Nuxt Content provides file-based CMS with Vue components, ideal for Vue teams building content sites. Use prerender for fully static output or hybrid for mixed content/app pages.
      - id: d:framework-app
        question: Does the app need real-time features (websockets, live updates, collaborative editing)?
        branches:
          - condition: "yes"
            next: d:framework-realtime
          - condition: "no"
            next: d:framework-select
      - id: d:framework-realtime
        question: What is your deployment target?
        branches:
          - condition: edge
            next: d:realtime-edge
          - condition: node
            next: d:realtime-node
        terminal:
          recommendation:
            - item: "SvelteKit + WebSocket (Durable Objects)"
            - item: "Next.js + Socket.io (Node server)"
            - item: "Hono + WS (Cloudflare Workers)"
          rationale: Real-time apps need persistent connections. SvelteKit with Durable Objects or Next.js with Socket.io provide the best developer experience for bidirectional data flow. For edge, Hono + Durable Objects avoids server management.
      - id: d:realtime-edge
        terminal:
          recommendation:
            - item: "Hono + Durable Objects + Svelte"
            - item: "Next.js + Server Actions + SSE"
          rationale: Edge deployment requires connection-oriented infrastructure. Cloudflare Durable Objects provide coordinated WebSocket connections. Server-Sent Events through Next.js Server Actions offer a simpler alternative for one-way real-time updates.
      - id: d:framework-app-with-nuxt
        question: Do you need to deploy at the edge for low latency?
        branches:
          - condition: "yes"
            next: d:realtime-edge
          - condition: "no"
            next: d:realtime-node
      - id: d:content-deploy
        terminal:
          recommendation:
            - item: "Cloudflare Pages + Astro"
            - item: "Vercel + Next.js"
          rationale: Static content deploys anywhere. Cloudflare Pages offers free unlimited bandwidth. Vercel provides integrated image optimization and analytics.
      - id: d:framework-select
        question: What framework does your team know best?
        branches:
          - condition: react
            next: d:framework-app-react
          - condition: vue
            next: d:framework-app-vue
          - condition: svelte
            next: d:framework-app-svelte
      - id: d:framework-app-react
        terminal:
          recommendation:
            - item: "Next.js App Router + Server Components"
            - item: "TanStack Query + Zustand"
            - item: "shadcn/ui + Tailwind"
          rationale: Next.js App Router is the standard for React apps. Server Components reduce client JS by 30-50%. TanStack Query handles server state. shadcn/ui provides unstyled, accessible components.
      - id: d:framework-app-vue
        terminal:
          recommendation:
            - item: "Nuxt 3 + Pinia"
            - item: "Nuxt UI + Tailwind"
          rationale: Nuxt 3 is the standard for Vue apps with auto-imports, file-based routing, and the Nitro server engine. Pinia for state, Nuxt UI for components.
      - id: d:framework-app-svelte
        terminal:
          recommendation:
            - item: "SvelteKit + Svelte 5 Runes"
            - item: "shadcn-svelte + Tailwind"
          rationale: SvelteKit provides the fastest full-stack Svelte experience. Runes ($state/$derived/$effect) provide granular reactivity. shadcn-svelte ports the component model to Svelte.
extends: []
implements: []
uses:
  - concept:software-architecture
  - concept:project-planning
part_of: concept:software-engineering
solves:
  - problem:technology-selection-paralysis
  - problem:architecture-decision-framework
  - problem:tech-stack-evaluation
alternatives: []
---
The Tech Stack Decision Framework structures technology selection as a decision tree rather than a subjective preference. Each dimension of a project (team size, performance requirements, deployment target, data complexity, timeline) maps to concrete technology recommendations through the decision trees.

The decision trees are designed to be traversed either manually (for human architects) or programmatically via `atlas decide` (for AI agents). An agent can ask the user questions in sequence, build up a project profile, and output a complete technology stack recommendation with rationales for each choice.

The framework emphasizes that there is no single "best" stack — only stacks that are optimal for specific project profiles. A real-time collaborative tool needs different infrastructure than a marketing site, and a solo developer has different constraints than a 20-person team. The decision trees make these trade-offs explicit rather than implicit.
