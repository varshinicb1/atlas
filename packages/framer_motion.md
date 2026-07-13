---
kind: Package
id: package:framer/motion
name: Framer Motion
version: "12"
purpose: Document Framer Motion animation patterns — layout animations, gestures, scroll, variants, and shared layout for React
problem_solved: Provides a reference for Framer Motion's declarative animation primitives, gesture handling, layout animations, and shared layout patterns, reducing janky transitions, layout shift bugs, and accessibility gaps from missing reduced-motion support.
install: npm install framer-motion
dependencies:
  - package:react
  - package:react-dom
concepts:
  - name: Motion Components
    id: concept:framer-motion/motion-components
    description: HTML and SVG elements with the motion prefix (motion.div, motion.svg) that accept animation props like animate, initial, and exit.
  - name: Variants
    id: concept:framer-motion/variants
    description: Named animation states (hidden, visible, hover) defined as objects and orchestrated across parent-child components via propagation.
  - name: Layout Animations
    id: concept:framer-motion/layout-animations
    description: Animating layout changes smoothly using the layout prop, powered by FLIP (First, Last, Invert, Play) calculations.
  - name: Gestures
    id: concept:framer-motion/gestures
    description: Interaction-driven animations via whileHover, whileTap, whileDrag, whileFocus, and whileInView props.
  - name: Scroll Animations
    id: concept:framer-motion/scroll
    description: Animating elements based on scroll position using useScroll, useInView, and whileInView for viewport-triggered entries.
  - name: AnimatePresence
    id: concept:framer-motion/animatepresence
    description: A component that enables mount/unmount animations (exit prop) for elements removed from the React tree.
  - name: Keyframes
    id: concept:framer-motion/keyframes
    description: Array-based animation values that sequence through multiple states over the duration, creating multi-step animations.
  - name: SVG Animation
    id: concept:framer-motion/svg-animation
    description: Animating SVG path length, transforms, and shapes via motion.path, motion.circle, and other SVG motion components.
  - name: Spring Physics
    id: concept:framer-motion/spring-physics
    description: Natural-feeling animations driven by spring physics (stiffness, damping, mass) instead of CSS easing curves.
  - name: Reduced Motion
    id: concept:framer-motion/reduced-motion
    description: Respecting the user's prefers-reduced-motion setting to disable or simplify animations for accessibility.
  - name: Shared Layout
    id: concept:framer-motion/shared-layout
    description: Using layoutId to animate elements that change position between different parents, creating seamless shared element transitions.
  - name: Drag
    id: concept:framer-motion/drag
    description: Making elements draggable with the drag prop, supporting drag constraints, elastic bounds, and drag direction locking.
