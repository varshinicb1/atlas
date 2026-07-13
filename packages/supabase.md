---
kind: Package
id: package:supabase
name: Supabase Patterns
version: "2"
purpose: Document Supabase patterns — database, authentication, realtime, storage, and Edge Functions for building full-stack applications with a Firebase-like backend-as-a-service on PostgreSQL.
problem_solved: Provides an open-source Firebase alternative built on PostgreSQL with auto-generated REST and GraphQL APIs, built-in auth, real-time subscriptions, file storage, and serverless functions — eliminating backend boilerplate while retaining raw SQL access for complex queries.
install: npm install @supabase/supabase-js @supabase/ssr
dependencies:
  - concept:postgresql
  - concept:authentication
  - concept:realtime
concepts:
  - name: Supabase Client
    id: concept:supabase/client
    description: "createClient(url, anonKey) returns a JavaScript client with database, auth, storage, and realtime APIs. The anon key is safe for client-side use because Row Level Security (RLS) policies control data access. Use @supabase/ssr for Next.js App Router compatibility with cookie-based session management."
  - name: Database (PostgreSQL)
    id: concept:supabase/database
    description: "Every Supabase project is a full PostgreSQL database. Use the SQL Editor in the dashboard or raw SQL for schema design. Supabase auto-generates REST (via PostgREST) and GraphQL (via pg_graphql) APIs from your schema. Full-text search, foreign keys, views, and triggers are supported."
  - name: Row Level Security (RLS)
    id: concept:supabase/rls
    description: "PostgreSQL security policies that restrict row access based on the authenticated user. Enable RLS on tables and write policies using SQL expressions: USING (user_id = auth.uid()). RLS is mandatory for client-side data access — without it, the anon key has no server-side protections."
  - name: Authentication
    id: concept:supabase/auth
    description: "Built-in auth with email/password, magic links, OAuth (Google, GitHub, Apple, 20+ providers), phone/SMS, and SSO. Auth stores users in the auth.users table. Integrates with RLS via auth.uid() and auth.email() functions. Session management via cookies (SSR) or localStorage (SPA)."
  - name: Realtime
    id: concept:supabase/realtime
    description: "WebSocket-based real-time engine that broadcasts database changes to subscribed clients. Subscribe to INSERT, UPDATE, DELETE, or * on any table. supabase.channel('changes').on('postgres_changes', { event: 'INSERT', schema: 'public', table: 'messages' }, callback). Broadcast and presence channels also supported."
  - name: Storage
    id: concept:supabase/storage
    description: "S3-compatible file storage with public/private buckets. Upload via supabase.storage.from('avatars').upload(path, file). Public URLs are cached at CDN. Private files require signed URLs: storage.from('docs').createSignedUrl(path, 60). RLS policies also apply to storage buckets."
  - name: Edge Functions
    id: concept:supabase/edge-functions
    description: "Deno-based serverless functions running at the edge. Deploy via supabase functions deploy my-function. Accept POST requests with JSON body, return Responses. Use for webhooks, custom API endpoints, and operations that need server-side computation (e.g., generating PDFs, processing payments)."
  - name: TypeScript Types
    id: concept:supabase/typescript
    description: "supabase gen types typescript --linked generates TypeScript types from your database schema. Import Database type and use supabase.from<Database['public']['Tables']['users']['Row']>(). Provides full autocompletion for select, insert, and update operations against your exact schema."
  - name: Supabase CLI
    id: concept:supabase/cli
    description: "Local development via supabase init, supabase start (Docker-based local Supabase stack), supabase db diff (migrations), supabase functions serve (local Edge Functions). supabase link connects to a remote project. Migrations are version-controlled SQL files in supabase/migrations/."
  - name: Row Level Security Policies
    id: concept:supabase/rls-policies
    description: "SQL policies written in the dashboard or via migration: CREATE POLICY \"users_view_own\" ON users FOR SELECT USING (auth.uid() = id). Common patterns: owner-only access, role-based access, team-based access through join tables, and admin override policies."
  - name: Multi-Factor Authentication
    id: concept:supabase/mfa
    description: "TOTP-based multi-factor authentication via authenticator apps. Enroll using supabase.auth.mfa.enroll(), challenge/verify via mfa.challenge() and mfa.verify(). App-level verification (rather than session-level) allows custom flows like step-up authentication for sensitive operations."
