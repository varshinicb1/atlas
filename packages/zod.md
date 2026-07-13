---
kind: Package
id: package:zod
name: Zod Schema Validation
version: "3.24"
purpose: Document Zod — TypeScript-first schema declaration and validation library with inferred types, transforms, refinements, and error formatting for runtime type safety.
problem_solved: Bridges the gap between TypeScript's compile-time type system and runtime data from external sources (APIs, forms, config files, databases) by providing a single source of truth where a schema definition produces both a type and a runtime validator that guarantees structural conformance.
install: npm install zod
dependencies:
  - concept:typescript
  - concept:data-validation
concepts:
  - name: Schemas
    id: concept:zod/schemas
    description: Declarative validation definitions using Zod primitives — z.string(), z.number(), z.object(), etc. Each schema is a composable, chainable validator that produces an inferred TypeScript type via z.infer<typeof schema>. Schemas are immutable; every method returns a new schema.
  - name: Type Inference
    id: concept:zod/type-inference
    description: z.infer<typeof MySchema> extracts the exact TypeScript type from a schema. z.input<typeof MySchema> provides the input type (before transforms). This eliminates manual type definitions that drift from validation logic.
  - name: Parsing
    id: concept:zod/parsing
    description: "schema.parse(data) validates and returns typed data or throws ZodError. schema.safeParse(data) returns a discriminated union { success: true, data } | { success: false, error } for error-safe handling without try/catch. schema.parseAsync for async refinements."
  - name: Object Schemas
    id: concept:zod/objects
    description: "z.object({ field: z.string() }) defines shape validation with per-field error messages, optional/mandatory fields, and .partial(), .required(), .pick(), .omit(), .extend() for schema composition. Nested objects, arrays, and unions are first-class."
  - name: Effects
    id: concept:zod/effects
    description: ".transform(val => ...) applies post-parse transformations (string-to-number, date parsing, normalization). .refine(val => boolean, { message }) adds custom validation. .preprocess(fn) modifies input before parsing. Effects are applied in order and compose."
  - name: Branded Types
    id: concept:zod/brands
    description: z.string().brand<"Email">() creates a branded type — a nominal-type phantom that prevents accidental assignment of raw strings to branded fields. TypeScript enforces the brand at compile time while Zod validates at runtime.
  - name: Discriminated Unions
    id: concept:zod/discriminated-unions
    description: "z.discriminatedUnion(\"type\", [SchemaA, SchemaB]) optimizes union validation by first checking the discriminant field (\"type\") and then validating against the matched schema only. More performant than z.union() for tagged unions with many variants."
  - name: Error Formatting
    id: concept:zod/errors
    description: ZodError provides field-level error details via error.format() (nested object) and error.flatten() (flat arrays). Supports custom error maps via z.customErrorMap. Issue paths are arrays of keys for programmatic access.
  - name: Defaults & Catch
    id: concept:zod/defaults
    description: .default(value) provides a fallback when input is undefined. .catch(value) replaces any invalid input with a default — useful for lenient parsing of user-generated content where you want best-effort instead of rejection.
  - name: Nullable & Optional
    id: concept:zod/nullable-optional
    description: .nullable() allows null values, .optional() allows undefined. .nullish() allows both. These can be combined with .default() for complete missing-data strategies.
  - name: Recursive Types
    id: concept:zod/recursive
    description: z.late.object() enables self-referential schemas for trees, linked lists, and nested comments. Must be wrapped in a lazy function to break the circular initialization. z.lazy(() => schema) for recursive lazy schemas.
  - name: Pipes
    id: concept:zod/pipes
    description: schema.pipe(nextSchema) chains two schemas — validates input against the first, then passes the result to the second for further validation/transformation. Equivalent to .transform() but using schemas instead of functions.
