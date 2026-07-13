---
kind: Package
id: package:anime/j s
name: anime.js Animations
version: "4"
purpose: anime.js animation patterns — timeline, stagger, motion, SVG morphing, scroll-driven animations, and performance
problem_solved: Provides a declarative, performant animation toolkit for the browser that covers common UI motion needs — entrance/exit, layout transitions, SVG morphing, scroll-linked effects, and complex staggered sequences — without pulling in a heavy rendering framework.
install: npm install animejs
dependencies: []
concepts:
  - name: Animation
    id: concept:anime/animation
    description: The core unit of work in anime.js — a single property tween from a start value to an end value over a duration, driven by an easing function.
  - name: Timeline
    id: concept:anime/timeline
    description: A sequential container that chains multiple animations with relative offsets, enabling precise orchestration of multi-step motion without nested callbacks.
  - name: Easing
    id: concept:anime/easing
    description: Mathematical functions that shape the rate of change over time — built-in (easeInOut, cubicBezier), Spring physics (stiffness, damping), or custom bezier curves.
  - name: Stagger
    id: concept:anime/stagger
    description: Offsetting the start of each target in a collection by a computed delay, creating cascading wave effects across lists, grids, and text characters.
  - name: Targets
    id: concept:anime/targets
    description: The DOM elements, objects, or CSS selectors that anime.js animates; supports single elements, NodeLists, arrays, and object literals.
  - name: Callbacks
    id: concept:anime/callbacks
    description: Lifecycle hooks (begin, update, loop, complete) that fire at named points during an animation's execution for side-effect wiring.
  - name: SVG
    id: concept:anime/svg
    description: Anime.js can animate SVG attributes — viewBox, fill, stroke-dasharray, transform — and morph path data (d attribute) between compatible shapes.
  - name: Scroll
    id: concept:anime/scroll
    description: Scroll-driven animations where progress is tied to the element's visibility in the viewport, using anime.scroll() for entrance, parallax, and reveal effects.
  - name: Motion
    id: concept:anime/motion
    description: Dedicated motion presets and utilities (anime.motion()) for common UI gestures — fade-in, slide-up, scale-in — with sensible defaults and easy overrides.
  - name: Keyframes
    id: concept:anime/keyframes
    description: An array of animation states passed to the keyframes property, allowing multi-step sequences on a single target without a timeline.
  - name: Performance
    id: concept:anime/performance
    description: Anime.js runs on requestAnimationFrame, batches DOM writes, and avoids layout thrashing by reading computed styles once per frame.
