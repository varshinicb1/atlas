---
kind: Package
id: package:event-driven
name: Event-Driven Architecture
version: "1"
purpose: Document event-driven architecture patterns — event sourcing, pub/sub, event streaming, idempotency, dead letter queues, and event schema management for scalable distributed systems.
problem_solved: Enables loosely coupled, scalable systems where components communicate through asynchronous events rather than direct calls, allowing independent scaling, failure isolation, and real-time data processing across distributed services.
install: No single install — uses message brokers and event stores.
dependencies:
  - concept:distributed-systems
  - concept:messaging
  - concept:software-architecture
concepts:
  - name: Events
    id: concept:event-driven/events
    description: "Immutable records of something that happened in the system: OrderPlaced, PaymentReceived, UserRegistered. Events are facts — they cannot be changed or deleted. Each event has an id, type, timestamp, producer, and payload (the data that changed). Events should be named in past tense."
  - name: Producer/Consumer
    id: concept:event-driven/producer-consumer
    description: "Producers publish events without knowing which consumers will process them. Consumers subscribe to event types without knowing which producers created them. This temporal decoupling means producers and consumers can be developed, deployed, and scaled independently."
  - name: Pub/Sub
    id: concept:event-driven/pubsub
    description: "Publish-Subscribe messaging model where events are broadcast to all subscribers via a topic/channel. Each subscriber receives a copy of every matching event. Supports fan-out (one event, many consumers). Implementations: Redis Pub/Sub (simple), RabbitMQ exchanges, Google Pub/Sub, AWS SNS+SQS."
  - name: Event Streams
    id: concept:event-driven/streams
    description: "Ordered, immutable sequence of events that can be replayed. Kafka partitions events by key for ordering guarantees. Consumers maintain their position (offset) and can replay from any point. Streams enable event sourcing, state reconstruction, and temporal queries."
  - name: Event Schema
    id: concept:event-driven/schema
    description: "Structured contracts for event payloads using Avro, Protobuf, or JSON Schema. Schema Registry (Confluent Schema Registry, Apicurio) stores and validates schemas, ensuring backward compatibility. Producers publish schemas; consumers use them to deserialize. Breaking changes require new event versions."
  - name: Idempotency
    id: concept:event-driven/idempotency
    description: "Ensuring that processing the same event multiple times produces the same result. Critical for at-least-once delivery systems. Use idempotency keys (event ID) in database unique constraints or check for processed event IDs before applying changes. The outcome must be the same regardless of retries."
  - name: Dead Letter Queue (DLQ)
    id: concept:event-driven/dlq
    description: "A holding queue for events that could not be processed successfully after retries. Events in the DLQ are examined manually or by a separate process. DLQ monitoring alerts on processing failures. Events can be replayed from the DLQ after fixing the consumer bug."
  - name: Event Sourcing
    id: concept:event-driven/event-sourcing
    description: "Storing the state of an entity as a sequence of events. The current state is derived by replaying all events for that entity. Provides complete audit trail, ability to rebuild state from scratch, and temporal queries. Requires snapshotting for performance (periodic state saves to avoid replaying all events)."
  - name: Competing Consumers
    id: concept:event-driven/competing-consumers
    description: "Multiple consumer instances processing events from the same queue, each handling a subset of messages. Used for parallel processing and scaling consumer throughput. Requires partitioned or sharded queues. Each message is processed by exactly one consumer in the group."
  - name: Outbox Pattern
    id: concept:event-driven/outbox
    description: "Publishing events reliably by writing them to an outbox table in the same database transaction as the business operation. A separate process (transactional log tailer, CDC, or poller) reads the outbox and publishes events. Guarantees exactly-once publication at the database level."
  - name: Event Versioning
    id: concept:event-driven/versioning
    description: "Strategies for evolving event schemas over time. Add-only (new fields are optional), new event version (OrderPlacedV2), or upcasting (transforming old events to new schema on read). Schema Registry enforces compatibility rules: backward (new readers can read old events), forward (old readers can read new events)."
