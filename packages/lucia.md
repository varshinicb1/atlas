---
kind: Package
id: package:lucia
name: Lucia Auth
version: "3.2"
purpose: Document Lucia — a library-agnostic authentication framework for TypeScript that provides session management, database adapters, OAuth integrations, and framework SDKs without prescribing a specific database or UI library.
problem_solved: Provides a modular, unopinionated auth layer that works with any database (Drizzle, Prisma, Kysely, MongoDB, SQLite) and any framework (Next.js, SvelteKit, Astro, Hono, Elysia) — unlike all-in-one auth solutions that lock you into their database and UI model.
install: npm install lucia
dependencies:
  - concept:authentication
  - concept:typescript
  - concept:session-management
concepts:
  - name: Adapters
    id: concept:lucia/adapters
    description: Database adapters that connect Lucia to your database — @lucia-auth/adapter-drizzle, @lucia-auth/adapter-prisma, @lucia-auth/adapter-kysely, @lucia-auth/adapter-mongoose, @lucia-auth/adapter-sqlite. Lucia provides the session/user table schemas, adapters implement read/write operations.
  - name: Sessions
    id: concept:lucia/sessions
    description: Lucia-managed sessions stored in your database. Session IDs are hashed before storage. Sessions have expiration (configurable, default 30 days) and idle timeout (configurable, default 7 days). Active sessions extend idle timeout on activity. Multiple sessions per user (device tracking).
  - name: OAuth
    id: concept:lucia/oauth
    description: "@lucia-auth/oauth provides OAuth 2.0 / OpenID Connect providers — Google, GitHub, Discord, Apple, Facebook, Microsoft, Spotify, Twitch, and more. Each provider returns user profile and tokens. Use Lucia's session API to create sessions after OAuth callback."
  - name: Framework Middleware
    id: concept:lucia/middleware
    description: Lucia provides framework middleware/helpers — validateSession() for SvelteKit hooks, auth() for Next.js middleware, handleRequest() for Astro, and handle() for Hono/Elysia. Middleware reads the session cookie, validates the session, and provides auth state to request handlers.
  - name: Passwords
    id: concept:lucia/passwords
    description: Password hashing via @lucia-auth/password (uses Scrypt with configurable parameters) or integrate any hashing library (bcrypt, argon2). Lucia stores credentials in a separate credentials table linked to users. Password reset via tokens stored in the database.
  - name: Email & Passwordless
    id: concept:lucia/email
    description: Passwordless authentication via email links or one-time codes. Store verification tokens in your database with expiration. Lucia provides the building blocks — you implement the email sending and UI.
  - name: User Management
    id: concept:lucia/users
    description: Lucia stores users in a database table. User attributes (username, email, avatar, role) are stored in the user table or a separate profile table. Lucia does not manage user profiles — that's your application concern. Webhooks for user lifecycle events.
  - name: Session Validation
    id: concept:lucia/session-validation
    description: Every request validates the session by reading the session cookie, looking up the session in the database, and checking expiration. Active sessions extend idle timeout. Expired sessions are deleted. Session validation is the core security primitive.
apis:
  - name: lucia.initialize(opts)
    id: api:lucia/initialize
    signature: "const lucia = new Lucia(adapter, { sessionCookie: { attributes: { secure: true } }, getUserAttributes: (dbUser) => ({ username: dbUser.username }) })"
    returns: A Lucia instance.
    description: Creates the main Lucia instance. adapter connects to your database. sessionCookie configures cookie attributes (secure, sameSite, domain). getUserAttributes maps database user fields to auth context.
  - name: lucia.createSession(userId, opts)
    id: api:lucia/createSession
    signature: "const session = await lucia.createSession(userId, { attributes: {} })"
    returns: Session object with id, userId, expiresAt, idleExpiresAt.
    description: Creates a new session for the given user. Store the session cookie with lucia.createSessionCookie(session.id). Session attributes are optionally stored alongside the session.
  - name: lucia.validateSession(sessionId)
    id: api:lucia/validateSession
    signature: "const { session, user } = await lucia.validateSession(sessionId)"
    returns: "{ session: Session | null, user: User | null }"
    description: Validates a session by ID. Returns null for expired or invalid sessions. Active sessions extending idle timeout are updated in the database. Use in middleware to check auth status on every request.
  - name: lucia.invalidateSession(sessionId)
    id: api:lucia/invalidateSession
    signature: "await lucia.invalidateSession(sessionId)"
    returns: void
    description: Invalidates a single session. Used for logout. Removes the session from the database. Also invalidateUserSessions(userId) for all sessions (password change, security events).
  - name: lucia.createSessionCookie(sessionId)
    id: api:lucia/createSessionCookie
    signature: "const cookie = lucia.createSessionCookie(session.id)"
    returns: A cookie configuration object.
    description: Creates a session cookie with the session ID. The cookie has HttpOnly, Secure, SameSite=Lax, and a configurable expiration. Use setCookie on the response to attach the cookie. lucia.createBlankSessionCookie() for logout cookie revocation.
  - name: githubAuth(creds)
    id: api:lucia/github-auth
    signature: "const githubAuth = new GitHub(env.GITHUB_CLIENT_ID, env.GITHUB_CLIENT_SECRET, redirectURI)"
    returns: An OAuth provider instance.
    description: "Creates a GitHub OAuth provider. Authorization URL: githubAuth.authorizeURL() with optional scopes. Callback: githubAuth.validateCallback(code) returns GitHubUser with access token. Create Lucia session after successful OAuth."
