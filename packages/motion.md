---
kind: Package
id: package:motion
name: Motion (Framer Motion successor)
version: "12"
purpose: Document Motion — the animation library for React and JavaScript, successor to Framer Motion, with layout animations, gestures, scroll-linked animations, and the motion component model.
problem_solved: Provides a declarative, React-friendly animation API that handles the hard parts of web animation — layout animations (FLIP), gesture-driven motion (drag, hover, tap), scroll-linked effects, and SVG path animation — without requiring imperative GSAP-style code or manual requestAnimationFrame management.
install: npm install motion
dependencies:
  - concept:react
  - concept:css-transforms
  - concept:web-animation
concepts:
  - name: motion Components
    id: concept:motion/components
    description: HTML/SVG elements prefixed with motion — motion.div, motion.span, motion.path. These are enhanced DOM elements that accept animation props (animate, initial, exit, whileHover, whileTap, whileInView, variants, transition). Every HTML and SVG element has a motion counterpart.
  - name: Animation Props
    id: concept:motion/animate-prop
    description: "The animate prop defines the target state of an animation. Values can be simple (animate={{ x: 100 }}), keyframes (animate={{ x: [0, 100, 50] }}), or variant labels (animate=\"visible\"). Supports all CSS properties including transform, opacity, colors, and SVG attributes."
  - name: Variants
    id: concept:motion/variants
    description: "Named animation states defined as objects — const variants = { hidden: { opacity: 0 }, visible: { opacity: 1 } }. Components reference variants via animate, initial, exit props. Variants support propagation: parent variant timing cascades to children via staggerChildren."
  - name: Layout Animations
    id: concept:motion/layout
    description: layout prop enables automatic FLIP (First, Last, Invert, Play) animations. When a component's layout changes (due to reorder, resize, mount/unmount), Motion animates the transition smoothly. layoutId prop shares layout identity across different components for animated shared-element transitions.
  - name: Gestures
    id: concept:motion/gestures
    description: whileHover, whileTap, whileFocus, whileDrag — gesture-driven animation triggers. whileDrag provides drag constraints, elastic bounds, and direction locking. onDragStart/onDragEnd callbacks. Combine with animate for gesture-responsive micro-interactions.
  - name: Scroll Animation
    id: concept:motion/scroll
    description: useScroll() returns scroll progress values (scrollYProgress, scrollXProgress). whileInView triggers animations when elements enter the viewport with once, amount, and margin options. useTransform maps scroll progress to animation values for parallax and progress-bar effects.
  - name: AnimatePresence
    id: concept:motion/animate-presence
    description: Wrapper that enables mount/unmount animations. Children with exit prop animate out before removal. Supports mode="wait" (sequential enter/exit), mode="popLayout" (absolute positioning during exit for smooth reflow). Essential for lists, modals, and route transitions.
  - name: useAnimate
    id: concept:motion/use-animate
    description: "Imperative animation hook — const [scope, animate] = useAnimate(). animate(element, keyframes, options) provides imperative control for complex sequences, chained animations, and non-React contexts. The scope ref auto-targets children for scoped cleanup."
  - name: MotionValue
    id: concept:motion/motion-value
    description: "A reactive numeric/color value that drives animations — useMotionValue(0) creates one. useTransform(motionValue, [inputRange], [outputRange]) maps values through a range transformation. MotionValues are the bridge between gesture/scroll input and visual output."
  - name: SVG Animation
    id: concept:motion/svg
    description: "motion.path, motion.circle, motion.g — animate SVG attributes (pathLength, fill, stroke, d). pathLength={1} animates drawing of paths. Animate SVG morphing via the d attribute. Combine with whileInView for scroll-triggered SVG reveals."
  - name: Reduced Motion
    id: concept:motion/reduced-motion
    description: Motion respects prefers-reduced-motion automatically — disables animations for users with motion sensitivity. Use useReducedMotion() hook for custom reduced-motion logic. Set MotionConfig reducedMotion="always" for testing or "never" for override.
