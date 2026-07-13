---
kind: Package
id: package:arcjet
name: Arcjet Security
version: "1.0"
purpose: Document Arcjet — an application security platform for developers with rate limiting, bot detection, email validation, and PII/data redaction, integrated as middleware for Next.js, SvelteKit, Node.js, and other web frameworks.
problem_solved: Replaces scattered security implementations (Express rate-limit, reCAPTCHA, email validation libraries, data sanitization) with a single middleware-based security layer that runs at the edge, providing bot detection via fingerprinting, rate limiting by user/session/IP, email reputation checks, and sensitive data redaction.
install: npm install @arcjet/next
dependencies:
  - concept:web-security
  - concept:rate-limiting
  - concept:bot-detection
concepts:
  - name: Arcjet Client
    id: concept:arcjet/client
    description: "Create an Arcjet client with aj = arcjet({ key, rules: [...] }). The client is a middleware factory — it takes rules and returns a protect function. Rules are composable and evaluated in order. The client is typically created once and exported from a shared module."
  - name: Rules
    id: concept:arcjet/rules
    description: Security rules that define protection behavior — fixedWindow rate limit, slidingWindow rate limit, tokenBucket rate limit, bot detection (verified bots, automated, allow/deny), email validation (MX record, disposable email, typo suggestions), sensitive info (PII, credit cards, API keys) detection and redaction.
  - name: Rate Limiting
    id: concept:arcjet/rate-limit
    description: Configurable rate limiting by IP, session, user ID, or custom key. Supports fixed window (N requests per window), sliding window (smooth rate calculation), and token bucket (burst allowance + sustained rate). Algorithm configurable per route.
  - name: Bot Detection
    id: concept:arcjet/bot-detection
    description: Classifies HTTP requests as human, browser automation (Puppeteer, Playwright), or script/framework (axios, curl, Python requests). Uses fingerprinting, User-Agent parsing, and behavioral analysis. Configure allow/deny lists for specific bot categories.
  - name: Email Validation
    id: concept:arcjet/email-validation
    description: Validates email addresses at registration — checks MX record existence, flags disposable/temporary email providers, suggests typo corrections (gmial.com -> gmail.com), and checks against known burners. Accept or reject with configurable severity.
  - name: Sensitive Data Detection
    id: concept:arcjet/sensitive-info
    description: Scans request bodies for PII (email, phone, SSN), financial data (credit cards, bank routing), and secrets (API keys, tokens, passwords). Can redact, block, or log. Works on form submissions, API JSON bodies, and query parameters.
  - name: Shield
    id: concept:arcjet/shield
    description: WAF-like protection against common web attacks — SQL injection, XSS, path traversal, and parameter pollution. Applied as a rule at the edge before the request reaches your application. No configuration needed — Shield runs with sensible defaults.
  - name: Edge Runtime
    id: concept:arcjet/edge
    description: Arcjet runs at the edge — deployed alongside your framework's middleware (Next.js Middleware, SvelteKit hooks, Cloudflare Workers). Rate limiting state and bot detection run close to the user, minimizing latency. WASM-based for sub-millisecond overhead.
  - name: Decision Engine
    id: concept:arcjet/decision
    description: Arcjet's response is a Decision object — allowed, denied, challenge (CAPTCHA), or rate limited. Each decision includes reason, remaining, reset, and IP. Decisions are evaluated in rule order, and the most severe action wins.
