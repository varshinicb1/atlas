pub const FLUTTER_PACKAGE_MD: &str = r#"---
kind: Package
id: package:__NAME__
name: __TITLE__
version: "0.1.0"
purpose: |
  Knowledge package covering Flutter and Dart patterns for __TITLE__.
problem_solved: |
  Helps Flutter engineers choose the right widgets, state management approaches,
  and architectural patterns for their use case.
install: |
  ```bash
  atlas install __NAME__.md
  ```
concepts:
  - name: Widget
    id: concept:widget___NAME__
    description: |
      TODO: Define a Flutter-specific widget concept.
      Widgets are the building blocks of Flutter UI.
  - name: State Management
    id: concept:state___NAME__
    description: |
      TODO: Define state management approaches for your domain.
      setState, Provider, Riverpod, BLoC, etc.
  - name: Navigation
    id: concept:navigation___NAME__
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
"#;

pub const RUST_PACKAGE_MD: &str = r#"---
kind: Package
id: package:__NAME__
name: __TITLE__
version: "0.1.0"
purpose: |
  Knowledge package covering Rust engineering patterns for __TITLE__.
problem_solved: |
  Helps Rust engineers choose the right error handling, concurrency, and
  type system patterns for their domain.
install: |
  ```bash
  atlas install __NAME__.md
  ```
concepts:
  - name: Error Handling
    id: concept:error_handling___NAME__
    description: |
      TODO: Define error handling patterns.
      Result<T, E>, anyhow, thiserror, custom error types.
  - name: Async Patterns
    id: concept:async___NAME__
    description: |
      TODO: Define async/await patterns.
      Tokio, async-std, futures, channels, select!
  - name: Type System
    id: concept:types___NAME__
    description: |
      TODO: Define type system patterns.
      Enums, generics, trait bounds, associated types.
apis:
  - name: Result<T, E>
    id: api:result
    signature: "enum Result<T, E> { Ok(T), Err(E) }"
    returns: T or propagated error
    description: |
      The standard Rust type for fallible operations.
      Use with ? operator for ergonomic error propagation.
workflows:
  - name: Error Handling Strategy
    id: workflow:errors
    description: Steps to design robust error handling.
    steps:
      - order: 1
        action: "Define domain-specific error types using thiserror"
      - order: 2
        action: "Impl From for each external error type"
      - order: 3
        action: "Return Result<T, MyError> from public functions"
      - order: 4
        action: "Use anyhow::Result in application code for simplicity"
"#;

pub const TYPESCRIPT_PACKAGE_MD: &str = r#"---
kind: Package
id: package:__NAME__
name: __TITLE__
version: "0.1.0"
purpose: |
  Knowledge package covering TypeScript and Node.js patterns for __TITLE__.
problem_solved: |
  Helps TypeScript engineers choose the right patterns for type safety,
  async control flow, and project structure.
install: |
  ```bash
  atlas install __NAME__.md
  ```
concepts:
  - name: Type System
    id: concept:types___NAME__
    description: |
      TODO: Define TypeScript type patterns.
      Generics, conditional types, mapped types, template literals.
  - name: Async Patterns
    id: concept:async___NAME__
    description: |
      TODO: Define async patterns. Promises, async/await, streams, event emitters.
  - name: Module Architecture
    id: concept:modules___NAME__
    description: |
      TODO: Define module organization. Barrel files, feature modules,
      dependency injection patterns.
apis:
  - name: fetch()
    id: api:fetch
    signature: "fetch(input: RequestInfo, init?: RequestInit): Promise<Response>"
    returns: Promise<Response>
    description: |
      The modern Web API for HTTP requests. Available in Node 18+, browsers, and edge runtimes.
workflows:
  - name: API Route Creation (Next.js)
    id: workflow:api_route
    description: Steps to create a Next.js API route.
    steps:
      - order: 1
        action: "Create app/api/{route}/route.ts"
      - order: 2
        action: "Export GET/POST/PUT/DELETE handlers"
      - order: 3
        action: "Parse request body and params"
      - order: 4
        action: "Return NextResponse.json()"
"#;

pub const PYTHON_PACKAGE_MD: &str = r#"---
kind: Package
id: package:__NAME__
name: __TITLE__
version: "0.1.0"
purpose: |
  Knowledge package covering Python patterns for __TITLE__.
problem_solved: |
  Helps Python engineers choose the right patterns for project structure,
  async programming, and dependency management.
install: |
  ```bash
  atlas install __NAME__.md
  ```
concepts:
  - name: Async/Await
    id: concept:async___NAME__
    description: |
      TODO: Define async Python patterns.
      asyncio, anyio, httpx, async generators.
  - name: Pydantic Models
    id: concept:pydantic___NAME__
    description: |
      TODO: Define data modeling patterns.
      BaseModel, validators, serialization, JSON schema generation.
apis:
  - name: async def
    id: api:async_def
    signature: "async def function_name(args) -> ReturnType:"
    returns: Coroutine
    description: |
      Python async function declaration. Must be awaited by the caller.
      Run with asyncio.run() or within an async context.
