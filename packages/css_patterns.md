---
kind: Package
id: package:css-patterns
name: Modern CSS Patterns
version: "3"
purpose: Document modern CSS patterns — Grid, Flexbox, Container Queries, Cascade Layers, custom properties, and logical properties for responsive and maintainable layouts.
problem_solved: Provides a reference for CSS layout and design techniques that replace hacks (floats, clears, magic numbers) with native, standardized patterns, enabling responsive designs that adapt to viewport, container, and user preferences without framework dependencies.
install: No installation needed — modern CSS features are native to browsers.
dependencies:
  - concept:css
  - concept:responsive-design
  - concept:web-standards
concepts:
  - name: Flexbox
    id: concept:css/flexbox
    description: "One-dimensional layout model for distributing space along a row or column. display: flex enables flexible alignment (justify-content, align-items, gap), wrapping (flex-wrap), and sizing (flex-grow, flex-shrink, flex-basis). Use for navigation bars, card rows, centering, and component-level layouts."
  - name: CSS Grid
    id: concept:css/grid
    description: "Two-dimensional layout system for rows and columns simultaneously. grid-template-columns: repeat(auto-fill, minmax(250px, 1fr)) creates responsive grids without media queries. grid-area and grid-template-areas provide visual placement. Subgrid for aligning nested grid items with parent tracks."
  - name: Container Queries
    id: concept:css/container-queries
    description: "Style elements based on their container's size rather than the viewport. container-type: inline-size, then @container (min-width: 400px) { ... }. Components can adapt to any placement width. Use for reusable widgets, sidebars, and dashboard cards that appear in multiple contexts."
  - name: Cascade Layers
    id: concept:css/layers
    description: "@layer base, components, utilities; controls the cascade order explicitly. Rules in later layers override earlier layers regardless of specificity. Solves specificity wars — developer styles override framework styles by layer, not by !important. Layers can be nested and re-ordered."
  - name: Custom Properties (CSS Variables)
    id: concept:css/custom-properties
    description: "--primary-color: #06c; define scoped variables resolved at computed time. Dynamic theming via --:root overrides. var(--spacing, 8px) with fallback. Custom properties cascade and can be changed by media queries, classes, or inline styles. Not supported in media query values."
  - name: Logical Properties
    id: concept:css/logical-properties
    description: "margin-inline-start, padding-block-end, border-inline-end replace physical directions (margin-left, padding-bottom) and adapt to writing modes. Combine with dir attributes for internationalized layouts. Future-proof for vertical writing and RTL without separate style overrides."
  - name: Aspect Ratio
    id: concept:css/aspect-ratio
    description: "aspect-ratio: 16/9 sets intrinsic aspect ratio independent of content. Before aspect-ratio, this required padding-top hacks. Combined with object-fit for responsive images and videos. Works on any element. Values can be ratios, decimals, or auto (inherit from intrinsic dimensions)."
  - name: Scroll Snap
    id: concept:css/scroll-snap
    description: "scroll-snap-type: x mandatory on container, scroll-snap-align: start on children creates snap points. Used for carousels, image galleries, and section-based scrolling without JavaScript. Supports proximity (gentler snap) and block/inline axes. Smooth D-pad navigation on mobile."
  - name: Color Functions
    id: concept:css/color-functions
    description: "color-mix(in srgb, var(--primary), white 20%) blends colors at the CSS level. oklch() and oklab() provide perceptually uniform color spaces. light-dark() returns different colors for light/dark themes based on prefers-color-scheme. HWB, LCH, and Display-P3 gamut for wider color range."
  - name: Viewport Units
    id: concept:css/viewport-units
    description: "dvh (dynamic viewport height), svh (smallest), lvh (largest) solve the mobile address bar problem that 100vh created. dvw, svw, lvw for widths. vi and vb for inline/block axis relative to writing mode. cqi/cqb for container query relative units."
  - name: Nesting
    id: concept:css/nesting
    description: "Native CSS nesting (& selector) mirrors the HTML structure: .card { & .title { font-weight: bold } }. The & refers to the parent selector. Nesting reduces repetition and improves readability. Similar to Sass nesting but with slightly different parsing rules."