failures:
  - id: failure:lucia/database-adapter-mismatch
    symptom: Sessions fail to create or validate with adapter-specific errors.
    cause: Adapter schema does not match Lucia's expected table structure (users table must have id, sessions table must have id/userId/expiresAt).
    fix: Follow the adapter's schema setup instructions exactly. Run the provided SQL or schema migration. Verify table names and column types match adapter expectations.
  - id: failure:lucia/cookie-not-set
    symptom: User is not authenticated after successful OAuth callback or login.
    cause: Lucia session cookie not attached to the response after creating the session.
    fix: "After lucia.createSession(), set the cookie on the response: response.headers.set('Set-Cookie', lucia.createSessionCookie(session.id).serialize()). Framework helpers (auth.setSession()) handle this automatically."
  - id: failure:lucia/session-expiry-fast
    symptom: Users are logged out after short inactivity even though session duration is set correctly.
    cause: Idle timeout default is shorter than expected (7 days). Session is expired on idle timeout even if absolute expiration is far.
    fix: Increase session.expiresIn (absolute expiration) and session.idlePeriod (idle timeout). Call lucia.validateSession() on every request to extend idle timeout. Verify the idlePeriod configuration in Lucia initialization.
  - id: failure:lucia/oauth-state-mismatch
    symptom: OAuth callback fails with "state mismatch" error.
    cause: OAuth state cookie not set or read correctly between the authorization and callback requests.
    fix: Generate a random state, store it in a cookie (HttpOnly, secure), and verify it in the callback handler. Lucia's GitHub.authorizeURL() returns a state — store and compare it against the callback state parameter.
extends: []
implements: []
uses:
  - concept:authentication
  - concept:typescript
  - concept:session-management
part_of: concept:typescript-auth-ecosystem
solves:
  - problem:framework-agnostic-authentication
  - problem:database-first-auth
  - problem:oauth-integration
alternatives:
  - package:clerk
  - package:next-auth-authjs
  - package:supabase-auth
  - package:better-auth
---
Lucia takes a fundamentally different approach from all-in-one auth solutions like Clerk or Supabase. Instead of owning the user database and UI, Lucia is a thin session management layer that lives in YOUR database. User records, sessions, and OAuth accounts are stored in your PostgreSQL, MySQL, SQLite, or MongoDB — not a third-party service. This gives you full control over user data, no vendor lock-in, and no dependency on external APIs for core authentication.

The adapter pattern is Lucia's architectural keystone. Every database adapter implements the same interface (getUser, createSession, validateSession, deleteSession, etc.). This means your auth code is database-agnostic — switching from SQLite to PostgreSQL is a one-line adapter swap. The adapter ecosystem covers Prisma, Drizzle, Kysely, Mongoose, SQLite (better-sqlite3, Bun SQLite), and Knex.

Session management in Lucia is designed for security. Session IDs are generated with sufficient entropy, hashed before storage (only the hash lives in your DB), validated on every request, and automatically expired. The dual expiration model (absolute + idle timeout) balances UX (auto-logout after inactivity) with security (forced re-authentication after N days). Framework middleware integrates session validation into the request lifecycle, making auth state available in every route handler.
