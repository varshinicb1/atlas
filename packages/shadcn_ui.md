---
kind: Package
id: package:shadcn-ui
name: shadcn/ui
version: "2.0"
purpose: Document the shadcn/ui component system — copy-paste component library built on Radix UI primitives with Tailwind CSS, customization via CSS variables, and registry-based distribution.
problem_solved: Replaces the traditional npm-published component library model where customization is limited to pre-defined props and theming APIs. shadcn/ui components live in your project as source code — you own them, modify them, and style them entirely with Tailwind classes and CSS variables, giving full control without fighting the library.
install: npx shadcn@latest init
dependencies:
  - concept:tailwind-css
  - concept:react
  - concept:radix-ui
  - concept:typescript
concepts:
  - name: Registry
    id: concept:shadcn/registry
    description: A JSON registry of components hosted at https://ui.shadcn.com/registry. The CLI downloads component source code directly into your project's components/ui directory. Components are not npm dependencies — they're your code. The registry also tracks versions for safe upgrades.
  - name: CLI
    id: concept:shadcn/cli
    description: npx shadcn@latest add button — downloads component source, dependencies, and updates configuration. Supports --all for bulk install, --overwrite for re-download, and --path for custom output directories. Detects project structure (Next.js, Vite, Remix) automatically.
  - name: Composition Pattern
    id: concept:shadcn/composition
    description: Components are designed for composition over configuration. Dialog uses Dialog.Trigger -> Dialog.Content -> Dialog.Header -> Dialog.Title -> Dialog.Description as nested sub-components, giving control over layout, order, and styling at every level.
  - name: CSS Variables
    id: concept:shadcn/css-variables
    description: Theming via a set of CSS custom properties (--background, --foreground, --card, --primary, --secondary, --muted, --accent, --destructive, --border, --ring, --radius). Changing these variables in :root or .dark re-themes every component instantly. Color values use HSL for perceptual uniformity.
  - name: Dark Mode
    id: concept:shadcn/dark-mode
    description: Theme toggling via a .dark class on the root element. CSS variables have light and dark values mapped via the class selector. The default starter adds a ThemeProvider component using next-themes for SSR-safe dark mode with system preference detection.
  - name: Variants
    id: concept:shadcn/variants
    description: Most UI components (Button, Input, Badge, Card) support variant and size props defined via cva (class-variance-authority). Variants like default, destructive, outline, secondary, ghost, link give pre-built visual styles while still being fully overridable via className.
  - name: Hooks Collection
    id: concept:shadcn/hooks
    description: Registry includes reusable React hooks — useMediaQuery, useDebounce, useLocalStorage, useCopyToClipboard, useIntersectionObserver, useOnClickOutside. These are installed as source alongside UI components.
  - name: Blocks & Charts
    id: concept:shadcn/blocks
    description: Pre-built page sections (dashboard, sidebar, authentication, cards) combining multiple components into functional layouts. Blocks include both the component code and a preview. Charts via recharts integration with shadcn/ui styling — area, bar, line, pie, radar charts.
  - name: Customization Strategy
    id: concept:shadcn/customization
    description: Since components are source code, customization is direct — edit the TSX to change behavior, override Tailwind classes, add new variants, or remove features. The recommended strategy is to extend rather than modify base components by creating wrapper components.
