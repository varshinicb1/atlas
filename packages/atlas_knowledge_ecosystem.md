---
kind: Package
id: package:atlas-knowledge-ecosystem
name: Atlas Knowledge Ecosystem
version: "1.0"
purpose: A meta-package that maps the entire Atlas knowledge graph — connecting all 35+ library packages, decision frameworks, and design patterns into a unified web development ontology for AI agents and human developers.
problem_solved: Connects isolated library packages into a navigable knowledge graph where an AI agent can reason across frameworks — from choosing a tech stack through deployment — by traversing edges between complementary packages (tech_stack_decisions → api_design → deployment_patterns → specific framework packages).
install: atlas solve atlas-knowledge-ecosystem.atlas "Map the react ecosystem"
dependencies:
  - concept:knowledge-engineering
  - concept:software-architecture
concepts:
  - name: Ecosystem Layers
    id: concept:atlas-ecosystem/layers
    description: "The Atlas knowledge graph organizes web development into five layers: (1) Foundation — languages, platforms, paradigms (TypeScript, JavaScript, Web Platform); (2) Frameworks — full-stack meta-frameworks (Next.js, Nuxt, SvelteKit, Remix, Astro); (3) Libraries — tool-specific packages (Zod, Prisma, TanStack Query, tRPC); (4) Infrastructure — deployment, CI/CD, security (Deployment Patterns, Arcjet, Docker); (5) Decisions — decision trees that connect layers (Tech Stack Decisions, API Design)."
  - name: Framework-Library Mapping
    id: concept:atlas-ecosystem/framework-map
    description: Every framework package (Next.js, SvelteKit, Nuxt, Remix) links to compatible library packages. Next.js → tRPC, Prisma/Drizzle, Zod, TanStack Query, shadcn/ui, Clerk, UploadThing. SvelteKit → Prisma/Drizzle, Lucia, Zod, Svelte stores, shadcn-svelte. Nuxt → Pinia, Nuxt UI, Prisma/Drizzle, Zod. This mapping enables agents to recommend complete, compatible stacks.
  - name: Cross-Package Edges
    id: concept:atlas-ecosystem/cross-edges
    description: The ecosystem package adds edges between complementary packages — Zod → tRPC (tRPC uses Zod for input validation), Prisma ↔ Zod (drizzle-zod generates Zod schemas from DB models), TanStack Query ↔ tRPC (tRPC uses TanStack Query under the hood in React), Clerk → Lucia (both solve auth, alternative pattern), Motion ↔ GSAP (both solve animation, different levels).
  - name: Decision-to-Package Mapping
    id: concept:atlas-ecosystem/decision-map
    description: Each terminal recommendation in Tech Stack Decisions maps to specific Atlas packages. "Use Next.js App Router" → typescript_nextjs package. "Use Prisma" → prisma package. "Use Zod for validation" → zod package. Agents traversing decision trees can automatically load the recommended packages' knowledge for implementation details.
  - name: Rapid Application Generation
    id: concept:atlas-ecosystem/rapid-gen
    description: The combined knowledge graph enables multi-package queries — "Build a full-stack app with Next.js, Prisma, tRPC, Clerk, and shadcn/ui" traverses typescript_nextjs → prisma → trpc → clerk → shadcn_ui → zod → api_design → deployment_patterns, producing a complete architecture with implementation details for each layer.
  - name: Knowledge Coverage Map
    id: concept:atlas-ecosystem/coverage
    description: Current coverage spans 14 technology domains — Frontend Frameworks (Next.js, Svelte, Nuxt, Remix, Astro), UI Components (shadcn/ui, Tailwind), State Management (Zustand, TanStack Query, Pinia), Validation (Zod), ORM/Database (Prisma, Drizzle), Auth (Clerk, Lucia, NextAuth), API Layer (tRPC, Hono, REST patterns), Testing (Playwright, Vitest), Animation (GSAP, Motion, Framer Motion, Anime.js), Security (Arcjet, Turnstile), Mobile (React Native, Capacitor), Tooling (Biome, TypeScript), Deployment (Deployment Patterns, Docker), Design (API Design, System Architecture).
