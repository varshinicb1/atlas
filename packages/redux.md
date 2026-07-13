---
kind: Package
id: package:redux
name: Redux State Management
version: "9.2"
purpose: Document Redux patterns — store, reducers, actions, slices, middleware, RTK Query, and TypeScript integration for predictable state management.
problem_solved: Provides a predictable state container with a unidirectional data flow, time-travel debugging, and middleware ecosystem that makes application state changes traceable, testable, and maintainable as applications grow in complexity.
install: npm install @reduxjs/toolkit react-redux
dependencies:
  - concept:javascript
  - concept:state-management
  - concept:functional-programming
concepts:
  - name: Store
    id: concept:redux/store
    description: "The single source of truth holding the entire application state tree. Created with configureStore({ reducer }). Dispatch actions to update state. subscribe to listen for changes. getState() reads the current state. Redux Toolkit's configureStore automatically sets up middleware and DevTools."
  - name: Actions
    id: concept:redux/actions
    description: "Plain objects with a type field describing what happened: { type: 'todos/todoAdded', payload: 'Buy milk' }. With Redux Toolkit, use createAction('todos/todoAdded') which generates an action creator and a matching type. Actions describe events, not setters."
  - name: Reducers
    id: concept:redux/reducers
    description: "Pure functions (state, action) => newState that specify how the state changes in response to actions. Must not mutate state directly — use immutable update logic. createReducer simplifies this with a builder syntax and Immer-powered mutable syntax in RTK."
  - name: Slices
    id: concept:redux/slices
    description: "createSlice({ name, initialState, reducers }) auto-generates action creators and reducer from a single config. The slice's reducers can use Immer's mutable syntax. Exports: slice.reducer, slice.actions. Slices are the recommended way to write Redux logic with RTK."
  - name: Middleware
    id: concept:redux/middleware
    description: "Functions that intercept dispatched actions before they reach the reducer. Used for logging, crash reporting, async handling (Redux Thunk), and side effects. configureStore adds default middleware (thunk, immutable state check, serializable state check). Custom middleware: store => next => action => {}."
  - name: Redux Thunk
    id: concept:redux/thunk
    description: "Async middleware that lets action creators return a function instead of an action. The function receives dispatch and getState. Used for API calls, delayed dispatch, and conditional dispatching. RTK's createAsyncThunk generates pending/fulfilled/rejected action types automatically."
  - name: RTK Query
    id: concept:redux/rtk-query
    description: "Data fetching and caching layer built into RTK. Define API endpoints with createApi, get automatic caching, polling, optimistic updates, pagination, and cache invalidation. Generates React hooks (useGetPostsQuery, useCreatePostMutation). Eliminates manual thunk and reducer code for API state."
  - name: Selectors
    id: concept:redux/selectors
    description: "Functions that extract and compute derived data from the store. useSelector hook subscribes to state changes. createSelector creates memoized selectors that avoid recomputation if inputs haven't changed. Selectors compose: one selector can feed into another."
  - name: Normalized State
    id: concept:redux/normalized-state
    description: "Storing data in a flat, normalized structure (by ID) instead of nested trees. createEntityAdapter provides normalized CRUD operations: upsertMany, removeOne, setAll, and selectors (selectIds, selectEntities, selectById). Avoids duplication and simplifies updates."
  - name: DevTools
    id: concept:redux/devtools
    description: "Redux DevTools browser extension that provides time-travel debugging, action history, state diff, and action dispatching. configureStore enables DevTools by default. Advanced: trace, maxAge (history limit), and action sanitization. Essential for debugging complex state flows."
  - name: Immutable Updates
    id: concept:redux/immutable
    description: "State must not be mutated. RTK uses Immer internally to let you write 'mutating' syntax (state.value = 5) that produces immutable updates under the hood. Without Immer, use spread operators, Object.assign, or array methods without mutating originals."
  - name: Middleware Pipeline
    id: concept:redux/middleware-pipeline
    description: "Middleware form a pipeline: action → middleware1 → middleware2 → reducer. Each middleware can log, modify, delay, or stop the action. The middleware signature: (store) => (next) => (action) => result. next passes the action to the next middleware in the chain."
