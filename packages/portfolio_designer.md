---
kind: Package
id: package:portfolio/designer
name: AI Designer Portfolio Patterns
version: "1.0"
purpose: "Document portfolio design patterns from andrewcunliffe.ai — a Senior Staff designer portfolio with Next.js, Framer Motion, Lenis scroll, and raw WebGL. Raven MCP creator, AI+HI design philosophy, agent-choreographer positioning."
problem_solved: "Reference for designing AI-focused designer portfolios — project storytelling, before/after comparisons, scroll-driven narrative, AI agent integration showcases, and personal branding for design leaders."
install: ""
dependencies:
  - package:react
  - package:nextjs
  - package:framer/motion
concepts:
  - name: Scroll-driven Narrative Design
    id: concept:portfolio/scroll-narrative
    description: "Portfolio sections unfold as the user scrolls — each project introduces itself with a number, title, and visual that transitions in with Lenis smooth scroll and Framer Motion."
  - name: Before/After Comparison Layout
    id: concept:portfolio/before-after
    description: "Draggable split-slider comparing design outputs with and without a tool or approach — same model, same brief, different results shown side by side."
  - name: AI Agent Integration Showcase
    id: concept:portfolio/ai-agent-showcase
    description: "Case studies that feature AI coding agents (Claude Code, Codex, Cursor) as collaborators — Linear tickets, agent prompts, and multi-variant exploration displayed as process artifacts."
  - name: Design Leadership Positioning
    id: concept:portfolio/design-leadership
    description: "Personal brand built around the idea of 'designer and agent choreographer' — positioning as the bottleneck for taste and direction, not hours in the chair."
  - name: Multi-project Grid Navigation
    id: concept:portfolio/project-grid
    description: "Numbered project list (01-06) with thumbnail previews and a fixed scroll bar; clicking a number scroll-snaps to that case study section."
  - name: Personal Brand Storytelling
    id: concept:portfolio/brand-storytelling
    description: "Bio section blends career narrative (eight years at Intuit, 82nd Airborne, Marin native) with a headshot and a quote that deflates self-seriousness."
  - name: Interactive Portfolio Navigation
    id: concept:portfolio/interactive-nav
    description: "Command palette (Cmd+K) for quick project lookup, fixed location/social header, and inline 'Next case study' links at the bottom of each project."
  - name: Tech Stack Transparency
    id: concept:portfolio/tech-transparency
    description: "Footer explicitly lists the tech stack used to build the portfolio (Next.js, Framer Motion, Lenis, raw WebGL) — pairing programmed with Raven MCP."
  - name: Multi-variant Exploration Display
    id: concept:portfolio/variant-exploration
    description: "Showing 3-9 design variants for a single screen as a contact sheet — demonstrates AI-generated options with human curation and selection process."
  - name: Metric-driven Storytelling
    id: concept:portfolio/metric-storytelling
    description: "Key results section per project using bold numbers with labels — 6 days from idea to TestFlight, 70 tools, 1 operator, < $5/mo subscription."
apis:
  - name: Scroll-triggered Animations
    id: api:portfolio/scroll-animations
    signature: "useScrollProgress() => { progress, direction, section }"
    returns: "Scroll progress value, scroll direction, and currently active section index."
    description: "Framer Motion scroll tracking hook that drives entrance animations, number transitions, and section-aware styling changes."
  - name: Grid Layout System
    id: api:portfolio/grid-system
    signature: "<ProjectGrid items={projects} onSelect={handleSelect} />"
    returns: "Responsive project thumbnail grid with hover preview and scroll-snap linking."
    description: "CSS Grid layout for project thumbnails that adapts from 1 column on mobile to 2 columns on desktop with numbered indicators."
  - name: Project Card Pattern
    id: api:portfolio/project-card
    signature: "<ProjectCard project={project} index={i} />"
    returns: "Animated project card with number, title, tagline, and thumbnail images."
    description: "Card component with staggered entrance animation, hover scale effect, and click-through to full case study."
  - name: Before/After Slider
    id: api:portfolio/before-after-slider
    signature: "<BeforeAfterSlider before={src} after={src} label='Before' />"
    returns: "Interactive image comparison with draggable split handle and Before/After labels."
    description: "Image comparison widget that lets users drag a slider to reveal the before image beneath the after image; used throughout case studies."
  - name: Navigation State Management
    id: api:portfolio/nav-state
    signature: "useNavState() => { activeSection, isScrolling, showCommandPalette }"
    returns: "Current section, scrolling state, and command palette visibility flag."
    description: "React context tracking which case study section is in view, whether the user is actively scrolling, and command palette open/closed state."
  - name: Theme/Aesthetic Consistency
    id: api:portfolio/theme-system
    signature: "const theme = useTheme() => { colors, typography, spacing, motion }"
    returns: "Design token object for colors, type scale, spacing units, and motion presets."
    description: "CSS custom property system for maintaining visual consistency — dark mode, brand colors, font scale, and animation curves."
  - name: Analytics Integration
    id: api:portfolio/analytics
    signature: "analytics.track(section, action, metadata)"
    returns: "void"
    description: "Discrete event tracking for scroll depth, project views, CTA clicks, and command palette usage."
  - name: CTA Flow Design
    id: api:portfolio/cta-flow
    signature: "<CTA label='Email' href='mailto:...' category='contact' />"
    returns: "Contact CTA cluster with email, LinkedIn, GitHub, and Raven MCP links."
    description: "Call-to-action layout at the bottom of each page — email link as primary CTA, social/project links as secondary, always visible in the footer."
  - name: Command Palette
    id: api:portfolio/command-palette
    signature: "<CommandPalette items={projects} onSelect={navigate} />"
    returns: "Overlay search/command palette for keyboard-driven project navigation."
    description: "Cmd+K activated overlay that lists all projects with fuzzy search; selecting one scrolls to or navigates to that case study."
  - name: Inline Image Grid
    id: api:portfolio/image-grid
    signature: "<ImageGrid images={assets} columns={mobile | desktop} />"
    returns: "Responsive image layout with captions, lightbox support, and lazy loading."
    description: "Image arrangement for case study content — single, side-by-side, or triptych layouts with optional overlay labels and asset previews."
