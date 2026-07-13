---
kind: Package
id: package:prisma
name: Prisma ORM
version: "6.0"
purpose: Document Prisma ORM patterns — schema design, queries, relations, migrations, and integration with full-stack TypeScript frameworks.
problem_solved: Eliminates the impedance mismatch between application code and database schemas by generating a type-safe client from a declarative schema definition language, replacing raw SQL with autocompleted, validated queries that cannot produce runtime type errors.
install: npx prisma init
dependencies:
  - concept:relational-databases
  - concept:typescript
concepts:
  - name: Schema
    id: concept:prisma/schema
    description: The single source of truth — a declarative DSL (schema.prisma) defining models, relations, enums, indexes, and database provider (PostgreSQL, MySQL, SQLite, SQL Server, MongoDB, CockroachDB). Every model maps to a database table with type-safe fields.
  - name: Models
    id: concept:prisma/models
    description: Entities defined in the schema with typed fields, default values, unique constraints, and relation annotations. Each model generates a corresponding TypeScript type and CRUD operations on the Prisma Client.
  - name: Relations
    id: concept:prisma/relations
    description: One-to-one, one-to-many, and many-to-many relationships modeled with @relation, fields, references, and optional cascade deletes. Prisma generates join-table models for implicit many-to-many or accepts explicit @relation for custom join tables.
  - name: Prisma Client
    id: concept:prisma/client
    description: The auto-generated type-safe database client — provides create, read, update, delete, upsert, and aggregate operations with full TypeScript inference. Every query is validated at compile time against the schema.
  - name: Migrations
    id: concept:prisma/migrations
    description: Version-controlled SQL migration system — prisma migrate dev creates migration files from schema changes, prisma migrate deploy applies them in production. Supports rollbacks, baseline, and shadow databases for diffing.
  - name: Prisma Studio
    id: concept:prisma/studio
    description: GUI database browser — prisma studio launches a local web UI for viewing, filtering, creating, and editing records directly. Useful for debugging during development.
  - name: Select & Include
    id: concept:prisma/select-include
    description: Field selection (select) and relation inclusion (include) patterns for controlling query shape. Select limits returned fields for performance; include eagerly loads related records in the same query, avoiding N+1.
  - name: Pagination
    id: concept:prisma/pagination
    description: Cursor-based (cursor, take, skip) and offset-based (skip, take) pagination. Cursor-based is preferred for large datasets as it remains stable under insertions. Supports skip with cursor for offset-on-cursor hybrid.
  - name: Middleware
    id: concept:prisma/middleware
    description: "Intercepts query operations (before/after) for logging, audit, soft-delete, encryption, or cache checks. Middleware functions receive params, next, and operation metadata. Caution: middleware is global and runs on every query."
  - name: Raw Queries
    id: concept:prisma/raw-queries
    description: "$queryRaw for SELECT queries and $executeRaw for mutations when Prisma's generated API cannot express the SQL needed. Supports parameterized inputs with template literals ($queryRaw`SELECT * FROM users WHERE id = ${id}`) to prevent injection."
  - name: Realtime & Pulse
    id: concept:prisma/pulse
    description: Prisma Pulse (preview) adds change data capture — stream database changes to the application in real-time via database replication slots. Enables live-updating UIs without polling.
  - name: Accelerate
    id: concept:prisma/accelerate
    description: Global database cache layer — sits between Prisma Client and the database. Caches queries at the edge with configurable TTL, reduces latency for read-heavy workloads. Uses the Prisma Data Platform proxy.
  - name: Defer
    id: concept:prisma/defer
    description: Postpones a write operation to a background queue, returning immediately to the client. Useful for non-critical writes (analytics, logs) that should not block the request-response cycle.
  - name: Validation
    id: concept:prisma/validation
    description: Schema-level validation via @default, @unique, @updatedAt, and database constraints. Prisma Client validates types at compile time; runtime validation requires Zod or other libraries at the API boundary.
