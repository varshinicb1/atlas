---
kind: Package
id: package:next-auth
name: NextAuth.js / Auth.js
version: "5"
purpose: Document Auth.js (NextAuth v5) authentication patterns — providers, database adapters, middleware, callbacks, sessions, and enterprise SSO for Next.js applications.
problem_solved: Provides a configurable authentication framework for Next.js that supports 80+ OAuth providers, database-backed sessions, JWT, and middleware-based route protection, eliminating the need to implement OAuth flows, session management, and CSRF protection from scratch.
install: npm install next-auth@beta @auth/core
dependencies:
  - concept:authentication
  - concept:next-js
  - concept:typescript
concepts:
  - name: Auth.js v5 (NextAuth)
    id: concept:next-auth/authjs
    description: "The fifth major version (re-branded as Auth.js framework). Server-first authentication with enhanced type safety, edge runtime support, and a modular provider system. Uses an auth() function (server-side) and useSession() (client-side). Replaces the /api/auth/[...nextauth] catch-all with a central auth config."
  - name: Providers
    id: concept:next-auth/providers
    description: "Authentication strategies: OAuth (Google, GitHub, Apple, Discord — 80+ built-in), Email (magic links via Resend/SendGrid), Credentials (email+password, requires database adapter), and WebAuthn (passkeys). Each provider has a clientId, clientSecret, and optional profile() callback for user mapping."
  - name: Database Adapters
    id: concept:next-auth/adapters
    description: "Persist users, accounts, sessions, and verification tokens in a database. Official adapters: Prisma, Drizzle, Kysely, MongoDB, Firebase, Supabase, D1. Adapters implement createUser, getUser, linkAccount, createSession, etc. Prisma adapter: Adapter(prisma) in the auth config."
  - name: Callbacks
    id: concept:next-auth/callbacks
    description: "Functions that run at each auth lifecycle stage: signIn (control who can sign in), redirect (customize post-auth redirect), jwt (modify the JWT token), session (expose data to the client). Callbacks receive token/user/account/profiles and must return the modified value."
  - name: Middleware
    id: concept:next-auth/middleware
    description: "Route protection via Next.js middleware. export { auth as middleware } from '@/auth' protects all routes. Customize with matcher config to exclude public paths. The auth() function in middleware checks the session and redirects unauthenticated users to the sign-in page."
  - name: Sessions
    id: concept:next-auth/sessions
    description: "JWT sessions (default) store user info in an encrypted cookie — no database lookup needed. Database sessions store session records and support revocation. session strategy: 'jwt' or 'database'. JWT is faster and works without a database; database sessions provide server-side control."
  - name: Server Actions & API Routes
    id: concept:next-auth/server-actions
    description: "signIn(provider, options) and signOut() are importable directly in Server Components and Server Actions. auth() returns the current session in server context. Protect Server Actions by calling auth() and checking for a user. Redirect after sign-in via redirectTo option."
  - name: Client API
    id: concept:next-auth/client
    description: "useSession() hook returns { data: Session | null, status: 'loading' | 'authenticated' | 'unauthenticated', update }. SessionProvider wraps the app root. signIn(provider) and signOut() client functions for button-driven auth. The SessionProvider is a client component that hydrates session state."
  - name: Credentials Provider
    id: concept:next-auth/credentials
    description: "Custom username/password authentication. Requires an authorize() callback that validates credentials against a database. Must use a database adapter for persistence. Credentials provider credentials are not chained; the authorize function runs on every sign-in attempt."
  - name: Email Provider
    id: concept:next-auth/email
    description: "Passwordless sign-in via magic links. Provider sends a one-time link to the user's email. Requires an email service (Resend, SendGrid, SMTP). from and server configure the email sender. generateVerificationToken and sendVerificationRequest can be customized."
  - name: OAuth Account Linking
    id: concept:next-auth/account-linking
    description: "When a user signs in with multiple providers, Auth.js links accounts by email. If the database adapter finds a user with the same email, the new provider account is linked to the existing user. Customize via the signIn callback to implement allow/deny logic."
