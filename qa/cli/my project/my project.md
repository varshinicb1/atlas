---
kind: Package
id: package:my project
name: my project
version: "0.1.0"
purpose: |
  Knowledge package covering {domain} patterns and best practices for my project.
problem_solved: |
  Helps engineers choose the right architecture patterns, APIs, and workflows
  when building {domain} applications with my project.
install: |
  ```bash
  atlas install my project.md
  ```
concepts:
  - name: Core Architecture
    id: concept:architecture_my project
    description: |
      The fundamental architectural patterns for my project. Covers component
      organization, data flow, and dependency management.
  - name: API Surface
    id: concept:api_my project
    description: |
      The public API surface of my project. Includes function signatures,
      configuration options, and extension points.
apis:
  - name: initialize()
    id: api:initialize_my project
    signature: "Config initialize(Options opts)"
    returns: Config
    description: |
      Initializes the my project runtime with the given configuration.
      Must be called once before any other API.
  - name: configure()
    id: api:configure_my project
    signature: "void configure(Settings settings)"
    returns: void
    description: |
      Applies runtime configuration overrides. Can be called multiple times
      to update settings dynamically.
workflows:
  - name: Setup and Configuration
    id: workflow:setup_my project
    description: |
      Step-by-step guide to set up my project in a new project.
    steps:
      - order: 1
        action: "Install the my project package via your package manager"
      - order: 2
        action: "Call initialize() with your project configuration"
      - order: 3
        action: "Configure integration points with configure()"
      - order: 4
        action: "Verify setup by running the smoke tests"
