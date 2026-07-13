---
kind: Package
id: package:tanstack-query
name: TanStack Query (React Query)
version: "5.60"
purpose: Document TanStack Query — server state management for React, Vue, Solid, and Svelte with automatic caching, background refetching, optimistic updates, and infinite queries.
problem_solved: Frees developers from managing server state lifecycle (loading, error, success, stale-while-revalidate, cache invalidation) by providing a declarative, hook-based API that handles caching, background synchronization, request deduplication, and garbage collection automatically.
install: npm install @tanstack/react-query
dependencies:
  - concept:react
  - concept:http-client-fetch
  - concept:state-management
concepts:
  - name: Queries
    id: concept:tanstack-query/queries
    description: "useQuery({ queryKey, queryFn }) — declarative data fetching with automatic caching, stale detection, background refetch, and request deduplication. The queryKey uniquely identifies cached data and enables targeted invalidation. Multiple components using the same key share one cache entry."
  - name: Query Keys
    id: concept:tanstack-query/query-keys
    description: "A serializable array that uniquely identifies a query — ['todos'] for a list, ['todo', id] for a single item. Keys are hierarchical: ['todos', 'list', { status: 'done' }] for filtered queries. The structure determines invalidation granularity — invalidating ['todos'] refetches all todos queries."
  - name: Mutations
    id: concept:tanstack-query/mutations
    description: "useMutation({ mutationFn }) — create, update, delete operations with onSuccess/onError/onSettled callbacks. Mutations trigger side effects like cache invalidation and optimistic updates. Unlike queries, mutations share no cache and are not automatically retried by default."
  - name: Cache Layer
    id: concept:tanstack-query/cache
    description: In-memory cache with configurable staleTime (how long data is considered fresh), gcTime (how long inactive data stays in cache before garbage collection), and retry policies. The cache is normalized by queryKey — updates to one key can trigger refetches of related keys.
  - name: Stale-While-Revalidate
    id: concept:tanstack-query/swr
    description: "The default strategy — return cached data immediately (if available) while refetching in the background. staleTime controls the threshold: during staleTime, data serves from cache without refetch. After staleTime, the next render triggers a background refetch."
  - name: Infinite Queries
    id: concept:tanstack-query/infinite
    description: "useInfiniteQuery({ queryKey, queryFn, getNextPageParam, initialPageParam }) — paginated data loading with cursor/offset tracking. Pages are stored as an array and concatenated. fetchNextPage() loads more. getNextPageParam extracts the cursor from the last page."
  - name: Optimistic Updates
    id: concept:tanstack-query/optimistic
    description: Update the cache immediately before the mutation network request completes, then rollback on error. Implemented via onMutate (update cache, save previous value), onError (rollback), onSettled (invalidate to sync with server). Creates instant UI feedback.
  - name: Query Invalidation
    id: concept:tanstack-query/invalidation
    description: "queryClient.invalidateQueries({ queryKey: ['todos'] }) marks matching queries as stale and triggers refetch. Can invalidate exactly (exact: true) or hierarchically (keys starting with prefix). Use case: after creating a todo, invalidate ['todos'] to refresh lists."
  - name: Query Deduplication
    id: concept:tanstack-query/dedup
    description: "Multiple components mounting simultaneously with the same queryKey share one network request. The query goes through a lifecycle: fetching -> success -> stale -> fetching again. During fetching, subsequent observers attach to the same promise without duplicate requests."
  - name: Persistence
    id: concept:tanstack-query/persist
    description: "@tanstack/query-persist-client-core with persisters for localStorage, AsyncStorage, or custom backends. Hydrates the cache on app initialization and dehydrates on changes. Enables offline-first patterns where cached data survives page refreshes and network failures."
  - name: DevTools
    id: concept:tanstack-query/devtools
    description: Floating panel (@tanstack/react-query-devtools) showing cache state, query status, timestamps, refetch buttons, and data inspection. Essential for debugging stale data, unexpected refetches, and cache invalidation issues during development.