apis:
  - name: motion.div
    id: api:framer-motion/motion-div
    signature: "<motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }} />"
    returns: A motion-enhanced div element.
    description: The primary building block — a div that accepts animation props for initial state, animate target, and exit animation.
  - name: motion.custom
    id: api:framer-motion/motion-custom
    signature: "const MotionComponent = motion(MyComponent)"
    returns: A motion-enhanced custom React component.
    description: Wraps any React component with motion capabilities so it can receive animation and gesture props.
  - name: animate
    id: api:framer-motion/animate
    signature: "animate(element, { x: 100 }, { duration: 0.5 })"
    returns: A promise that resolves when the animation completes.
    description: The imperative animation API for controlling animations outside the render tree, useful for programmatic sequences.
  - name: useAnimation
    id: api:framer-motion/useanimation
    signature: "const controls = useAnimation(); controls.start({ x: 100 })"
    returns: A controls object with start, stop, and set methods.
    description: Imperatively controls a motion component's animation from event handlers or external state.
  - name: useScroll
    id: api:framer-motion/usescroll
    signature: "const { scrollY, scrollYProgress } = useScroll()"
    returns: Reactive scroll position and progress values (MotionValues).
    description: Tracks the page or element scroll position and progress as a MotionValue for chaining into animations.
  - name: useInView
    id: api:framer-motion/useinview
    signature: "const ref = useInView({ once: true, margin: '-50px' })"
    returns: A ref to attach to the target element.
    description: Returns a ref that triggers animation when the element enters the viewport, with options for once, rootMargin, and threshold.
  - name: AnimatePresence
    id: api:framer-motion/animatepresence-api
    signature: "<AnimatePresence mode='wait'>{isVisible && <motion.div exit={{ ... }} />}</AnimatePresence>"
    returns: An animated mount/unmount wrapper.
    description: Wraps conditional children to animate their exit before removal; mode controls how enter/exit overlap.
  - name: LayoutGroup
    id: api:framer-motion/layoutgroup
    signature: "<LayoutGroup id='group-name'>{children}</LayoutGroup>"
    returns: A layout coordination context.
    description: Groups elements with layout props so their layout animations synchronize and do not conflict across the tree.
  - name: Reorder
    id: api:framer-motion/reorder
    signature: "<Reorder.Group values={items} onReorder={setItems}>...</Reorder.Group>"
    returns: A draggable reorder group.
    description: Provides drag-to-reorder functionality via Reorder.Group and Reorder.Item with automatic layout animation.
  - name: drag
    id: api:framer-motion/drag-api
    signature: "<motion.div drag='x' dragConstraints={{ left: 0, right: 100 }} />"
    returns: A draggable motion element.
    description: Enables dragging on an axis (x, y, or both) with optional constraints, elastic bounds, and momentum.
  - name: whileHover
    id: api:framer-motion/whilehover
    signature: "<motion.div whileHover={{ scale: 1.1 }} whileTap={{ scale: 0.95 }} />"
    returns: A gesture-responsive motion element.
    description: Animates to the provided target when the element is hovered, tapped, focused, or in view.
  - name: whileInView
    id: api:framer-motion/whileinview
    signature: "<motion.div initial={{ opacity: 0 }} whileInView={{ opacity: 1 }} />"
    returns: A viewport-triggered motion element.
    description: Triggers animation when the element enters the viewport; supports once, amount, and margin options.
examples:
  - id: example:framer-motion/fade-in-stagger
    language: tsx
    description: A list of items fading in with staggered delay using variants with transition staggerChildren.
  - id: example:framer-motion/layout-swap
    language: tsx
    description: Two elements swapping position with animated layout using layoutId for shared layout transitions.
  - id: example:framer-motion/drag-carousel
    language: tsx
    description: A horizontal draggable carousel with drag constraints and momentum snapping.
  - id: example:framer-motion/scroll-progress
    language: tsx
    description: A scroll progress bar using useScroll and useTransform to map scrollYProgress to width.
  - id: example:framer-motion/modal-exit
    language: tsx
    description: A modal with mount/unmount animation using AnimatePresence and exit prop.
