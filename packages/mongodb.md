---
kind: Package
id: package:mongodb
name: MongoDB Patterns
version: "8"
purpose: Document MongoDB patterns — document design, aggregation pipeline, indexing, replication, sharding, and Mongoose ODM patterns for Node.js applications.
problem_solved: Provides a flexible document database with a powerful query language, native replication, and horizontal scaling that handles schema-less data, nested structures, and rapid iteration without rigid table migrations.
install: npm install mongodb mongoose
dependencies:
  - concept:databases
  - concept:data-modeling
  - concept:node-js
concepts:
  - name: Documents
    id: concept:mongodb/documents
    description: "JSON-like BSON documents with dynamic schemas — each document can have different fields. Documents are stored in collections (analogous to tables). Maximum document size is 16MB. Nested arrays, sub-documents, and arrays of sub-documents are first-class citizens, enabling denormalized data models."
  - name: Collections
    id: concept:mongodb/collections
    description: "Groups of documents analogous to SQL tables. Collections are created implicitly on first insert. Options: capped collections (fixed-size, FIFO), time-series collections (optimized for time-series data), and clustered collections (primary key ordering). Schema validation via JSON Schema on collection creation."
  - name: Aggregation Pipeline
    id: concept:mongodb/aggregation
    description: "A pipeline of stages that process documents sequentially: $match (filter), $group (aggregate), $sort, $project (shape), $lookup (join), $unwind (flatten arrays), $bucket (categorize), $facet (multi-faceted). Each stage transforms the stream. Pipelines execute on the database server, reducing data transfer."
  - name: Indexes
    id: concept:mongodb/indexes
    description: "Data structures that accelerate queries: single field, compound, multikey (array fields), text (full-text search), geospatial (2dsphere), hashed (sharding), and TTL (auto-expire). Indexes trade write performance for read performance. Use explain() to verify index usage. The _id field is always indexed."
  - name: Replica Sets
    id: concept:mongodb/replication
    description: "Primary-secondary replication with automatic failover. A replica set has 3+ members: one primary (writes), multiple secondaries (reads with readPreference). Elections happen automatically when the primary is unavailable. Write concern and read preference control consistency and availability trade-offs."
  - name: Sharding
    id: concept:mongodb/sharding
    description: "Horizontal scaling by distributing data across shards using a shard key. mongos routes queries to the correct shard. Chunks (ranges of shard key values) are balanced automatically. Choose a shard key with high cardinality and even distribution. Hashed shard keys distribute writes evenly."
  - name: Mongoose Schemas
    id: concept:mongodb/mongoose-schemas
    description: "Mongoose provides a schema-based solution with validation, casting, middleware (pre/post hooks), virtuals, and population (cross-collection references). Schema types include String, Number, Date, Buffer, Boolean, Mixed, ObjectId, Array, Decimal128, and Map."
  - name: Change Streams
    id: concept:mongodb/change-streams
    description: "Real-time stream of data changes (inserts, updates, replaces, deletes, invalidations) via the oplog. Useful for event-driven architectures, search index sync, cache invalidation, and audit logging. Requires replica set. Filter by $match pipeline. Resume via resumeToken."
  - name: Transactions
    id: concept:mongodb/transactions
    description: "Multi-document ACID transactions (since 4.0) across multiple collections and documents. Use withSession() and session.withTransaction(). Transactions have a 60-second default timeout and 16MB size limit. Use for financial operations and multi-document atomicity requirements."
  - name: Geospatial Queries
    id: concept:mongodb/geospatial
    description: "2dsphere indexes enable geospatial queries: $near (proximity), $geoWithin (polygon/circle), $geoIntersects. Store coordinates as GeoJSON: { type: 'Point', coordinates: [lng, lat] }. Geospatial indexes support spheres (Earth) for accurate distance calculations."
  - name: TTL Indexes
    id: concept:mongodb/ttl
    description: "Indexes that automatically expire documents after a specified time. Create with { expireAfterSeconds: seconds } on a date field. MongoDB runs a background thread every 60 seconds that deletes expired documents. Useful for session stores, logs, temporary tokens, and event data with retention policies."
