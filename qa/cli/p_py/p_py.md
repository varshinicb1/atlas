---
kind: Package
id: package:p_py
name: p py
version: "0.1.0"
purpose: |
  Knowledge package covering Python patterns for p py.
problem_solved: |
  Helps Python engineers choose the right patterns for project structure,
  async programming, and dependency management.
install: |
  ```bash
  atlas install p_py.md
  ```
concepts:
  - name: Async/Await
    id: concept:async_p_py
    description: |
      TODO: Define async Python patterns.
      asyncio, anyio, httpx, async generators.
  - name: Pydantic Models
    id: concept:pydantic_p_py
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
