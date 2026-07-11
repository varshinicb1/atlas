---
kind: Package
id: package:flutter/riverpod
name: Riverpod
version: 2.5.1
purpose: Compile-time safe state management for Flutter
problem_solved: Reactive state management with provider scoping and disposal
install: flutter pub add riverpod
dependencies:
  - concept:flutter/widgets
concepts:
  - name: State Management
    id: concept:flutter/state_management
    description: Pattern for managing and sharing application state across widgets with predictable updates and lifecycle management
  - name: Provider
    id: concept:flutter/provider
    description: The core building block that creates, reads, and disposes state objects with compile-time safety and auto-disposal
  - name: Notifier
    id: concept:flutter/notifier
    description: A synchronous state provider exposing mutable state via a class-based API replacing the older StateNotifier pattern
  - name: AsyncNotifier
    id: concept:flutter/async_notifier
    description: An asynchronous state provider for initializing and managing state that depends on async operations like API calls or database reads
  - name: ConsumerWidget
    id: concept:flutter/consumer_widget
    description: A StatelessWidget subclass that gains access to WidgetRef for watching and reading providers without needing a BuildContext
  - name: WidgetRef
    id: concept:flutter/widget_ref
    description: The reference object passed to consumers providing watch, read, invalidate, and listen methods for interacting with providers
  - name: ProviderContainer
    id: concept:flutter/provider_container
    description: The top-level container that holds all provider instances and their states, enabling provider override for testing
  - name: Reactive State Problem
    id: problem:flutter/reactive_state
    description: Challenge of managing UI state that reactively updates across widget boundaries without manual propagation
  - name: Flutter Widgets Framework
    id: concept:flutter/widgets
    description: The core Flutter widget framework providing the build context, element tree, and rendering pipeline
  - name: Provider (legacy)
    id: package:flutter/provider
    description: The legacy Provider package for dependency injection and state management in Flutter
  - name: BLoC Pattern
    id: package:flutter/bloc
    description: Business Logic Component pattern for Flutter using streams and events for predictable state management
  - name: GetIt
    id: package:flutter/get_it
    description: Service locator pattern for Flutter providing simple dependency injection without a widget tree dependency
apis:
  - name: ref.watch()
    id: api:flutter/riverpod/watch
    signature: WidgetRef.watch<T>(ProviderBase<T>) → T
    returns: T
    description: Returns the current state and subscribes to changes
  - name: ref.read()
    id: api:flutter/riverpod/read
    signature: WidgetRef.read<T>(ProviderBase<T>) → T
    returns: T
    description: Returns the current state without subscribing
  - name: ref.invalidate()
    id: api:flutter/riverpod/invalidate
    signature: WidgetRef.invalidate<T>(ProviderBase<T>) → void
    description: Invalidates a provider, causing it to rebuild
  - name: ref.listen()
    id: api:flutter/riverpod/listen
    signature: WidgetRef.listen<T>(ProviderBase<T>, void Function(T?, T))
    description: Listens to state changes without rebuilding
  - name: Notifier.build()
    id: api:flutter/riverpod/notifier_build
    signature: Notifier<T>.build() → T
    returns: T
    description: Initializes the notifier state
  - name: AsyncNotifier.build()
    id: api:flutter/riverpod/async_notifier_build
    signature: AsyncNotifier<T>.build() → Future<T>
    returns: Future<T>
    description: Initializes the async notifier state
examples:
  - id: example:flutter/riverpod/counter
    language: dart
    description: Basic counter using StateNotifier
  - id: example:flutter/riverpod/async_user
    language: dart
    description: Async user fetch using AsyncNotifier
failures:
  - id: failure:flutter/riverpod/provider_disposed
    symptom: Tried to use a provider after it was disposed
    cause: Provider was invalidated while still being watched
    fix: Ensure the provider's lifetime matches its usage scope
  - id: failure:flutter/riverpod/cyclic_dependency
    symptom: Stack overflow / infinite recursion
    cause: Two providers depend on each other
    fix: Refactor to avoid circular provider dependencies
part_of: concept:flutter/state_management
extends:
  - concept:flutter/provider
solves:
  - problem:flutter/reactive_state
alternatives:
  - package:flutter/provider
  - package:flutter/bloc
  - package:flutter/get_it
---

# Riverpod

Riverpod is a compile-time safe, testable state management library for Flutter.
It supports synchronous and asynchronous providers, auto-disposal, and
family modifiers for parameterized providers.

## Notifier

A `Notifier` is the recommended way to create a provider that exposes
mutable state. It replaces the older `StateNotifier` pattern.

```dart
class CounterNotifier extends Notifier<int> {
  @override
  int build() => 0;

  void increment() => state++;
}

final counterProvider = NotifierProvider<CounterNotifier, int>(
  CounterNotifier.new,
);
```

## AsyncNotifier

`AsyncNotifier` extends `Notifier` for asynchronous initialization.
Use it when the provider needs to fetch data (API calls, database reads).

```dart
class UserNotifier extends AsyncNotifier<User> {
  @override
  Future<User> build() async {
    final id = ref.read(userIdProvider);
    return await fetchUser(id);
  }
}

final userProvider = AsyncNotifierProvider<UserNotifier, User>(
  UserNotifier.new,
);
```

## ConsumerWidget

Use `ConsumerWidget` instead of `StatelessWidget` when you need to
watch providers.

```dart
class UserProfile extends ConsumerWidget {
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final userAsync = ref.watch(userProvider);
    return userAsync.when(
      data: (user) => Text(user.name),
      loading: () => CircularProgressIndicator(),
      error: (e, _) => Text('Error: $e'),
    );
  }
}
```
