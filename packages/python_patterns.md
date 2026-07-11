---
kind: Package
id: package:python/patterns
name: Python Engineering Patterns
version: "3.12"
purpose: Document idiomatic Python patterns for production systems — typing, dataclasses, async, context managers, packaging, and testing.
problem_solved: Provides a reference for Python's dynamic typing, data-model protocols, async event loop, and packaging conventions, reducing footguns around mutability, import cycles, and packaging metadata.
install: pip install pydantic pytest
dependencies:
  - pydantic
  - pytest
concepts:
  - name: Python Ecosystem
    id: "Python Ecosystem"
    description: The broader Python language, standard library, tooling, and community ecosystem these patterns operate within.
  - name: Dataclasses
    id: concept:python/dataclasses
    description: Decorator-based classes that auto-generate __init__, __repr__, __eq__ and other dunder methods from annotated fields, reducing boilerplate for data containers.
  - name: Type Hints & Typing
    id: concept:python/typing
    description: Static type annotations via the typing module (List, Dict, Optional, Union, generics) combined with tools like mypy for static analysis.
  - name: Async/Await
    id: concept:python/async
    description: Coroutine-based concurrency built on asyncio, where async def defines coroutines and await yields control to the event loop.
  - name: Event Loop
    id: concept:python/event-loop
    description: The core asyncio scheduler that drives coroutines, manages tasks, and multiplexes I/O. One loop runs per thread; asyncio.run boots it.
  - name: Context Managers
    id: concept:python/context-managers
    description: Objects implementing __enter__/__exit__ (or using @contextmanager) to guarantee resource setup and teardown, core to the with statement.
  - name: Generators
    id: concept:python/generators
    description: Functions using yield to produce a lazy sequence, enabling memory-efficient streaming and coroutine-based pipelines.
  - name: Decorators
    id: concept:python/decorators
    description: Functions wrapping other functions to add behavior (timing, caching, auth). functools.wraps preserves metadata.
  - name: Virtual Environments
    id: concept:python/venv
    description: Isolated interpreter environments (venv, virtualenv, pipx) that pin dependency sets per project to avoid system-wide conflicts.
  - name: pyproject.toml Packaging
    id: concept:python/packaging
    description: PEP 518/621 metadata file that declares build system, dependencies, and tool configuration (pytest, mypy, ruff) replacing setup.py.
  - name: Mutable Default Arguments
    id: concept:python/mutable-defaults
    description: A classic footgun where default argument values are bound once at function definition, so shared mutable objects leak state across calls.
  - name: Import System
    id: concept:python/imports
    description: Python's module resolution via sys.path, importlib, and __init__.py. Circular imports arise when two modules import each other at module load time.
  - name: Protocols (Structural Typing)
    id: concept:python/protocols
    description: typing.Protocol enables duck-typing checked statically by structure rather than explicit inheritance (PEP 544).
  - name: Pydantic Models
    id: concept:python/pydantic
    description: Runtime-validated data classes that parse, coerce, and validate structured input using type annotations.
  - name: GIL
    id: concept:python/gil
    description: The Global Interpreter Lock serializes bytecode execution in CPython, making CPU-bound multithreading ineffective but protecting reference counts.
  - name: f-strings
    id: concept:python/fstrings
    description: Formatted string literals (f"...") that evaluate expressions at runtime; the modern, performant way to interpolate values.
apis:
  - name: dataclass
    id: api:python/dataclass
    signature: "@dataclass / @dataclass(frozen=True, slots=True) class C: x: int"
    returns: A class with generated dunder methods.
    description: Decorator that synthesizes __init__, __repr__, __eq__ from field annotations; frozen makes instances immutable, slots reduces memory.
  - name: asyncio.run
    id: api:python/asyncio-run
    signature: "asyncio.run(coro) -> T"
    returns: The return value of the coroutine.
    description: Creates a new event loop, runs the coroutine until completion, then closes the loop. Entry point for asyncio programs.
  - name: asyncio.gather
    id: api:python/asyncio-gather
    signature: "asyncio.gather(*coros, return_exceptions=False) -> list"
    returns: A coroutine resolving to a list of results in order.
    description: Concurrently awaits multiple awaitables; by default the first exception propagates and cancels the rest.
  - name: await
    id: api:python/await
    signature: "value = await coro"
    returns: The coroutine's result.
    description: Suspends the coroutine, yielding to the event loop until the awaited awaitable completes.
  - name: with statement
    id: api:python/with
    signature: "with open('f') as fh: ..."
    returns: void
    description: Enters a context manager and guarantees __exit__ runs even on exception, releasing resources deterministically.
  - name: "@contextmanager"
    id: api:python/contextmanager
    signature: "@contextmanager def gen(): setup; yield res; teardown"
    returns: A context manager.
    description: Turns a generator into a context manager; code before yield is setup, after yield is teardown.
  - name: functools.lru_cache
    id: api:python/lru-cache
    signature: "@lru_cache(maxsize=128) def f(...)"
    returns: A memoized wrapper.
    description: Caches recent calls keyed by arguments; dramatically speeds pure functions but must not wrap functions with mutable args.
  - name: pathlib.Path
    id: api:python/pathlib
    signature: "Path('a/b.txt').read_text()"
    returns: File contents or a Path object.
    description: Object-oriented filesystem paths replacing string manipulation and os.path; supports read_text, write_text, glob, iterdir.
  - name: type hints
    id: api:python/type-hints
    signature: "def f(x: int) -> str: ..."
    returns: The annotated function.
    description: Optional static annotations consumed by mypy/pyright for type checking; no runtime enforcement.
  - name: pytest.raises
    id: api:python/pytest-raises
    signature: "with pytest.raises(ValueError): f()"
    returns: void
    description: Context manager asserting that the enclosed block raises the expected exception type.
