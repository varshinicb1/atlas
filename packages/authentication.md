---
kind: Package
id: package:authentication
name: Authentication Patterns
version: "2"
purpose: Document authentication patterns — JWT, OAuth 2.0, session-based auth, password hashing, MFA, RBAC, and security best practices for web and API authentication.
problem_solved: Provides a reference for implementing secure user authentication across web and API applications, covering credential storage, token management, session handling, and authorization strategies that prevent common vulnerabilities like credential theft, CSRF, and session hijacking.
install: No single install — use auth libraries for your stack.
dependencies:
  - concept:security
  - concept:http-protocol
  - concept:cryptography
concepts:
  - name: Password Hashing
    id: concept:auth/password-hashing
    description: "Never store passwords in plain text. Use bcrypt (cost factor 12+), Argon2id (memory-hard, time-cost, parallelism), or scrypt. Hash on the server, compare hashes never the original. Use a unique salt per password (built into bcrypt/Argon2). Old MD5/SHA-1/SHA-256 without salting are insufficient."
  - name: Session-Based Auth
    id: concept:auth/sessions
    description: "Server stores session data (user ID, expiry, metadata) and gives the client a session ID (cookie). The browser sends the cookie on every request. Sessions can be revoked server-side. Storage: Redis (fast, TTL-based expiry), database (auditable, persistent). Session IDs must be cryptographically random."
  - name: JWT (JSON Web Tokens)
    id: concept:auth/jwt
    description: "Self-contained tokens with base64url-encoded header, payload, and signature. The server signs the token (RS256, HS256); the client stores it (localStorage, httpOnly cookie). No server-side session store needed. Tokens contain claims (sub, exp, iat, iss). Stateless but cannot be revoked without a blocklist."
  - name: OAuth 2.0
    id: concept:auth/oauth2
    description: "Authorization framework with four parties: resource owner (user), client (app), authorization server, resource server. Authorization Code flow (with PKCE) is the gold standard. The client gets an auth code, exchanges it for tokens (access + refresh), and uses the access token to call APIs. Scopes limit permissions."
  - name: OIDC (OpenID Connect)
    id: concept:auth/oidc
    description: "Identity layer on top of OAuth 2.0. Adds an ID token (JWT) containing user identity (sub, name, email). The /userinfo endpoint returns additional claims. OIDC discovery at /.well-known/openid-configuration. Used for SSO — sign in with Google, GitHub, Microsoft, or any OIDC provider."
  - name: Refresh Tokens
    id: concept:auth/refresh-tokens
    description: "Long-lived tokens (days/weeks) that obtain new access tokens without re-authentication. Access tokens are short-lived (15-60 min). Refresh tokens should be stored securely (httpOnly cookie) and rotated (old refresh token invalidated when a new one is issued). Detection of refresh token reuse signals theft."
  - name: MFA (Multi-Factor Authentication)
    id: concept:auth/mfa
    description: "Additional verification beyond passwords: TOTP (authenticator apps, RFC 6238), SMS codes (less secure due to SIM swapping), hardware keys (WebAuthn/FIDO2, most secure), backup codes. TOTP uses a shared secret and 30-second rotating codes. WebAuthn uses public-key cryptography — no shared secrets."
  - name: RBAC (Role-Based Access Control)
    id: concept:auth/rbac
    description: "Users have roles; roles have permissions. Check permissions on every request: if (!user.hasRole('admin')) return 403. Roles: admin, editor, viewer. Permissions: posts.create, posts.edit, posts.delete. RBAC is simple but can become rigid. ABAC (Attribute-Based) provides finer-grained control using user/resource/environment attributes."
  - name: CSRF Protection
    id: concept:auth/csrf
    description: "Cross-Site Request Forgery — an attacker tricks the browser into making authenticated requests. Protection: SameSite=Strict/Lax cookies (prevents cross-site cookie sending), CSRF tokens (hidden form field validated server-side), or Origin/Referer header checks. APIs with CORS + token auth are not vulnerable."
  - name: Rate Limiting
    id: concept:auth/rate-limiting
    description: "Prevent brute-force attacks on auth endpoints. Per-IP and per-account rate limiting. Implement exponential backoff after failed attempts. Lock accounts temporarily after N failures (with unlock via email). Login endpoints: 5 attempts/min per account. Registration: 3/min per IP."
  - name: Secure Cookie Configuration
    id: concept:auth/secure-cookies
    description: "HttpOnly (inaccessible to JS), Secure (HTTPS only), SameSite=Strict (same-site requests only), Path=/ (scoped to the app). Prefix with __Host- for additional protection. Max-Age or Expires for session duration. Domain attribute should not be too permissive."
