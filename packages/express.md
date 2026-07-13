---
kind: Package
id: package:express
name: Express.js Patterns
version: "5"
purpose: Document Express.js web framework patterns — middleware, routing, error handling, validation, and production best practices for building Node.js HTTP APIs.
problem_solved: Provides a minimal, unopinionated web framework for Node.js that handles HTTP request routing, middleware composition, and response sending, letting developers focus on application logic rather than raw HTTP server details.
install: npm install express @types/express
dependencies:
  - concept:node-js
  - concept:http-protocol
  - concept:javascript
concepts:
  - name: Middleware
    id: concept:express/middleware
    description: "Functions that access the request, response, and next callback in the request-response cycle. Middleware can execute code, modify req/res, end the request, or call next() to pass control. Express 5 supports async middleware with automatic error forwarding (rejected promises go to error handler)."
  - name: Routing
    id: concept:express/routing
    description: "app.get('/users/:id', handler) maps HTTP methods and URL patterns to handler functions. Route parameters (:id) are available at req.params. Express 5 supports path patterns with wildcards, regex, and array of handlers. Router instances modularize routes across files."
  - name: Error Handling
    id: concept:express/error-handling
    description: "Middleware with four arguments (err, req, res, next) catches errors passed via next(err). Express 5 automatically catches async errors. Define a global error handler as the last middleware for consistent error responses. Production error handlers omit stack traces."
  - name: Request Object
    id: concept:express/request
    description: "req object extends Node's IncomingMessage with params, query, body, cookies, path, ip, protocol, and xhr. req.get() for headers. Express 5 adds req.param(key) for combined params/query/body lookup. req.body requires body-parser or express.json() middleware."
  - name: Response Object
    id: concept:express/response
    description: "res extends ServerResponse with status(), json(), send(), sendFile(), redirect(), render(), and attachment(). Express 5 adds res.sendStatus() shorthand. Chain methods: res.status(201).json({id: 1}). set() and get() for headers."
  - name: Body Parsing
    id: concept:express/body-parsing
    description: "express.json() parses JSON request bodies. express.urlencoded({extended: true}) parses form data. express.raw() and express.text() for other formats. These are middleware functions that add parsed bodies to req.body. Without them, req.body is undefined."
  - name: Static Files
    id: concept:express/static
    description: "express.static('public') serves files from a directory. Supports caching (maxAge), dotfiles control, and index files. In production, use a reverse proxy (Nginx, CDN) for static assets instead of Express. express.static runs as middleware."
  - name: Router
    id: concept:express/router
    description: "express.Router() creates modular, mountable route handlers. Routers have their own middleware, params, and routes. Mount with app.use('/api', apiRouter). Allows organizing routes by domain (users, products, orders) across separate files."
  - name: CORS
    id: concept:express/cors
    description: "Cross-Origin Resource Sharing handled by the cors package. app.use(cors({origin: 'https://example.com', credentials: true})) enables secure cross-origin requests. Preflight OPTIONS requests are handled automatically. Use per-route or per-origin configuration."
  - name: Compression
    id: concept:express/compression
    description: "compression middleware gzip/brotli-compresses responses. app.use(compression({level: 6, threshold: 1024})) reduces bandwidth by 70-90%. Apply before routes so all responses are compressed. Disable for streaming responses."
  - name: Helmet
    id: concept:express/helmet
    description: "A collection of security middleware setting HTTP headers: Content-Security-Policy, X-Frame-Options, X-XSS-Protection, Strict-Transport-Security, X-Content-Type-Options. app.use(helmet()) at the top of middleware stack. Configure specific headers via helmet({contentSecurityPolicy: {...}})."