examples:
  - id: example:python/dataclass-basic
    language: python
    description: A frozen, slotted dataclass with defaults.
  - id: example:python/async-gather
    language: python
    description: Concurrent fetches using asyncio.gather with aiohttp.
  - id: example:python/context-manager
    language: python
    description: A custom context manager using @contextmanager for a database transaction.
  - id: example:python/pydantic-validate
    language: python
    description: Validating API input with a Pydantic model.
failures:
  - id: failure:python/mutable-default
    symptom: A list or dict argument accumulates state across calls (e.g., appended items persist).
    cause: Mutable default arguments (def f(x=[])) are shared across invocations because defaults bind at definition time.
    fix: Use None as the default and initialize inside the function (x=None; x = x or []).
  - id: failure:python/blocking-in-async
    symptom: The entire async app stalls while one coroutine runs.
    cause: Calling a blocking (CPU-bound or sync I/O) function inside an async coroutine without awaiting, starving the event loop.
    fix: Offload to a thread with asyncio.to_thread or loop.run_in_executor, or use aiohttp/httpx instead of requests.
  - id: failure:python/circular-import
    symptom: ImportError or partially-initialized module at startup.
    cause: Two modules import each other at module top level, creating a cycle the import system cannot resolve.
    fix: Move imports inside functions, merge modules, or restructure shared code into a third module.
  - id: failure:python/event-loop-closed
    symptom: "\"RuntimeError: Event loop is closed\" inside Jupyter or after asyncio.run reuse."
    cause: Creating tasks on a closed loop or reusing an event loop across separate asyncio.run calls.
    fix: Create a fresh loop per scope or use asyncio.run once; in notebooks use nest_asyncio or a single persistent loop.
  - id: failure:python/gil-cpu-bound
    symptom: CPU-bound multithreaded Python does not use all cores and may run slower.
    cause: The GIL serializes bytecode; threading only helps for I/O-bound work, not CPU-bound work.
    fix: Use multiprocessing, concurrent.futures.ProcessPoolExecutor, or C extensions/release-the-GIL libraries (numpy, orjson).
  - id: failure:python/packaging-missing
    symptom: "ModuleNotFoundError after pip install, or editable install does not pick up changes."
    cause: Missing or incorrect pyproject.toml [project] metadata or no build backend configured.
    fix: Declare dependencies and build-system in pyproject.toml and install with pip install -e . for development.
  - id: failure:python/immutable-default-copy
    symptom: Unexpected mutation or "object is not subscriptable" after freezing a dataclass.
    cause: frozen=True dataclasses raise on attribute assignment; slots=True changes attribute access semantics and breaks some introspection.
    fix: Use dataclasses.replace() to create modified copies, or drop frozen/slots if mutability is needed.
extends:
  - concept:python/typing
uses:
  - concept:python/dataclasses
  - concept:python/async
  - concept:python/context-managers
  - concept:python/venv
part_of: "Python Ecosystem"
depends_on:
  - package:typescript/nextjs
solves:
  - idiomatic, low-boilerplate Python for data and backend services
alternatives:
  - Ruff + mypy linters
  - Poetry for packaging
  - UV for dependency management
---
# Python Engineering Patterns

Python's readability-first design rewards small, explicit, protocol-driven code. Dataclasses (`@dataclass`) collapse the boilerplate of plain data containers — annotating fields generates `__init__`, `__repr__`, and `__eq__` automatically. With `frozen=True` you get immutability, and `slots=True` cuts per-instance memory by storing fields in a fixed array instead of a `__dict__`. When you need copies with changes, `dataclasses.replace()` produces a new instance rather than mutating in place.

Typing has matured from an afterthought into a first-class workflow. The `typing` module supplies `Optional`, `Union`, `Callable`, `TypeVar`, and `typing.Protocol` for structural (duck) typing verified by `mypy` or `pyright` — no runtime cost, but it catches whole categories of `None`-handling and shape errors before execution. `Pydantic` bridges static and runtime typing: it validates and coerces untrusted input (JSON from an API) into typed models, raising clear errors on malformed data.

Concurrency in Python splits along two axes. For I/O-bound work, `async`/`await` on top of `asyncio` lets a single thread drive thousands of in-flight operations; `asyncio.gather` runs awaitables concurrently and `asyncio.run` boots the loop. The cardinal rule is to never call blocking code inside a coroutine — use `asyncio.to_thread` or async-native libraries like `httpx`/`aiohttp`, or you starve the event loop and stall everything. For CPU-bound work the Global Interpreter Lock (GIL) makes threads useless; reach for `multiprocessing` or `ProcessPoolExecutor` instead. Context managers (`with`, `@contextmanager`) guarantee deterministic cleanup of files, locks, and transactions, which is essential for correct async and multi-process code.

Packaging finally standardized on `pyproject.toml` (PEP 518/621), declaring the build backend, runtime dependencies, and tool config in one file. Virtual environments isolate each project's dependencies, and editable installs (`pip install -e .`) let you iterate without reinstalling. Testing with `pytest` — fixtures for setup, `pytest.raises` for exceptions, and parametrization for breadth — pairs naturally with `mypy` in CI to ship confident Python.