apis:
  - name: "display: grid"
    id: api:css/display-grid
    signature: "display: grid; grid-template-columns: [col1] 1fr [col2] 2fr [col3] 1fr; grid-template-rows: auto 1fr auto; gap: 1rem;"
    returns: A two-dimensional grid layout.
    description: "Creates a grid container. Named grid lines enable placement: grid-column: col1 / col3. grid-auto-rows: min-content handles dynamic content. The subgrid value inherits parent track sizing for alignment."
  - name: "@container"
    id: api:css/container-query
    signature: "@container (min-width: 400px) and (max-width: 800px) { .card { flex-direction: row; } }"
    returns: Container-conditional styles.
    description: "Applies styles when the container meets size conditions. Requires container-type on the parent. container-name assigns a named container. Supports style() queries for custom property conditions: @container style(--theme: dark) { ... }."
  - name: "@layer"
    id: api:css/layer
    signature: "@layer base, components, utilities; @layer base { h1 { font-size: 2rem } } @layer utilities { .text-sm { font-size: 0.875rem } }"
    returns: Layer-ordered cascade.
    description: "Defines explicit cascade layers. Later layers override earlier ones. Unlayered styles take highest priority. @import url(…) layer(name) imports into a specific layer. Layers can be nested: @layer base.theme { ... }."
  - name: color-mix()
    id: api:css/color-mix
    signature: "color-mix(in oklch, var(--accent) 30%, var(--surface) 70%)"
    returns: A blended color value.
    description: "Mixes two colors in a given color space (srgb, oklch, hsl, etc.). The percentage controls the first color's proportion. The second color gets the remainder. Useful for hover states, theming, and creating color scales from a single primary."
  - name: container-type
    id: api:css/container-type
    signature: "container-type: inline-size; container-name: sidebar; container: sidebar / inline-size;"
    returns: A container query containment context.
    description: "Establishes a containment context for container queries. inline-size creates a query container based on inline direction (writing-mode aware). Normal-size is more restrictive (no layout queries). The container shorthand combines name and type."
sections:
  - title: Responsive Grid Layout
    id: section:css/responsive-grid
    content: |
      A responsive card grid that adapts without media queries:

      ```css
      .grid {
          display: grid;
          grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
          gap: clamp(1rem, 3vw, 2rem);
          padding: clamp(1rem, 5vw, 3rem);
      }

      .card {
          container-type: inline-size;
          display: flex;
          flex-direction: column;
      }

      /* Switch to row layout when the card container is wide enough */
      @container (min-width: 350px) {
          .card { flex-direction: row; align-items: center; }
          .card img { width: 120px; height: 120px; }
      }

      /* User preference: reduced motion */
      @media (prefers-reduced-motion: reduce) {
          *, *::before, *::after { animation-duration: 0.01ms !important; }
      }
      ```
  - title: Theming with Layers and Custom Properties
    id: section:css/theming-layers
    content: |
      Cascade layers enforce theme precedence without specificity battles:

      ```css
      @layer base, components, utilities, overrides;

      @layer base {
          :root { --color-primary: #0066cc; --color-surface: #ffffff; --space-xs: 0.25rem; }
          [data-theme="dark"] { --color-primary: #66b3ff; --color-surface: #1a1a2e; }
      }

      @layer components {
          .btn {
              background: var(--color-primary);
              padding: var(--space-xs) 1rem;
          }
      }

      @layer utilities {
          .bg-primary { background: var(--color-primary); }
          .text-sm { font-size: 0.875rem; }
      }

      @layer overrides {
          /* Override any component or utility by layer position */
          .btn-hero { background: linear-gradient(to right, var(--color-primary), #9900cc); }
      }
      ```

      Layers guarantee that overrides always win over utilities, components, and base styles — regardless of specificity.
---
