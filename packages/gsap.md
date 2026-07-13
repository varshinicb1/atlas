---
kind: Package
id: package:gsap
name: GSAP (GreenSock Animation Platform)
version: "3.12"
purpose: Provide high-performance, cross-browser animation primitives for the web — timelines, tweens, scroll-driven animation, morphing, and physics — with zero-dependency tree-shakeable modules.
problem_solved: Eliminates the complexity gap between CSS transitions/animations and custom JavaScript animation loops, giving developers deterministic control over timing, easing, sequencing, and scroll-based motion without fighting browser paint cycles.
install: npm install gsap
dependencies:
  - concept:dom-api
  - concept:css-transforms
  - concept:request-animation-frame
concepts:
  - name: Tween
    id: concept:gsap/tween
    description: The core animation unit — animates one or more properties of a target (DOM element, object, array) from current or defined values to end values over a duration, with configurable ease, delay, and callbacks.
  - name: Timeline
    id: concept:gsap/timeline
    description: A sequencing container for tweens — add(), insert(), and nest tweens with absolute or relative positioning. Supports pause/play/reverse/seek and timeScale for scrubbing. Think of it as a master clock for choreographed multi-element animations.
  - name: Easing
    id: concept:gsap/easing
    description: Over 30 built-in easing functions (power1–4, elastic, bounce, back, rough, slowMo, steps, custom) plus the ability to define custom cubic-bezier or SVG-based paths via CustomEase. Eases can be applied per-prop for stagger effects.
  - name: ScrollTrigger
    id: concept:gsap/scrolltrigger
    description: Links animation progress to the viewport scroll position. Supports start/end markers, toggle actions (play/pause/reset/reverse), pinning, snap points, scrub mode, and matchMedia for responsive breakpoints. The de facto standard for scroll-driven web animation.
  - name: MotionPath
    id: concept:gsap/motionpath
    description: Animates an element along an SVG path or a custom array of points. Handles rotation-to-tangent, path offset, and alignment. Useful for explainer animations, loading indicators, and decorative UI motion.
  - name: MorphSVG
    id: concept:gsap/morphsvg
    description: Morphs one SVG shape into another by interpolating path data. Handles differing point counts via shape-adjustment algorithms. Plugin required (CDN or club GSAP).
  - name: Flip
    id: concept:gsap/flip
    description: Animates DOM elements between layout states (FLIP — First, Last, Invert, Play). Tracks position/size changes after DOM mutations and smoothly interpolates, eliminating manual calculation of start/end states for list reorder, grid shuffle, or mount/unmount transitions.
  - name: Draggable
    id: concept:gsap/draggable
    description: Adds drag interaction to any element — axis-constrained, hit-test snapping, liveSnap, bounds, inertia, and live-tweening during drag. Integrates with ScrollTrigger for drag-scroll hybrid patterns.
  - name: Observer
    id: concept:gsap/observer
    description: Unified pointer/touch/scroll event abstraction. Normalizes mouse, touch, and wheel events into a single API with delta tracking, axis-locking, and tolerance thresholds. Replaces manual addEventListener management for drag and scroll primitives.
  - name: TextPlugin
    id: concept:gsap/textplugin
    description: Animates text content by typing out or erasing characters sequentially. Supports step-based revealing and can work alongside other tweens for combined typewriter + fade effects.
  - name: CustomEase
    id: concept:gsap/customease
    description: Generates a bezier-based easing function from an SVG path or control points. Club GSAP feature. Enables organic, bespoke motion curves that no cubic-bezier can express.
  - name: SplitText
    id: concept:gsap/splittext
    description: Splits text into characters, words, or lines as wrapped DOM elements for per-unit animation. Club GSAP plugin. Combines with Timeline for sophisticated text reveals (stagger, direction, blur).
  - name: inertia
    id: concept:gsap/inertia
    description: Applies momentum-based continuation of an animation after a drag or flick — calculates velocity at release and smoothly decelerates to a stop or snap point. Powers kinetic scrolling and throw physics.
  - name: stagger
    id: concept:gsap/stagger
    description: A built-in tween config that offsets the start time of each target in an array. Supports grid-based (rows/columns), from/to/center alignment, and per-element ease sequencing for cascading entrance animations.
  - name: keyframes
    id: concept:gsap/keyframes
    description: Animate a target through multiple states using an array of progress points, similar to CSS @keyframes but with full GSAP easing control, labels, and per-keyframe callbacks.