examples:
  - id: example:portfolio/raven-mcp-showcase
    language: tsx
    description: "Raven MCP case study with before/after sliders, metric cards (6000+ installs, 70 tools), and agent session screenshots."
  - id: example:portfolio/highlvl-case-study
    language: tsx
    description: "HighLvl case study showing the 6-day build timeline, multi-variant contact sheet, agent workflow artifacts, three-theme screenshot triptych."
  - id: example:portfolio/before-after-slider
    language: tsx
    description: "Draggable image comparison used across multiple case studies — Fogline site, Oddlot workshop, Nexus product page."
  - id: example:portfolio/project-grid-nav
    language: tsx
    description: "Numbered project grid (01-06) on the home page with scroll-snap linking and hover thumbnail previews."
  - id: example:portfolio/command-palette
    language: tsx
    description: "Cmd+K command palette overlay listing all projects with fuzzy search and instant scroll navigation."
failures:
  - id: failure:portfolio/scroll-jank
    symptom: "Scroll-driven animations stutter or skip frames; sections snap unpredictably."
    cause: "Heavy Framer Motion animations running on the main thread without scroll debouncing or layout thrashing."
    fix: "Use Lenis for smooth scroll, debounce scroll handlers, animate with transform/opacity only, and profile with DevTools performance tab."
  - id: failure:portfolio/image-overload
    symptom: "Page load is slow; layout shifts as high-res images load; Lighthouse score drops."
    cause: "Multiple full-width high-resolution images loading simultaneously without lazy loading, proper sizing, or aspect-ratio containment."
    fix: "Use next/image with explicit width/height, lazy loading below the fold, WebP/AVIF formats, and proper aspect-ratio CSS."
  - id: failure:portfolio/weak-project-narrative
    symptom: "Visitors scroll past projects without understanding the impact or the designer's role in the work."
    cause: "Case studies show only final screenshots without process context, metric of impact, or the designer's specific contribution."
    fix: "Lead each case study with a metric summary (6 days, 70 tools, 1 operator), show process artifacts (Linear tickets, agent prompts), and state the designer's role explicitly."
  - id: failure:portfolio/no-clear-cta
    symptom: "Visitors land, scroll, and leave without contacting the designer."
    cause: "Contact information is buried in a footer or missing alt-text CTA; no 'let's work' prompt at natural scroll-terminus points."
    fix: "Surface email and social links as a persistent footer element; add 'Next case study' links between projects; include a prominent 'let's work' section at the end."
  - id: failure:portfolio/mobile-animation-breakage
    symptom: "Before/after sliders and scroll-triggered animations fail or feel broken on mobile devices."
    cause: "Touch events are not handled for slider interaction; scroll animations rely on wheel events that mobile browsers fire differently."
    fix: "Add touch event handlers for slider drag on mobile; test scroll animations on actual iOS/Android devices; reduce animation complexity on low-power devices."
extends: []
uses:
  - concept:portfolio/scroll-narrative
  - concept:portfolio/before-after
  - concept:portfolio/ai-agent-showcase
part_of: concept:domain/portfolio-design
depends_on:
  - package:react
  - package:nextjs
  - package:framer/motion
solves:
  - problem:designer-portfolio-standout
alternatives:
  - package:squarespace
  - package:webflow
---
# AI Designer Portfolio Patterns

Andrew Cunliffe's portfolio at andrewcunliffe.ai redefines the designer portfolio format for the AI era. Built with Next.js, Framer Motion, Lenis smooth scroll, and raw WebGL, it positions the designer as an 'agent choreographer' — someone who directs AI coding agents rather than writing every line of code themselves.

The portfolio opens with a scroll-driven narrative: the bio section blends career highlights (eight years at Intuit, 82nd Airborne veteran, Marin native) with a headshot and a self-deprecating quote that deflates the serious tone. Below, six numbered case studies (01-06) form a scroll-snapping project grid. Each case study opens into a full-width section with metric cards, before/after comparison sliders, and process artifacts.

The Raven MCP case study is the centerpiece. It features before/after sliders (draggable image comparison showing the same design brief with and without Raven), metric cards showing 6000+ installs and 70 tools across 9 knowledge layers, and screenshots of the MCP being used inside coding agents. The HighLvl case study shows the 6-day build timeline, a multi-variant contact sheet of design options generated by agents, and a three-theme screenshot triptych.

The command palette (Cmd+K) provides keyboard-driven project navigation with fuzzy search. A fixed header shows location (Marin County) and current time. Each project page ends with a 'Next case study' link and a persistent footer cluster of email, LinkedIn, GitHub, and Raven MCP links. The footer explicitly states 'Hand-coded with Next.js, Framer Motion, Lenis, and raw WebGL' and 'Pair programmed with Raven MCP' — tech stack transparency as a branding tool.
