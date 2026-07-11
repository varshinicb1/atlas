---
kind: Package
id: package:rust/patterns
name: Rust Engineering Patterns
version: "1.0"
purpose: Document idiomatic Rust programming patterns for production systems
problem_solved: Provides a reference for Rust's unique ownership-based memory safety, zero-cost abstractions, and concurrency model
install: cargo add tokio serde serde_json
dependencies:
  - package:tokio
  - package:serde
  - package:serde_json
concepts:
  - name: Error Handling
    id: concept:rust/error-handling
    description: Idiomatic error patterns using Result/Option with the ? operator and thiserror/anyhow crates
  - name: Async Programming
    id: concept:rust/async
    description: tokio-based async/await model for cooperative multitasking and non-blocking I/O
  - name: Serialization
    id: concept:rust/serde
    description: serde framework providing derive macros for JSON, binary, and custom format serialization
  - name: Ownership & Borrowing
    id: concept:rust/ownership-borrowing
    description: Core memory safety model enforcing single ownership and controlled borrowing at compile time
  - name: Lifetimes
    id: concept:rust/lifetimes
    description: Compile-time tracking of reference validity to prevent dangling pointers
  - name: Traits & Generics
    id: concept:rust/traits-generics
    description: Polymorphism via trait bounds and generic parameters without runtime overhead
  - name: Pattern Matching
    id: concept:rust/pattern-matching
    description: Exhaustive destructuring with match, if let, and while let for control flow
  - name: Smart Pointers
    id: concept:rust/smart-pointers
    description: Box, Rc, Arc, Cell, RefCell providing heap allocation, reference counting, and interior mutability
  - name: Systems Programming
    id: concept:domain/systems-programming
    description: The broader domain of systems-level software development that Rust excels at.
apis:
  - name: Result<T, E>
    id: api:rust/result
    signature: "enum Result<T, E> { Ok(T), Err(E) }"
    description: Recoverable error handling enum with Ok and Err variants
  - name: Option<T>
    id: api:rust/option
    signature: "enum Option<T> { Some(T), None }"
    description: Optional value handling enum for nullable values without null pointers
  - name: "? operator"
    id: api:rust/question-operator
    signature: "expr?"
    description: Early propagation of Err or None from a function returning Result or Option
  - name: async fn / .await
    id: api:rust/async-await
    signature: "async fn foo() -> T { ... }; foo().await"
    description: Asynchronous function syntax with .await suspension points for non-blocking execution
  - name: "#[derive(Serialize, Deserialize)]"
    id: api:rust/serde-derive
    signature: "#[derive(Serialize, Deserialize)] struct Foo { ... }"
    description: Auto-derive serde Serialize and Deserialize trait implementations on structs and enums
  - name: match expression
    id: api:rust/match
    signature: "match value { pattern => expr, ... }"
    description: Exhaustive pattern matching with arm guards and destructuring
  - name: impl Trait for Type
    id: api:rust/impl-trait
    signature: "impl TraitName for TypeName { fn method(&self) { ... } }"
    description: Implementing traits on types to enable shared behavior and polymorphism
  - name: dyn Trait
    id: api:rust/dyn-trait
    signature: "Box<dyn Trait>"
    description: Dynamic dispatch via trait objects for runtime polymorphism
  - name: Box::new()
    id: api:rust/box-new
    signature: "let b = Box::new(value);"
    description: Heap allocation returning an owned pointer for recursive types and trait objects
  - name: Arc::new() / Mutex::new()
    id: api:rust/arc-mutex
    signature: "let shared = Arc::new(Mutex::new(value));"
    description: Thread-safe reference counting with interior mutability for shared mutable state across threads
examples:
  - id: example:rust/result-usage
    language: rust
    description: Propagating errors with Result and ? in a file reader
  - id: example:rust/async-tokio
    language: rust
    description: Async HTTP request with tokio and reqwest
  - id: example:rust/serde-json
    language: rust
    description: Serializing and deserializing a struct to/from JSON
failures:
  - id: failure:rust/unwrap-none
    symptom: Runtime panic with "called Option::unwrap() on a None value"
    cause: Calling .unwrap() on an Option or Result that contains None or Err
    fix: Use pattern matching with match or if let, or combinators like unwrap_or, unwrap_or_else, and expect
  - id: failure:rust/mutex-poisoning
    symptom: PoisonError when attempting to lock a Mutex
    cause: Another thread panicked while holding the Mutex lock, marking it as poisoned
    fix: Use .lock().unwrap_or_else(|e| e.into_inner()) to recover, or handle with .is_poisoned() checks
  - id: failure:rust/async-deadlock
    symptom: Task hangs indefinitely with no progress
    cause: Improperly ordered async locks or mixed sync Mutex across .await points
    fix: Use tokio::sync::Mutex instead of std::sync::Mutex across .await boundaries; enforce consistent lock ordering
  - id: failure:rust/lifetime-mismatch
    symptom: Compile error 'borrowed value does not live long enough'
    cause: Returning a reference to a local variable or storing a reference that outlives its source
    fix: Adjust lifetime annotations, change ownership semantics, or use Arc instead of references
  - id: failure:rust/orphan-rule
    symptom: Compile error 'only traits defined in the current crate can be implemented for arbitrary types'
    cause: Implementing a foreign trait on a foreign type violates the orphan rule
    fix: Use the newtype pattern to wrap the foreign type in a local struct, then implement the trait on the wrapper
  - id: failure:rust/iterator-invalidation
    symptom: Compile error 'cannot borrow *self as mutable because it is also borrowed as immutable'
    cause: Holding a reference into a collection while mutating the same collection
    fix: Restructure code to avoid holding references across mutation boundaries, or collect iteration results first
extends:
  - concept:rust/ownership-borrowing
uses:
  - concept:rust/lifetimes
  - concept:rust/async
  - concept:rust/serde
part_of: concept:domain/systems-programming
solves:
  - problem:memory-safe-systems-programming
alternatives:
  - package:rust-design-patterns
  - package:rust-api-guidelines
---

# Rust Engineering Patterns

Rust's engineering patterns center on ownership and borrowing--a compile-time memory safety model that eliminates entire classes of bugs like use-after-free, double-free, and data races without needing a garbage collector. Every value in Rust has exactly one owner at any time, and references follow strict borrowing rules: either one mutable reference or any number of immutable references. This ownership discipline, combined with the Result<T, E> and Option<T> enums, forces engineers to handle the failure and absence paths explicitly, making programs more robust by construction.

The async model built on tokio provides cooperative multitasking where .await points act as explicit yield boundaries. Unlike thread-per-request models, async Rust allows massive concurrency with minimal overhead, but requires care: mixing std::sync::Mutex across .await calls can cause deadlocks, favoring tokio::sync::Mutex instead. The type system's trait-based generics and dyn dispatch offer zero-cost polymorphism--trait bounds are monomorphized at compile time--while the serde framework uses derive macros to generate efficient serialization and deserialization code automatically.

Rust's pattern matching with exhaustive match expressions and if let bindings enables safe, expressive destructuring of enums, structs, and references. Smart pointers like Box<T> for heap allocation, Arc<T> for thread-safe reference counting, and RefCell<T> for interior mutability give fine-grained control over memory layout and sharing semantics. Together, these patterns solve memory-safe systems programming with zero-cost abstractions, making Rust suitable for everything from embedded firmware to web services.