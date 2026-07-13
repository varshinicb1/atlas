---
kind: Package
id: package:observability
name: Observability Patterns
version: "2"
purpose: Document observability patterns — logging, metrics, distributed tracing, OpenTelemetry, structured logging, alerting, and dashboard strategies for understanding production system behavior.
problem_solved: Provides a unified approach to understanding what is happening in production systems through three pillars — logs, metrics, and traces — enabling teams to debug issues, detect anomalies, measure performance, and understand system behavior without guessing.
install: npm install @opentelemetry/api @opentelemetry/sdk-node @opentelemetry/instrumentation-http
dependencies:
  - concept:distributed-systems
  - concept:monitoring
  - concept:software-architecture
concepts:
  - name: Three Pillars
    id: concept:observability/three-pillars
    description: "Logs (discrete events with timestamps and metadata), Metrics (aggregated numeric measurements — counters, histograms, gauges), Traces (end-to-end request paths across services). Each pillar answers different questions: logs tell what happened, metrics tell how many/how fast, traces tell where the latency went."
  - name: Structured Logging
    id: concept:observability/structured-logging
    description: "Logging in machine-parseable format (JSON) instead of free text. Each log line has timestamp, level, service, message, and context fields. Structured logs can be queried, filtered, and aggregated. Tools: pino (Node.js fast), winston, logback, structlog. Avoid string interpolation — pass structured context."
  - name: Log Levels
    id: concept:observability/log-levels
    description: "TRACE (detailed internals, for deep debugging), DEBUG (diagnostic info, only during development), INFO (normal events — request started, user registered), WARN (unexpected but handled — slow query, retry), ERROR (failure — request failed, exception), FATAL (unrecoverable — process exit). Configure minimum level via environment variable."
  - name: Distributed Tracing
    id: concept:observability/tracing
    description: "Tracking a single request as it traverses multiple services. Each service creates spans (units of work: HTTP call, DB query, cache access) with start time, duration, and attributes. Spans are linked via trace ID (propagated in HTTP headers: traceparent). Tools: Jaeger, Zipkin, Datadog APM, Honeycomb."
  - name: OpenTelemetry (OTel)
    id: concept:observability/opentelemetry
    description: "The industry standard for generating, collecting, and exporting telemetry data. Provides vendor-neutral APIs, SDKs, and instrumentation libraries. OTel Collector receives data, processes it (sampling, batching, transformation), and exports to backends (Prometheus, Jaeger, Datadog, Grafana). Spans, metrics, and logs unified in one pipeline."
  - name: Metrics Types
    id: concept:observability/metrics-types
    description: "Counter (increment-only — request count, errors), Gauge (point-in-time value — memory usage, queue depth), Histogram (distribution — request latency, response size). Metrics have name, labels/attributes, and value. Prometheus exposition format is the most common. Rate, histogram quantiles (p50, p99), and aggregation over labels reveal system behavior."
  - name: RED Method
    id: concept:observability/red-method
    description: "Monitoring methodology for services: Rate (requests per second), Errors (failed requests per second), Duration (latency distributions). Every service should have RED metrics. Aligns with SLI (Service Level Indicators). A dashboard showing RED for each service provides at-a-glance health assessment."
  - name: USE Method
    id: concept:observability/use-method
    description: "Resource monitoring methodology: Utilization (percent busy), Saturation (queued work), Errors (failure count). Applied to infrastructure (CPU, memory, disk, network). Tools: Prometheus + node_exporter, cAdvisor for containers. Overlap with RED: RED for services, USE for resources — together they cover the full stack."
  - name: Sampling
    id: concept:observability/sampling
    description: "Not every request needs full tracing. Head-based sampling (decide at root span: keep 5% of all traces). Tail-based sampling (decide after seeing all spans: keep traces with errors or high latency). Adaptive sampling adjusts based on traffic volume. Important for cost control at scale."
  - name: Alerting
    id: concept:observability/alerting
    description: "AlertManager (Prometheus) or Grafana Alerts evaluate rules (rate of 5xx errors > 1% for 5 minutes). Alert severity: P0 (critical — pager), P1 (high — Slack within 15 min), P2 (medium — next business day), P3 (low — tracked). Every alert needs a runbook. Fire only when human action is required — avoid alert fatigue."
  - name: Dashboards
    id: concept:observability/dashboards
    description: "Organize by service: RED panels on top (latency heatmap, error rate, request rate), USE panels for infrastructure, logs panel showing recent errors. Each panel has a purpose. Use templating (dropdown for environment, service, host). Share dashboards as JSON or Terraform. Grafana is the most popular dashboarding tool."
  - name: Health Check APIs
    id: concept:observability/health-checks
    description: "Standard endpoints: /healthz (liveness — process alive), /readyz (readiness — can accept traffic), /metrics (Prometheus endpoint), /startupz (startup — initial load complete). Include dependency checks (database, cache, downstream services) but with care — cascading failures can happen if all services check each other."
