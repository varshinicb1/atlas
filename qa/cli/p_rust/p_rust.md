---
kind: Package
id: package:p_rust
name: p rust
version: "0.1.0"
purpose: |
  Knowledge package covering Rust engineering patterns for p rust.
problem_solved: |
  Helps Rust engineers choose the right error handling, concurrency, and
  type system patterns for their domain.
install: |
  ```bash
  atlas install p_rust.md
  ```
concepts:
  - name: Error Handling
    id: concept:error_handling_p_rust
    description: |
      TODO: Define error handling patterns.
      Result<T, E>, anyhow, thiserror, custom error types.
  - name: Async Patterns
    id: concept:async_p_rust
    description: |
      TODO: Define async/await patterns.
      Tokio, async-std, futures, channels, select!
  - name: Type System
    id: concept:types_p_rust
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
