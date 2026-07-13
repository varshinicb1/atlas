---
kind: Package
id: package:responsive-design
name: Responsive Design Patterns
version: "3"
purpose: Document responsive design patterns — mobile-first workflow, breakpoints, fluid typography, adaptive layouts, touch targets, and performance optimization across devices.
problem_solved: Provides a systematic approach to building interfaces that work across phones, tablets, laptops, and large screens without separate codebases, eliminating common responsive failures like overlapping elements, unreadable text, and broken navigation on small viewports.
install: No installation needed — these are CSS and design patterns.
dependencies:
  - concept:css
  - concept:ui-design
  - concept:web-standards
concepts:
  - name: Mobile-First
    id: concept:responsive/mobile-first
    description: "Designing for the smallest screen first, then enhancing with media queries for larger screens. The base CSS targets mobile (single-column, stacked layout), and min-width media queries progressively add multi-column, sidebars, and richer layouts. Ensures the mobile experience is never an afterthought."
  - name: Breakpoints
    id: concept:responsive/breakpoints
    description: "Viewport width thresholds where layout adapts. Common breakpoints: sm (640px, phones landscape), md (768px, tablets), lg (1024px, laptops), xl (1280px, desktops), 2xl (1536px, large screens). Choose breakpoints based on content, not device sizes. Use CSS custom properties for consistent breakpoint references."
  - name: Fluid Typography
    id: concept:responsive/fluid-typography
    description: "Type that scales smoothly between viewport sizes without breakpoint jumps. CSS clamp(): font-size: clamp(1rem, 2.5vw, 2rem). The minimum, preferred (fluid), and maximum values create a continuous scale. Viewport units (vw, cqi) combined with rem create predictable scaling."
  - name: Fluid Spacing & Sizing
    id: concept:responsive/fluid-spacing
    description: "Padding, margins, and widths that scale with the viewport using clamp() or min()/max(). padding: clamp(1rem, 3vw, 3rem). Width: min(100%, 1200px) for max-width without a separate max-width property. gap: clamp(0.5rem, 2vw, 2rem) for responsive gutters."
  - name: Adaptive Navigation
    id: concept:responsive/navigation
    description: "Navigation patterns that change with viewport: hamburger menu (mobile), horizontal nav (tablet+), mega-menu (desktop). The <nav> element + aria attributes for accessibility. Prioritize content: hide secondary links on mobile, show core actions (search, cart) as icons with labels hidden."
  - name: Touch Targets
    id: concept:responsive/touch-targets
    description: "Minimum 44x44px touch targets for all interactive elements on mobile (Apple HIG, Material Design). Use padding or min-width/height to hit the target size even when the visible element is smaller. Add touch-action: manipulation to eliminate 300ms tap delay."
  - name: Responsive Images
    id: concept:responsive/images
    description: "<picture> element with <source media> for art direction, srcset with w descriptors for resolution switching. Provide multiple image sizes and let the browser choose based on viewport and pixel density. loading='lazy' for below-fold images. aspect-ratio to prevent layout shifts."
  - name: Container Queries
    id: concept:responsive/container-queries
    description: "Components adapt to their container's width, not the viewport. container-type: inline-size on parent, @container (min-width: 400px) for child styles. Enables truly reusable responsive components that work in any layout context — sidebar, main content, or modal."
  - name: Data Tables on Mobile
    id: concept:responsive/tables
    description: "Wide tables on narrow screens: horizontal scroll (overflow-x: auto), card layout (convert rows to stacked cards with role='cell' labels), or priority columns (hide less important columns via media queries). The <template> approach hides column headers on mobile and shows inline labels."
  - name: Print Styles
    id: concept:responsive/print
    description: "@media print { ... } removes backgrounds, hides navigation/sidebars, expands URLs, and ensures readable contrast. Use @page for margin control. Avoid floats/cols in print. Display href text after links: a[href]:after { content: ' (' attr(href) ')' }."
  - name: Performance Patterns
    id: concept:responsive/performance
    description: "Responsive performance: loading appropriate assets per device. Conditional JS (matchMedia), debounced resize handlers, passive scroll listeners. CSS containment (contain: layout style paint) isolates off-screen sections. Serving smaller images to mobile reduces bandwidth by 60-80%."