apis:
  - name: db.collection.find()
    id: api:mongodb/find
    signature: "db.collection.find(filter: object, projection?: object): Cursor"
    returns: A cursor over matching documents.
    description: "Queries documents matching the filter criteria. filter uses MongoDB query operators ($gt, $lt, $in, $regex, $text, $exists, $elemMatch). Projection specifies which fields to include/exclude. Returns a Cursor that can be iterated or converted to an array."
  - name: db.collection.aggregate()
    id: api:mongodb/aggregate
    signature: "db.collection.aggregate(pipeline: object[], options?: { allowDiskUse, maxTimeMS, batchSize }): AggregationCursor"
    returns: A cursor over the aggregation results.
    description: "Executes an aggregation pipeline. Each stage transforms the document stream. Common stages: $match, $group, $sort, $project, $lookup, $unwind, $addFields, $count, $facet, $bucket. Use allowDiskUse for large sorts/group operations."
  - name: db.collection.updateOne()
    id: api:mongodb/updateone
    signature: "db.collection.updateOne(filter: object, update: object, options?: { upsert, arrayFilters }): UpdateResult"
    returns: An UpdateResult with matched/modified counts.
    description: "Updates the first document matching the filter. Use $set, $unset, $inc, $push, $pull, $addToSet, $rename operators. upsert: true creates a document if none matches. arrayFilters targets specific array elements."
  - name: db.collection.createIndex()
    id: api:mongodb/create-index
    signature: "db.collection.createIndex(keys: object, options?: { unique, sparse, expireAfterSeconds, name }): string"
    returns: The name of the created index.
    description: "Creates an index on the specified field(s). keys: { field: 1 } (ascending) or { field: -1 } (descending). Compound: { a: 1, b: -1 }. Unique indexes prevent duplicate values. Sparse indexes only index documents with the field."
  - name: db.collection.watch()
    id: api:mongodb/watch
    signature: "db.collection.watch(pipeline?: object[], options?: { batchSize, resumeAfter, startAfter, startAtOperationTime, fullDocument }): ChangeStream"
    returns: A ChangeStream emitting change events.
    description: "Opens a change stream that emits events on data changes. pipeline filters events ($match, $project). fullDocument: 'updateLookup' includes the current document version on updates. resumeAfter and startAfter enable resuming from a known point."
  - name: mongoose.model()
    id: api:mongodb/mongoose-model
    signature: "mongoose.model(name: string, schema: Schema, collection?: string): Model"
    returns: A Mongoose Model class.
    description: "Creates a Model from a Schema. The Model provides CRUD methods (save, find, findById, findOneAndUpdate, deleteOne), static methods, virtuals, and middleware (pre/post hooks). Methods are async and return Promises."
sections:
  - title: Document Design Patterns
    id: section:mongodb/document-design
    content: |
      Choose between embedding and referencing based on access patterns:

      ```javascript
      // Embedding — for tightly coupled, frequently read-together data
      const orderSchema = new Schema({
          userId: { type: ObjectId, ref: 'User' },
          items: [{
              productId: { type: ObjectId, ref: 'Product' },
              name: String,
              price: Number,
              quantity: Number,
          }],
          total: Number,
          createdAt: { type: Date, default: Date.now },
      });

      // One query to get an order with all items — no joins needed
      const order = await Order.findById(orderId);

      // Referencing — for independent entities that grow unbounded
      const userSchema = new Schema({
          name: String,
          email: String,
          // Orders are referenced, not embedded, because a user can have unlimited orders
      });

      const orderSchemaRef = new Schema({
          userId: { type: ObjectId, ref: 'User', index: true },
          total: Number,
          // Items are embedded because an order is a bounded set
      });

      // Populate the reference
      const orderWithUser = await Order.findById(orderId).populate('userId');
      ```
  - title: Aggregation Pipeline
    id: section:mongodb/aggregation
    content: |
      Complex reporting with the aggregation pipeline:

      ```javascript
      const results = await db.collection('orders').aggregate([
          { $match: { status: 'completed', createdAt: { $gte: startDate } } },
          { $group: {
              _id: { $dateToString: { format: '%Y-%m-%d', date: '$createdAt' } },
              totalRevenue: { $sum: '$total' },
              totalOrders: { $sum: 1 },
              averageOrderValue: { $avg: '$total' },
          }},
          { $sort: { _id: -1 } },
          { $project: {
              date: '$_id',
              revenue: { $round: ['$totalRevenue', 2] },
              orders: '$totalOrders',
              aov: { $round: ['$averageOrderValue', 2] },
              _id: 0,
          }},
          // Join with user data for additional context
          { $lookup: {
              from: 'users',
              localField: 'userId',
              foreignField: '_id',
              as: 'user',
          }},
          { $unwind: { path: '$user', preserveNullAndEmptyArrays: true } },
          { $project: { 'user.password': 0 } }, // Exclude sensitive fields
      ]).toArray();
      ```

      The pipeline processes millions of documents server-side without transferring raw data to the application.
---
