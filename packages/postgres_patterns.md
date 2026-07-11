---
kind: Package
id: package:postgres/patterns
name: PostgreSQL Patterns
version: "16"
purpose: Document PostgreSQL patterns for correct, fast, and reliable relational data — indexing, query planning, transactions, JSONB, pooling, and migrations.
problem_solved: Provides a reference for relational modeling, the cost-based planner, isolation levels, and operational pitfalls, reducing slow queries, lock contention, migration drift, and connection exhaustion.
install: |
  ```bash
  brew install postgresql@16
  ```
dependencies:
  - package:postgresql
concepts:
  - name: Relational Databases
    id: concept:domain/relational-databases
    description: The broader category of relational, SQL-based database systems that PostgreSQL exemplifies.
  - name: B-tree Indexing
    id: concept:pg/btree
    description: The default balanced-tree index accelerating equality and range scans; composite indexes follow leftmost-prefix rules.
  - name: Query Planner & EXPLAIN
    id: concept:pg/planner
    description: The cost-based optimizer choosing scan/join strategies; EXPLAIN ANALYZE reveals the real plan and row estimates vs actuals.
  - name: Transactions & Isolation
    id: concept:pg/transactions
    description: ACID units with READ COMMITTED default; REPEATABLE READ and SERIALIZABLE prevent anomalies at the cost of serialization failures.
  - name: MVCC
    id: concept:pg/mvcc
    description: Multiversion concurrency control keeps old row versions so readers never block writers; vacuum reclaims dead tuples.
  - name: JSONB
    id: concept:pg/jsonb
    description: Binary JSON storage with indexing via GIN, enabling semi-structured data with queryable, type-preserving fields.
  - name: Connection Pooling
    id: concept:pg/pooling
    description: Reusing backend connections (pgBouncer, PgCat) to avoid the per-connection process overhead that exhausts memory under load.
  - name: Foreign Keys & Constraints
    id: concept:pg/constraints
    description: Referential integrity (FK), UNIQUE, CHECK, and NOT NULL that enforce data correctness at the database layer.
  - name: VACUUM & Autovacuum
    id: concept:pg/vacuum
    description: Reclaims dead tuples from updates/deletes and prevents transaction-ID wraparound; autovacuum runs it in the background.
  - name: Partitioning
    id: concept:pg/partitioning
    description: Range/list partitioning splits large tables into child tables to prune scans and ease maintenance (e.g., by time).
  - name: Common Table Expressions
    id: concept:pg/cte
    description: WITH clauses that name subqueries; in PG12+ they are inlined (not an optimization fence) unless MATERIALIZED.
  - name: Replication
    id: concept:pg/replication
    description: Streaming physical and logical replication for read replicas, failover, and downstream change data capture.
  - name: Extensions
    id: concept:pg/extensions
    description: Add-on modules (pg_stat_statements, PostGIS, pgcrypto) loaded via CREATE EXTENSION to extend core capabilities.
  - name: Migrations
    id: concept:pg/migrations
    description: Versioned, reversible schema changes managed by tools (Flyway, Liquibase, Alembic, django-migrations) to avoid drift.
apis:
  - name: EXPLAIN ANALYZE
    id: api:pg/explain
    signature: "EXPLAIN (ANALYZE, BUFFERS) SELECT ..."
    returns: The execution plan with timings and buffer counts.
    description: Runs the query and reports the real plan; compare estimated vs actual rows to spot bad statistics or missing indexes.
  - name: CREATE INDEX
    id: api:pg/create-index
    signature: "CREATE INDEX CONCURRENTLY idx ON t (a, b);"
    returns: An index object.
    description: Builds an index; CONCURRENTLY avoids locking writes but cannot run inside a transaction block.
  - name: BEGIN / COMMIT
    id: api:pg/begin
    signature: "BEGIN; ...; COMMIT;"
    returns: Transaction control.
    description: Opens and closes a transaction; ROLLBACK aborts it. Sets the isolation level with SET TRANSACTION.
  - name: jsonb operator ->
    id: api:pg/jsonb-op
    signature: "col -> 'field'  /  col @> '{\"k\":\"v\"}'"
    returns: JSONB value or boolean containment.
    description: Extracts a JSONB field and tests containment; pair with a GIN index for fast searches.
  - name: pg_stat_statements
    id: api:pg/stat-statements
    signature: "SELECT query, mean_exec_time FROM pg_stat_statements ORDER BY mean_exec_time DESC;"
    returns: Aggregated query statistics.
    description: After enabling the extension, shows the slowest queries by total/mean time for targeted tuning.
  - name: VACUUM (ANALYZE)
    id: api:pg/vacuum
    signature: "VACUUM (ANALYZE, VERBOSE) my_table;"
    returns: void
    description: Reclaims dead tuples and refreshes planner statistics; autovacuum normally handles this automatically.
  - name: pg_dump
    id: api:pg/dump
    signature: "pg_dump -Fc mydb > mydb.dump"
    returns: A custom-format backup file.
    description: Logical backup of a database; restore with pg_restore. Use for migrations and point-in-time copies.
