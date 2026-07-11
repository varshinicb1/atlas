---
kind: Package
id: package:qa/unicode
name: UnicodeTest
version: 1.0.0
purpose: Unicode regression probe
problem_solved: Ensure non-ASCII is handled
install: echo ok
dependencies:
  - package:flutter/widgets
concepts:
  - name: Cafe Concept ☕
    id: concept:qa/cafe
  - name: Nihongo Node 日本語
    id: concept:qa/nihongo
apis:
  - name: doThing()
    id: api:qa/dot
    description: Returns cafe and 日本語 safely
part_of: concept:flutter/state_management
---

# UnicodeTest

Cafe and 日本語 body.