apis:
  - name: motion.div
    id: api:motion/component
    signature: "<motion.div animate={{ x: 100 }} transition={{ type: 'spring' }} />"
    returns: A motion-enhanced div React component.
    description: "The core API — prefix any HTML/SVG element with motion to make it animatable. Props: animate, initial, exit, transition, variants, layout, whileHover, whileTap, whileInView, style, className — all standard React props work as expected."
  - name: useScroll(options)
    id: api:motion/useScroll
    signature: "const { scrollYProgress } = useScroll(opts?: { container?, target?, axis?, offset? }): { scrollX, scrollY, scrollXProgress, scrollYProgress }"
    returns: MotionValues for scroll position and progress.
    description: "Tracks scroll progress as a reactive MotionValue (0 = start, 1 = end). Optionally scoped to a container ref or target element. The offset array defines [start, end] positions relative to the viewport."
  - name: useTransform(value, input, output)
    id: api:motion/useTransform
    signature: "const opacity = useTransform(scrollYProgress, [0, 1], [1, 0]): MotionValue<number>"
    returns: A transformed MotionValue.
    description: Maps a MotionValue through input/output ranges. Supports clamp, extrapolateLeft, extrapolateRight options. Use for parallax (map scroll to y), progress bars (map scroll to width), and color transitions.
  - name: useSpring(value, config)
    id: api:motion/useSpring
    signature: "const smoothValue = useSpring(motionValue, opts?: { stiffness, damping, mass }): MotionValue<number>"
    returns: A spring-animated MotionValue.
    description: Wraps a MotionValue with spring physics — smoothes out rapid changes (mouse tracking, scroll) by interpolating through a spring simulation. Configurable stiffness (tension), damping (resistance), and mass (inertia).
  - name: AnimatePresence
    id: api:motion/animate-presence
    signature: "<AnimatePresence mode='wait'>{isOpen && <motion.div exit={{ opacity: 0 }} />}</AnimatePresence>"
    returns: A React fragment wrapper.
    description: "Enables exit animations for unmounting children. Key props: mode ('sync' | 'wait' | 'popLayout'), initial (false to skip initial animation), onExitComplete callback. Each child needs a unique key and exit prop."
  - name: motion.div layout
    id: api:motion/layout-prop
    signature: "<motion.div layout layoutId='card' layoutDependency={data}>"
    returns: A layout-animated component.
    description: Enables FLIP animations. When the component's position/size changes, Motion animates the transition. layoutId shares identity across different components for shared-element transitions. layoutDependency prevents unnecessary measurements.
  - name: whileInView
    id: api:motion/while-in-view
    signature: "<motion.div initial={{ opacity: 0 }} whileInView={{ opacity: 1 }} viewport={{ once: true, amount: 0.3 }}>"
    returns: A scroll-triggered animation prop.
    description: Animates when the element enters the viewport. viewport prop configures once (animate only first time), amount (visibility threshold, 0-1), margin (IntersectionObserver rootMargin).
  - name: staggerChildren
    id: api:motion/stagger
    signature: "const variants = { visible: { transition: { staggerChildren: 0.1, delayChildren: 0.2 } } }"
    returns: A variant animation configuration.
    description: Staggers the animation of child components by the specified interval. Set on the parent variant's transition. delayChildren adds initial delay before staggering. staggerDirection for reverse or alternate ordering.
failures:
  - id: failure:motion/layout-animation-expensive
    symptom: Layout animations cause jank or frame drops on mount with many elements.
    cause: layout prop triggers measurement and animation for every component. Many layout animations simultaneously overwhelm the compositor.
    fix: Use layout only on elements that actually change position. Prefer whileInView for initial reveals instead of layout. Use MotionConfig reducedMotion for low-power devices. Consider layoutDependency to skip unnecessary measurements.
  - id: failure:motion/animate-presence-key-mismatch
    symptom: AnimatePresence does not animate exit — elements disappear instantly.
    cause: Direct children of AnimatePresence are missing unique and stable keys, or exit animation is not defined on the motion component.
    fix: Ensure every direct child of AnimatePresence has a unique key prop. Add exit prop with the desired animation to each motion child. Use mode='wait' if children should not overlap during transition.
  - id: failure:motion/gesture-not-firing
    symptom: whileHover, whileTap, or whileDrag props have no effect.
    cause: The motion component lacks required CSS (position, touch-action) or the gesture is blocked by parent event handlers.
    fix: "Ensure the element has position: relative (for drag). Set touch-action: none for drag on mobile. Verify no parent stops event propagation. For whileHover on non-interactive elements, add style={{ cursor: 'pointer' }}."
  - id: failure:motion/scroll-animation-precision
    symptom: Scroll-linked animations stutter or snap to end position.
    cause: useTransform output extrapolated beyond the input range, or scroll progress calculation has rounding errors.
    fix: "Set extrapolate: 'clamp' on useTransform to prevent values outside the output range. Use useScroll with explicit offset: ['start start', 'end end'] for precise viewport-relative tracking."
extends: []
implements: []
uses:
  - concept:react
  - concept:css-transforms
  - concept:web-animation
part_of: concept:react-animation-ecosystem
solves:
  - problem:declarative-react-animation
  - problem:layout-transitions
  - problem:gesture-driven-ui
alternatives:
  - package:gsap
  - package:anime-js
  - package:css-animations
  - package:react-spring
---
Motion (formerly Framer Motion, now standalone as the motion library) brings animation into React's declarative model. Instead of imperatively creating tweens and timelines, you declare what you want with props: animate={{ x: 100 }} means "animate x to 100 when this component mounts." The library handles the imperative Web Animations API and CSS transition orchestration underneath.

The layout animation system is Motion's killer feature. By adding the layout prop to a motion component, any change in its position or size (due to reorder, sibling removal, or style change) is automatically animated. The layoutId prop enables shared-element transitions — two different components sharing a layoutId animate smoothly between each other, enabling photo grid to detail view, list to card, and tab transitions without manual calculation.

Motion's architecture separates concerns through three layers:
1. **motion components** — the declarative React API
2. **MotionValues** — reactive animation primitives that bridge React and the animation engine
3. **useAnimate** — imperative escape hatch for complex sequences
This layered approach means simple cases are one-liners (motion.div animate={{ x: 100 }}), while complex cases (choreographed multi-element sequences with scroll input) compose the same primitives through useScroll, useTransform, and useSpring.