workflows:
  - name: FastAPI Project Setup
    id: workflow:fastapi
    description: Steps to set up a FastAPI project.
    steps:
      - order: 1
        action: "Install: pip install fastapi uvicorn"
      - order: 2
        action: "Create app/ package with main.py, models.py, routes/"
      - order: 3
        action: "Define Pydantic models for request/response schemas"
      - order: 4
        action: "Create route handlers with type annotations"
      - order: 5
        action: "Run: uvicorn app.main:app --reload"
"#;

pub const KNOWLEDGE_PACKAGE_MD: &str = r#"---
kind: Package
id: package:__NAME__
name: __TITLE__
version: "0.1.0"
purpose: |
  Knowledge package covering {domain} patterns and best practices for __TITLE__.
problem_solved: |
  Helps engineers choose the right architecture patterns, APIs, and workflows
  when building {domain} applications with __TITLE__.
install: |
  ```bash
  atlas install __NAME__.md
  ```
concepts:
  - name: Core Architecture
    id: concept:architecture___NAME__
    description: |
      The fundamental architectural patterns for __TITLE__. Covers component
      organization, data flow, and dependency management.
  - name: API Surface
    id: concept:api___NAME__
    description: |
      The public API surface of __TITLE__. Includes function signatures,
      configuration options, and extension points.
apis:
  - name: initialize()
    id: api:initialize___NAME__
    signature: "Config initialize(Options opts)"
    returns: Config
    description: |
      Initializes the __TITLE__ runtime with the given configuration.
      Must be called once before any other API.
  - name: configure()
    id: api:configure___NAME__
    signature: "void configure(Settings settings)"
    returns: void
    description: |
      Applies runtime configuration overrides. Can be called multiple times
      to update settings dynamically.
workflows:
  - name: Setup and Configuration
    id: workflow:setup___NAME__
    description: |
      Step-by-step guide to set up __TITLE__ in a new project.
    steps:
      - order: 1
        action: "Install the __TITLE__ package via your package manager"
      - order: 2
        action: "Call initialize() with your project configuration"
      - order: 3
        action: "Configure integration points with configure()"
      - order: 4
        action: "Verify setup by running the smoke tests"
"#;

pub const DECISION_TREE_YAML: &str = r#"id: __NAME___decision
trigger:
  intent: choose_solution
  tags: [__NAME__]
root: start_node
nodes:
  - id: start_node
    question: "TODO: Define the yes/no question to start the decision tree?"
    node_type: boolean
    branches:
      - condition: "answer=true"
        next: terminal_yes
      - condition: "answer=false"
        next: terminal_no
  - id: terminal_yes
    terminal:
      recommendation:
        - node_id: "concept:solution_a"
          confidence: 0.8
      rationale: "Recommendation based on {domain} best practices."
  - id: terminal_no
    terminal:
      recommendation:
        - node_id: "concept:solution_b"
          confidence: 0.9
      rationale: "Recommendation based on {domain} best practices."
"#;

pub const TYPESCRIPT_7_PACKAGE_MD: &str = r#"---
kind: Package
id: package:__NAME__
name: __TITLE__
version: "7.0.0"
purpose: |
  Knowledge package capturing TypeScript 7 (Project Corsa) migration patterns,
  breaking changes, and performance tuning for __TITLE__.
problem_solved: |
  Helps TypeScript teams plan and execute the migration from TS 5/6 to TS 7,
  covering strict mode fixes, parallel checking flags, module resolution changes,
  and CI pipeline optimization.
install: |
  ```bash
  atlas install __NAME__.md
  npx typescript@7 init
  ```
concepts:
  - name: Project Corsa
    id: concept:corsa___NAME__
    description: |
      TODO: Document specific aspects of TS 7's Go-native compiler relevant
      to your project. Include flag changes, behavioral differences, and
      migration notes.
  - name: Strict Mode Impact
    id: concept:strict___NAME__
    description: |
      TODO: Document how strict mode being default affects your codebase.
      List the specific strict checks you need to address.
  - name: Parallel Checking
    id: concept:parallel___NAME__
    description: |
      TODO: Document your --checkers and --builders configuration.
      Include CI-specific tuning recommendations.
apis:
  - name: tsc --checkers
    id: api:checkers
    signature: "tsc --checkers <N>"
    returns: Exit code 0 on success
    description: |
      Parallel type checker. N=0 auto-detects CPU count.
      Recommended CI value: N=CORES-1.
  - name: tsc --builders
    id: api:builders
    signature: "tsc --build --builders <N>"
    returns: Exit code 0 on success
    description: |
      Parallel project reference builder. Only meaningful with --build.
workflows:
  - name: Migration Assessment for __TITLE__
    id: workflow:assess___NAME__
    description: Assess migration scope and plan execution.
    steps:
      - order: 1
        action: "Run npx typescript@7 init to detect deprecated settings"
      - order: 2
        action: "Count strict mode errors with tsc --noEmit --strict"
      - order: 3
        action: "Estimate effort: 1 hour per 10K lines for strict fixes"
      - order: 4
        action: "Plan CI pipeline update with --checkers flag"
"#;

pub const GITIGNORE: &str = r#"# Atlas build artifacts
*.atlas
*.ir.json

# Python
__pycache__/
*.egg-info/
"#;
