---
kind: Package
id: package:vector-databases
name: Vector Database Patterns
version: "1.0"
purpose: Document vector database patterns — embedding storage, similarity search, hybrid search, indexing strategies, and multi-tenancy across Pinecone, Weaviate, Qdrant, Milvus, and Chroma.
problem_solved: Provides a unified mental model for storing and searching high-dimensional vectors (embeddings) at scale, enabling semantic search, RAG, recommendation, and anomaly detection without coupling to a specific database vendor.
install: pip install chromadb qdrant-client pinecone-client weaviate-client pymilvus
dependencies:
  - concept:embeddings
  - concept:llm-patterns
  - concept:rag-patterns
concepts:
  - name: Vector Indexing
    id: concept:vectordb/indexing
    description: "Data structures that accelerate approximate nearest neighbor (ANN) search. Algorithms include HNSW (Hierarchical Navigable Small World — graph-based, fast but memory-intensive), IVF (Inverted File Index — cluster-based, good accuracy/speed balance), and DiskANN (SSD-optimized for billion-scale). Index parameters trade recall for speed and memory."
  - name: Similarity Metrics
    id: concept:vectordb/similarity-metrics
    description: "Distance functions that measure vector similarity: cosine similarity (angle-based, common for text embeddings), Euclidean distance (L2, magnitude-sensitive), dot product (efficient with normalized vectors when combined with cosine), and Manhattan distance (L1, robust to outliers). Choose based on how your embedding model was trained."
  - name: Metadata Filtering
    id: concept:vectordb/metadata-filtering
    description: Pre-filtering or post-filtering search results by metadata fields (author, date, category, source URL). Pre-filtering applies filters before vector search (reduces the search space). Post-filtering runs vector search first then filters results. Pre-filtering is faster but may miss relevant results across filter boundaries.
  - name: Hybrid Search
    id: concept:vectordb/hybrid-search
    description: "Combining vector similarity with keyword/sparse retrieval (BM25, SPLADE) using fusion algorithms (RRF — Reciprocal Rank Fusion, weighted sum, or DPR-style learned fusion). Weaviate and Qdrant support hybrid search natively. Improves recall for exact-term matches where semantic search alone may miss precision."
  - name: HNSW Algorithm
    id: concept:vectordb/hnsw
    description: "The most widely-used ANN algorithm across vector databases. Builds multi-layer navigable graphs where higher layers have fewer nodes for coarse search and lower layers have dense connections for fine-tuning. Parameters: M (edges per node, default 16), ef_construction (build quality vs speed), ef (search depth)."
  - name: Multi-tenancy
    id: concept:vectordb/multi-tenancy
    description: "Isolating data per tenant (organization, user, project) in a single database instance. Strategies: per-tenant collection/index (Pinecone namespaces, Qdrant collections), tenant ID in metadata (requires filtering), or per-tenant database. Collection-per-tenant is most performant for strict isolation."
  - name: Filtered Search
    id: concept:vectordb/filtered-search
    description: Combining vector similarity with structured query filters. Filters act on metadata fields and can be equality, range, geospatial, or tag-based. Most vector databases support pre-filter (apply filter before vector search) or post-filter (apply filter after). Pre-filter is faster when filters are highly selective.
  - name: Upsert & Versioning
    id: concept:vectordb/upsert
    description: "Insert or update vectors by ID in a single atomic operation. Vector databases use upsert for incremental indexing: if a vector with the same ID exists, it is replaced; otherwise, it is created. Some databases (Weaviate) maintain versioning and can roll back to a previous state."
  - name: Batching & Throttling
    id: concept:vectordb/batching
    description: "Sending vectors in batches (100-1000 per request) instead of individually. Most vector databases have request size limits and rate limits. Batching improves throughput by reducing HTTP overhead. Implement client-side throttling with exponential backoff for rate-limited endpoints."
  - name: Quantization
    id: concept:vectordb/quantization
    description: "Reducing vector precision (32-bit float to 8-bit or binary) to decrease memory usage and speed up search. Scalar quantization (SQ) maps each dimension to fewer bits. Product quantization (PQ) splits vectors into sub-vectors and quantizes each subspace. Binary quantization (for 0/1 vectors) is the most aggressive."
  - name: Clustering
    id: concept:vectordb/clustering
    description: "Grouping similar vectors into clusters using k-means or hierarchical clustering. Some vector databases (Milvus) use IVF which partitions the space into clusters. Each query searches the closest clusters first. Clustering reduces the search space from all vectors to a subset."
  - name: Collection/Index Management
    id: concept:vectordb/collection-management
    description: "Creating and managing logical groupings of vectors. A collection (Qdrant, Milvus) or index (Pinecone) defines the schema, vector dimensions, distance metric, and index configuration. Collections can be scaled independently (replicas, shards) based on workload."