apis:
  - name: useQuery(options)
    id: api:tanstack-query/useQuery
    signature: "useQuery<TData, TError>(options: { queryKey, queryFn, staleTime?, gcTime?, retry?, enabled?, select?, refetchInterval?, ... }): UseQueryResult<TData, TError>"
    returns: "{ data, isLoading, isFetching, error, refetch, isStale, ... }"
    description: Primary hook for reading data. queryKey must be unique. queryFn returns the data. Returns loading/error/success states and control functions. isLoading is initial load only; isFetching is any background refetch.
  - name: useMutation(options)
    id: api:tanstack-query/useMutation
    signature: "useMutation<TData, TError, TVariables, TContext>(options: { mutationFn, onMutate?, onSuccess?, onError?, onSettled?, retry? }): UseMutationResult<TData, TError, TVariables, TContext>"
    returns: "{ mutate, mutateAsync, isPending, isError, error, data, reset }"
    description: Hook for data mutations. mutate function triggers the mutation. mutateAsync returns a promise. onMutate runs before mutation — used for optimistic updates and context capture for rollback.
  - name: useInfiniteQuery(options)
    id: api:tanstack-query/useInfiniteQuery
    signature: "useInfiniteQuery<TData, TError>(options: { queryKey, queryFn, initialPageParam, getNextPageParam, getPreviousPageParam?, maxPages? }): UseInfiniteQueryResult"
    returns: "{ data, fetchNextPage, fetchPreviousPage, hasNextPage, isFetchingNextPage, ... }"
    description: Hook for paginated/cursor-based lists. data.pages contains all loaded pages. data.pageParams contains the params for each page. fetchNextPage() appends pages. getNextPageParam returns undefined when no more pages exist.
  - name: queryClient.invalidateQueries(filter)
    id: api:tanstack-query/invalidateQueries
    signature: "queryClient.invalidateQueries(filter: { queryKey, exact?, refetchType?, predicate? }): Promise<void>"
    returns: Promise resolving after refetches.
    description: "Marks matching queries as stale and optionally refetches them. Without exact: true, matches queries whose key starts with the given prefix. refetchType controls whether to refetch (active, inactive, all)."
  - name: queryClient.setQueryData(key, data)
    id: api:tanstack-query/setQueryData
    signature: "queryClient.setQueryData<TData>(queryKey: QueryKey, updater: TData | ((old: TData | undefined) => TData)): TData"
    returns: The updated cache data.
    description: Directly sets cache data for a query key without refetching. Used for optimistic updates and cache seeding. The updater can be a value or a function receiving the old data.
  - name: queryClient.fetchQuery(options)
    id: api:tanstack-query/fetchQuery
    signature: "queryClient.fetchQuery<TData>(options: { queryKey, queryFn, ... }): Promise<TData>"
    returns: Promise resolving to the fetched data.
    description: Imperatively fetches and caches a query outside of React (event handlers, server code). Returns the data directly. Does not throw on error — always returns promise.
  - name: QueryClientProvider
    id: api:tanstack-query/provider
    signature: "<QueryClientProvider client={queryClient}><App /></QueryClientProvider>"
    returns: React context provider.
    description: Wraps the application to provide the query client to all useQuery/useMutation hooks. Must be mounted once at the root. The queryClient instance should be stable (useRef or module-level for SSR).
  - name: useSuspenseQuery(options)
    id: api:tanstack-query/useSuspenseQuery
    signature: "useSuspenseQuery<TData>(options: { queryKey, queryFn, ... }): SuspenseQueryResult<TData>"
    returns: "{ data } — never returns loading state (throws promise for Suspense)."
    description: React Suspense integration — throws a promise while loading, allowing Suspense boundaries to show fallbacks. Removes the isLoading branch from your component. Must be used inside <Suspense>.
failures:
  - id: failure:tanstack-query/stale-cache
    symptom: UI shows old data after a mutation on a different page.
    cause: Mutations do not automatically invalidate related queries. After creating a resource, the list query still shows cached old state.
    fix: "Always call queryClient.invalidateQueries({ queryKey: ['list'] }) in mutation's onSuccess. Use blanket invalidate or optimistic updates for responsive UIs."
  - id: failure:tanstack-query/infinite-scroll-reset
    symptom: useInfiniteQuery resets to page 1 when filters change.
    cause: The queryKey includes filter values — changing filters creates a new cache entry starting from scratch.
    fix: "This is expected behavior. Preserve scroll position externally. Consider keeping a global page state if you want to persist across filter changes. Use placeholderData: keepPreviousData for smooth transitions."
  - id: failure:tanstack-query/query-fn-context
    symptom: React Query v5 deprecation warnings for unused params.
    cause: queryFn receives a QueryFunctionContext with queryKey, signal, meta, pageParam. Not destructuring signal prevents request cancellation on unmount.
    fix: "Destructure { signal } from the queryFn context and pass it to fetch. On unmount, React Query aborts the signal to cancel in-flight requests."
  - id: failure:tanstack-query/ssr-hydration
    symptom: Client-side data flash or refetch immediately after SSR/SSG.
    cause: Server-prefetched data via prefetchQuery was not dehydrated and hydrated properly.
    fix: Use HydrationBoundary to wrap the app. On the server, prefetch queries into a QueryClient and dehydrate. Pass the dehydrated state to the client. For Next.js, use @tanstack/react-query/next-persist-client.
extends: []
implements: []
uses:
  - concept:react
  - concept:http-client-fetch
part_of: concept:react-ecosystem
solves:
  - problem:server-state-management
  - problem:api-caching
  - problem:loading-error-handling-patterns
alternatives:
  - package:swr
  - package:apollo-client
  - package:zustand-with-fetch
---
TanStack Query solves the fundamental problem of asynchronous server state: it is not owned by the client. Unlike client state (theme, form input, toggle), server state is asynchronous, shared, potentially stale, and subject to race conditions. TanStack Query abstracts the entire lifecycle — fetch, cache, refetch, invalidate, garbage collect — into declarative hooks.

The query key system is the architectural foundation. Keys are not just identifiers — they form a hierarchical namespace where ['todos', 'list', { status }] is a child of ['todos']. Invalidating ['todos'] refetches all todos queries, while invalidating ['todos', 'list', { status: 'done' }] refetches only the filtered list. This enables precise cache control: after creating a todo, invalidate ['todos'] once; every list view refetches automatically.

React Query v5 introduced a cleaner API with object-only signatures, removed isLoading for isPending (which triggers during initial load with no cache), and fully integrated AbortSignal for cancellation. The devtools are essential for understanding cache behavior — they show real-time query states, cache entries, refetch triggers, and stale timestamps. For production, use the React Query ESLint plugin to catch common patterns like missing query keys or unused dependencies.
