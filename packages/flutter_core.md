---
kind: Package
id: package:flutter_core
name: Flutter SDK
version: "3.29"
purpose: Provides the foundational framework for building natively compiled, multi-platform applications from a single codebase.
problem_solved: Enables developers to create high-performance, visually expressive, reactive UIs that run on mobile, web, and desktop from a single Dart codebase.
install: |
  ```yaml
  dependencies:
    flutter:
      sdk: flutter
  ```
dependencies:
  - "sdk:dart"
concepts:
  - name: Widget
    id: concept:widget
    description: The core building block of Flutter UI. Everything in Flutter is a Widget — a lightweight description of part of the user interface.
  - name: Element
    id: concept:element
    description: The instantiation of a Widget in the tree. Elements form a concrete tree that manages the lifecycle of the associated widget.
  - name: BuildContext
    id: concept:build_context
    description: The location of a widget in the widget tree. Used to locate parent widgets, theme data, and media queries.
  - name: StatefulWidget
    id: concept:stateful_widget
    description: A widget that has mutable state. Can rebuild itself when internal state changes via setState.
  - name: StatelessWidget
    id: concept:stateless_widget
    description: A widget with no mutable state. Its configuration is fully determined by constructor arguments and InheritedWidgets.
  - name: RenderObject
    id: concept:render_object
    description: The object that performs layout and painting. Abstracts the concrete rendering details from the widget layer.
  - name: InheritedWidget
    id: concept:inherited_widget
    description: A widget that propagates data down the tree efficiently. Descendants can opt-in to rebuilds when data changes.
  - name: Key
    id: concept:key
    description: Controls widget identity across rebuilds. Prevents state loss when widgets change position in the tree.
  - name: Flutter SDK
    id: "package:flutter"
    description: The Flutter SDK framework itself — the upstream package that flutter_core depends on.
apis:
  - name: setState(VoidCallback)
    id: api:set_state
    signature: "void setState(VoidCallback fn)"
    returns: void
    description: Marks the widget as dirty and schedules a rebuild with the new state.
  - name: build(BuildContext)
    id: api:build
    signature: "Widget build(BuildContext context)"
    returns: Widget
    description: Describes the UI for this widget. Called whenever the widget needs to repaint.
  - name: initState()
    id: api:init_state
    signature: "void initState()"
    returns: void
    description: Called when State is created. Used to initialize controllers, animation controllers, and listeners.
  - name: dispose()
    id: api:dispose
    signature: "void dispose()"
    returns: void
    description: Called when State is removed permanently. Used to clean up controllers and subscriptions.
  - name: didUpdateWidget(covariant T)
    id: api:did_update_widget
    signature: "void didUpdateWidget(covariant T oldWidget)"
    returns: void
    description: Called when the widget's configuration changes. Used to respond to new widget properties.
  - name: createState()
    id: api:create_state
    signature: "State createState()"
    returns: State
    description: Creates the mutable State for a StatefulWidget. Called once per location in the tree.
  - name: mounted
    id: api:mounted
    signature: "bool mounted"
    returns: bool
    description: Whether the State is currently in the tree. Must be checked before calling setState.
  - name: MediaQuery.of(context)
    id: api:media_query_of
    signature: "MediaQueryData of(BuildContext context)"
    returns: MediaQueryData
    description: Gets screen metrics (size, padding, text scaling) from the widget tree.
examples:
  - id: example:counter_app
    language: dart
    description: A minimal counter app demonstrating StatefulWidget, setState, and build.
  - id: example:inherited_theme
    language: dart
    description: Theme propagation using an InheritedWidget down the widget tree.
failures:
  - id: failure:set_state_after_dispose
    symptom: "State throws an exception when setState is called after the widget has been removed from the tree."
    cause: "An asynchronous callback (e.g., future.then, timer callback) completes after the widget has been disposed."
    fix: "Check mounted before calling setState"
  - id: failure:context_after_async_gap
    symptom: "A BuildContext is used after an asynchronous gap."
    cause: "The BuildContext becomes unsafe after async boundaries since the widget may have been removed."
    fix: "Capture required data synchronously or check mounted before using context after await."
  - id: failure:unbounded_animation_controllers
    symptom: "AnimationController leaks resources and continues ticking indefinitely."
    cause: "AnimationController is created in initState but never disposed."
    fix: "Always call controller.dispose() in the dispose method of the State."
extends:
  - "concept:widget"
implements: []
uses: []
part_of: "package:flutter"
solves:
  - "problem:cross_platform_reactive_ui"
alternatives:
  - "framework:react_native"
---

# Flutter SDK Core

Flutter is an open-source UI framework by Google for building natively compiled applications for mobile, web, and desktop from a single Dart codebase. The core of Flutter is the widget system — a composable, reactive tree of lightweight objects that describe the UI configuration.

## Widget Tree and Element Tree

Every Flutter app is built from widgets arranged in a tree. Each widget has a corresponding `Element` that manages its lifecycle and position in the tree. The `BuildContext` is the handle to the element's location and is passed to the `build()` method.

## Stateful vs Stateless

Use `StatelessWidget` when the UI depends only on its configuration and inherited data. Use `StatefulWidget` when the widget needs mutable state that changes over time. Always call `dispose()` on controllers, streams, and animation controllers to avoid memory leaks.