apis:
  - name: qdrant_client.QdrantClient.search()
    id: api:vectordb/qdrant-search
    signature: "client.search(collection_name: str, query_vector: list[float], limit: int, query_filter: Filter | None, score_threshold: float | None) -> list[ScoredPoint]"
    returns: A list of scored points with payload.
    description: "Searches for the nearest vectors in a Qdrant collection. Supports optional pre-filtering with query_filter and score_threshold for minimum similarity. Returns ScoredPoint objects with id, version, score, and payload."
  - name: pinecone.Index.query()
    id: api:vectordb/pinecone-query
    signature: "index.query(vector: list[float] | None, id: str | None, top_k: int, include_metadata: bool, filter: dict | None, namespace: str) -> QueryResponse"
    returns: Query results with matches and namespace info.
    description: "Queries a Pinecone index by vector or by ID (if an existing vector is used as the query). Results include matches with id, score, and optional metadata. The namespace parameter partitions data within an index for multi-tenancy."
  - name: client.collections.create()
    id: api:vectordb/collection-create
    signature: "client.create_collection(name: str, vectors_config: VectorParams | dict, optimizers_config: OptimizersConfigDiff | None) -> bool"
    returns: True if the collection was created.
    description: "Creates a new collection with specified vector configuration: size (dimensions), distance (Cosine, Euclid, Dot), and optional on-disk storage. Milvus also requires shards_num and index parameters at creation time."
  - name: chromadb.Collection.add()
    id: api:vectordb/chromadb-add
    signature: "collection.add(ids: list[str], embeddings: list[list[float]] | None, metadatas: list[dict] | None, documents: list[str] | None) -> None"
    returns: None.
    description: "Adds vectors and optional metadata/documents to a Chroma collection. If embeddings are not provided but the collection has an embedding function, it computes embeddings from documents automatically. Chroma stores metadata and documents for retrieval context."
  - name: weaviate_client.query.get()
    id: api:vectordb/weaviate-query
    signature: "client.query.get(class_name: str, properties: list[str]).with_near_vector({vector: list[float]}).with_limit(limit).with_where(where_filter: dict).do() -> dict"
    returns: A dict with the data key containing results.
    description: "Queries a Weaviate class for objects near the given vector. Combine with .with_where() for metadata filtering, .with_group_by() for aggregation, and .with_additional('distance') for similarity scores."
sections:
  - title: Choosing a Vector Database
    id: section:vectordb/choosing
    content: |
      | Database | Best For | Indexing | Scalability | Search Type |
      |----------|----------|----------|-------------|-------------|
      | Pinecone | Managed serverless, production RAG | HNSW | Auto-scaling, up to 5M/ pod | Vector + metadata |
      | Qdrant | Self-hosted, advanced filtering | HNSW, custom | Multi-node, sharding | Vector + full-filter |
      | Weaviate | Hybrid search (vector + BM25) | HNSW | Multi-node, replication | Hybrid, graphql |
      | Milvus | Billion-scale, GPU acceleration | IVF, HNSW, DiskANN | Distributed, sharding | Vector + scalar |
      | Chroma | Local dev, lightweight | HNSW (brute-force fallback) | Single-node | Vector + metadata |
  - title: Building a RAG Pipeline with Vector Search
    id: section:vectordb/rag-pipeline
    content: |
      A production RAG pipeline integrates embedding generation with vector search:

      ```python
      from openai import OpenAI
      import chromadb

      oai = OpenAI()
      chroma_client = chromadb.PersistentClient(path="./chroma_db")
      collection = chroma_client.get_or_create_collection(name="knowledge_base")

      # Index documents
      def index_documents(docs: list[dict]):
          texts = [d["text"] for d in docs]
          ids = [d["id"] for d in docs]
          metadatas = [{"source": d["source"], "date": d["date"]} for d in docs]

          response = oai.embeddings.create(model="text-embedding-3-small", input=texts)
          embeddings = [r.embedding for r in response.data]

          collection.add(ids=ids, embeddings=embeddings, metadatas=metadatas, documents=texts)

      # Search
      def search(query: str, k: int = 4, filter: dict | None = None):
          response = oai.embeddings.create(model="text-embedding-3-small", input=query)
          query_embedding = response.data[0].embedding
          results = collection.query(
              query_embeddings=[query_embedding],
              n_results=k,
              where=filter
          )
          return results["documents"][0]
      ```
  - title: Performance Optimization
    id: section:vectordb/performance
    content: |
      Optimize vector search performance with these strategies:

      ```python
      # Qdrant: tune HNSW parameters for production
      from qdrant_client.http.models import HnswConfigDiff

      client.update_collection(
          collection_name="products",
          hnsw_config=HnswConfigDiff(
              m=16,              # 8-64; higher = better recall, more memory
              ef_construct=200,  # 100-500; higher = better index quality, slower build
              full_scan_threshold=10000  # vectors below this use brute force
          )
      )

      # Batch upsert for throughput
      from qdrant_client.http.models import PointStruct

      def batch_upsert(points: list[PointStruct], batch_size=256):
          for i in range(0, len(points), batch_size):
              client.upsert(
                  collection_name="products",
                  points=points[i:i + batch_size],
                  wait=False  # Fire-and-forget for higher throughput
              )

      # Use quantized vectors for memory savings
      # Pinecone supports dimension reduction in embeddings API
      response = oai.embeddings.create(
          model="text-embedding-3-small",
          input=texts,
          dimensions=256  # Reduce from 1536 to 256 — 6x memory savings
      )
      ```
---
