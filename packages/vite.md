---
kind: Package
id: package:vite
name: Vite Build Tool
version: "6"
purpose: Document Vite patterns — dev server, build configuration, plugins, code splitting, environment variables, and SSR for modern web development.
problem_solved: Provides a fast build tool and dev server that uses native ES modules for instant server start and Hot Module Replacement (HMR), eliminating the slow bundling cycles of Webpack-based toolchains during development.
install: npm create vite@latest
dependencies:
  - concept:javascript
  - concept:bundling
  - concept:node-js
concepts:
  - name: Dev Server
    id: concept:vite/dev-server
    description: "Vite serves source files as native ES modules over HTTP during development. No bundling happens — the browser imports modules directly. This enables sub-second server start regardless of project size. Cold starts are instant because only the entry module is transformed on first load."
  - name: Hot Module Replacement (HMR)
    id: concept:vite/hmr
    description: "When a file changes, Vite sends only the updated module to the browser, preserving application state. HMR boundaries are per-module. React, Vue, and Svelte have dedicated HMR handlers. Frame-perfect updates without page reload. Falls back to full reload when HMR boundaries cannot be determined."
  - name: Build (Rollup)
    id: concept:vite/build
    description: "Production builds use Rollup for tree-shaking, code splitting, and minification. vite build produces optimized static assets in dist/. Output includes hashed filenames for cache busting, CSS extraction, and preload directives generation. lib mode builds libraries for npm publishing."
  - name: Plugins
    id: concept:vite/plugins
    description: "Vite plugins extend the dev server and build pipeline. The plugin API is Rollup-compatible with Vite-specific hooks. Official plugins: @vitejs/plugin-react (with Fast Refresh), @vitejs/plugin-vue, @vite/plugin-legacy (legacy browser support). Community plugins for everything from WASM to PWA."
  - name: CSS & Preprocessors
    id: concept:vite/css
    description: "Built-in CSS handling with PostCSS, CSS modules (.module.css), and preprocessor support (Sass, Less, Stylus). CSS imported in JS is hot-reloaded. PostCSS config is auto-detected from postcss.config.js. CSS code splitting extracts per-chunk CSS in production builds."
  - name: TypeScript
    id: concept:vite/typescript
    description: "Vite transpiles TypeScript using esbuild (no type checking — 20-30x faster than tsc). Type checking is delegated to an external process (tsc --noEmit or vue-tsc). Supports tsconfig paths resolution. Decoration support, JSX transform, and isolatedModules semantics."
  - name: Environment Variables
    id: concept:vite/env-vars
    description: "VITE_ prefix variables from .env files are exposed to client code via import.meta.env. .env.development, .env.production, .env.local for different modes. Mode defaults to 'development' for dev and 'production' for build. Custom modes via --mode flag."
  - name: Code Splitting
    id: concept:vite/code-splitting
    description: "Dynamic import() is the primary splitting mechanism — Vite automatically creates separate chunks for dynamically imported modules. build.rollupOptions.output.manualChunks for manual splitting (vendor chunks, framework chunks). Async routes load on demand, reducing initial bundle size."
  - name: Asset Handling
    id: concept:vite/assets
    description: "Imported assets (images, fonts, JSON, WASM) return their public URL or inlined content. Small assets (< 4KB) are inlined as base64. Public directory (public/) serves files at root without processing. SVG imports can be URLs or inline strings depending on the query (?url, ?raw)."
  - name: Server-Side Rendering
    id: concept:vite/ssr
    description: "Vite provides SSR primitives: ssrLoadModule() and ssrRewriteStacktrace(). The SSR module transforms ESM for Node.js consumption. Frameworks like Nuxt, Astro, and SvelteKit use Vite's SSR. Manual SSR setup is possible with vite build --ssr for Node.js entry bundles."
  - name: Proxy
    id: concept:vite/proxy
    description: "Dev server proxy for API requests. server.proxy: { '/api': { target: 'http://localhost:3000', changeOrigin: true, rewrite: path => path.replace(/^\\/api/, '') } }. Handles WebSocket proxying. Supports bypass, configure, and cookie domain rewrite."
  - name: Optimize Dependencies
    id: concept:vite/optimize-deps
    description: "Vite pre-bundles dependencies using esbuild on first start. Converts CommonJS to ESM, flattens dependency trees, and caches the result in node_modules/.vite. When dependencies change (new npm install), Vite auto-invalidates the cache. opt-out with optimizeDeps.exclude."