decision_trees:
  - id: decision:build-app
    trigger:
      intent: build-fullstack-app
      domain: web-development
      tags: [architecture, fullstack, quickstart]
    root: d:build-start
    nodes:
      - id: d:build-start
        question: What type of application are you building?
        branches:
          - condition: saas-dashboard
            next: d:build-saas
          - condition: content-site
            next: d:build-content
          - condition: api-service
            next: d:build-api
          - condition: mobile-app
            next: d:build-mobile
      - id: d:build-saas
        terminal:
          recommendation:
            - item: "Frontend: Next.js App Router + shadcn/ui + Tailwind"
            - item: "Data Layer: Prisma/Drizzle ORM + PostgreSQL"
            - item: "Validation: Zod"
            - item: "API: tRPC + TanStack Query"
            - item: "Auth: Clerk"
            - item: "File Uploads: UploadThing"
            - item: "Deployment: Vercel (frontend) + Railway (DB)"
            - item: "Security: Arcjet (rate limit + bot detection)"
          rationale: The most productive stack for SaaS dashboards. Next.js Server Components for initial load, tRPC for type-safe API calls, Prisma for type-safe DB access, Clerk for auth (components handle login/signup/MFA), shadcn/ui for consistent UI. All packages are in the Atlas registry for inline knowledge access.
      - id: d:build-content
        terminal:
          recommendation:
            - item: "Framework: Next.js SSG or Astro"
            - item: "CMS: MDX + Contentlayer or Nuxt Content"
            - item: "UI: Tailwind + shadcn/ui"
            - item: "Animation: Motion or GSAP"
            - item: "Deployment: Cloudflare Pages or Vercel"
            - item: "Search: Algolia or Meilisearch"
          rationale: Content sites prioritize fast builds and global CDN delivery. Astro islands for interactivity, SSG for zero-cost scaling. Motion for scroll-triggered animations, GSAP for complex timeline animations. Cloudflare Pages for free global distribution.
      - id: d:build-api
        terminal:
          recommendation:
            - item: "Framework: Hono (edge) or tRPC (full-stack)"
            - item: "Validation: Zod"
            - item: "Database: Prisma/Drizzle + PostgreSQL"
            - item: "Auth: Lucia (self-hosted) or JWT"
            - item: "Rate Limiting: Arcjet"
            - item: "Documentation: OpenAPI (Hono) or auto (tRPC)"
            - item: "Deployment: Cloudflare Workers (edge)"
          rationale: APIs prioritize performance and type safety. Hono for edge-deployed REST APIs, tRPC for full-stack TypeScript apps. Zod validates at the boundary. Arcjet handles rate limiting and bot detection. Cloudflare Workers for sub-50ms global latency.
      - id: d:build-mobile
        terminal:
          recommendation:
            - item: "Framework: Expo (React Native) or Capacitor (web-based)"
            - item: "State: Zustand + TanStack Query"
            - item: "Navigation: React Navigation (Expo) or file-based (Capacitor)"
            - item: "Auth: Clerk (supports mobile SDKs)"
            - item: "API: tRPC client or REST + Zod validation"
            - item: "Deployment: EAS Build (Expo) or App Store + Play Store (Capacitor)"
          rationale: Expo for React-based mobile apps with native features, Capacitor for wrapping existing web apps. Zustand for lightweight client state, TanStack Query for API caching. Use Expo's managed workflow for faster development, bare workflow for custom native modules.
extends: []
implements: []
uses:
  - concept:knowledge-engineering
  - concept:software-architecture
part_of: concept:atlas-knowledge
solves:
  - problem:atlas-package-navigation
  - problem:technology-compatibility-mapping
  - problem:full-stack-architecture-generation
alternatives: []
---
The Atlas Knowledge Ecosystem is a meta-package that does not contain library-specific knowledge — instead, it connects all other packages through a unified ontology. An AI agent using `atlas solve` on this package can answer questions that span multiple technology domains and reference the specific packages needed for implementation details.

The five-layer model (Foundation → Frameworks → Libraries → Infrastructure → Decisions) enables queries at any abstraction level. A query like "What's the best way to handle forms?" resolves through the Decisions layer (framework selection) → Frameworks layer (Next.js forms via Server Actions) → Libraries layer (Zod validation, shadcn/ui Form component) → Infrastructure layer (Arcjet for spam protection).

For rapid application generation, agents traverse the Build App decision tree and then load each recommended package's knowledge sequentially. The decision tree ensures compatible technology choices are made together — for example, choosing tRPC automatically brings Zod (required by tRPC for input validation) and TanStack Query (used by tRPC React client under the hood).
