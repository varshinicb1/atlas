---
kind: Package
id: package:unktmpl
name: unktmpl
version: "0.1.0"
purpose: |
  Knowledge package covering {domain} patterns and best practices for unktmpl.
problem_solved: |
  Helps engineers choose the right architecture patterns, APIs, and workflows
  when building {domain} applications with unktmpl.
install: |
  ```bash
  atlas install unktmpl.md
  ```
concepts:
  - name: Core Architecture
    id: concept:architecture_unktmpl
    description: |
      The fundamental architectural patterns for unktmpl. Covers component
      organization, data flow, and dependency management.
  - name: API Surface
    id: concept:api_unktmpl
    description: |
      The public API surface of unktmpl. Includes function signatures,
      configuration options, and extension points.
apis:
  - name: initialize()
    id: api:initialize_unktmpl
    signature: "Config initialize(Options opts)"
    returns: Config
    description: |
      Initializes the unktmpl runtime with the given configuration.
      Must be called once before any other API.
  - name: configure()
    id: api:configure_unktmpl
    signature: "void configure(Settings settings)"
    returns: void
    description: |
      Applies runtime configuration overrides. Can be called multiple times
      to update settings dynamically.
workflows:
  - name: Setup and Configuration
    id: workflow:setup_unktmpl
    description: |
      Step-by-step guide to set up unktmpl in a new project.
    steps:
      - order: 1
        action: "Install the unktmpl package via your package manager"
      - order: 2
        action: "Call initialize() with your project configuration"
      - order: 3
        action: "Configure integration points with configure()"
      - order: 4
        action: "Verify setup by running the smoke tests"