apis:
  - name: z.string()
    id: api:zod/string
    signature: "z.string(validations?: { message? }): ZodString"
    returns: A string schema.
    description: Validates string values. Chains .min(), .max(), .length(), .email(), .url(), .uuid(), .regex(), .includes(), .startsWith(), .endsWith(), .datetime(), .ip(), .emoji(), .cuid2(), .ulid().
  - name: z.number()
    id: api:zod/number
    signature: "z.number(validations?: { message? }): ZodNumber"
    returns: A number schema.
    description: Validates number values. Chains .min(), .max(), .int(), .positive(), .nonnegative(), .negative(), .nonpositive(), .multipleOf(), .finite(), .safe().
  - name: z.object(shape)
    id: api:zod/object
    signature: "z.object(shape: { [key]: ZodType }): ZodObject"
    returns: An object schema.
    description: "Defines an object with per-field schemas. Methods: .pick(), .omit(), .partial(), .required(), .extend(), .merge(), .passthrough(), .strict(), .strip(), .catchall()."
  - name: z.array(schema)
    id: api:zod/array
    signature: "z.array(schema: ZodType): ZodArray"
    returns: An array schema.
    description: Validates arrays where every element matches the given schema. Chains .min(), .max(), .length(), .nonempty(). Use .element to access the inner schema.
  - name: z.enum(values)
    id: api:zod/enum
    signature: "z.enum(values: [string, ...string[]]): ZodEnum<[string, ...string[]]>"
    returns: An enum schema.
    description: Creates a union of literal string values. TypeScript infers the exact string literal union. Tuples in enums require at least one value — empty enums use z.never().
  - name: z.union(schemas)
    id: api:zod/union
    signature: "z.union(schemas: [ZodType, ...ZodType[]]): ZodUnion"
    returns: A union schema.
    description: "Validates against any of the provided schemas. Uses short-circuit evaluation: first matching schema wins. z.discriminatedUnion is preferred for tagged unions with many variants."
  - name: z.intersection(A, B)
    id: api:zod/intersection
    signature: "z.intersection(A: ZodType, B: ZodType): ZodIntersection"
    returns: An intersection schema.
    description: Validates data that conforms to both schemas. Merges types — equivalent to A & B. Use A.merge(B) on objects for a flatter API.
  - name: schema.parse(data)
    id: api:zod/parse
    signature: "schema.parse(data: unknown): T"
    returns: Typed data or throws ZodError.
    description: Validates and returns typed data. Throws on invalid input. Use safeParse for error-safe handling. Use parseAsync for async refinements.
  - name: z.infer<typeof schema>
    id: api:zod/infer
    signature: "type T = z.infer<typeof schema>"
    returns: The inferred TypeScript type.
    description: Extracts the output TypeScript type from a schema. Use z.input<typeof schema> for the input type (before transforms). The core value proposition of Zod — one definition, two type systems.
  - name: schema.transform(fn)
    id: api:zod/transform
    signature: "schema.transform(fn: (val: Input, ctx: RefinementCtx) => Output): ZodEffects"
    returns: A new schema with a post-parse transformation.
    description: "Transforms validated data. Access the validation context for issues and addIssues. Transforms compose: each transform receives the output of the previous transform or parse."
failures:
  - id: failure:zod/parse-throws
    symptom: Uncaught ZodError crashes the server or produces 500 responses for bad input.
    cause: Using .parse() without try/catch in API routes where user input may be invalid.
    fix: Use .safeParse() in request handlers and return user-friendly 400 errors with formatted issues. Reserve .parse() for trusted data where failure is a programming error.
  - id: failure:zod/type-drift
    symptom: Inferred type differs from expected runtime shape after transforms or refinements.
    cause: Adding .transform() or .refine() that changes the output shape but the inferred z.infer type does not capture the change correctly.
    fix: Use z.input and z.output explicitly for schemas with complex transforms. Type-assert or construct the output in transform return.
  - id: failure:zod/recursive-schema-stack
    symptom: Maximum call stack exceeded when parsing recursive data structures.
    cause: Recursive schemas without lazy wrapping cause infinite initialization.
    fix: Wrap recursive references in () => schema or use z.late.object(). Ensure base cases are non-recursive.
  - id: failure:zod/discriminated-union-typo
    symptom: Union validation passes for wrong variant; downstream code receives unexpected shape.
    cause: Discriminated union key value misspelled or case mismatch.
    fix: Use z.literal() with exact string values. Enable strict mode. Validate with safeParse and inspect the discriminator field.
extends: []
implements: []
uses:
  - concept:typescript
  - concept:data-validation
part_of: concept:typescript-ecosystem
solves:
  - problem:runtime-type-safety
  - problem:api-input-validation
  - problem:form-validation-typescript
alternatives:
  - package:joi
  - package:yup
  - package:superstruct
  - package:valibot
---
Zod's foundational insight is that **TypeScript types are compile-time only** — they disappear at runtime. Any boundary where external data enters the system (HTTP requests, form submissions, config files, database records, localStorage) is a type-safety gap. Zod fills this gap by using schemas as a single source of truth that produce both a runtime validator and a compile-time type.

The schema composition model is what makes Zod scalable. z.object({ profile: z.object({ address: z.object({...}) }) }) with .pick(), .omit(), .partial(), and .extend() lets you define a base schema and derive variants for create, update, response, and filtering — each with a precise TypeScript type. Branded types add nominal typing (z.string().brand<"UserId">()) to prevent mixing up raw strings and validated identifiers, while discriminated unions optimize parsing of tagged union types by checking the discriminator first.

Zod's error handling is designed for user-facing validation. ZodError.format() produces a nested object mirroring the schema structure with per-field errors, ideal for form libraries. ZodError.flatten() gives flat arrays for simple display. Custom error maps allow internationalization and consistent error messages across the application.