apis:
  - name: app.use()
    id: api:express/use
    signature: "app.use([path: string, ...] middleware: (req, res, next) => void): App"
    returns: The Express app (chainable).
    description: "Mounts middleware at the specified path (defaults to '/'). Middleware runs for every matching request. Called in the order registered. path can be a string, pattern, or array of paths."
  - name: app.get() / app.post() / app.put() / app.delete()
    id: api:express/route-methods
    signature: "app.get(path: string, ...handlers: RequestHandler[]): App"
    returns: The Express app (chainable).
    description: "Registers a route handler for the given HTTP method and path. Multiple handlers are treated as sub-stack middleware. Route params captured via :paramName path segments. Express 5 supports wildcard patterns and RegExp paths."
  - name: Router()
    id: api:express/router
    signature: "express.Router(options?: { caseSensitive, mergeParams, strict }): Router"
    returns: A new Router instance.
    description: "Creates an isolated router with its own middleware stack. Router.get(), .post(), .use() work like the app. Export routers and mount in the app. mergeParams: true makes parent route params available in nested routers."
  - name: res.json()
    id: api:express/res-json
    signature: "res.json(body: any): Response"
    returns: The response (chainable).
    description: "Sends a JSON response with Content-Type: application/json. Serializes the body object. Sets Content-Length automatically. JSON serialization handles dates (toISOString) and circular references gracefully."
  - name: next(err)
    id: api:express/next-error
    signature: "next(err?: any): void"
    returns: void
    description: "Passes control to the next middleware. If err is provided, jumps to the error-handling middleware (defined with 4 parameters). Calling next('route') skips to the next route handler. Calling without arguments passes to the next middleware."
  - name: Error handler middleware
    id: api:express/error-handler
    signature: "app.use((err: Error, req: Request, res: Response, next: NextFunction) => void): void"
    returns: void
    description: "Four-argument middleware that catches errors. Must have exactly 4 parameters. Production should return safe errors (no stack traces). Always call next(err) upstream to reach this. Async handlers in Express 5 forward rejected promises automatically."
sections:
  - title: REST API Structure
    id: section:express/rest-api
    content: |
      Organize a REST API with routers, validation, and error handling:

      ```typescript
      import express, { Request, Response, NextFunction } from 'express';
      import { userRouter } from './routes/users';
      import { productRouter } from './routes/products';

      const app = express();

      // Global middleware
      app.use(express.json());
      app.use(helmet());
      app.use(cors({ origin: process.env.ORIGIN }));
      app.use(morgan('combined'));

      // Routes
      app.use('/api/v1/users', userRouter);
      app.use('/api/v1/products', productRouter);

      // 404 handler
      app.use((req, res) => {
          res.status(404).json({ error: 'Not found' });
      });

      // Global error handler
      app.use((err: Error, req: Request, res: Response, next: NextFunction) => {
          const status = (err as any).statusCode || 500;
          res.status(status).json({
              error: process.env.NODE_ENV === 'production'
                  ? 'Internal server error'
                  : err.message,
              ...(process.env.NODE_ENV !== 'production' && { stack: err.stack }),
          });
      });

      export default app;
      ```
  - title: Router with Validation
    id: section:express/router-validation
    content: |
      Domain-specific router with async handlers and Zod validation:

      ```typescript
      import { Router } from 'express';
      import { z } from 'zod';

      export const userRouter = Router();

      const createUserSchema = z.object({
          name: z.string().min(2).max(100),
          email: z.string().email(),
          age: z.number().int().positive().optional(),
      });

      userRouter.post('/', async (req, res, next) => {
          try {
              const data = createUserSchema.parse(req.body);
              const user = await createUser(data);
              res.status(201).json(user);
          } catch (err) {
              if (err instanceof z.ZodError) {
                  return res.status(400).json({ errors: err.errors });
              }
              next(err);
          }
      });

      userRouter.get('/:id', async (req, res, next) => {
          try {
              const user = await getUserById(req.params.id);
              if (!user) return res.status(404).json({ error: 'User not found' });
              res.json(user);
          } catch (err) {
              next(err);
          }
      });
      ```

      In Express 5, async middleware is supported natively — wrap route handlers in a `catch(next)` wrapper or use `express-async-errors` to eliminate try/catch around every handler.
---
