---
kind: Package
id: package:clerk
name: Clerk Auth
version: "5.0"
purpose: Document Clerk — user management and authentication platform with pre-built UI components, session management, multi-factor authentication, organization management, and framework-specific SDKs for Next.js, Remix, and React.
problem_solved: Eliminates the complexity of building authentication from scratch — password hashing, session tokens, OAuth flows, MFA, password reset, email verification, user profiles, organization management — by providing drop-in React components, webhook-based user sync, and a full management API.
install: npm install @clerk/nextjs
dependencies:
  - concept:authentication
  - concept:react
  - concept:web-security
concepts:
  - name: Clerk Provider
    id: concept:clerk/provider
    description: "<ClerkProvider> wraps the application — provides auth context to all Clerk hooks and components. Configure with publishableKey from the Clerk Dashboard. Framework-specific: @clerk/nextjs wraps Next.js App Router with automatic middleware protection."
  - name: Authentication Components
    id: concept:clerk/components
    description: Pre-built, customizable UI components — <SignIn />, <SignUp />, <UserButton />, <UserProfile />, <OrganizationSwitcher />, <OrganizationProfile />, <CreateOrganization />. Components are fully themed via CSS variables or Tailwind and handle all auth flows (OAuth, email, password, MFA).
  - name: Hooks
    id: concept:clerk/hooks
    description: useAuth() (session status and permissions), useUser() (current user data), useOrganization() (active org), useSession() (session details), useSignIn()/useSignUp() (custom auth flows). All hooks are reactive — UI updates when auth state changes.
  - name: Middleware
    id: concept:clerk/middleware
    description: Next.js middleware.ts — clerkMiddleware() protects routes based on public/private configuration. Checks session, redirects to sign-in, and passes auth state to the application. Supports route groups, custom matchers, and API route protection.
  - name: Webhooks
    id: concept:clerk/webhooks
    description: Clerk sends webhooks (user.created, user.updated, session.created, organization.created, etc.) to your server. Use Svix for reliable delivery with retries. Sync user data to your database, trigger onboarding emails, or update search indexes on auth events.
  - name: Organizations
    id: concept:clerk/organizations
    description: Multi-tenant support — create organizations, invite members, manage roles (admin, member, custom), and scope permissions. <OrganizationSwitcher /> lets users switch between org and personal account. Organization context is available via useOrganization().
  - name: Session Management
    id: concept:clerk/session
    description: Clerk manages sessions via HTTP-only cookies and/or JWT tokens. Sessions auto-refresh, handle device rotation, and support concurrent active sessions. Revoke sessions from the Dashboard or API for security events.
  - name: MFA
    id: concept:clerk/mfa
    description: Multi-factor authentication via authenticator apps (TOTP), SMS codes, backup codes, and hardware security keys (WebAuthn). Enforce MFA per-user or across the organization. MFA flows are handled automatically by Clerk components.
  - name: OAuth / Social Login
    id: concept:clerk/oauth
    description: 30+ OAuth providers — Google, GitHub, Microsoft, Apple, Discord, Slack, X, LinkedIn, Facebook, GitLab, Bitbucket, Dropbox, Atlassian, HubSpot, Notion, and more. Configured in Clerk Dashboard. User profile data populated automatically from provider responses.
  - name: User Data Sync
    id: concept:clerk/user-sync
    description: Keep your database in sync with Clerk's user data via webhooks or the Clerk Backend API. Access metadata, external accounts, and custom user data stored in Clerk's private and public metadata fields.
  - name: Theming
    id: concept:clerk/theming
    description: Customize all Clerk components via appearance prop — baseTheme, variables (colorPrimary, colorBackground, colorText, borderRadius, fontFamily), elements (target specific sub-components), and layout (showOptionalFields, socialButtonsPlacement, shimmer).
