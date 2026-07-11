---
kind: Package
id: package:../evilpkg
name: ../evilpkg
version: "0.1.0"
purpose: |
  Knowledge package covering {domain} patterns and best practices for ../evilpkg.
problem_solved: |
  Helps engineers choose the right architecture patterns, APIs, and workflows
  when building {domain} applications with ../evilpkg.
install: |
  ```bash
  atlas install ../evilpkg.md
  ```
concepts:
  - name: Core Architecture
    id: concept:architecture_../evilpkg
    description: |
      The fundamental architectural patterns for ../evilpkg. Covers component
      organization, data flow, and dependency management.
  - name: API Surface
    id: concept:api_../evilpkg
    description: |
      The public API surface of ../evilpkg. Includes function signatures,
      configuration options, and extension points.
apis:
  - name: initialize()
    id: api:initialize_../evilpkg
    signature: "Config initialize(Options opts)"
    returns: Config
    description: |
      Initializes the ../evilpkg runtime with the given configuration.
      Must be called once before any other API.
  - name: configure()
    id: api:configure_../evilpkg
    signature: "void configure(Settings settings)"
    returns: void
    description: |
      Applies runtime configuration overrides. Can be called multiple times
      to update settings dynamically.
workflows:
  - name: Setup and Configuration
    id: workflow:setup_../evilpkg
    description: |
      Step-by-step guide to set up ../evilpkg in a new project.
    steps:
      - order: 1
        action: "Install the ../evilpkg package via your package manager"
      - order: 2
        action: "Call initialize() with your project configuration"
      - order: 3
        action: "Configure integration points with configure()"
      - order: 4
        action: "Verify setup by running the smoke tests"