examples:
  - id: example:pg/composite-index
    language: sql
    description: A composite index exploiting leftmost-prefix for a multi-column filter.
  - id: example:pg/jsonb-gin
    language: sql
    description: GIN-indexed JSONB column with containment queries.
  - id: example:pg/cte-upsert
    language: sql
    description: INSERT ... ON CONFLICT (upsert) inside a CTE for idempotent writes.
  - id: example:pg/partition-range
    language: sql
    description: Range-partitioning a large events table by month.
failures:
  - id: failure:pg/seq-scan-slow
    symptom: A query full-scans a million-row table and times out.
    cause: Missing index on the filtered/join column, or the planner estimates a seq scan cheaper due to stale statistics.
    fix: Add a targeted index (composite if multi-column), run ANALYZE, and use EXPLAIN to confirm index usage.
  - id: failure:pg/nplus1
    symptom: Application issues one query per row; latency explodes with row count.
    cause: Loading a parent then querying children in a loop instead of joining or batching.
    fix: Use a JOIN, IN (...), or a single aggregated query; apply eager loading in the ORM.
  - id: failure:pg/connection-exhaustion
    symptom: "\"FATAL: remaining connection slots are reserved\" under load."
    cause: App opens a new connection per request without pooling; max_connections exceeded.
    fix: Put pgBouncer/PgCat in transaction mode in front, use a pool in the app, and tune max_connections/pool size.
  - id: failure:pg/serialization-failure
    symptom: "\"could not serialize access due to concurrent update\" at SERIALIZABLE/REPEATABLE READ."
    cause: Two transactions touched overlapping rows; the stronger isolation level aborts one to preserve consistency.
    fix: Retry the aborted transaction (it's designed to be safe to retry); keep transactions short; reduce contention.
  - id: failure:pg/lock-contention
    symptom: Queries hang; pg_locks shows many waiting on a single row/table lock.
    cause: A long transaction holds a lock (e.g., ALTER TABLE, uncommitted write) blocking others.
    fix: Commit/rollback promptly, run DDL in maintenance windows, and use NOWAIT/timeout to fail fast.
  - id: failure:pg/migration-drift
    symptom: Environments have different schemas; deploy fails on a constraint that already exists.
    cause: Manual schema edits outside the migration tool, or non-transactional DDL in some engines.
    fix: Make every schema change a versioned, reversible migration; forbid manual edits; verify with CI against a shadow DB.
  - id: failure:pg/bloat
    symptom: Table size and query times grow far beyond data volume.
    cause: UPDATE/DELETE churn creating dead tuples that autovacuum cannot reclaim fast enough.
    fix: Tune autovacuum (scale_factor/cost_limit), run VACUUM (FULL) in maintenance, or consider partitioning.
extends:
  - concept:pg/transactions
uses:
  - concept:pg/btree
  - concept:pg/planner
  - concept:pg/jsonb
  - concept:pg/mvcc
  - concept:pg/pooling
  - concept:pg/migrations
part_of: concept:domain/relational-databases
depends_on:
  - package:docker/kubernetes
solves:
  - problem:relational-data-storage
alternatives:
  - package:mysql
  - package:sqlite
  - package:cockroachdb
---
# PostgreSQL Patterns

PostgreSQL is a powerful object-relational database whose correctness and extensibility come from a few core ideas. The cost-based planner picks how to execute each query, and your job is to give it good options: appropriate indexes, fresh statistics, and well-written SQL. `EXPLAIN (ANALYZE, BUFFERS)` is the single most important tool — it shows the real plan with actual row counts, so you can spot when estimates diverge from reality (a sign of stale statistics needing `ANALYZE` or a missing index). The default B-tree index accelerates equality and range scans; composite indexes follow leftmost-prefix rules, so `(a, b)` serves queries filtering on `a` alone but not on `b` alone.

Concurrency is handled by MVCC: each update or delete leaves the old version in place so readers see a consistent snapshot without blocking writers. That old version becomes a "dead tuple" that `VACUUM` (and autovacuum) must reclaim, or tables bloat and queries slow. Transactions wrap work in `BEGIN`/`COMMIT` with `READ COMMITTED` by default; `REPEATABLE READ` and `SERIALIZABLE` prevent anomalies but may raise serialization failures on concurrent writes — which are safe to retry. This is why keeping transactions short and avoiding long-held locks (a stuck `ALTER TABLE` or uncommitted write) prevents contention storms visible in `pg_locks`.

JSONB gives you the best of both worlds: store semi-structured data in a binary, type-preserving format and index it with a GIN index for fast containment queries (`@>`). Use it for schemaless payloads and attributes, but keep your relational core in typed columns with foreign keys, UNIQUE, and CHECK constraints for integrity. Connection pooling is mandatory at scale — each PostgreSQL connection is an OS process, so an app that opens one per request will exhaust `max_connections`; put pgBouncer or PgCat in transaction mode in front. For analytics over huge tables, partitioning (range by time) prunes scans and eases archival. Operational visibility comes from extensions like `pg_stat_statements` (find your slowest queries) and replication for read scaling and failover. Finally, every schema change must be a versioned, reversible migration — manual edits cause environment drift that breaks deploys. These patterns yield data that is correct by construction, fast under load, and safe to evolve.