apis:
  - name: OpenTelemetry span creation
    id: api:observability/otel-span
    signature: "const tracer = opentelemetry.trace.getTracer('my-service'); const span = tracer.startSpan('processOrder', { attributes: { orderId, amount } }); // ... biz logic span.end();"
    returns: A span object (auto-closes in async contexts).
    description: "Creates a tracing span with name and optional attributes. Spans are automatically nested if created within an active span context. Always end spans to flush them. Use span.setAttribute() and span.addEvent() for additional context."
  - name: Prometheus counter
    id: api:observability/prometheus-counter
    signature: "const orderCounter = new Counter({ name: 'orders_created_total', help: 'Total orders created', labelNames: ['status', 'payment_method'] }); orderCounter.inc({ status: 'completed', payment_method: 'card' });"
    returns: A counter metric (incrementable).
    description: "Creates a Prometheus counter metric. Call inc() to increment. Labels provide dimensionality — query by label combinations. Naming convention: metric_<unit>_total for counters, metric_<unit>_seconds for histograms."
  - name: Histogram observation
    id: api:observability/histogram
    signature: "const latencyHistogram = new Histogram({ name: 'http_request_duration_seconds', help: 'HTTP latency in seconds', buckets: [0.01, 0.05, 0.1, 0.25, 0.5, 1, 2.5, 5, 10] }); const end = latencyHistogram.startTimer(); // ... end({ method: 'GET', route: '/api/users' });"
    returns: A histogram metric (observes values).
    description: "Records latency observations into buckets. Default buckets cover 0.001-10s. startTimer() creates a timer that records on call. Labels for dimensionality. Exposes _count, _sum, _bucket for PromQL histograms: histogram_quantile(0.99, rate(...))."
  - name: Structured logger
    id: api:observability/structured-logger
    signature: "logger.info({ userId, action: 'login', ip: req.ip, duration: elapsed }, 'User logged in'); // or logger.error({ err, requestId }, 'Failed to process order');"
    returns: Logged message (side effect).
    description: "Logs a structured (JSON) record. First argument is the context object (any JSON-serializable data), second is the message. Never interpolate variables into the message string — add them as context. Enable automatic request ID injection via middleware."
sections:
  - title: OpenTelemetry Setup
    id: section:observability/otel-setup
    content: |
      Auto-instrument a Node.js app with OpenTelemetry:

      ```typescript
      // instrumentation.ts — must be imported before app code
      import { NodeSDK } from '@opentelemetry/sdk-node';
      import { getNodeAutoInstrumentations } from '@opentelemetry/auto-instrumentations-node';
      import { OTLPTraceExporter } from '@opentelemetry/exporter-trace-otlp-http';
      import { OTLPMetricExporter } from '@opentelemetry/exporter-metrics-otlp-http';
      import { PeriodicExportingMetricReader } from '@opentelemetry/sdk-metrics';
      import { Resource } from '@opentelemetry/resources';
      import { SEMRESATTRS_SERVICE_NAME } from '@opentelemetry/semantic-conventions';

      const sdk = new NodeSDK({
          resource: new Resource({
              [SEMRESATTRS_SERVICE_NAME]: 'order-service',
          }),
          traceExporter: new OTLPTraceExporter({ url: 'http://otel-collector:4318/v1/traces' }),
          metricReader: new PeriodicExportingMetricReader({
              exporter: new OTLPMetricExporter({ url: 'http://otel-collector:4318/v1/metrics' }),
              exportIntervalMillis: 60000,
          }),
          instrumentations: [getNodeAutoInstrumentations()],
      });
      sdk.start();
      ```
  - title: Structured Logging Best Practices
    id: section:observability/logging
    content: |
      Log everything as JSON with pino:

      ```typescript
      import pino from 'pino';

      const logger = pino({
          level: process.env.LOG_LEVEL || 'info',
          transport: process.env.NODE_ENV !== 'production'
              ? { target: 'pino-pretty', options: { colorize: true } }
              : undefined,
          redact: ['req.headers.authorization', 'req.body.password', 'user.email'],
          serializers: {
              req: (req) => ({ method: req.method, url: req.url, headers: req.headers }),
              err: pino.stdSerializers.err,
          },
      });

      // Log request context
      app.use((req, res, next) => {
          req.log = logger.child({ requestId: req.id, ip: req.ip });
          const start = Date.now();
          res.on('finish', () => {
              req.log.info({ statusCode: res.statusCode, duration: Date.now() - start }, 'request completed');
          });
          next();
      });

      // Usage in handlers
      app.get('/api/orders/:id', async (req, res) => {
          req.log.info({ orderId: req.params.id }, 'fetching order');
          try {
              const order = await db.findOrder(req.params.id);
              req.log.info({ orderFound: !!order }, 'order lookup result');
              res.json(order);
          } catch (err) {
              req.log.error({ err, orderId: req.params.id }, 'failed to fetch order');
              res.status(500).json({ error: 'Internal error' });
          }
      });
      ```

      Logging middleware adds requestId, duration, and statusCode to every request automatically. All logs are JSON and can be ingested by any log aggregator (Elasticsearch, Loki, Datadog, Axiom).
---
