---
kind: Package
id: package:microservices
name: Microservices Architecture
version: "2"
purpose: Document microservices architecture patterns — service decomposition, communication, data management, resilience, observability, and deployment strategies for distributed systems.
problem_solved: Provides a reference for decomposing monolithic applications into independently deployable, loosely coupled services that can be developed, scaled, and maintained by separate teams, reducing deployment risk and enabling technology diversity.
install: No single install — apply these patterns to any language/stack.
dependencies:
  - concept:distributed-systems
  - concept:api-design
  - concept:software-architecture
concepts:
  - name: Service Decomposition
    id: concept:microservices/decomposition
    description: "Splitting a system into services by business capability (Subdomain/Capability pattern) — each service owns a specific business function (Orders, Payments, Shipping). Decomposition follows bounded contexts from Domain-Driven Design. Services should be small enough for a single team to own but large enough to be independently useful."
  - name: API Gateway
    id: concept:microservices/api-gateway
    description: "A single entry point that routes requests to internal services, handles authentication, rate limiting, request transformation, and response aggregation. Backend for Frontend (BFF) pattern: separate gateways for mobile, web, and third-party APIs. Gateway can offload cross-cutting concerns from individual services."
  - name: Inter-Service Communication
    id: concept:microservices/communication
    description: "Synchronous (REST, gRPC) for request-response workflows, asynchronous (message queues, event streams) for event-driven flows. gRPC preferred for internal low-latency calls. REST for simpler integrations. Async messaging (Kafka, RabbitMQ, SQS) decouples services and improves resilience."
  - name: Saga Pattern
    id: concept:microservices/saga
    description: "Managing distributed transactions across services without two-phase commit. Choreography: each service publishes events and listens for events to trigger the next step. Orchestration: a coordinator service tells each service what to do and executes compensating actions on failure. Each step has a compensating transaction for rollback."
  - name: CQRS
    id: concept:microservices/cqrs
    description: "Command Query Responsibility Segregation — separate read models (queries) from write models (commands). Commands use the primary database; queries use denormalized, read-optimized views. Enables different scaling for reads vs writes. Often paired with Event Sourcing for complete audit trails."
  - name: Event Sourcing
    id: concept:microservices/event-sourcing
    description: "Storing state changes as an append-only sequence of events rather than the current state. The current state is derived by replaying events. Provides complete audit trail, temporal queries (state at any point), and event-driven integration. Events are immutable and stored in an event store (EventStoreDB, Kafka)."
  - name: Service Discovery
    id: concept:microservices/service-discovery
    description: "Mechanism for services to find each other's network locations. Client-side: services query a registry (Consul, etcd, Eureka) to get target addresses. Server-side: a load balancer (Kubernetes Service, AWS ALB) routes to healthy instances. Health checks determine service availability."
  - name: Circuit Breaker
    id: concept:microservices/circuit-breaker
    description: "Fail-fast pattern that prevents cascading failures. When failures exceed a threshold, the circuit opens — subsequent calls fail immediately without reaching the downstream service. After a timeout, the circuit half-opens to test if the service recovered. Libraries: resilience4j, Hystrix, Polly."
  - name: Bulkhead
    id: concept:microservices/bulkhead
    description: "Isolating resources (thread pools, connections) per service or client to prevent one misbehaving caller from exhausting all resources. If service A's thread pool is full, service B's calls are unaffected. Implemented via separate thread pools, semaphores, or connection pools per dependency."
  - name: Sidecar Pattern
    id: concept:microservices/sidecar
    description: "Co-locating a helper process (sidecar) with the main service container. The sidecar handles cross-cutting concerns: logging, monitoring, service mesh proxy (Envoy, Linkerd), configuration refresh, TLS termination. Both containers share a network namespace and filesystem volume."
  - name: Strangler Fig
    id: concept:microservices/strangler-fig
    description: "Incremental migration from monolith to microservices. Add new functionality as microservices alongside the monolith. Route traffic to new services gradually. The monolith is 'strangled' over time as features are extracted. Requires a routing layer (proxy or gateway) to split traffic between old and new."
  - name: Conway's Law
    id: concept:microservices/conways-law
    description: "Organizations design systems that mirror their communication structure. Each microservice should be owned by a single team. Teams should own their services end-to-end (build, deploy, operate). Services communicate via well-defined APIs, minimizing cross-team synchronization."
