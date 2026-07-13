---
kind: Package
id: package:react-router
name: React Router Patterns
version: "7"
purpose: Document React Router patterns — routing configuration, loaders, actions, nested routes, URL parameters, and navigation for React applications.
problem_solved: Provides a declarative routing library for React that synchronizes the UI with the URL, enabling nested layouts, data loading, mutation handling, and navigation while maintaining a single source of truth for the application's URL structure.
install: npm install react-router-dom @remix-run/react
dependencies:
  - concept:react
  - concept:url-routing
  - concept:typescript
concepts:
  - name: Router Types
    id: concept:react-router/router-types
    description: "createBrowserRouter (client-side routing with full URL support), createMemoryRouter (in-memory routing for testing), createHashRouter (hash-based for static hosting), createStaticRouter (SSR). BrowserRouter is the standard for production web apps. Each creates routes and history management."
  - name: Route Configuration
    id: concept:react-router/route-config
    description: "Declarative route tree with createBrowserRouter([{ path, element, loader, action, children }]). Nested routes inherit parent layouts. path supports dynamic segments (:id), optional segments (?), splats (*), and patterns. Index routes render at the parent path."
  - name: Loaders
    id: concept:react-router/loaders
    description: "Server-side or client-side data fetching functions called before rendering a route. Loaders receive params, request, and context. Return data via useLoaderData(). Loaders can throw Response to trigger error boundaries or redirect. Run in parallel for sibling routes."
  - name: Actions
    id: concept:react-router/actions
    description: "Mutations handled at the route level via form submissions (POST, PUT, PATCH, DELETE). useFetcher().Form or <Form method='post'>. Actions receive the request and params, return data or redirect. After an action, all loaders for the current page revalidate automatically."
  - name: Navigation
    id: concept:react-router/navigation
    description: "Link (declarative), useNavigate (imperative), useFetcher (programmatic without navigation). Link accepts to (string or object), replace, and state. useNavigate returns a function: navigate('/path'), navigate(-1) (back). Fetcher navigation does not change the URL."
  - name: URL Parameters
    id: concept:react-router/url-params
    description: "useParams() returns the dynamic segments from the URL. For path /users/:id, useParams() returns { id: '123' }. useSearchParams() for query string manipulation — returns [searchParams, setSearchParams] similar to useState."
  - name: Nested Routes & Layouts
    id: concept:react-router/nested-routes
    description: "Nested routes rendered inside parent layout via <Outlet />. The parent route provides the shell (header, sidebar, footer), and child routes render in the Outlet. Layouts persist across child navigation — the header does not unmount when the child route changes."
  - name: Error Boundaries
    id: concept:react-router/error-boundaries
    description: "errorElement on routes catches render errors, loader errors, and action errors. Error messages available via useRouteError(). Nested error boundaries catch errors in their subtree. Root-level error element provides a fallback for unhandled errors."
  - name: Pending & Optimistic UI
    id: concept:react-router/pending-ui
    description: "useNavigation returns { state: 'idle' | 'loading' | 'submitting', location }. Show loading indicators during navigation. useFetcher provides state for non-navigation mutations. useFormAction for optimistic UI — showing expected values before the action completes."
  - name: Lazy Routes
    id: concept:react-router/lazy-routes
    description: "Route.lazy: () => import('./routes/dashboard') code-splits route modules. The lazy function returns the route properties (loader, action, Component, ErrorBoundary). Loaders and actions defined in the module are tree-shakeable, reducing the initial bundle."
  - name: Scroll Restoration
    id: concept:react-router/scroll
    description: "Built-in scroll restoration — React Router restores scroll position when navigating back. Customize via <ScrollRestoration /> component and getKey prop. The key maps locations to scroll positions. Disable with useScrollRestoration(false)."
