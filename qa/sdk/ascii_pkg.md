---
kind: Package
id: package:qa/ascii
name: AsciiTest
version: 1.0.0
purpose: ascii only
problem_solved: Ensure ascii works
install: echo ok
dependencies:
  - package:flutter/widgets
concepts:
  - name: Ascii Concept
    id: concept:qa/asc
apis:
  - name: doThing()
    id: api:qa/dot2
    description: Returns ascii safely
