---
kind: Package
id: package:redis
name: Redis Patterns
version: "7.4"
purpose: Document Redis patterns — caching, data structures, pub/sub, rate limiting, distributed locks, and session management for high-performance applications.
problem_solved: Provides an in-memory data structure store with sub-millisecond latency, built-in replication, persistence, and atomic operations that solves caching, real-time messaging, rate limiting, and distributed coordination without running a separate database.
install: npm install redis ioredis @redis/client
dependencies:
  - concept:caching
  - concept:data-structures
  - concept:distributed-systems
concepts:
  - name: Strings
    id: concept:redis/strings
    description: "The most basic Redis data type — values up to 512MB. Commands: SET, GET, MGET, MSET, INCR, DECR, INCRBY, APPEND, STRLEN. Used for caching, counters, session tokens, and simple key-value storage. Atomic operations like INCR are safe for concurrent counters."
  - name: Lists
    id: concept:redis/lists
    description: "Ordered collections of string elements (linked lists). Commands: LPUSH, RPUSH, LPOP, RPOP, LLEN, LRANGE, LTRIM. Used for message queues, activity feeds, and recent items lists. LPUSH + LTRIM creates a capped collection. Lists are not suitable for random access by index (O(N))."
  - name: Sets
    id: concept:redis/sets
    description: "Unordered collections of unique strings. Commands: SADD, SREM, SMEMBERS, SISMEMBER, SUNION, SINTER, SDIFF. Used for tags, unique visitors, likes, and set operations (intersection for mutual friends, union for recommendations). O(1) membership checks."
  - name: Sorted Sets
    id: concept:redis/sorted-sets
    description: "Unique elements with floating-point scores, ordered by score. Commands: ZADD, ZRANGE, ZREVRANGE, ZRANK, ZSCORE, ZINCRBY, ZREMRANGEBYSCORE. Used for leaderboards, rate limiters, time-series (score = timestamp), and priority queues. Elements are unique; updating a score moves the element."
  - name: Hashes
    id: concept:redis/hashes
    description: "Maps of string fields to string values — like a JSON object. Commands: HSET, HGET, HMGET, HGETALL, HINCRBY, HDEL, HLEN. Used for representing objects (user profiles, product details), avoiding multiple keys. More memory-efficient than storing many string keys for the same object."
  - name: Pub/Sub
    id: concept:redis/pubsub
    description: "Publish-subscribe messaging — PUBLISH channel message sends to all SUBSCRIBE clients. SUBSCRIBE channel, PSUBSCRIBE pattern (glob). Fire-and-forget: messages are not persisted; clients that are offline miss them. Used for real-time notifications, chat, and intra-service events."
  - name: Expiration & TTL
    id: concept:redis/ttl
    description: "Set a time-to-live on any key via EXPIRE key seconds or SET key value EX seconds. TTL (remaining seconds), PTTL (milliseconds). Expired keys are evicted lazily (on access) or periodically (every 100ms). Used for cache TTL, session expiry, and temporary rate limit counters."
  - name: Transactions (MULTI/EXEC)
    id: concept:redis/transactions
    description: "MULTI starts a transaction, commands are queued, EXEC executes atomically. WATCH key implements optimistic locking — if the watched key changes before EXEC, the transaction aborts. Unlike SQL transactions, there is no rollback; if one command fails, others may still execute."
  - name: Lua Scripting
    id: concept:redis/lua
    description: "Atomic scripts executed server-side via EVAL. EVAL 'return redis.call(\"SET\", KEYS[1], ARGV[1])' 1 mykey myvalue. Scripts are atomic and block the server. Used for complex multi-key operations, rate limiting, and distributed locks. SCRIPT LOAD caches the script, EVALSHA runs it."
  - name: Persistence
    id: concept:redis/persistence
    description: "RDB (point-in-time snapshots) and AOF (append-only file logging every write). RDB is faster to load, AOF is more durable (every second or every write). Combine both for safety. BGSAVE forks the process. Disable persistence when Redis is used purely as a cache."
  - name: Clustering
    id: concept:redis/cluster
    description: "Automatic sharding across multiple Redis nodes (16384 hash slots). Cluster mode supports partial availability and automatic failover. Nodes gossip to detect failures. Clients connect to any node and are redirected (MOVED, ASK) to the correct shard. Not all multi-key operations work across slots."
  - name: Redis Stack
    id: concept:redis/stack
    description: "Redis extended with modules: RediSearch (full-text search, indexing, aggregation), RedisJSON (native JSON document store with JSONPath), RedisTimeSeries (time-series data with downsampling), RedisGraph (property graph), RedisBloom (probabilistic data structures)."