apis:
  - name: bcrypt.hash()
    id: api:auth/bcrypt-hash
    signature: "bcrypt.hash(password: string, saltRounds: number) -> Promise<string>"
    returns: A hashed password string ($2b$10$...).
    description: "Hashes a password using bcrypt with the specified number of salt rounds (cost factor). 10-12 rounds is standard (250ms per hash). Returns a string containing the algorithm version, cost factor, salt, and hash. bcrypt.compare() verifies against the hash."
  - name: jsonwebtoken.sign()
    id: api:auth/jwt-sign
    signature: "jwt.sign(payload: object, secret: string, options?: { algorithm, expiresIn, issuer, subject }) -> string"
    returns: A JWT string (header.payload.signature).
    description: "Creates a signed JWT. payload contains claims (sub=userId, role). secret is the HMAC key (HS256) or private key (RS256). expiresIn: '15m', '7d'. Standard claims: iss (issuer), sub (subject), aud (audience), exp (expiry), iat (issued at)."
  - name: jwt.verify()
    id: api:auth/jwt-verify
    signature: "jwt.verify(token: string, secret: string, options?: { algorithms, issuer, audience }) -> object | string"
    returns: The decoded payload if valid; throws if invalid.
    description: "Verifies the JWT signature and checks expiration (exp) and other options (issuer, audience). Returns the decoded payload. Throws TokenExpiredError on expiry, JsonWebTokenError on invalid signature. Verify on every request to protected routes."
  - name: oauth2 authorization code flow
    id: api:auth/oauth2-code-flow
    signature: "GET /authorize?response_type=code&client_id=ID&redirect_uri=URI&scope=SCOPE&state=STATE -> POST /token?grant_type=authorization_code&code=AUTH_CODE&redirect_uri=URI&code_verifier=PKCE -> { access_token, refresh_token, id_token, expires_in }"
    returns: Tokens from the authorization server.
    description: "The standard OAuth 2.0 flow. Authorization code is single-use and exchanged for tokens server-side. PKCE (code_challenge + code_verifier) protects against authorization code interception. state parameter prevents CSRF on the auth callback."
sections:
  - title: JWT Authentication Middleware
    id: section:auth/jwt-middleware
    content: |
      Express middleware for JWT verification and role-based access:

      ```typescript
      import jwt from 'jsonwebtoken';
      import { Request, Response, NextFunction } from 'express';

      interface AuthRequest extends Request {
          user?: { userId: string; role: string };
      }

      function authenticate(req: AuthRequest, res: Response, next: NextFunction) {
          const authHeader = req.headers.authorization;
          if (!authHeader?.startsWith('Bearer ')) {
              return res.status(401).json({ error: 'Missing token' });
          }
          try {
              const token = authHeader.split(' ')[1];
              const payload = jwt.verify(token, process.env.JWT_SECRET!) as { userId: string; role: string };
              req.user = payload;
              next();
          } catch (err) {
              if (err instanceof jwt.TokenExpiredError) {
                  return res.status(401).json({ error: 'Token expired', code: 'TOKEN_EXPIRED' });
              }
              return res.status(403).json({ error: 'Invalid token' });
          }
      }

      function authorize(...roles: string[]) {
          return (req: AuthRequest, res: Response, next: NextFunction) => {
              if (!req.user || !roles.includes(req.user.role)) {
                  return res.status(403).json({ error: 'Insufficient permissions' });
              }
              next();
          };
      }

      // Usage
      app.get('/api/admin/users', authenticate, authorize('admin'), handler);
      ```
  - title: OAuth 2.0 with PKCE
    id: section:auth/oauth-pkce
    content: |
      Browser-based OAuth flow with PKCE for public clients:

      ```typescript
      // Generate PKCE challenge pair
      function generatePKCE() {
          const verifier = crypto.randomBytes(32).toString('base64url');
          const challenge = crypto.createHash('sha256').update(verifier).digest('base64url');
          return { verifier, challenge };
      }

      // Step 1: Redirect to authorization server
      const { verifier, challenge } = generatePKCE();
      sessionStorage.setItem('code_verifier', verifier);

      const authUrl = new URL('https://accounts.google.com/o/oauth2/v2/auth');
      authUrl.searchParams.set('client_id', GOOGLE_CLIENT_ID);
      authUrl.searchParams.set('redirect_uri', `${window.location.origin}/auth/callback`);
      authUrl.searchParams.set('response_type', 'code');
      authUrl.searchParams.set('scope', 'openid email profile');
      authUrl.searchParams.set('code_challenge', challenge);
      authUrl.searchParams.set('code_challenge_method', 'S256');
      authUrl.searchParams.set('state', crypto.randomUUID());
      window.location.href = authUrl.toString();

      // Step 2: Exchange code for tokens (server-side)
      const verifier = sessionStorage.getItem('code_verifier');
      const response = await fetch('https://oauth2.googleapis.com/token', {
          method: 'POST',
          headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
          body: new URLSearchParams({
              code: receivedCode,
              client_id: GOOGLE_CLIENT_ID,
              redirect_uri: `${window.location.origin}/auth/callback`,
              grant_type: 'authorization_code',
              code_verifier: verifier,
          }),
      });
      const { access_token, refresh_token, id_token } = await response.json();
      ```
---