apis:
  - name: "@media queries"
    id: api:responsive/media-queries
    signature: "@media (min-width: 768px) and (max-width: 1023px), (orientation: portrait) { .sidebar { display: none } }"
    returns: Conditional CSS block.
    description: "Applies styles based on viewport characteristics: width, height, orientation, aspect-ratio, resolution, hover capability, pointer type, and prefers-color-scheme. Combine multiple conditions with and, or, and not. Use min-width for mobile-first, max-width for desktop-first strategies."
  - name: clamp()
    id: api:responsive/clamp
    signature: "clamp(minimum: 1rem, preferred: 2.5vw, maximum: 3rem)"
    returns: A clamped CSS value.
    description: "Returns a value between the minimum and maximum, scaling with the preferred value. The preferred value is often viewport-relative (vw, cqi) or a percentage. When the viewport is small, the minimum applies; large, the maximum. In between, the value scales linearly."
  - name: srcset + sizes
    id: api:responsive/srcset
    signature: "<img src='photo-800.jpg' srcset='photo-400.jpg 400w, photo-800.jpg 800w, photo-1200.jpg 1200w' sizes='(max-width: 600px) 100vw, (max-width: 1200px) 50vw, 33vw' alt=''>"
    returns: The optimal image source for the viewport.
    description: "srcset lists image URLs with their intrinsic widths (w descriptor) or pixel density (x descriptor). sizes tells the browser what display width the image will occupy at different viewports. The browser selects the smallest appropriate source."
  - name: "@container"
    id: api:responsive/container-query
    signature: "@container sidebar (min-width: 300px) { .card { grid-template-columns: 1fr 1fr; } }"
    returns: Container-conditional styles.
    description: "Applies styles when the nearest named container meets width/height conditions. Requires contain: layout inline-size on the container. Supports style() queries for custom property conditions. Use for reusable components that change layout based on available space."
  - name: <picture> element
    id: api:responsive/picture
    signature: "<picture> <source media='(min-width: 1200px)' srcset='hero-xl.webp' type='image/webp'> <source media='(min-width: 768px)' srcset='hero-md.webp'> <img src='hero-fallback.jpg' alt=''> </picture>"
    returns: The most appropriate image source.
    description: "Provides multiple image sources for different viewports, formats (WebP, AVIF), and resolutions. The browser selects the first matching <source>. Always include a fallback <img> for browsers that do not support <picture>."
sections:
  - title: Mobile-First Layout
    id: section:responsive/mobile-first-layout
    content: |
      Build a responsive page layout using mobile-first methodology:

      ```css
      /* Base: mobile (single column, stacked) */
      .page {
          display: grid;
          gap: 1rem;
          padding: clamp(1rem, 3vw, 2rem);
      }
      .sidebar { display: none; } /* Hidden on mobile */
      .nav { display: flex; flex-direction: column; }
      .cards { display: grid; gap: 1rem; }

      /* Tablet: show sidebar, horizontal nav */
      @media (min-width: 768px) {
          .page { grid-template-columns: 240px 1fr; }
          .sidebar { display: block; }
          .nav { flex-direction: row; gap: 1rem; }
          .cards { grid-template-columns: repeat(2, 1fr); }
      }

      /* Desktop: wider layout, 3-column cards */
      @media (min-width: 1024px) {
          .page { grid-template-columns: 280px 1fr 300px; }
          .cards { grid-template-columns: repeat(3, 1fr); }
      }
      ```
  - title: Fluid Typography System
    id: section:responsive/fluid-typography
    content: |
      Create a type scale that adapts smoothly to every viewport:

      ```css
      :root {
          --text-sm: clamp(0.75rem, 1vw, 0.875rem);
          --text-base: clamp(1rem, 1.5vw, 1.125rem);
          --text-lg: clamp(1.125rem, 2vw, 1.375rem);
          --text-xl: clamp(1.25rem, 2.5vw, 1.75rem);
          --text-2xl: clamp(1.5rem, 3.5vw, 2.5rem);
          --text-3xl: clamp(2rem, 5vw, 3.5rem);
          --leading-tight: 1.15;
          --leading-normal: 1.5;
      }

      h1 { font-size: var(--text-3xl); line-height: var(--leading-tight); }
      h2 { font-size: var(--text-2xl); }
      body { font-size: var(--text-base); line-height: var(--leading-normal); }
      small { font-size: var(--text-sm); }
      ```

      The fluid type scale uses clamp() with a formula: minimum = 1rem at 320px, preferred = linear interpolation, maximum = large size at 1200px+. This eliminates breakpoint-dependent font size jumps.
---