apis:
  - name: SET key value
    id: api:redis/set
    signature: "SET key value [NX | XX] [EX seconds | PX milliseconds | EXAT unix | PXAT unix] [GET]"
    returns: OK (string) or null if NX/XX conditions not met.
    description: "Sets a key to a string value. NX sets only if the key does not exist. XX sets only if the key exists. EX sets expiry in seconds. GET returns the old value. The most commonly used Redis command."
  - name: GET key
    id: api:redis/get
    signature: "GET key: string | null"
    returns: The value or nil if the key does not exist.
    description: "Returns the string value of a key. Returns nil if the key does not exist or has expired. O(1) time complexity. For hashes, use HGET."
  - name: ZADD key score member
    id: api:redis/zadd
    signature: "ZADD key [NX | XX] [GT | LT] [CH] score member [score member ...]: number"
    returns: Number of new elements added.
    description: "Adds one or more members to a sorted set with the given scores. If the member exists, the score is updated. CH returns the count of changed elements. Options: NX (new only), XX (existing only), GT/LT (update only if new score is greater/less)."
  - name: EVAL script numkeys
    id: api:redis/eval
    signature: "EVAL script: string numkeys: number [key ...] [arg ...]: any"
    returns: The return value of the Lua script.
    description: "Executes a Lua script atomically on the server. Keys should be passed as KEYS[1]... and arguments as ARGV[1]... to follow cluster hash-tag conventions. Cached via SCRIPT LOAD for repeated use."
  - name: PUBLISH channel message
    id: api:redis/publish
    signature: "PUBLISH channel: string message: string: number"
    returns: Number of clients that received the message.
    description: "Broadcasts a message to all subscribers of the given channel. Returns the count of subscribers that received it (in the current shard for cluster mode). Messages are fire-and-forget with no persistence."
  - name: SETEX key seconds value
    id: api:redis/setex
    signature: "SETEX key: string seconds: number value: string: OK"
    returns: OK
    description: "Atomic SET + EXPIRE. Equivalent to SET key value EX seconds. The combined command is atomic, so no race condition exists between the SET and the EXPIRE."
sections:
  - title: Caching Pattern
    id: section:redis/caching
    content: |
      Cache-aside pattern with automatic TTL:

      ```typescript
      import { createClient } from 'redis';

      const redis = createClient({ url: process.env.REDIS_URL });
      await redis.connect();

      async function getUser(id: string): Promise<User> {
          const cacheKey = `user:${id}`;

          // Try cache
          const cached = await redis.get(cacheKey);
          if (cached) return JSON.parse(cached);

          // Cache miss — load from database
          const user = await db.query('SELECT * FROM users WHERE id = $1', [id]);

          // Store in cache with TTL (15 minutes)
          await redis.setEx(cacheKey, 900, JSON.stringify(user));
          return user;
      }

      // Invalidate on update
      async function updateUser(id: string, data: Partial<User>): Promise<void> {
          await db.query('UPDATE users SET ... WHERE id = $1', [data, id]);
          await redis.del(`user:${id}`); // Invalidate cache
      }
      ```
  - title: Rate Limiting with Sorted Sets
    id: section:redis/rate-limiting
    content: |
      Sliding window rate limiter using sorted sets:

      ```typescript
      async function checkRateLimit(userId: string, limit: number, windowSec: number): Promise<boolean> {
          const key = `ratelimit:${userId}`;
          const now = Date.now();
          const windowStart = now - windowSec * 1000;

          // Remove old entries outside the window
          await redis.zRemRangeByScore(key, 0, windowStart);

          // Count current entries
          const count = await redis.zCard(key);

          if (count >= limit) return false; // Rate limited

          // Add current request
          await redis.zAdd(key, { score: now, value: `${now}-${Math.random()}` });
          await redis.expire(key, windowSec); // Cleanup TTL
          return true;
      }

      // Usage in an API route
      if (!(await checkRateLimit(req.ip, 100, 60))) {
          return res.status(429).json({ error: 'Too many requests' });
      }
      ```
---