apis:
  - name: NextAuth(config)
    id: api:next-auth/auth
    signature: "export const { handlers: { GET, POST }, auth, signIn, signOut } = NextAuth(config: AuthConfig)"
    returns: Object with handlers, auth, signIn, signOut functions.
    description: "Creates the auth configuration. config includes providers, adapter, callbacks, session strategy, pages (custom sign-in page), trustHost, and debug. The handlers handle API routes, auth() checks sessions server-side, signIn/signOut are callable in actions."
  - name: auth()
    id: api:next-auth/auth-fn
    signature: "const session = await auth() -> Session | null"
    returns: The current session or null.
    description: "Server-side function that returns the current session. Use in Server Components, Server Actions, Route Handlers, and middleware. Returns null when unauthenticated. The Session type is augmented by callbacks (session callback adds custom fields)."
  - name: signIn(provider, options)
    id: api:next-auth/signin
    signature: "await signIn('github', { redirectTo: '/dashboard', redirect: true })"
    returns: Redirects or returns void.
    description: "Initiates sign-in flow for the given provider. Callable from both server (actions) and client (hooks). redirectTo specifies post-auth destination. For server actions with redirect: true, Next.js handles the redirect. When redirect: false, returns { url, error }."
  - name: useSession()
    id: api:next-auth/use-session
    signature: "const { data: session, status, update } = useSession() -> { data: Session | null, status: 'loading' | 'authenticated' | 'unauthenticated', update: () => Promise<Session | null> }"
    returns: Session state with update function.
    description: "Client-side hook returning the current session session. status is 'loading' during hydration, 'authenticated' when logged in, 'unauthenticated' when not. update() refreshes the session without a page reload (useful after profile updates)."
  - name: middleware matcher
    id: api:next-auth/matcher
    signature: "export const config = { matcher: ['/((?!api|_next/static|_next/image|favicon.ico).*)'] }"
    returns: A Next.js middleware config object.
    description: "Defines which paths the auth middleware runs on. The matcher is a list of route patterns. Exclude public assets, API routes that should be accessible, and auth pages. Without a matcher, middleware runs on every request."
sections:
  - title: Auth Configuration
    id: section:next-auth/auth-config
    content: |
      Set up the auth configuration with multiple providers and Prisma adapter:

      ```typescript
      import NextAuth from 'next-auth';
      import GitHub from 'next-auth/providers/github';
      import Google from 'next-auth/providers/google';
      import Resend from 'next-auth/provients/resend';
      import { PrismaAdapter } from '@auth/prisma-adapter';
      import { prisma } from './prisma';

      export const { handlers, auth, signIn, signOut } = NextAuth({
          adapter: PrismaAdapter(prisma),
          session: { strategy: 'jwt' },
          providers: [
              GitHub({ clientId: process.env.GITHUB_ID!, clientSecret: process.env.GITHUB_SECRET! }),
              Google,
              Resend({ from: 'noreply@myapp.com' }),
          ],
          callbacks: {
              jwt({ token, user }) {
                  if (user) token.role = user.role;
                  return token;
              },
              session({ session, token }) {
                  session.user.id = token.sub!;
                  session.user.role = token.role as string;
                  return session;
              },
          },
          pages: {
              signIn: '/auth/signin',
              error: '/auth/error',
          },
      });
      ```
  - title: Route Protection
    id: section:next-auth/route-protection
    content: |
      Protect routes using middleware and server-side auth checks:

      ```typescript
      // middleware.ts
      export { auth as middleware } from '@/auth';
      export const config = {
          matcher: ['/((?!api/auth|_next/static|_next/image|favicon.ico|public).*)'],
      };

      // Server Component protection
      import { auth } from '@/auth';
      import { redirect } from 'next/navigation';

      export default async function DashboardPage() {
          const session = await auth();
          if (!session) redirect('/auth/signin');
          return <div>Welcome {session.user.name}</div>;
      }

      // Server Action protection
      'use server';
      export async function deleteAccount() {
          const session = await auth();
          if (!session?.user) throw new Error('Unauthorized');
          await prisma.user.delete({ where: { id: session.user.id } });
      }
      ```
---
