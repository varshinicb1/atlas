---
kind: Package
id: package:tailwind
name: Tailwind CSS Patterns
version: "4"
purpose: Document Tailwind CSS patterns — utility-first workflow, responsive design, theming, component extraction, and optimization
problem_solved: Provides a reference for Tailwind CSS's utility-first workflow, responsive breakpoints, dark mode, custom theming, and build optimization, reducing CSS bloat, inconsistent spacing, and slow dev builds.
install: npm install tailwindcss @tailwindcss/vite
dependencies:
  - package:react
  - package:npm
concepts:
  - name: Utility-First
    id: concept:tailwind/utility-first
    description: Building UIs from hundreds of composable single-purpose classes (flex, text-center, p-4) instead of writing custom CSS.
  - name: Responsive Design
    id: concept:tailwind/responsive
    description: "Prefixing utilities with breakpoint variants (sm:, md:, lg:, xl:, 2xl:) to apply styles conditionally at different viewport widths."
  - name: Dark Mode
    id: concept:tailwind/dark-mode
    description: "Using the dark: variant to apply styles when the user's system theme or a toggled class matches dark mode."
  - name: Custom Theme
    id: concept:tailwind/custom-theme
    description: Extending the default design tokens through tailwind.config or CSS-based theme() references for brand-specific colors, fonts, and spacing.
  - name: Layers
    id: concept:tailwind/layers
    description: The @tailwind directives (base, components, utilities) that control CSS output order and specificity for different concerns.
  - name: Plugins
    id: concept:tailwind/plugins
    description: Registering third-party or custom utilities via the Tailwind plugin system, adding new variants and utilities programmatically.
  - name: Component Extraction
    id: concept:tailwind/component-extraction
    description: Extracting repeated utility patterns into reusable components (React, Vue) or using @apply in a component class for shared styles.
  - name: Arbitrary Values
    id: concept:tailwind/arbitrary-values
    description: Using square bracket syntax (w-[356px], bg-[#1da1f2]) for one-off values that are not in the design system.
  - name: Animations
    id: concept:tailwind/animations
    description: Tailwind's built-in animation utilities (animate-spin, animate-pulse, animate-bounce) with configurable keyframes and durations.
  - name: Optimization
    id: concept:tailwind/optimization
    description: Removing unused CSS via JIT compilation, purging in production builds, and keeping the output as small as the classes used in templates.
  - name: Container Queries
    id: concept:tailwind/container-queries
    description: Using the @tailwindcss/container-queries plugin to style elements based on their container size rather than the viewport.
  - name: Design Tokens
    id: concept:tailwind/design-tokens
    description: A centralized set of values (colors, spacing, typography) defined in the Tailwind config and consumed as CSS custom properties or utility classes.
apis:
  - name: '@tailwind'
    id: api:tailwind/at-tailwind
    signature: "@tailwind base; @tailwind components; @tailwind utilities"
    returns: Injected CSS directives.
    description: Directives that inject Tailwind's base styles, component classes, and utility classes into your CSS entry point.
  - name: '@apply'
    id: api:tailwind/apply
    signature: ".btn { @apply bg-blue-500 text-white font-bold py-2 px-4 rounded; }"
    returns: Compiled utility class styles.
    description: Inlines Tailwind utilities into custom CSS selectors; useful for abstracting repeated patterns but discouraged for new projects.
  - name: '@layer'
    id: api:tailwind/layer
    signature: "@layer components { .card { ... } }"
    returns: Layer-ordered CSS rules.
    description: Wraps custom CSS into Tailwind's layer system (base, components, utilities) for predictable specificity ordering.
  - name: theme()
    id: api:tailwind/theme
    signature: ".foo { color: theme('colors.blue.500'); }"
    returns: A CSS-in-JS reference to a design token.
    description: A CSS function that resolves to a value from the Tailwind theme config, enabling inline style reuse of design tokens.
  - name: config
    id: api:tailwind/config
    signature: "export default { theme: { extend: { colors: { brand: '#ff0000' } } } }"
    returns: A Tailwind configuration object.
    description: The configuration file (tailwind.config) that extends the default design system, adds plugins, and customizes variants.
  - name: screens
    id: api:tailwind/screens
    signature: "screens: { tablet: '640px', laptop: '1024px' }"
    returns: A custom breakpoint map.
    description: "Defines responsive breakpoints in config that become available as variant prefixes (tablet:, laptop:)."
  - name: plugins
    id: api:tailwind/plugins-api
    signature: "plugins: [require('@tailwindcss/forms'), require('daisyui')]"
    returns: An array of plugin registrations.
    description: Registers official and third-party plugins that add utilities, components, or variants to Tailwind.
  - name: variants
    id: api:tailwind/variants
    signature: "variants: { extend: { backgroundColor: ['active'] } }"
    returns: Extended variant definitions.
    description: Adds new pseudo-class or media-query variants to existing utilities, enabling state-specific styles.
  - name: daisyUI
    id: api:tailwind/daisyui
    signature: "plugins: [require('daisyui')]"
    returns: A Tailwind plugin with prebuilt UI components.
    description: A popular Tailwind plugin that adds component classes (btn, card, modal) built entirely from utility classes.
  - name: dark variant
    id: api:tailwind/dark-variant
    signature: "<div class='bg-white dark:bg-gray-900 text-black dark:text-white'>"
    returns: A dark mode conditional class.
    description: "The dark: prefix applies styles only when the parent has a .dark class or the system theme matches dark mode."
  - name: arbitrary values
    id: api:tailwind/arbitrary-values-api
    signature: "class='w-[calc(100%-2rem)] bg-[#1da1f2] top-[117px]'"
    returns: A one-off utility class.
    description: Square bracket syntax for values outside the design system; supports calc, CSS variables, and any valid CSS value.
  - name: container queries
    id: api:tailwind/container-queries-api
    signature: "class='@container @md:flex @[400px]:text-lg'"
    returns: Container-query-aware utility classes.
    description: The @ prefix enables container query variants that respond to the container's width rather than the viewport.
examples:
  - id: example:tailwind/responsive-card
    language: html
    description: "A card component that renders single-column on mobile and row on desktop using sm: and md: prefixes."
  - id: example:tailwind/dark-mode-toggle
    language: html
    description: A page with dark mode classes toggled by a JavaScript-controlled class on the html element.
  - id: example:tailwind/custom-theme-button
    language: html
    description: A button using custom brand colors from an extended tailwind.config theme.
  - id: example:tailwind/container-query-grid
    language: html
    description: A grid of cards using container queries to change layout as the container shrinks or grows.
  - id: example:tailwind/arbitrary-gradient
    language: html
    description: A gradient background built with arbitrary values for precise color stops.
failures:
  - id: failure:tailwind/css-purge
    symptom: "Classes work in development but disappear in production builds."
    cause: Tailwind's JIT engine only generates classes that appear as complete unbroken strings in source files. Dynamic class construction (className={'p-' + size}) is not detected.
    fix: Use full class names in source files; use the safelist option in config to include dynamically constructed classes; prefer conditional class objects to string concatenation.
  - id: failure:tailwind/specificity-war
    symptom: "Tailwind utilities are overridden by other CSS, or vice versa; styles do not apply as expected."
    cause: Custom CSS without proper layer ordering has higher specificity than Tailwind utilities; @import or external stylesheets override framework utilities.
    fix: Put custom CSS in the correct @layer (components or utilities); avoid !important; keep external CSS in the lowest layer or use Tailwind's prefix option.
  - id: failure:tailwind/apply-overuse
    symptom: "Custom CSS files grow large; @apply blocks become hard to maintain and dev builds slow down."
    cause: Using @apply excessively instead of extracting components in the UI framework (React/Vue components); @apply hides utility patterns and bloats output.
    fix: Favor component extraction in the framework layer; reserve @apply for utility libraries or when working in a non-component context like WordPress themes.
  - id: failure:tailwind/missing-dark-mode
    symptom: "Dark mode classes do not apply when the system theme changes or the toggle is flipped."
    cause: Tailwind's dark mode defaults to media-based (system theme only); with class-based strategy, the .dark class parent is missing or not toggled.
    fix: "Set darkMode: 'class' in config; add a script that toggles the .dark class on the html or body element based on user preference or toggle."
  - id: failure:tailwind/breakpoint-gap
    symptom: "Layout breaks at intermediate viewport sizes between defined breakpoints."
    cause: Tailwind provides fixed breakpoints (640, 768, 1024, 1280px); there are gaps where no responsive variant applies and the default (mobile) style is used.
    fix: Design for the gap ranges by adding custom breakpoints; prefer fluid typography and spacing that do not rely on exact breakpoint boundaries.
  - id: failure:tailwind/jit-cold-start
    symptom: "Dev server is slow on first load or after clearing cache; classes take seconds to appear."
    cause: Tailwind's JIT engine scans all source files on startup and generates classes, which is slower for large projects with many template files.
    fix: Narrow the content globs in config to only the directories with Tailwind classes; exclude node_modules and build artifacts from scanning.
  - id: failure:tailwind/arbitrary-overuse
    symptom: "Design system drift — different arbitrary values for the same intent (w-[357px], w-[358px]) appear across the codebase."
    cause: Developers use arbitrary values as a shortcut instead of extending the theme config with named tokens.
    fix: Add new design tokens to tailwind.config theme.extend; enforce code review to flag arbitrary values that should be design tokens.
extends:
  - concept:tailwind/utility-first
uses:
  - concept:tailwind/responsive
  - concept:tailwind/dark-mode
  - concept:tailwind/custom-theme
  - concept:tailwind/layers
  - concept:tailwind/plugins
  - concept:tailwind/component-extraction
  - concept:tailwind/arbitrary-values
  - concept:tailwind/animations
  - concept:tailwind/optimization
  - concept:tailwind/container-queries
  - concept:tailwind/design-tokens
part_of: concept:domain/web-platform
depends_on:
  - package:react
  - package:npm
solves:
  - problem:css-bloat
alternatives:
  - package:bootstrap
  - package:css-modules
  - package:styled-components
---
# Tailwind CSS Patterns

Tailwind CSS is a utility-first framework that lets you build custom UIs by composing hundreds of single-purpose classes directly in your markup. Instead of writing custom CSS, you apply utilities like `flex`, `items-center`, `p-4`, and `text-lg` to HTML elements. Each utility does one thing and does it well. The result is no naming conventions, no specificity wars, no dead CSS — every class in your HTML is guaranteed to exist and be relevant.

Responsive design uses breakpoint prefixes. `sm:flex-row` applies flex-row on screens 640px and wider; `md:grid-cols-3` applies three-column grid at 768px and wider. Build mobile-first: start with the base (mobile) layout, then layer on larger breakpoints. If an element is `flex-col` by default and `md:flex-row`, it stacks vertically on mobile and switches to horizontal on desktop. The default breakpoints (640, 768, 1024, 1280, 1536) cover most devices, but you can customize them in `tailwind.config`.

Dark mode works through the `dark:` variant. With `darkMode: 'class'` in config, wrap your page in a `.dark` class and all `dark:bg-gray-900` styles activate. Toggle the class via JavaScript or match the user's `prefers-color-scheme` media query. Pair this with the system-theme default for a seamless experience: render `<html class="">` by default and add `.dark` only when the user toggles or the system says dark.

Component extraction keeps your templates clean. When a pattern repeats — like a button with `bg-blue-500 text-white font-bold py-2 px-4 rounded hover:bg-blue-600` — extract it into a React or Vue component rather than using `@apply`. Framework components give you props, variants, and composition; `@apply` is a leaky abstraction that duplicates styles in your CSS output and makes changes harder. Reserve `@apply` for situations where you cannot use a framework component, like WordPress `body_class` or email templates.

Optimization is built in. Tailwind's JIT engine scans your templates and generates only the classes you used, producing a CSS file often under 10 kB gzipped. The `content` paths in config tell the JIT engine where to look. If a class works in dev but disappears in production, your `content` glob is missing the file, or you are building classes dynamically with string concatenation. Use full class names and conditionally apply them with arrays or objects.