apis:
  - name: Kafka Producer
    id: api:event-driven/kafka-producer
    signature: "producer.send(new ProducerRecord<>(topic, key, value)); producer.flush();"
    returns: RecordMetadata with offset and partition.
    description: "Publishes a record to a Kafka topic. The key determines partitioning (null = round-robin). Value is serialized (Avro, JSON, Protobuf). acks=all ensures replication to all in-sync replicas. Callback handles success/failure. flush() blocks until all sends complete."
  - name: Kafka Consumer
    id: api:event-driven/kafka-consumer
    signature: "consumer.subscribe(Pattern.compile('orders.*')); ConsumerRecords<String, OrderEvent> records = consumer.poll(Duration.ofMillis(100));"
    returns: A batch of consumer records.
    description: "Subscribes to topics matching a pattern. poll() returns a batch of records. Records are processed in a loop. Commit offsets (automatic or manual) track position. enable.auto.commit=false for at-least-once processing with manual offset committing after successful processing."
  - name: Event Store append
    id: api:event-driven/event-store-append
    signature: "eventstore.appendToStream(streamName: string, expectedVersion: number, events: Event[]): WriteResult"
    returns: Write result with next expected version.
    description: "Appends events to an event stream. expectedVersion is an optimistic concurrency check. -2 means 'any', 0 means 'stream must exist', -1 means 'stream must not exist'. Events are appended atomically — either all or none are written."
  - name: Schema compatibility
    id: api:event-driven/schema-compatibility
    signature: "POST /compatibility/subjects/{subject}/versions with { schema: '...' } returns { is_compatible: boolean }"
    returns: Compatibility check result.
    description: "Checks if a new schema version is compatible with existing versions according to the configured compatibility level (BACKWARD, FORWARD, FULL, NONE, BACKWARD_TRANSITIVE, etc.). Run this in CI before deploying new event producers."
sections:
  - title: Event-Driven Order Processing
    id: section:event-driven/order-processing
    content: |
      Events for an order lifecycle with multiple services:

      ```typescript
      // Producer: Order Service
      await eventStore.appendToStream(`order-${orderId}`, expectedVersion, [
          { eventType: 'OrderPlaced', data: { orderId, customerId, items, total } },
      ]);

      // Publish to Kafka for downstream services
      await kafkaProducer.send({
          topic: 'orders',
          key: orderId,
          value: { eventType: 'OrderPlaced', orderId, customerId, items, total },
      });
      ```

      Consumer pattern with idempotency:

      ```typescript
      class PaymentService {
          async handleOrderPlaced(event: OrderEvent) {
              // Idempotency check
              const alreadyProcessed = await redis.exists(`payment:processed:${event.id}`);
              if (alreadyProcessed) return;

              // Process payment in a transaction
              const result = await db.transaction(async (tx) => {
                  // Record event processing
                  await tx.execute('INSERT INTO processed_events (event_id) VALUES (?) ON CONFLICT DO NOTHING', [event.id]);
                  // Process payment
                  await tx.execute('INSERT INTO payments (order_id, amount, status) VALUES (?, ?, ?)', [event.orderId, event.total, 'completed']);
                  return true;
              });

              if (result) {
                  await kafkaProducer.send({
                      topic: 'payments',
                      key: event.orderId,
                      value: { eventType: 'PaymentProcessed', orderId: event.orderId, transactionId: result.transactionId },
                  });
              }
          }
      }
      ```
  - title: Dead Letter Handling
    id: section:event-driven/dlq
    content: |
      Configuring and consuming from a dead letter queue:

      ```typescript
      // Kafka consumer with DLQ
      consumer.subscribe(['orders']);

      while (true) {
          const records = await consumer.consume(100);
          for (const record of records) {
              try {
                  await process(record);
                  await consumer.commitOffsets([record]);
              } catch (error) {
                  console.error(`Failed to process event ${record.key}`, error);
                  // Publish to DLQ
                  await kafkaProducer.send({
                      topic: 'orders-dlq',
                      key: record.key,
                      value: JSON.stringify({
                          originalEvent: record.value,
                          error: error.message,
                          failedAt: new Date().toISOString(),
                          retryCount: record.headers['retry-count'] || 0,
                      }),
                  });
                  await consumer.commitOffsets([record]); // Skip failed event
              }
          }
      }
      ```

      Monitor DLQ size and alert on growth. Replay DLQ events after fixing the consumer — either manually or via a replay tool that re-publishes to the original topic.
---