failures:
  - id: failure:framer-motion/layout-shift
    symptom: Elements jump or snap into position instead of animating smoothly during layout changes.
    cause: Missing the layout prop on elements whose size or position changes; layout animation requires layout or layoutId.
    fix: Add layout prop to both the old and new position for animated layout; use layoutId for elements moving between parents.
  - id: failure:framer-motion/missing-exit
    symptom: Exit animations do not play; elements disappear instantly on unmount.
    cause: The conditional element is not wrapped in AnimatePresence, or exit prop is missing from the motion component.
    fix: Wrap the conditional branch in <AnimatePresence> and add an exit prop to the motion component defining the closing state.
  - id: failure:framer-motion/gesture-conflict
    symptom: Drag and scroll conflict; the page scrolls when trying to drag, or dragging captures scroll events.
    cause: Drag and scroll handlers are on the same axis without separation; drag='x' still needs scroll listener management.
    fix: Isolate drag to one axis (drag='x') and use dragControls for fine-grained control; avoid drag on scrollable containers.
  - id: failure:framer-motion/performance-hiccup
    symptom: Animations stutter or drop frames, especially with many animated elements.
    cause: Animating non-transform properties (width, height, top, left) that trigger layout recalculations on every frame.
    fix: Animate transform properties (scale, x, y, rotate) instead; use opacity sparingly; consider will-change on animated elements.
  - id: failure:framer-motion/variant-inheritance
    symptom: Child variants do not animate when the parent variant changes.
    cause: Child motion components lack the variants prop or parent/child variant names do not match for propagation.
    fix: Define matching variant names (e.g., open/closed) on both parent and child; set propagate="children" or rely on default inheritance.
  - id: failure:framer-motion/reduced-motion-ignore
    symptom: Animations play despite the user having prefers-reduced-motion enabled, causing discomfort.
    cause: No reduced-motion check; animations run unconditionally for all users.
    fix: Use useReducedMotion() hook to conditionally disable animations; wrap in a MotionConfig with reducedMotion="user" prop.
  - id: failure:framer-motion/keyframe-sync
    symptom: Multi-step keyframe animations play in wrong order or skip steps.
    cause: Keyframe arrays with different lengths or mismatched object keys across keyframes cause undefined behavior.
    fix: Ensure every keyframe object has the same keys; pad with null values if a key is unchanged at a step.
extends:
  - concept:framer-motion/motion-components
uses:
  - concept:framer-motion/variants
  - concept:framer-motion/layout-animations
  - concept:framer-motion/gestures
  - concept:framer-motion/scroll
  - concept:framer-motion/animatepresence
  - concept:framer-motion/keyframes
  - concept:framer-motion/svg-animation
  - concept:framer-motion/spring-physics
  - concept:framer-motion/reduced-motion
  - concept:framer-motion/shared-layout
  - concept:framer-motion/drag
part_of: concept:domain/web-platform
depends_on:
  - package:react
  - package:react-dom
solves:
  - problem:animation-declarative
alternatives:
  - package:gsap
  - package:react-spring
  - package:css-animations
---
# Framer Motion

Framer Motion 12 is a declarative animation library for React built on the motion component system. Any HTML or SVG element prefixed with `motion.` becomes animatable — `motion.div`, `motion.svg`, `motion.path`. These components accept props like `initial` (the state before mount), `animate` (the target state), `exit` (the state on unmount, used with `AnimatePresence`), and `transition` (duration, easing, or spring physics). The library uses CSS transforms and opacity for performant animations, falling back to JavaScript animations for non-transform properties that CSS cannot handle efficiently.

Variants are named animation states that simplify complex orchestration. A parent defines variant objects (`hidden`, `visible`) and children can inherit and respond with stagger timing. `transition` properties like `staggerChildren` and `delayChildren` are set on the parent variant, and each child variant animates in sequence. This pattern eliminates manual timeout stitching for staggered lists, accordions, and multi-step reveals.

Layout animations are Framer Motion's killer feature. Adding the `layout` prop to an element makes it animate smoothly to any new position or size caused by layout shifts — no measurements, no state tracking. The underlying FLIP technique (First, Last, Invert, Play) captures the element's start and end rects in a ResizeObserver and animates the delta. `layoutId` extends this across the component tree: when an element with `layoutId="card"` moves from one parent to another, Framer Motion recognizes it as the same element and animates the transition. Use `LayoutGroup` to coordinate multiple layout animations so they do not conflict.

Gestures and scroll make UIs interactive. `whileHover`, `whileTap`, `whileFocus`, `whileDrag`, and `whileInView` map directly to animation targets. `useScroll` gives you reactive scroll progress as a `MotionValue`, and `useTransform` lets you map one MotionValue range to another (e.g., map `scrollYProgress` from 0-1 to `scale` from 0.8-1). `useInView` returns a ref — simpler than Intersection Observer boilerplate. Always respect `useReducedMotion()` or `<MotionConfig reducedMotion="user">` to avoid triggering vestibular disorders with unnecessary motion.