apis:
  - name: gsap.to(targets, vars)
    id: api:gsap/to
    signature: "gsap.to(targets: string | Element | ArrayLike, vars: TweenVars): Tween"
    returns: A Tween instance (callable for control).
    description: Animates from current state to defined end values. The most commonly used method for entrance animations and state transitions.
  - name: gsap.from(targets, vars)
    id: api:gsap/from
    signature: "gsap.from(targets: string | Element | ArrayLike, vars: TweenVars): Tween"
    returns: A Tween instance.
    description: Animates from a defined start state to the current state. Used for reveal effects where elements begin off-screen or invisible and animate into place.
  - name: gsap.fromTo(targets, fromVars, toVars)
    id: api:gsap/fromTo
    signature: "gsap.fromTo(targets: string | Element | ArrayLike, fromVars: TweenVars, toVars: TweenVars): Tween"
    returns: A Tween instance.
    description: Explicitly defines both start and end states for full control over the animation envelope. Use when the current state is not the desired start or end.
  - name: gsap.timeline(vars)
    id: api:gsap/timeline
    signature: "gsap.timeline(vars?: TimelineVars): Timeline"
    returns: A Timeline instance.
    description: Creates a timeline for sequencing tweens. Supports relative positioning with +=N, -=N, <, > labels, and nested timelines.
  - name: gsap.registerPlugin(...plugins)
    id: api:gsap/registerPlugin
    signature: "gsap.registerPlugin(...plugins: GSAPPlugin[]): void"
    returns: undefined
    description: Registers GSAP plugins (ScrollTrigger, Draggable, MorphSVG, etc.) for tree-shaking. Required before using any plugin in production builds.
  - name: gsap.set(targets, vars)
    id: api:gsap/set
    signature: "gsap.set(targets: string | Element | ArrayLike, vars: TweenVars): Tween"
    returns: A Tween instance (immediate).
    description: Sets properties immediately with zero duration. Useful for establishing initial states before from() tweens or resetting after animations complete.
  - name: gsap.matchMedia()
    id: api:gsap/matchMedia
    signature: "gsap.matchMedia(): MatchMedia"
    returns: A MatchMedia instance.
    description: Creates a responsive context that applies/reverts tweens and ScrollTriggers based on media query changes. Automatically cleans up on resize or orientation change.
  - name: gsap.context(func, scope?)
    id: api:gsap/context
    signature: "gsap.context(func: () => void, scope?: Element | string): GSAPContext"
    returns: A GSAPContext instance with revert().
    description: Scopes all GSAP animations created inside func to a DOM element or ref. Auto-reverts on cleanup — ideal for React useEffect and Svelte onMount to prevent stale animations.
  - name: ScrollTrigger.create(vars)
    id: api:gsap/scrolltrigger-create
    signature: "ScrollTrigger.create(vars: ScrollTriggerVars): ScrollTrigger"
    returns: A ScrollTrigger instance.
    description: "Creates a scroll-linked animation trigger. Key vars: trigger, start/end (string positions), scrub (boolean|number), pin, toggleActions, onEnter/onLeave callbacks."
  - name: gsap.quickTo(target, property)
    id: api:gsap/quickTo
    signature: "gsap.quickTo(target: Element, property: string): (value: number) => Tween"
    returns: A function that sets the property to a value with animation.
    description: Returns a pre-compiled setter function for mouse-follow and parallax patterns. Significantly faster than creating individual tweens per frame in pointer-move handlers.
failures:
  - id: failure:gsap/missing-plugin-registration
    symptom: ScrollTrigger, Draggable, or MorphSVG methods do nothing — no errors thrown.
    cause: Plugin not registered via gsap.registerPlugin() before use.
    fix: Call gsap.registerPlugin(ScrollTrigger) at the app entry point. Verify the plugin import resolves correctly.
  - id: failure:gsap/stale-refs-in-react
    symptom: Animations applied to elements that no longer exist; memory warnings.
    cause: Tween targets stored React refs that have been unmounted without cleanup.
    fix: Use gsap.context() inside useEffect and return context.revert() as the cleanup. Alternatively, call tween.kill() in the cleanup function.
  - id: failure:gsap/scrolltrigger-pin-issues
    symptom: Pinned element jumps, clips, or does not unpin at the expected end point.
    cause: Incorrect start/end values, missing parent overflow rules, or conflicting CSS transforms.
    fix: "Ensure the pin container has overflow: visible; use ScrollTrigger.refresh() after layout changes; debug with markers: true."
  - id: failure:gsap/performance-stutter
    symptom: Animation jank on mobile or complex pages with 50+ concurrent tweens.
    cause: Animating layout-triggering properties (width, height, top, left) instead of compositor-only (transform, opacity).
    fix: Use x/y/scale/rotation/opacity properties. Enable will-change on animated elements. Use quickTo for pointer-bound animations. Set gsap.ticker.lagSmoothing(0).
  - id: failure:gsap/timeline-overlap
    symptom: Tweens in a timeline fire simultaneously or in the wrong order.
    cause: Incorrect position parameter — default is 0 (start of timeline), not "+=0" (end of previous).
    fix: Use "+=0.5" for sequential with gap, "<" for overlap start, or labels for named positions.
extends: []
implements: []
uses:
  - concept:dom-api
  - concept:css-transforms
  - concept:request-animation-frame
part_of: concept:web-animation
solves:
  - problem:complex-web-animation
  - problem:scroll-driven-motion
  - problem:cross-browser-animation-consistency
alternatives:
  - package:framer-motion
  - package:anime-js
  - package:css-animations
  - package:waapi
---
GSAP is the most mature and performant animation library for the web, operating at a lower level than Framer Motion with full control over the rendering tick. Its architecture separates **tweens** (atomic property animations) from **timelines** (sequencing) and **plugins** (scroll, drag, morph, text), making it tree-shakeable and framework-agnostic.

The key design insight is that **every animation is a tween** — even ScrollTrigger and Draggable create tweens internally. This means the timing model (ease, duration, delay, repeat, yoyo) is consistent across all animation types. Timelines add a temporal dimension: they act as a grouping and sequencing mechanism where each child tween's position parameter determines when it starts relative to the timeline's master clock.

ScrollTrigger revolutionized scroll-based animation by making scroll position a **direct input to animation progress**, separate from element lifecycle. This decoupling means animations can be reversed, scrubbed, or linked to multiple scroll containers without manual scroll event math. The scrub mode interpolates the animation proportionally to scroll, enabling the "parallax scroll" and "progress bar" patterns that defined 2020s web design.

For React integration, the canonical pattern is `useGSAP()` (a thin wrapper around `gsap.context()` that auto-cleans on unmount) combined with refs for target elements. Avoid imperative DOM queries inside animation code — always pass refs or selector strings scoped to the component. For mouse-follow effects, `gsap.quickTo()` pre-compiles the animation pipeline for 60fps pointer tracking without per-frame GC pressure.