apis:
  - name: anime
    id: api:anime/anime
    signature: "anime(params: AnimeParams): AnimeInstance"
    returns: An AnimeInstance with .play(), .pause(), .reverse(), .seek(), .finished.
    description: The primary factory function. Accepts targets, properties, duration, easing, and callbacks. Returns a controller for the running animation.
  - name: timeline
    id: api:anime/timeline
    signature: "anime.timeline(params?: TimelineParams): AnimeTimeline"
    returns: An AnimeTimeline with .add(), .play(), .pause(), .seek().
    description: Creates a sequential timeline. Add child animations with .add() specifying relative offsets ('-=' or '+=') for precise sequencing.
  - name: stagger
    id: api:anime/stagger
    signature: "anime.stagger(value: number | number[], options?: StaggerOptions): Function"
    returns: A function that returns the computed delay for each target index.
    description: Generates staggered delay values based on grid position (start, from, axis) or sequential index, enabling cascading effects across collections.
  - name: set
    id: api:anime/set
    signature: "anime.set(targets, props: object): void"
    returns: void
    description: Instantly sets CSS or object properties on targets without animation — useful for resetting state before a sequence or hiding elements on mount.
  - name: random
    id: api:anime/random
    signature: "anime.random(min: number, max: number): number"
    returns: A random float in [min, max].
    description: Utility that returns a random number each call; useful with anime() for organic, non-deterministic motion.
  - name: get
    id: api:anime/get
    signature: "anime.get(target: Element, prop: string): number"
    returns: The current computed value of the property on the target.
    description: Reads a CSS or SVG attribute value from a target element, used to read current state before chaining.
  - name: path
    id: api:anime/path
    signature: "anime.path(pathEl: SVGPathElement, prop?: 'x' | 'y' | 'angle'): Function"
    returns: A function that returns coordinates or angle at a given normalized progress.
    description: Binds an animation to an SVG path — elements follow the path's contour when translateX/translateY use the returned value.
  - name: bezier
    id: api:anime/bezier
    signature: "anime.bezier(x1: number, y1: number, x2: number, y2: number): Function"
    returns: A custom easing function.
    description: Constructs a cubic-bezier easing curve from control points, registered via anime.easing and usable as the easing parameter.
  - name: scroll
    id: api:anime/scroll
    signature: "anime.scroll(params: ScrollParams): AnimeScrollInstance"
    returns: An AnimeScrollInstance with progress, play, pause methods.
    description: Creates a scroll-driven animation whose progress (0–1) is bound to the element's intersection with the viewport; supports enter, leave, and container offsets.
  - name: motion
    id: api:anime/motion
    signature: "anime.motion(type: string, targets, overrides?: object): AnimeInstance"
    returns: An AnimeInstance configured with the named preset.
    description: Applies a built-in motion preset (fadeIn, slideUp, scaleIn) to targets with overridable duration, easing, and delay.
  - name: utils
    id: api:anime/utils
    signature: "anime.utils: { clamp, lerp, map, round, snap, throttle }"
    returns: An object of utility functions.
    description: Collection of helpers — clamp(value, min, max), lerp(a, b, t), map(num, inMin, inMax, outMin, outMax) — for manual tween math outside of animations.
  - name: createStory
    id: api:anime/createstory
    signature: "anime.createStory(storyDef: StoryDefinition): AnimeStory"
    returns: An AnimeStory controller with play(), pause(), goto(), and state.
    description: V4 API that groups related timelines into a "story" with named scenes, enabling narrative-style animation sequences with branching and state transitions.
  - name: finished
    id: api:anime/finished
    signature: "animeInstance.finished: Promise<void>"
    returns: A promise that resolves when the animation completes.
    description: A promise attached to every AnimeInstance, allowing await-based sequencing and integration with async/await flows.
examples:
  - id: example:anime/basic-tween
    language: js
    description: Animate a box's opacity and translateY on click.
  - id: example:anime/timeline-orchestration
    language: js
    description: Chain three element animations with relative offsets using timeline.
  - id: example:anime/stagger-grid
    language: js
    description: Cascade a scale animation across a grid of cards using stagger with grid axis.
  - id: example:anime/svg-morph
    language: js
    description: Morph one SVG path shape into another over 1.5 seconds.
  - id: example:anime/scroll-reveal
    language: js
    description: Fade in sections as they scroll into view using anime.scroll().
failures:
  - id: failure:anime/target-not-found
    symptom: Animation runs silently but no visual change occurs; console shows no error.
    cause: The targets selector matched zero elements, or the element was not yet in the DOM when anime() was called.
    fix: Ensure the DOM is ready (await DOMContentLoaded or call after mount) and that the selector matches existing elements.
  - id: failure:anime/incompatible-svg-morph
    symptom: SVG path morph produces a jumbled, crossed-over shape or no visible change.
    cause: The source and target path data have a different number of commands or points; anime.js cannot interpolate mismatched paths.
    fix: Normalize both paths to the same number and types of commands using an SVG path editor or a normalization script.
  - id: failure:anime/stagger-overflow
    symptom: Staggered animation takes far too long to complete; later items appear frozen.
    cause: A large stagger delay multiplied by many targets produces a total duration exceeding the intended animation length.
    fix: Reduce the stagger base delay, limit the number of staggered targets, or set a max stagger value via the grid or from parameters.
  - id: failure:anime/callback-mutation
    symptom: Animation stutters or skips frames when callbacks fire.
    cause: Expensive synchronous work (DOM reads, layout thrashing) inside begin, update, or complete callbacks.
    fix: Keep callbacks lightweight; defer heavy work to requestIdleCallback or setTimeout; batch DOM reads before writes.
  - id: failure:anime/scroll-container
    symptom: Scroll-driven animation never triggers or fires at the wrong scroll position.
    cause: "The scroll container is not the window (e.g., a div with overflow: scroll) and the container option was omitted."
    fix: "Pass { container: '#scrolling-div' } to anime.scroll() when the scrollable parent is not the document."
