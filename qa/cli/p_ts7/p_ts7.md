---
kind: Package
id: package:p_ts7
name: p ts7
version: "7.0.0"
purpose: |
  Knowledge package capturing TypeScript 7 (Project Corsa) migration patterns,
  breaking changes, and performance tuning for p ts7.
problem_solved: |
  Helps TypeScript teams plan and execute the migration from TS 5/6 to TS 7,
  covering strict mode fixes, parallel checking flags, module resolution changes,
  and CI pipeline optimization.
install: |
  ```bash
  atlas install p_ts7.md
  npx typescript@7 init
  ```
concepts:
  - name: Project Corsa
    id: concept:corsa_p_ts7
    description: |
      TODO: Document specific aspects of TS 7's Go-native compiler relevant
      to your project. Include flag changes, behavioral differences, and
      migration notes.
  - name: Strict Mode Impact
    id: concept:strict_p_ts7
    description: |
      TODO: Document how strict mode being default affects your codebase.
      List the specific strict checks you need to address.
  - name: Parallel Checking
    id: concept:parallel_p_ts7
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
  - name: Migration Assessment for p ts7
    id: workflow:assess_p_ts7
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
