---
kind: Package
id: package:p_flutter
name: p flutter
version: "0.1.0"
purpose: |
  Knowledge package covering Flutter and Dart patterns for p flutter.
problem_solved: |
  Helps Flutter engineers choose the right widgets, state management approaches,
  and architectural patterns for their use case.
install: |
  ```bash
  atlas install p_flutter.md
  ```
concepts:
  - name: Widget
    id: concept:widget_p_flutter
    description: |
      TODO: Define a Flutter-specific widget concept.
      Widgets are the building blocks of Flutter UI.
  - name: State Management
    id: concept:state_p_flutter
    description: |
      TODO: Define state management approaches for your domain.
      setState, Provider, Riverpod, BLoC, etc.
  - name: Navigation
    id: concept:navigation_p_flutter
    description: |
      TODO: Define navigation patterns. GoRouter, Navigator 2.0, etc.
apis:
  - name: build(BuildContext)
    id: api:build
    signature: "Widget build(BuildContext context)"
    returns: Widget
    description: |
      The required method for all widgets. Returns the widget tree
      for the given build context.
workflows:
  - name: Stateful Widget Creation
    id: workflow:create_stateful
    description: Steps to create a stateful widget in Flutter.
    steps:
      - order: 1
        action: "Create a class extending StatefulWidget"
      - order: 2
        action: "Override createState() to return a State subclass"
      - order: 3
        action: "Override build() to describe the UI"
      - order: 4
        action: "Call setState() to trigger rebuilds on state changes"
