---
kind: Package
id: package:go/patterns
name: Go Engineering Patterns
version: "1.23"
purpose: Document idiomatic Go patterns for concurrent, networked services — goroutines, channels, error handling, interfaces, and modules.
problem_solved: Provides a reference for Go's composition-over-inheritance model, explicit error handling, and CSP-style concurrency, reducing deadlocks, goroutine leaks, and nil-interface panics.
install: go get github.com/spf13/cobra
dependencies:
  - package:go/cobra
concepts:
  - name: Go Ecosystem
    id: concept:domain/go-ecosystem
    description: The broader Go language, toolchain, and module ecosystem these patterns operate within.
  - name: Goroutines
    id: concept:go/goroutines
    description: Lightweight, multiplexed threads of execution scheduled by the Go runtime; launched with the go keyword and costing only a few KB of stack.
  - name: Channels
    id: concept:go/channels
    description: Typed conduits for communicating between goroutines, supporting both buffered and unbuffered (synchronous) semantics and select-based multiplexing.
  - name: Error Handling
    id: concept:go/error-handling
    description: Explicit, value-based errors returned as the last function result; the error interface and errors.Is/As for wrapping and inspection.
  - name: Interfaces
    id: concept:go/interfaces
    description: Implicit, structural contracts satisfied by any type implementing the methods; enable duck typing and dependency inversion without declaration.
  - name: Context
    id: concept:go/context
    description: A request-scoped value/cancel/deadline carrier passed through call chains to propagate cancellation and timeouts across goroutines.
  - name: Modules
    id: concept:go/modules
    description: Versioned dependency units declared in go.mod (module path, go version, require directives) replacing GOPATH-based development.
  - name: Struct Embedding
    id: concept:go/embedding
    description: Anonymous fields that promote methods and fields, providing composition-based code reuse analogous to mixins.
  - name: Defer
    id: concept:go/defer
    description: Schedules a function call to run when the enclosing function returns; used for unlock, close, and cleanup in LIFO order.
  - name: WaitGroup
    id: concept:go/waitgroup
    description: A counter that blocks until a collection of goroutines finishes; Add, Done, Wait coordinate fan-out work.
  - name: sync.Mutex
    id: concept:go/mutex
    description: Mutual-exclusion lock for protecting shared memory across goroutines; must not be copied after first use.
  - name: Select
    id: concept:go/select
    description: Like switch for channels — waits on multiple channel operations and proceeds with the first ready case (or default for non-blocking).
  - name: nil Interface
    id: concept:go/nil-interface
    description: A typed-nil value (e.g., (*T)(nil) wrapped in an interface) is not equal to a true nil interface, a subtle comparison trap.
  - name: Generics
    id: concept:go/generics
    description: Type parameters (Go 1.18+) enabling typed containers and functions with constraints, reducing unsafe interface{} usage.
  - name: Testing
    id: concept:go/testing
    description: The built-in testing package with table-driven tests, t.Run subtests, and httptest for HTTP handlers.
apis:
  - name: go func
    id: api:go/go-func
    signature: "go f(args)"
    returns: void (runs concurrently)
    description: Spawns a goroutine executing f; the caller proceeds immediately without waiting for completion.
  - name: make(chan T)
    id: api:go/make-chan
    signature: "ch := make(chan int, 0)  // or make(chan int, 16)"
    returns: A channel.
    description: Creates an unbuffered channel (synchronous hand-off) or buffered channel of given capacity.
  - name: errors.New / fmt.Errorf
    id: api:go/errors
    signature: "errors.New(\"boom\") / fmt.Errorf(\"wrap: %w\", err)"
    returns: An error value.
    description: Constructs errors; %w wraps an underlying error for later errors.Is/As inspection.
  - name: context.WithTimeout
    id: api:go/context-timeout
    signature: "ctx, cancel := context.WithTimeout(parent, 5*time.Second)"
    returns: A derived context and a cancel function.
    description: Returns a context that is cancelled after the timeout; always call cancel to release resources.
  - name: defer
    id: api:go/defer-stmt
    signature: "defer f.Close()"
    returns: void (runs at function exit)
    description: Registers f to run when the function returns; arguments are evaluated immediately.
  - name: sync.WaitGroup
    id: api:go/waitgroup-api
    signature: "var wg sync.WaitGroup; wg.Add(1); defer wg.Done(); wg.Wait()"
    returns: void
    description: Waits for spawned goroutines to call Done, blocking at Wait until the counter hits zero.
  - name: select
    id: api:go/select-stmt
    signature: "select { case v := <-ch: ...; default: ... }"
    returns: void
    description: Blocks until one case is ready; default makes it non-blocking, time.After adds timeouts.
  - name: range over channel
    id: api:go/range-chan
    signature: "for v := range ch { ... }"
    returns: void
    description: Iterates received values until the channel is closed, then exits the loop.
  - name: sync.Once
    id: api:go/once
    signature: "var o sync.Once; o.Do(setup)"
    returns: void
    description: Guarantees the function runs exactly once even under concurrent invocation; ideal for lazy singletons.