apis:
  - name: prisma.model.create(data)
    id: api:prisma/create
    signature: "prisma.model.create(data: CreateInput): Promise<Model>"
    returns: The created record.
    description: "Inserts a new record. Supports nested create for relations (create: { post: { create: { title: '...' } } }). Validates all required fields and unique constraints at the database level."
  - name: prisma.model.findUnique(where)
    id: api:prisma/findUnique
    signature: "prisma.model.findUnique(where: { id: string } & SelectInclude): Promise<Model | null>"
    returns: The matching record or null.
    description: Finds a single record by unique identifier (primary key, @unique, or compound unique). Use findUniqueOrThrow for non-null guarantee. Always returns one row or null — never an array.
  - name: prisma.model.findMany(opts)
    id: api:prisma/findMany
    signature: "prisma.model.findMany(opts?: { where, orderBy, skip, take, cursor, select, include, distinct }): Promise<Model[]>"
    returns: An array of matching records.
    description: Queries multiple records with filtering, sorting, pagination, and field selection. Supports complex where clauses with AND/OR/NOT, nested relation filters, and full-text search.
  - name: prisma.model.update(where, data)
    id: api:prisma/update
    signature: "prisma.model.update(where: { id: string }, data: UpdateInput): Promise<Model>"
    returns: The updated record.
    description: Updates a single matching record. Throws if no record found (use updateMany for conditional updates). Supports nested connect/disconnect/set for relation mutations.
  - name: prisma.model.delete(where)
    id: api:prisma/delete
    signature: "prisma.model.delete(where: { id: string }): Promise<Model>"
    returns: The deleted record.
    description: Deletes a record by unique identifier. Throws NotFoundError if not found. Use deleteMany for bulk deletion without error on empty matches.
  - name: prisma.model.upsert(where, create, update)
    id: api:prisma/upsert
    signature: "prisma.model.upsert(where: { id: string }, create: CreateInput, update: UpdateInput): Promise<Model>"
    returns: The created or updated record.
    description: Atomic create-or-update — if a record matching where exists, apply update; otherwise, apply create. Not a true UPSERT at the SQL level (SQL uses two operations with a transaction).
  - name: prisma.model.count(where)
    id: api:prisma/count
    signature: "prisma.model.count(where?: WhereInput): Promise<number>"
    returns: The count of matching records.
    description: Returns the total number of records matching the filter without loading data. Used for pagination total counts and dashboard metrics.
  - name: prisma.model.aggregate(pipeline)
    id: api:prisma/aggregate
    signature: "prisma.model.aggregate(pipeline: { _count?, _avg?, _sum?, _min?, _max? }): Promise<AggregateResult>"
    returns: Aggregated values.
    description: Computes aggregate functions (_avg, _sum, _min, _max, _count) over matching records. Supports groupBy for per-field aggregation.
  - name: prisma.$transaction(ops)
    id: api:prisma/transaction
    signature: "prisma.$transaction(ops: PrismaPromise<any>[]): Promise<any[]>"
    returns: Array of operation results.
    description: Executes multiple operations in a database transaction. Atomically commits all or rolls back on any failure. Interactive API for read-write-transaction patterns with isolation level control.
  - name: prisma.$queryRaw(sql)
    id: api:prisma/queryRaw
    signature: "prisma.$queryRaw<T>(sql: TemplateStringsArray, ...params: any[]): Promise<T>"
    returns: Query result (array of rows).
    description: Executes a raw SQL query and returns the result as an array of records. Parameterized via template literal to prevent SQL injection. Use $queryRawUnsafe for dynamic strings.
failures:
  - id: failure:prisma/n-plus-one
    symptom: Extremely slow page loads with many database queries; N+1 select problems.
    cause: Accessing relation properties in a loop triggers a separate query per item.
    fix: Use include or select to eagerly load relations in the parent query. Use Prisma's Fluent API for chained relation access that Prisma batches internally.
  - id: failure:prisma/ghost-fields
    symptom: Queries return empty objects or TypeScript errors on accessed fields.
    cause: Using select in one query and include in another, caching first result shape. Also caused by omitting selected fields from the type.
    fix: Always align select/include with the query. Use Prisma.validator or $Default to define reusable selection types. Avoid caching raw query results across different select patterns.
  - id: failure:prisma/migration-merge-conflicts
    symptom: Multiple developers create migrations that conflict on the same schema version.
    cause: Two or more developers run prisma migrate dev on the same branch without pulling the latest migrations first.
    fix: Always run prisma migrate dev after pulling. Prisma detects drift and rewrites the migration. For production, use prisma migrate deploy. Consider migration baselining for large existing databases.
  - id: failure:prisma/huge-json-columns
    symptom: Queries on models with JSON fields are slow; serialization overhead on every row.
    cause: Prisma deserializes JSON columns on every query even when they are not needed.
    fix: Use select to exclude JSON columns unless explicitly needed. Consider normalizing JSON into related models. Use Prisma's JsonNull for explicit null handling.
  - id: failure:prisma/connection-pool-exhaustion
    symptom: Intermittent timeout errors under load; ECONNREFUSED on database connections.
    cause: Default connection pool size (10) is too small for serverless or high-concurrency workloads.
    fix: "Increase pool size via DATABASE_URL parameter (pgbouncer: true for PgBouncer). Use Prisma Accelerate for serverless. Set connection_limit in the connection string."
extends: []
implements: []
uses:
  - concept:relational-databases
  - concept:typescript
part_of: concept:full-stack-data-layer
solves:
  - problem:type-safe-database-access
  - problem:database-schema-management
  - problem:orm-boilerplate
alternatives:
  - package:drizzle
  - package:typeorm
  - package:knex
---
Prisma's architecture revolves around the **schema** as the single source of truth. Unlike traditional ORMs that map classes to tables, Prisma generates a client from the schema — there are no entity classes to maintain, no repository pattern, and no active record lifecycle hooks. This makes the data layer declarative: define the schema, run migrations, and use the generated client.

The relation model is Prisma's most distinctive feature. Relations are declared in the schema with @relation annotations, and the generated client provides fluent relation navigation (user.posts) that Prisma translates to JOIN queries under the hood. The include/select pattern gives precise control over query shape — include eagerly loads relations, select restricts fields — and TypeScript infers the return type from the query itself, so removing a field from select immediately breaks all code that references it.

Migrations in Prisma are dual-mode: prisma migrate dev creates human-readable SQL migration files during development, while prisma migrate deploy applies them in CI/CD. The shadow database (a temporary clone) ensures differential migrations don't lose data. For schema prototyping, prisma db push directly applies schema changes without creating migration files — useful for early development before migration hygiene matters.