apis:
  - name: Circuit Breaker (Resilience4j)
    id: api:microservices/circuit-breaker-api
    signature: "CircuitBreakerConfig config = CircuitBreakerConfig.custom().failureRateThreshold(50).waitDurationInOpenState(Duration.ofSeconds(30)).slidingWindowSize(10).build();"
    returns: A configured circuit breaker.
    description: "Creates a circuit breaker that monitors failure rate over a sliding window. When the threshold is exceeded, the circuit opens and subsequent calls fail fast. After waitDurationInOpenState, calls are allowed through to test recovery."
  - name: Outbox pattern
    id: api:microservices/outbox
    signature: "BEGIN; INSERT INTO outbox (aggregate_id, event_type, payload) VALUES (?, ?, ?); UPDATE orders SET status = 'shipped' WHERE id = ?; COMMIT;"
    returns: Atomic database + event log write.
    description: "Ensures reliable event publishing by writing events to an outbox table in the same database transaction as the business operation. A separate process (CDC or poller) reads the outbox and publishes events to the message broker. Guarantees at-least-once delivery."
  - name: Health Check endpoint
    id: api:microservices/health-check
    signature: "GET /healthz returns 200 OK with { status: 'healthy', version: '1.2.3', checks: { database: { status: 'up', latency_ms: 5 }, cache: { status: 'up' } } }"
    returns: Service health status.
    description: "Standardized health endpoint. liveness probe checks if the service is alive (no deadlock). readiness probe checks if the service can accept traffic (dependencies available). Used by orchestrators (K8s) and load balancers for routing decisions."
sections:
  - title: Saga Orchestration Pattern
    id: section:microservices/saga
    content: |
      Managing a multi-step order workflow with compensating transactions:

      ```typescript
      // Orchestrator service
      class OrderSaga {
          async execute(orderId: string) {
              try {
                  await this.inventory.reserve(orderId);
                  await this.payment.charge(orderId);
                  await this.shipping.schedule(orderId);
                  await this.order.confirm(orderId);
              } catch (error) {
                  // Compensating actions in reverse order
                  await this.order.cancel(orderId);
                  await this.shipping.cancel(orderId).catch(log);
                  await this.payment.refund(orderId).catch(log);
                  await this.inventory.release(orderId).catch(log);
                  throw error;
              }
          }
      }
      ```

      Each service exposes both a forward action and a compensating action. The orchestrator maintains saga state and handles retries with idempotency keys.
  - title: Service Boundaries and APIs
    id: section:microservices/api-design
    content: |
      Design APIs for microservices communication:

      ```typescript
      // Internal gRPC service definition (service.proto)
      service OrderService {
          rpc CreateOrder(CreateOrderRequest) returns (OrderResponse);
          rpc GetOrder(GetOrderRequest) returns (OrderResponse);
          rpc UpdateStatus(UpdateStatusRequest) returns (Empty);
      }

      message CreateOrderRequest {
          string order_id = 1;
          string customer_id = 2;
          repeated LineItem items = 3;
          // Idempotency key prevents duplicate creation
          string idempotency_key = 4;
      }
      ```

      Each service owns its data. Do not share databases between services. Use event-driven integration for cross-service workflows:

      ```typescript
      // Event published after order creation
      interface OrderCreatedEvent {
          eventId: string;
          aggregateId: string;  // Order ID
          eventType: 'OrderCreated';
          timestamp: string;
          payload: {
              customerId: string;
              totalAmount: number;
              items: Array<{ productId: string; quantity: number }>;
          };
      }
      ```
---