examples:
  - id: example:go/worker-pool
    language: go
    description: Fan-out worker pool consuming jobs from a buffered channel.
  - id: example:go/context-cancel
    language: go
    description: Propagating cancellation through an HTTP request tree with context.
  - id: example:go/error-wrap
    language: go
    description: Wrapping and inspecting errors with %w, errors.Is, errors.As.
  - id: example:go/table-test
    language: go
    description: Table-driven test with t.Run subtests.
failures:
  - id: failure:go/goroutine-leak
    symptom: Memory grows over time and goroutines never exit; pprof shows ever-increasing goroutine count.
    cause: A goroutine blocked forever on a channel with no sender/receiver, or a context that is never cancelled.
    fix: Always provide a cancellation path (context.WithCancel/Timeout), ensure channels are closed, and use errgroup for bounded fan-out.
  - id: failure:go/deadlock
    symptom: "fatal error: all goroutines are asleep - deadlock!"
    cause: Two or more goroutines waiting on each other (e.g., unbuffered channel sends with no receiver, or lock ordering inversion).
    fix: Use buffered channels or launch receivers first; establish a consistent lock acquisition order; add timeouts.
  - id: failure:go/nil-interface-panic
    symptom: "\"if err != nil\" but later type assertion panics, or a returned interface is unexpectedly non-nil."
    cause: Returning a typed nil pointer as an interface value; the interface is non-nil because it carries a type.
    fix: Return a true nil interface (var err error) or check the concrete value before wrapping; avoid returning concrete nil as interface.
  - id: failure:go/unclosed-body
    symptom: File descriptor exhaustion or connection reuse failures after many HTTP calls.
    cause: Forgetting resp.Body.Close() on net/http responses.
    fix: Always defer resp.Body.Close() (after a nil check) or wrap with defer and io.Copy(io.Discard, resp.Body).
  - id: failure:go/copied-mutex
    symptom: "\"Copying locks\" vet warning or lost synchronization and data races."
    cause: Copying a struct that contains a sync.Mutex or sync.WaitGroup by value.
    fix: Pass such structs by pointer (*sync.Mutex), or use sync.Once / atomic where copying semantics matter.
  - id: failure:go/module-version
    symptom: "\"go: cannot find main module\" or inconsistent dependency versions across machines."
    cause: Missing or misconfigured go.mod, or a missing go directive; proxy/GOPROXY misconfiguration.
    fix: Run go mod init, run go mod tidy, and pin GOFLAGS/ GOPROXY; commit go.sum.
  - id: failure:go/race-condition
    symptom: Flaky, non-deterministic data corruption only under load.
    cause: Unsynchronized concurrent access to shared memory across goroutines.
    fix: Run go test -race; protect with sync.Mutex, atomic, or communicate via channels (share memory by communicating).
extends:
  - concept:go/error-handling
uses:
  - concept:go/goroutines
  - concept:go/channels
  - concept:go/interfaces
  - concept:go/context
part_of: concept:domain/go-ecosystem
depends_on:
  - package:docker/kubernetes
solves:
  - problem:concurrent-network-services
alternatives:
  - package:rust/no-gc-latency
  - package:python/rapid-dev
  - package:java/ecosystem
---
# Go Engineering Patterns

Go's design philosophy favors simplicity and explicitness: a small language, composable concurrency, and no hidden control flow. Goroutines are the unit of concurrency — launched with the `go` keyword, they cost only a few kilobytes of stack and are scheduled cooperatively by the runtime's M:N scheduler onto OS threads. The idiomatic way to coordinate them is not shared memory but communicating sequential processes: channels (`make(chan T)`) transfer ownership of data between goroutines, embodying the motto "do not communicate by sharing memory; instead, share memory by communicating."

Channels come in unbuffered (synchronous hand-off, sender blocks until a receiver is ready) and buffered (capacity-limited, decouples producer and consumer) forms. `select` multiplexes over multiple channel operations, `time.After` adds timeouts, and iterating `for v := range ch` drains a channel until it is closed. Fan-out workloads typically combine a `sync.WaitGroup` (Add/Done/Wait) with a context for cancellation. The golden rule is to always give a goroutine an exit path — a cancelled context or a closed channel — or it leaks, accumulating memory and goroutines forever.

Error handling is explicit and value-based: functions return `error` as their last result and callers must check it. Wrapping with `fmt.Errorf("...: %w", err)` preserves the chain so `errors.Is` and `errors.As` can inspect root causes — crucial for structured logging. A notorious trap is the nil interface: a function returning a typed nil pointer (`(*T)(nil)`) as an `error` interface is NOT equal to a true `nil`, so `if err != nil` unexpectedly passes. Return a bare `nil` interface instead.

Interfaces in Go are satisfied implicitly — any type with the right method set qualifies, with no `implements` declaration. This enables dependency inversion and testability (mock the small interface, not the concrete type) and powers `io.Reader`/`io.Writer` ubiquity. `context.Context` threads cancellation, deadlines, and request-scoped values through call graphs, keeping long operations interruptible. Since Go 1.18, generics reduce the old `interface{}` casts in containers and utilities. Modules (go.mod) replaced GOPATH-based development, giving reproducible, versioned dependencies via `go mod tidy` and `go.sum`. Testing is built in: table-driven tests with `t.Run` subtests and `httptest` for handlers, with `go test -race` catching data races.