apis:
  - name: npx shadcn@latest add [component]
    id: api:shadcn/add
    signature: "npx shadcn@latest add button"
    returns: Files downloaded to components/ui/.
    description: Downloads the specified component(s) from the registry into your project. Automatically resolves dependencies between components and installs required npm packages.
  - name: npx shadcn@latest init
    id: api:shadcn/init
    signature: "npx shadcn@latest init"
    returns: Generated components.json config and base CSS variables.
    description: Initializes shadcn/ui in your project — detects framework, creates components.json config file, sets up CSS variables in globals.css, and optionally installs base components.
  - name: cn() utility
    id: api:shadcn/cn
    signature: "import { cn } from '@/lib/utils'; cn('base-class', variant && 'conditional-class', className): string"
    returns: Merged className string.
    description: Combines clsx and tailwind-merge for conflict-free class merging. Handles Tailwind class deduplication (e.g., px-4 overrides px-2 when both appear). Every component uses cn for className prop merging.
  - name: cva(base, variants)
    id: api:shadcn/cva
    signature: "cva('base-classes', { variants: { variant: { default: '...', destructive: '...' } }, defaultVariants: { variant: 'default' } })"
    returns: A variant function (cva) that returns className string.
    description: Class Variance Authority — defines component variant combinations with base, compound, and default variant classes. Used as the core variant system in every shadcn/ui component.
  - name: Button
    id: api:shadcn/button
    signature: "<Button variant='default' size='default'>Click</Button>"
    returns: A styled button element.
    description: "Polymorphic button component — renders as <button> by default, <a> when href is provided, or any custom element via asChild prop (uses Radix Slot). Variants: default, destructive, outline, secondary, ghost, link. Sizes: default, sm, lg, icon."
  - name: Dialog
    id: api:shadcn/dialog
    signature: "<Dialog><DialogTrigger>Open</DialogTrigger><DialogContent><DialogHeader><DialogTitle>Title</DialogTitle></DialogHeader></DialogContent></Dialog>"
    returns: A modal dialog with accessible behavior.
    description: "Modal dialog built on Radix Dialog. Sub-components: Trigger, Content, Header, Footer, Title, Description, Close. Supports controlled open state, portal rendering, focus trap, and escape key dismiss."
  - name: Card
    id: api:shadcn/card
    signature: "<Card><CardHeader><CardTitle>Title</CardTitle><CardDescription>Desc</CardDescription></CardHeader><CardContent>...</CardContent><CardFooter>Actions</CardFooter></Card>"
    returns: A styled card container.
    description: Composite card component with Header, Title, Description, Content, and Footer sub-components. All are style-only wrappers — no behavior logic — making them purely presentational.
  - name: Form
    id: api:shadcn/form
    signature: "<Form {...form}><FormField control={form.control} name='email' render={({ field }) => (<FormItem><FormLabel>Email</FormLabel><FormControl><Input {...field} /></FormControl><FormMessage /></FormItem>)} /></Form>"
    returns: A form with validation and error display.
    description: React Hook Form integration with zod validation. Provides FormField, FormItem, FormLabel, FormControl, FormDescription, FormMessage sub-components for consistent form layout and validation feedback.
failures:
  - id: failure:shadcn/tailwind-v4-incompatibility
    symptom: Components render without styles after upgrading to Tailwind v4.
    cause: shadcn/ui components use Tailwind v3 syntax. Tailwind v4 uses CSS-first config with @import instead of @tailwind directives.
    fix: Stick with Tailwind v3 until shadcn/ui officially supports v4, or manually migrate component classes to Tailwind v4 syntax (most classes are unchanged, but config and theme extension APIs differ).
  - id: failure:shadcn/server-component-conflict
    symptom: Rendered component in Next.js 14+ throws "Event handler cannot be used in Server Component".
    cause: shadcn/ui components with interactivity (Dialog, DropdownMenu, Popover, Button with onClick) used in a Server Component without "use client" directive.
    fix: Extract interactive UI into Client Components. Create a wrapper file with "use client" at the top and re-export shadcn components from it.
  - id: failure:shadcn/overwrite-customizations
    symptom: "Custom component edits are lost after re-running shadcn add [component]."
    cause: The CLI overwrites existing files by default when adding the same component.
    fix: "Use --overwrite=false or manually back up customizations. Better strategy: never modify base shadcn components — create wrapper components that compose them."
  - id: failure:shadcn/missing-dependencies
    symptom: Build errors about missing module lucide-react, tailwind-merge, or class-variance-authority.
    cause: shadcn CLI does not always install transitive dependencies listed in the registry.
    fix: "Check the registry JSON for the component's dependencies array and install missing packages manually. Run npx shadcn@latest add [component] again after installing."
extends: []
implements: []
uses:
  - concept:tailwind-css
  - concept:react
  - concept:radix-ui
part_of: concept:react-ui-ecosystem
solves:
  - problem:react-component-library-customization
  - problem:consistent-ui-across-react-projects
  - problem:accessible-ui-components
alternatives:
  - package:material-ui
  - package:radix-ui
  - package:headless-ui
  - package:ark-ui
---
shadcn/ui upended the React component library model by abandoning npm distribution. Instead of publishing components as opaque packages, it distributes source code through a CLI + registry system. Every component is a .tsx file in your project's components/ui/ directory. You can edit anything — styles, behavior, structure — without patch-package or forking a repo.

The theming system uses CSS variables exclusively, avoiding runtime theme objects and JavaScript style injection. Colors are defined as HSL triples (hue saturation lightness) which enable perceptual color manipulation — darkening primary, lightening background, generating ring focus colors — through simple CSS calc(). The .dark class variant duplicates root variables with dark-mode values, and next-themes or a manual class toggle switches between them.

The variant system via cva (class-variance-authority) gives each component a predefined visual vocabulary (default, destructive, outline, secondary, ghost, link) while keeping the implementation in plain Tailwind classes. Because the classes are in your source, you can add variants, rename them, or remove ones you don't use. The cn() utility wraps clsx and tailwind-merge to resolve class conflicts when consumers pass custom className props — ensuring standard Tailwind override behavior.