apis:
  - name: configureStore()
    id: api:redux/configure-store
    signature: "configureStore({ reducer: RootReducer | { [key: string]: Reducer }, middleware?: (getDefaultMiddleware) => Middleware[], devTools?: boolean }): EnhancedStore"
    returns: A Redux store with middleware and DevTools configured.
    description: "Creates the Redux store with sensible defaults: thunk middleware, immutable state check, serializable check, and DevTools. Accepts a single reducer or a slice-of-state map that creates a combined reducer automatically."
  - name: createSlice()
    id: api:redux/create-slice
    signature: "createSlice({ name: string, initialState: State, reducers: { [key]: (state, action) => void }, extraReducers?: (builder) => void }): Slice"
    returns: A slice object with reducer and actions.
    description: "Generates a reducer function and matching action creators. Reducers use Immer for mutable syntax. extraReducers handles actions from other slices or async thunks. The name field prefixes action types (name/reducerName)."
  - name: createAsyncThunk()
    id: api:redux/create-async-thunk
    signature: "createAsyncThunk<ReturnType, ArgType>(typePrefix: string, payloadCreator: (arg, thunkAPI) => Promise<ReturnType>, options?: {}): AsyncThunk"
    returns: An async thunk action creator.
    description: "Generates three action types: pending/fulfilled/rejected. The payload creator receives the argument and thunkAPI ({dispatch, getState, rejectWithValue}). Handle in extraReducers with builder.addCase.fulfilled, .pending, .rejected."
  - name: createEntityAdapter()
    id: api:redux/create-entity-adapter
    signature: "createEntityAdapter<T>(options?: { selectId?, sortComparer? }): EntityAdapter<T>"
    returns: An entity adapter with CRUD operations and selectors.
    description: "Generates normalized CRUD reducers and memoized selectors. Manages state as { ids: string[], entities: Record<string, T> }. Methods: addOne, addMany, setOne, setAll, removeOne, removeMany, updateOne, upsertOne."
  - name: createApi()
    id: api:redux/create-api
    signature: "createApi({ reducerPath, baseQuery, tagTypes, endpoints: (builder) => ({...}) }): Api"
    returns: An API object with generated React hooks.
    description: "RTK Query's core — defines API endpoints with builder.query and builder.mutation. Automatically generates useQuery, useMutation hooks. Handles caching, refetching, polling, optimistic updates, and cache invalidation via tags."
  - name: useSelector()
    id: api:redux/use-selector
    signature: "const result = useSelector(selector: (state) => T, equalityFn?: (a, b) => boolean): T"
    returns: The selected state value.
    description: "React hook that subscribes to the Redux store. Runs the selector on every state change. Re-renders when the selector returns a different value. Default equality is ===; use shallowEqual for objects or custom equality functions."
sections:
  - title: Typical Store Setup
    id: section:redux/store-setup
    content: |
      Redux Toolkit with slices and async thunks:

      ```typescript
      import { createSlice, createAsyncThunk, configureStore } from '@reduxjs/toolkit';
      import { useDispatch, useSelector, TypedUseSelectorHook } from 'react-redux';

      interface Todo {
          id: number;
          title: string;
          completed: boolean;
      }

      // Async thunk
      export const fetchTodos = createAsyncThunk('todos/fetchTodos',
          async (userId: number) => {
              const response = await fetch(`/api/todos?userId=${userId}`);
              return response.json() as Promise<Todo[]>;
          }
      );

      // Slice
      const todosSlice = createSlice({
          name: 'todos',
          initialState: { items: [] as Todo[], status: 'idle' as 'idle' | 'loading' | 'succeeded' | 'failed' },
          reducers: {
              toggleTodo: (state, action) => {
                  const todo = state.items.find(t => t.id === action.payload);
                  if (todo) todo.completed = !todo.completed;
              },
          },
          extraReducers: builder => {
              builder
                  .addCase(fetchTodos.pending, state => { state.status = 'loading'; })
                  .addCase(fetchTodos.fulfilled, (state, action) => {
                      state.status = 'succeeded';
                      state.items = action.payload;
                  })
                  .addCase(fetchTodos.rejected, state => { state.status = 'failed'; });
          },
      });

      export const { toggleTodo } = todosSlice.actions;

      const store = configureStore({ reducer: { todos: todosSlice.reducer } });
      export type RootState = ReturnType<typeof store.getState>;
      export type AppDispatch = typeof store.dispatch;
      export const useAppDispatch = () => useDispatch<AppDispatch>();
      export const useAppSelector: TypedUseSelectorHook<RootState> = useSelector;
      ```
  - title: RTK Query API
    id: section:redux/rtk-query
    content: |
      Declare API endpoints and get auto-generated hooks:

      ```typescript
      import { createApi, fetchBaseQuery } from '@reduxjs/toolkit/query/react';

      export const api = createApi({
          reducerPath: 'api',
          baseQuery: fetchBaseQuery({ baseUrl: '/api' }),
          tagTypes: ['Posts', 'Users'],
          endpoints: builder => ({
              getPosts: builder.query<Post[], void>({
                  query: () => 'posts',
                  providesTags: ['Posts'],
              }),
              createPost: builder.mutation<Post, Partial<Post>>({
                  query: body => ({ url: 'posts', method: 'POST', body }),
                  invalidatesTags: ['Posts'],
              }),
              updatePost: builder.mutation<Post, { id: number } & Partial<Post>>({
                  query: ({ id, ...body }) => ({ url: `posts/${id}`, method: 'PATCH', body }),
                  invalidatesTags: (result, error, { id }) => [{ type: 'Posts', id }],
                  async onQueryStarted({ id, ...body }, { dispatch, queryFulfilled }) {
                      // Optimistic update
                      const patchResult = dispatch(
                          api.util.updateQueryData('getPosts', undefined, draft => {
                              const post = draft.find(p => p.id === id);
                              if (post) Object.assign(post, body);
                          })
                      );
                      try { await queryFulfilled; }
                      catch { patchResult.undo(); }
                  },
              }),
          }),
      });

      export const { useGetPostsQuery, useCreatePostMutation, useUpdatePostMutation } = api;
      ```
---