apis:
  - name: arcjet(config)
    id: api:arcjet/create
    signature: "const aj = arcjet({ key: process.env.ARCJET_KEY, rules: [rateLimit({ max: 100, window: '1m' }), protectSignup] })"
    returns: An Arcjet client.
    description: Creates the Arcjet client. key is required from Arcjet dashboard. rules is an array of protection rules. Characteristics like fingerprinting are enabled by default. The client is a singleton — create once and export.
  - name: aj.protect(request, opts)
    id: api:arcjet/protect
    signature: "const decision = await aj.protect(request, { ip: req.headers.get('x-forwarded-for'), sessionId: getSessionId(req) })"
    returns: Decision object with isDenied, reason, remaining, reset, ip.
    description: Evaluates all rules against the request. Returns a Decision. Call ap.protect() in middleware or route handlers before processing the request. If decision.isDenied, return an error response. Otherwise, proceed.
  - name: rateLimit(opts)
    id: api:arcjet/rate-limit
    signature: "rateLimit({ max: 10, window: '10s', algorithm: 'slidingWindow', match: '/api/auth/login' })"
    returns: A rate limiting rule.
    description: "Rate limits requests by IP (default) or custom key. window: '10s', '1m', '1h'. algorithm: 'fixedWindow' (resets at window boundary), 'slidingWindow' (smooth), 'tokenBucket' (burst + sustained). match restricts the rule to a route pattern."
  - name: detectBot(opts)
    id: api:arcjet/bot
    signature: "detectBot({ mode: 'LIVE', block: ['AUTOMATED', 'BROWSER'] })"
    returns: A bot detection rule.
    description: "Classifies the client as HUMAN, AUTOMATED (bot library), BROWSER (Playwright/Puppeteer), or LIKELY_AUTOMATED. mode: 'LIVE' blocks, 'DRY_RUN' logs. block specifies which categories to deny. allow for bypass list."
  - name: validateEmail(opts)
    id: api:arcjet/email
    signature: "validateEmail({ block: ['DISPOSABLE', 'INVALID_MX'], onTypo: 'SUGGEST' })"
    returns: An email validation rule.
    description: "Validates email addresses extracted from request body. block flags which categories to deny. onTypo: 'SUGGEST' returns correction suggestion. Must be combined with sensitiveInfo or used with manual email field extraction."
  - name: sensitiveInfo(opts)
    id: api:arcjet/sensitive-info-api
    signature: "sensitiveInfo({ deny: ['EMAIL', 'PHONE', 'CREDIT_CARD'], mode: 'LIVE' })"
    returns: A sensitive data detection rule.
    description: "Scans request for PII and secrets. deny specifies which categories to block. mode: 'LIVE' (block), 'DRY_RUN' (log only). Redacts matched data by default. Use with validateEmail for combined email validation + PII detection."
failures:
  - id: failure:arcjet/key-not-configured
    symptom: All requests are blocked or Arcjet throws initialization errors.
    cause: ARCJET_KEY environment variable is missing or invalid. Arcjet requires a valid API key from the dashboard.
    fix: Add ARCJET_KEY to .env.local. Verify the key is correct in the Arcjet dashboard. Restart the development server after adding the key. Use a different key for production vs development.
  - id: failure:arcjet/rate-limit-too-aggressive
    symptom: Legitimate users are rate-limited during normal usage patterns.
    cause: Rate limit window too short, max too low, or not accounting for API calls made by the frontend.
    fix: Increase max or window duration. Use a more permissive rate limit for authenticated users. Use slidingWindow algorithm for smoother rate enforcement. Test with realistic traffic patterns.
  - id: failure:arcjet/bot-false-positive
    symptom: Real users are detected as bots and blocked or challenged.
    cause: Aggressive bot detection settings or fingerprinting collisions from shared IPs (corporate networks, VPNs).
    fix: Add known user IPs to allow list. Reduce bot detection mode to DRY_RUN initially to observe classifications. Use session-based identification alongside IP for more accurate detection. Check the Arcjet dashboard for classification details.
  - id: failure:arcjet/middleware-order
    symptom: Arcjet protection does not apply to certain routes or runs after sensitive logic.
    cause: Arcjet middleware registered after route handlers or too late in the middleware chain — requests are already processed before protection runs.
    fix: Register Arcjet middleware as early as possible in the middleware chain. In Next.js, use middleware.ts at the root. In SvelteKit, add check in handle hook before processing the request. Always call ap.protect() before route logic runs.
extends: []
implements: []
uses:
  - concept:web-security
  - concept:rate-limiting
  - concept:bot-detection
part_of: concept:web-security-stack
solves:
  - problem:api-abuse-prevention
  - problem:automated-attack-mitigation
  - problem:user-input-validation-security
alternatives:
  - package:express-rate-limit
  - package:cloudflare-waf
  - package:turnstile
  - package:re2
---
Arcjet reimagines application security as a developer-first middleware layer rather than a separate WAF or runtime agent. Instead of configuring nginx rate limiting, integrating reCAPTCHA, and writing validation logic separately, Arcjet provides a unified API where security rules compose as middleware functions.

The decision engine evaluates all rules in order and returns a single Decision. Rate limiting runs first (fastest, no external dependencies), then bot detection (User-Agent + fingerprinting), then email validation and sensitive info scanning (require body parsing). The most severe action (deny > challenge > rate_limit > allow) wins — for example, a rate-limited request from an automated bot returns the deny action with the rate limit reason.

Arcjet runs at the edge via WASM and can be deployed in Edge Middleware (Next.js, Cloudflare Workers) or Node.js. The edge deployment is critical for performance — rate limiting state and bot detection decisions happen at the CDN edge, not on your origin server. This means blocked requests never reach your application, saving compute and bandwidth.
