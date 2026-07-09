# Data Schema

Field-level schema for every node kind in the Engineering IR. All fields are
typed; `req` = required, `opt` = optional. Every node implicitly carries the
**common envelope** below.

## Common envelope (all nodes)

| Field | Type | Req | Notes |
| --- | --- | --- | --- |
| `id` | string | req | `kind:domain/package@ver/path` |
| `kind` | enum | req | node kind (see ENGINEERING_IR §3) |
| `name` | string | req | human label |
| `version` | string | opt | semver or `*` |
| `category` | string | opt | taxonomy tag |
| `provenance` | `[SourceRef]` | req | source id + locator + checksum |
| `confidence` | float | req | 0..1 from extraction |
| `status` | enum | req | `VERIFIED \| UNVERIFIED \| UNRESOLVED` |
| `updated_at` | int64 | req | unix ms |
| `checksum` | string | req | content hash |

`SourceRef = { source_id, locator, line?, hash }`

## Package

| Field | Type |
| --- | --- |
| `purpose` | string |
| `problem_solved` | string |
| `dependencies[]` | node id |
| `interfaces[]` | node id |
| `install` | string (command) |
| `examples[]` | example id |
| `alternatives[]` | alternative id |
| `best_practices[]` | string |
| `anti_patterns[]` | string |
| `license` | string |
| `popularity` | int (downloads/stars) |
| `maintenance` | enum (active/stale/deprecated) |
| `known_issues[]` | failure id |
| `migration_guides[]` | node id |
| `agent_instructions` | string |
| `verification_rules[]` | rule id |

## Class / Function / API

| Field | Type |
| --- | --- |
| `signature` | string |
| `parameters[]` | `{name, type, optional}` |
| `returns` | type |
| `throws[]` | type |
| `lifecycle` | enum (stable/experimental/deprecated) |
| `since` | version |
| `breaking_changes[]` | `{version, note}` |
| `examples[]` | example id |
| `agent_instructions` | string |

## Concept

| Field | Type |
| --- | --- |
| `problem_solved` | string |
| `related[]` | node id |
| `alternatives[]` | alternative id |
| `decision_trees[]` | tree id |

## Example

| Field | Type |
| --- | --- |
| `language` | string |
| `code` | string |
| `runnable` | bool |
| `expected_output?` | string |
| `references_node` | node id |

## FailureMode

| Field | Type |
| --- | --- |
| `symptom` | string |
| `cause` | string |
| `fix` | string |
| `affects[]` | node id |
| `version_range` | string |

## Decision (decision tree node)

| Field | Type |
| --- | --- |
| `question` | string |
| `type` | enum (boolean\|categorical\|lookup) |
| `branches[]` | `{condition, next}` |
| `terminal?` | `{recommendation[], rationale}` |

## Architecture

| Field | Type |
| --- | --- |
| `role` | string |
| `components[]` | node id |
| `tradeoffs` | string |

## Schema versioning

- Schema version lives in `IR.meta.schema_version`.
- Adding optional fields = non-breaking (bump minor).
- Adding kinds/edges or changing required fields = breaking (bump major);
  Runtime rejects binaries whose major schema it doesn't support.