apis:
  - name: createBrowserRouter(routes)
    id: api:react-router/create-browser-router
    signature: "createBrowserRouter(routes: RouteObject[], opts?: { basename, future, hydrationData, window }): Router"
    returns: A router instance to pass to RouterProvider.
    description: "Creates a browser-based router with the route tree. Each route object has path, element/Component, loader, action, errorElement/ErrorBoundary, children, and lazy. The returned router is passed to <RouterProvider router={router} />."
  - name: useLoaderData()
    id: api:react-router/use-loader-data
    signature: "useLoaderData<T>() -> T"
    returns: The data returned by the route's loader.
    description: "Returns the data from the route's loader function. The type parameter should match the loader's return type. Only accessible in the route's component and its children (via Outlet context). Type-safe when the loader and component are in the same module."
  - name: useFetcher()
    id: api:react-router/use-fetcher
    signature: "useFetcher<T>() -> { Form, load, submit, data, state, formAction, formData, formMethod }"
    returns: A fetcher object for non-navigation data operations.
    description: "Loads data or submits actions without navigating. fetcher.load('/api/data') fetches and caches. fetcher.submit(formData, { method: 'post', action }) submits to an action. state tracks 'idle' | 'loading' | 'submitting'. Ideal for add-to-cart, inline saves, and search."
  - name: <Form method='post'>
    id: api:react-router/form
    signature: "<Form method='post' action='/path' replace={false} relative='route'>"
    returns: An HTML form that navigates with the action.
    description: "Progressive enhancement form — works without JS (native form submission) and with JS (client-side navigation + action call). onSubmit is not needed; the action handles the mutation. After the action runs, all page loaders revalidate automatically."
  - name: <Outlet />
    id: api:react-router/outlet
    signature: "<Outlet context?: unknown /> -> ReactElement | null"
    returns: The matched child route element.
    description: "Renders the matched child route inside the parent's layout. Pass context via Outlet's context prop and access it in children via useOutletContext(). An Outlet without a matching child renders null (not a blank page)."
  - name: useSearchParams()
    id: api:react-router/use-search-params
    signature: "useSearchParams(defaultInit?: URLSearchParamsInit) -> [URLSearchParams, SetURLSearchParams]"
    returns: A tuple of search params and a setter.
    description: "Reads and updates the URL query string. setSearchParams({ key: 'value' }) appends to the current URL. setSearchParams(prev => { prev.set('key', 'val'); return prev; }) for functional updates. The update causes a navigation with replace: true."
sections:
  - title: Route Configuration with Data Loading
    id: section:react-router/routes
    content: |
      Nested routes with loaders and actions:

      ```typescript
      import { createBrowserRouter, RouterProvider, useLoaderData, Link, Outlet } from 'react-router-dom';

      const router = createBrowserRouter([
          {
              path: '/',
              element: <RootLayout />,
              errorElement: <ErrorPage />,
              children: [
                  { index: true, element: <Home /> },
                  {
                      path: 'dashboard',
                      loader: async () => {
                          const res = await fetch('/api/dashboard');
                          if (!res.ok) throw new Response('Not Found', { status: 404 });
                          return res.json();
                      },
                      element: <Dashboard />,
                  },
                  {
                      path: 'users/:userId',
                      loader: async ({ params }) => {
                          const user = await fetch(`/api/users/${params.userId}`);
                          if (!user.ok) throw new Response('User not found', { status: 404 });
                          return user.json();
                      },
                      element: <UserProfile />,
                      children: [
                          { index: true, element: <UserOverview /> },
                          { path: 'settings', element: <UserSettings /> },
                      ],
                  },
              ],
          },
      ]);

      function RootLayout() {
          return (
              <div>
                  <nav><Link to="/dashboard">Dashboard</Link></nav>
                  <main><Outlet /></main>
              </div>
          );
      }

      function Dashboard() {
          const data = useLoaderData<DashboardData>();
          return <div>...</div>;
      }
      ```
  - title: Form Mutations
    id: section:react-router/actions
    content: |
      Mutation handling with automatic revalidation:

      ```typescript
      import { Form, useActionData, useNavigation, redirect } from 'react-router-dom';

      // In the route definition
      {
          path: 'posts/new',
          action: async ({ request }) => {
              const formData = await request.formData();
              const title = formData.get('title');
              const body = formData.get('body');

              const res = await fetch('/api/posts', {
                  method: 'POST',
                  headers: { 'Content-Type': 'application/json' },
                  body: JSON.stringify({ title, body }),
              });

              if (!res.ok) {
                  const errors = await res.json();
                  return { errors }; // Available via useActionData
              }
              return redirect('/posts'); // Navigate to the list
          },
          element: <NewPost />,
          errorElement: <ErrorPage />,
      }

      function NewPost() {
          const actionData = useActionData() as { errors?: Record<string, string> } | undefined;
          const navigation = useNavigation();
          const isSubmitting = navigation.state === 'submitting';

          return (
              <Form method="post">
                  <input name="title" />
                  {actionData?.errors?.title && <span>{actionData.errors.title}</span>}
                  <textarea name="body" />
                  <button type="submit" disabled={isSubmitting}>
                      {isSubmitting ? 'Creating...' : 'Create Post'}
                  </button>
              </Form>
          );
      }
      ```

      After the action completes, React Router automatically re-runs all loaders on the current page — the post list updates without explicit refetching.
---