apis:
  - name: <ClerkProvider>
    id: api:clerk/provider
    signature: "<ClerkProvider publishableKey={pk} afterSignInUrl='/dashboard' afterSignUpUrl='/onboarding'><App /></ClerkProvider>"
    returns: Auth context provider.
    description: "Wraps the application root. Required for all Clerk hooks and components. Props: publishableKey, afterSignInUrl, afterSignUpUrl, signInUrl, signUpUrl, appearance, and dynamic."
  - name: clerkMiddleware()
    id: api:clerk/middleware
    signature: "export default clerkMiddleware((auth, req) => { if (!auth.userId) { return redirectToSignIn(req) } }), { publicRoutes: ['/', '/about', '/pricing'] })"
    returns: Next.js middleware handler.
    description: Protects routes based on auth status. publicRoutes bypass auth check. After auth check, redirects to sign-in or returns 401. Passes auth info in request headers for server components.
  - name: auth()
    id: api:clerk/auth
    signature: "const { userId, sessionClaims, orgId, orgRole } = auth()"
    returns: Auth state object.
    description: Server-side auth check — use in Server Components, API routes, and Route Handlers. Returns userId, sessionClaims, orgId, orgRole, and has(). Throws if unauthenticated. Must be called in a route/layout or with clerkClient() for server-only code.
  - name: currentUser()
    id: api:clerk/current-user
    signature: "const user = await currentUser(): User | null"
    returns: Full User object or null.
    description: Server-side user data — returns the full Clerk User object with email addresses, phone numbers, external accounts, imageUrl, and metadata. Must be called in a Server Component or Route Handler.
  - name: useUser()
    id: api:clerk/use-user
    signature: "const { user, isLoaded, isSignedIn } = useUser(): { user: User | null, isLoaded: boolean, isSignedIn: boolean }"
    returns: Client-side user state.
    description: React hook for current user data. isLoaded false during initial load. isSignedIn true when authenticated. user provides full profile, imageUrl, emailAddresses, and metadata.
  - name: useAuth()
    id: api:clerk/use-auth
    signature: "const { userId, orgId, orgRole, orgSlug, sessionId, isLoaded, isSignedIn, signOut, getToken } = useAuth()"
    returns: Auth state and helpers.
    description: Client-side auth state hook. getToken() returns a signed JWT for API authorization. signOut() ends the session. orgId/orgRole for multi-tenant apps. isLoaded false during hydration.
  - name: <SignIn /> / <SignUp />
    id: api:clerk/sign-in
    signature: "<SignIn routing='path' path='/sign-in' signUpUrl='/sign-up' fallbackRedirectUrl='/dashboard' />"
    returns: Pre-built auth form component.
    description: "Full authentication forms with email/password, OAuth buttons, MFA, password reset, and social login flows. routing: 'path' (URL-based) or 'hash' (SPA). Fully customizable via appearance prop."
  - name: clerkClient.users.createUser(data)
    id: api:clerk/admin-api
    signature: "const user = await clerkClient.users.createUser({ emailAddress: ['user@example.com'], password: '...', publicMetadata: { plan: 'pro' } })"
    returns: Created User object.
    description: Server-side admin API for user management. Create, update, delete users, manage sessions, set metadata, and trigger verification emails. Requires Clerk Secret Key. Use in webhook handlers or admin panels.
failures:
  - id: failure:clerk/middleware-missing
    symptom: Protected page briefly renders before redirecting to sign-in.
    cause: Next.js middleware not running or async middleware returning before auth check.
    fix: Ensure clerkMiddleware() is exported from middleware.ts. Add publicRoutes for pages that should render before auth. Move heavy data fetching to useEffect or after auth check.
  - id: failure:clerk/env-variable-missing
    symptom: ClerkProvider throws "Missing publishable key" error.
    cause: NEXT_PUBLIC_CLERK_PUBLISHABLE_KEY or CLERK_SECRET_KEY not set in .env.local.
    fix: Add both env variables from Clerk Dashboard -> API Keys. Restart the dev server after setting. Verify they are in the correct .env file (local vs production).
  - id: failure:clerk/webhook-verification
    symptom: Webhook handler receives invalid payloads or signature errors.
    cause: Svix webhook signature not verified before processing the payload.
    fix: Use Clerk's Webhook utility — verify Svix headers (svix-id, svix-signature, svix-timestamp) with the webhook signing secret from Clerk Dashboard. Use @clerk/nextjs/webhooks or Svix library for verification.
  - id: failure:clerk/org-scope-missing
    symptom: Organization API returns empty or null for org-related queries.
    cause: ClerkProvider missing the organization prop, or org is not active in the session.
    fix: Wrap org-specific pages with the organization context. Use <OrganizationSwitcher /> to let users select/switch orgs. Pass organization prop to ClerkProvider if the page is org-scoped.
  - id: failure:clerk/session-expired
    symptom: API calls fail with 401 after long user inactivity.
    cause: Session token expired but getToken() returns the expired token.
    fix: "Clerk auto-refreshes sessions. Force refresh by calling await getToken({ template: '...', skipCache: true }). Use the session lifecycle webhooks to detect and handle session expiry server-side."
extends: []
implements: []
uses:
  - concept:authentication
  - concept:react
part_of: concept:b2b-saas-stack
solves:
  - problem:user-authentication
  - problem:user-management
  - problem:multi-tenant-organization-access
alternatives:
  - package:next-auth-authjs
  - package:supabase-auth
  - package:firebase-auth
  - package:lucia-auth
---
Clerk's value proposition is that authentication is a product, not a library. Instead of building auth screens, session management, email flows, and MFA yourself, Clerk provides production-ready UI components that handle the entire auth lifecycle. The components are server-side renderable, SEO-friendly, and accessible by default.

The architecture is component-first with framework adapters. @clerk/nextjs integrates at the Next.js middleware layer — clerkMiddleware() intercepts requests, validates sessions, and redirects unauthenticated users before any page renders. Server Components access auth via auth() (session check) and currentUser() (full profile), while client components use hooks (useUser, useAuth). No context switching between server and client auth patterns.

Organizations are a first-class concept in Clerk — not an add-on. Multi-tenant apps get organization management (create, invite, role-based permissions) through pre-built components and hooks. The <OrganizationSwitcher /> component allows users to toggle between personal and organization contexts, and org-scoped webhooks fire on membership changes and role updates.