apis:
  - name: supabase.from().select()
    id: api:supabase/select
    signature: "supabase.from('users').select('id, name, email', { count: 'exact', head: false }) -> PostgrestFilterBuilder"
    returns: A query builder that is awaitable or chainable.
    description: "Starts a SELECT query on the specified table. The first argument is a comma-separated column list or a GraphQL-style nested query (projects(*), tasks(*)). Chain .eq(), .neq(), .gt(), .lt(), .in(), .like(), .ilike(), .is(), .contains(), .order(), .limit(), .range()."
  - name: supabase.from().insert()
    id: api:supabase/insert
    signature: "supabase.from('users').insert(values: object | object[], options?: { defaultToNull, count }) -> PostgrestBuilder"
    returns: A query builder that resolves to inserted rows.
    description: "Inserts one or more rows into the table. Pass a single object or an array for batch insert. RLS policies determine if the insert is allowed. Returns the inserted rows (if RLS allows reading them). Upsert via .upsert() for on-conflict update."
  - name: supabase.auth.signInWithOAuth()
    id: api:supabase/signin-oauth
    signature: "supabase.auth.signInWithOAuth({ provider: 'github', options: { redirectTo: 'https://myapp.com/callback', scopes: 'read:user' } }) -> Promise<{ data, error }>"
    returns: An AuthResponse with session data or error.
    description: "Signs in using an OAuth provider. Opens a popup or redirects the browser to the provider's auth page. redirectTo controls the post-auth redirect. scopes requests additional permissions. Works in both browser and mobile (via universal links)."
  - name: supabase.channel().on().subscribe()
    id: api:supabase/realtime-subscribe
    signature: "supabase.channel('custom-name').on('postgres_changes', { event: '*', schema: 'public', table: 'messages', filter: 'user_id=eq.123' }, payload => {...}).subscribe() -> RealtimeChannel"
    returns: A RealtimeChannel subscription.
    description: "Subscribes to real-time database changes. event is INSERT, UPDATE, DELETE, or *. filter uses PostgREST syntax. The callback receives the changed record in payload.new and payload.old. Returns a channel object — call .unsubscribe() to stop listening."
  - name: supabase.storage.from().upload()
    id: api:supabase/storage-upload
    signature: "supabase.storage.from('bucket').upload(path: string, file: File | Blob | ArrayBuffer, options?: { cacheControl, contentType, upsert }) -> Promise<{ data: { path }, error }>"
    returns: Upload result with file path.
    description: "Uploads a file to the specified bucket and path. RLS policies on the storage.objects table control access. upsert: true overwrites existing files. Returns the file path for generating public or signed URLs."
sections:
  - title: Basic CRUD with RLS
    id: section:supabase/crud
    content: |
      Client-side queries protected by Row Level Security:

      ```typescript
      import { createClient } from '@supabase/supabase-js';
      import type { Database } from './database.types';

      const supabase = createClient<Database>(
          process.env.NEXT_PUBLIC_SUPABASE_URL!,
          process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!
      );

      // RLS policy ensures only the owner can read their own tasks
      const { data: tasks, error } = await supabase
          .from('tasks')
          .select('id, title, completed, created_at')
          .eq('user_id', userId) // Redundant because RLS filters, but documents intent
          .order('created_at', { ascending: false })
          .limit(20);

      // Insert (RLS policy with (auth.uid() = user_id) for INSERT)
      const { data: newTask } = await supabase
          .from('tasks')
          .insert({ title: 'Buy groceries', user_id: userId })
          .select()
          .single();
      ```
  - title: Next.js App Router Integration
    id: section:supabase/nextjs
    content: |
      Server-side client for SSR with cookie-based auth:

      ```typescript
      import { createServerClient } from '@supabase/ssr';
      import { cookies } from 'next/headers';

      export function createClient() {
          return createServerClient(
              process.env.NEXT_PUBLIC_SUPABASE_URL!,
              process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!,
              {
                  cookies: {
                      async getAll() { const store = await cookies(); return store.getAll(); },
                      async setAll(cookiesList) {
                          const store = await cookies();
                          cookiesList.forEach(({ name, value, options }) => store.set(name, value, options));
                      },
                  },
              }
          );
      }

      // In a Server Component
      export default async function ProfilePage() {
          const supabase = createClient();
          const { data: { user } } = await supabase.auth.getUser();
          const { data: profile } = await supabase.from('profiles').select('*').eq('id', user?.id).single();
          return <div>{profile?.username}</div>;
      }
      ```

      The SSR client uses cookie-based auth for seamless session handling across server and client.
---