extends:
  - concept:anime/animation
uses:
  - concept:anime/timeline
  - concept:anime/stagger
  - concept:anime/svg
  - concept:anime/scroll
  - concept:anime/keyframes
part_of: concept:domain/web-platform
depends_on: []
solves:
  - problem:browser-animation
alternatives:
  - package:gsap
  - package:framer-motion
  - package:css-animations
---
# anime.js Animations

Anime.js v4 is a lightweight, standalone JavaScript animation library that runs on `requestAnimationFrame` and targets CSS properties, SVG attributes, DOM transforms, and plain JavaScript objects. Its API is declarative — you describe the end state, duration, easing, and stagger, and anime.js computes every intermediate frame in a zero-dependency ~20 kB bundle.

The central primitive is `anime()`, which accepts a parameters object with `targets`, animated properties, `duration` (default 1000 ms), `easing` (default 'easeOutElastic'), and lifecycle callbacks. Each call returns an `AnimeInstance` with methods for imperative control: `.play()`, `.pause()`, `.reverse()`, `.seek(progress)`, and a `.finished` promise that resolves when the animation completes. For sequential motion, `anime.timeline()` chains animations with relative offsets (`'+=500'`), avoiding nested-callback hell while keeping every step independently configurable.

Staggered animations are a hallmark of anime.js. `anime.stagger()` generates per-target delays from a base value, optionally modulated by grid position (`from: 'center'`, `axis: 'x'`) so that cards, list items, or text characters cascade in natural wave patterns. For SVG work, anime.js animates `d` attribute morphing — morph path A to path B — as long as the command count matches, and the `anime.path()` helper binds an element's translation to follow the contour of an SVG path for motion along curves.

Scroll-driven animations arrived as a first-class feature in v4. `anime.scroll()` links animation progress (0–1) to an element's visibility in the viewport (or a custom scroll container), producing entrance reveals, parallax layers, and progress indicators without intersection-observer boilerplate. For common UI gestures — fade-in, slide-up, scale-in — `anime.motion()` provides named presets with sensible defaults that can be tuned per use case.

Performance is designed in: anime.js batches DOM writes, avoids forced synchronous layouts by reading computed styles once per frame, and respects the browser's paint cycle. Even with hundreds of staggered targets, frame rate stays steady as long as callbacks remain lightweight. The `anime.utils` module exposes `clamp`, `lerp`, `map`, `snap`, and `throttle` for manual interpolation work outside the animation pipeline.

## Decision Tree

- **Need a simple property tween?** → `anime({ targets, translateX: 200, duration: 800 })`
- **Need to chain multiple steps in order?** → `anime.timeline().add(...).add(...)`
- **Need to animate a collection with a wave effect?** → `anime({ targets: '.item', scale: stagger(0.1, { from: 'center' }) })`
- **Need to morph one SVG shape into another?** → `anime({ targets: '#path', d: newPath })` — ensure command count matches
- **Need an animation that plays as the user scrolls?** → `anime.scroll({ targets, translateX: 200, container: '#scroll-div' })`
- **Need a quick entrance with sensible defaults?** → `anime.motion('fadeIn', '.box')`
- **Need to set properties instantly (no animation)?** → `anime.set(targets, { opacity: 0 })`
- **Need to read a current CSS value before animating?** → `anime.get(element, 'opacity')`
- **Need a custom easing curve?** → `anime({ easing: anime.bezier(0.25, 0.1, 0.25, 1) })`
- **Need an element to follow an SVG path?** → `anime({ targets, translateX: path(pathEl, 'x'), translateY: path(pathEl, 'y') })`
- **Need to organize complex sequences into named scenes?** → `anime.createStory({ scenes: [...] })`
- **Need random values for organic motion?** → `anime({ translateX: () => anime.random(-50, 50) })`
- **Need to await completion in async code?** → `await anime({ targets }).finished`