apis:
  - name: defineConfig()
    id: api:vite/define-config
    signature: "defineConfig(config: UserConfig | (env) => UserConfig): UserConfig"
    returns: A typed Vite configuration.
    description: "Helper for config typing and auto-completion. Accepts an object or a function receiving { mode, command, ssrBuild }. Command is 'serve' (dev) or 'build'. Common fields: plugins, build, resolve, server, css, optimizeDeps, envPrefix."
  - name: import.meta.env
    id: api:vite/import-meta-env
    signature: "import.meta.env: { MODE: string, BASE_URL: string, PROD: boolean, DEV: boolean, SSR: boolean, VITE_*: string }"
    returns: Environment variables object.
    description: "Access to compile-time environment variables. MODE is 'development' or 'production' by default. VITE_ prefixed variables are exposed. Custom env vars can be accessed. Use it for API URLs, feature flags, and runtime configuration."
  - name: Glob Import
    id: api:vite/glob-import
    signature: "const modules = import.meta.glob('./pages/*.tsx', { eager: false, query: '?raw', import: 'default' })"
    returns: A record mapping file paths to import functions or modules.
    description: "Bulk imports matching files. With eager: true, returns loaded modules synchronously. Without eager, returns lazy import functions. query appends URL params. import selects a specific export. Routes can be auto-registered from file patterns."
  - name: build.rollupOptions.output
    id: api:vite/rollup-output
    signature: "build: { rollupOptions: { output: { manualChunks: (id, { getModuleInfo }) => string | void, entryFileNames: string, chunkFileNames: string, assetFileNames: string } } }"
    returns: Build output configuration.
    description: "Customizes the Rollup output. manualChunks defines custom splitting (vendor: /node_modules/). entryFileNames/chunkFileNames/assetFileNames control output filenames with [name], [hash], and [ext] placeholders."
  - name: optimizeDeps.include
    id: api:vite/optimize-deps-include
    signature: "optimizeDeps: { include: string[], exclude: string[], esbuildOptions: {...} }"
    returns: Dependency optimization configuration.
    description: "Controls which dependencies are pre-bundled. include forces deps to be pre-bundled (useful for CommonJS deps Vite doesn't detect). exclude keeps deps as native ESM (speeds up cold start for large deps). esbuildOptions passes config to the esbuild bundler."
sections:
  - title: Project Configuration
    id: section:vite/config
    content: |
      Typical vite.config.ts for a React + TypeScript project:

      ```typescript
      import { defineConfig } from 'vite';
      import react from '@vitejs/plugin-react';
      import path from 'path';

      export default defineConfig({
          plugins: [react()],
          resolve: {
              alias: {
                  '@': path.resolve(__dirname, './src'),
              },
          },
          server: {
              port: 3000,
              proxy: {
                  '/api': {
                      target: 'http://localhost:8080',
                      changeOrigin: true,
                  },
              },
          },
          build: {
              rollupOptions: {
                  output: {
                      manualChunks: {
                          vendor: ['react', 'react-dom'],
                          ui: ['@radix-ui/react-dialog', '@radix-ui/react-dropdown-menu'],
                      },
                  },
              },
              sourcemap: true,
              minify: 'terser',
              terserOptions: { compress: { drop_console: true } },
          },
      });
      ```
  - title: Plugin Development
    id: section:vite/plugins
    content: |
      A simple Vite plugin that injects a build timestamp:

      ```typescript
      import { Plugin } from 'vite';

      function buildTimestampPlugin(): Plugin {
          let timestamp: string;

          return {
              name: 'build-timestamp',
              apply: 'build', // Only during build, not dev
              buildStart() {
                  timestamp = new Date().toISOString();
              },
              transformIndexHtml(html) {
                  return html.replace(
                      '</head>',
                      `<meta name="build-timestamp" content="${timestamp}"></head>`
                  );
              },
              // Vite-specific hook
              configureServer(server) {
                  server.middlewares.use((req, res, next) => {
                      console.log(`[${new Date().toISOString()}] ${req.method} ${req.url}`);
                      next();
                  });
              },
          };
      }
      ```

      Vite plugins use a Rollup-compatible API plus Vite-specific hooks (configureServer, handleHotUpdate, transformIndexHtml). The name must be unique.
---
