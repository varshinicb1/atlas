---
kind: Package
id: package:langchain
name: LangChain Framework
version: "0.3"
purpose: Document LangChain patterns — LLM chains, agents, RAG, tool calling, and model-agnostic abstractions for building LLM-powered applications.
problem_solved: Provides composable abstractions over LLM providers, vector stores, and document loaders so developers can build retrieval-augmented generation, multi-step agent reasoning, and tool-using chatbots without coupling to a specific model or embedding API.
install: pip install langchain langchain-core langchain-community
dependencies:
  - concept:llm-patterns
  - concept:python
  - concept:vector-databases
concepts:
  - name: Runnable Interface
    id: concept:langchain/runnable
    description: The core abstraction — any component (model, prompt, retriever, chain) implements the Runnable interface with .invoke(), .batch(), .stream(), and .astream(). Runnables compose via the pipe operator (|) for declarative pipelines that stream intermediate results.
  - name: Chat Models
    id: concept:langchain/chat-models
    description: Abstractions over LLM chat APIs (OpenAI, Anthropic, Google, Ollama, local) that accept a list of BaseMessage objects and return an AIMessage. Provide a unified interface for system/user/assistant messages, tool calling, structured output, and token counting.
  - name: Prompt Templates
    id: concept:langchain/prompts
    description: ChatPromptTemplate and MessagesPlaceholder for constructing prompts from templates with dynamic variables. Supports few-shot examples, partial formatting, and schema-aligned prompts for structured output generation.
  - name: Chains
    id: concept:langchain/chains
    description: Composed sequences of Runnables — prompt | model | outputParser. LangChain Expression Language (LCEL) uses the pipe operator to build type-safe, streamable, and batchable chains that can include branching, fallbacks, and parallel execution.
  - name: Retrieval-Augmented Generation (RAG)
    id: concept:langchain/rag
    description: "Pipeline pattern: load documents → split into chunks → embed → store in vector DB → retrieve relevant chunks → inject into prompt → generate answer. LangChain provides document loaders, text splitters (RecursiveCharacterTextSplitter), and retrievers (VectorStoreRetriever, EnsembleRetriever, MultiQueryRetriever)."
  - name: Agents
    id: concept:langchain/agents
    description: LLM-driven systems that decide which tools to call, in what order, and how to interpret results. LangChain agents use a ReAct (Reasoning + Acting) loop — the model thinks, acts, observes, and repeats until it arrives at a final answer or reaches a max iteration limit.
  - name: Tools
    id: concept:langchain/tools
    description: Functions decorated with @tool that expose a name, description, and parameter schema to the LLM. Tools wrap APIs, calculators, database queries, web searches, or custom code — the model chooses which tool to invoke based on the description and argument schema.
  - name: Document Loaders
    id: concept:langchain/document-loaders
    description: "Unified interface for loading documents from sources: PDF (PyPDFLoader), websites (WebBaseLoader), databases, YouTube transcripts, Notion, Confluence, and more. Each loader returns a list of Document objects with page_content and metadata."
  - name: Text Splitters
    id: concept:langchain/text-splitters
    description: "Chunking strategies that split documents into semantically coherent pieces for embedding and retrieval. RecursiveCharacterTextSplitter splits on paragraph/line/word boundaries with configurable chunk size and overlap. SemanticChunker uses embedding similarity to find natural breakpoints."
  - name: Embeddings
    id: concept:langchain/embeddings
    description: "An abstraction over embedding models (OpenAI, HuggingFace, Cohere, Ollama) that converts text into vector representations. Used by vector stores for similarity search. Support batch embedding, async embedding, and dimension-agnostic interfaces."
  - name: Vector Stores
    id: concept:langchain/vectorstores
    description: "A unified API for vector databases (Chroma, Pinecone, Weaviate, Qdrant, FAISS) that supports add_documents(), similarity_search(), and as_retriever(). The retriever wraps the store with search configuration (k, score threshold, MMR)."
  - name: Callbacks
    id: concept:langchain/callbacks
    description: "Observability hooks that fire on chain start/end, LLM start/end, tool start/end, and streaming tokens. ConsoleCallbackHandler logs to stdout, LangSmith handles tracing. Custom callbacks extend BaseCallbackHandler for monitoring, logging, and cost tracking."
  - name: LangSmith
    id: concept:langchain/langsmith
    description: "A tracing and evaluation platform that captures every LLM call, chain step, and tool invocation. Provides run comparison, dataset management, feedback collection, and regression testing. Traces are logged via LANGCHAIN_API_KEY and visible in the LangSmith dashboard."
  - name: Structured Output
    id: concept:langchain/structured-output
    description: "Using .with_structured_output(schema) on chat models to get typed, JSON-matching responses instead of free text. Schemas can be Pydantic models or JSON Schema dicts. The model is instructed to return a valid JSON matching the schema, reducing parsing errors."
apis:
  - name: ChatPromptTemplate.from_messages()
    id: api:langchain/chat-prompt-template
    signature: "ChatPromptTemplate.from_messages(messages: list[tuple[str, str] | BaseMessage]): ChatPromptTemplate"
    returns: A prompt template accepting the declared variables.
    description: "Creates a prompt from a list of (role, content) pairs. Role is 'system', 'user', 'assistant', or 'placeholder'. Variables in content strings are interpolated at invocation time."
  - name: pipe operator (|)
    id: api:langchain/pipe
    signature: "chain = prompt | model | output_parser"
    returns: A RunnableSequence.
    description: The LCEL pipe operator that chains Runnables. Output of the left side is passed as input to the right. Supports streaming, batching, and async across the entire pipeline.
  - name: Runnable.with_structured_output()
    id: api:langchain/with-structured-output
    signature: "model.with_structured_output(schema: type | dict, *, method: str = 'function_calling'): Runnable"
    returns: A runnable that returns structured data matching the schema.
    description: "Wraps a chat model to return structured output matching a Pydantic model or JSON Schema. Uses the model's native tool-calling or JSON mode. Raises if the schema is complex and the model does not support function calling."
  - name: create_react_agent()
    id: api:langchain/create-react-agent
    signature: "create_react_agent(model: ChatModel, tools: list[BaseTool], prompt: str | None = None): Runnable"
    returns: An agent execution runnable.
    description: "Creates a ReAct agent that iteratively reasons and calls tools until a final answer is produced. Returns AgentFinish with the output or AgentAction for the next tool call. Supports stop conditions and max iterations."
  - name: RecursiveCharacterTextSplitter.split_documents()
    id: api:langchain/text-splitter
    signature: "splitter.split_documents(documents: list[Document]) -> list[Document]"
    returns: A list of split document chunks.
    description: "Chunks documents using recursive boundary splitting (paragraph → sentence → word). Configurable via chunk_size (character count), chunk_overlap (overlap between chunks), and separators (custom list of split characters)."
  - name: vectorstore.as_retriever()
    id: api:langchain/as-retriever
    signature: "vectorstore.as_retriever(search_kwargs: dict | None = None): VectorStoreRetriever"
    returns: A retriever wrapping the vector store.
    description: "Converts a vector store into a retriever with search configuration. Supports search_type='similarity', 'mmr', or 'similarity_score_threshold' and search_kwargs for k, fetch_k, lambda_mult, and score_threshold."
  - name: RunnablePassthrough.assign()
    id: api:langchain/runnable-assign
    signature: "RunnablePassthrough.assign(**kwargs: Callable | Runnable) -> RunnableParallel"
    returns: A runnable that passes through input with added computed fields.
    description: "Creates a parallel branch that passes through the original input while computing additional fields. Each kwarg is a callable or Runnable that receives the input and returns the field value."
  - name: "@tool decorator"
    id: api:langchain/tool-decorator
    signature: "@tool\ndef my_tool(arg1: type, arg2: type) -> return_type:\n    \"\"\"description\"\"\""
    returns: A BaseTool instance.
    description: "Decorator that converts a function into a LangChain tool. The function name is the tool name, the docstring is the description, and type hints form the parameter schema. Supports args_schema override via a Pydantic model."
sections:
  - title: Building a RAG Pipeline
    id: section:langchain/rag-pipeline
    content: |
      A RAG pipeline loads documents, chunks them, creates embeddings, stores them in a vector store, and retrieves relevant context at query time:

      ```python
      from langchain_community.document_loaders import WebBaseLoader
      from langchain.text_splitter import RecursiveCharacterTextSplitter
      from langchain_openai import OpenAIEmbeddings, ChatOpenAI
      from langchain_chroma import Chroma
      from langchain_core.runnables import RunnablePassthrough
      from langchain_core.output_parsers import StrOutputParser
      from langchain_core.prompts import ChatPromptTemplate

      loader = WebBaseLoader("https://example.com/docs")
      docs = loader.load()
      splitter = RecursiveCharacterTextSplitter(chunk_size=1000, chunk_overlap=200)
      chunks = splitter.split_documents(docs)

      vectorstore = Chroma.from_documents(chunks, OpenAIEmbeddings())
      retriever = vectorstore.as_retriever(search_kwargs={"k": 4})

      prompt = ChatPromptTemplate.from_template(
          "Answer using only this context: {context}\\n\\nQuestion: {question}"
      )
      model = ChatOpenAI(model="gpt-4o")

      def format_docs(docs):
          return "\\n\\n".join(d.page_content for d in docs)

      chain = (
          {"context": retriever | format_docs, "question": RunnablePassthrough()}
          | prompt | model | StrOutputParser()
      )
      result = chain.invoke("What is the main concept?")
      ```
  - title: Agent with Tools
    id: section:langchain/agent-tools
    content: |
      Agents use a ReAct loop — the LLM reasons about which tool to call next, calls it, observes the result, and repeats until satisfied:

      ```python
      from langchain.agents import create_react_agent, AgentExecutor
      from langchain.tools import tool
      from langchain_openai import ChatOpenAI
      from langchain_core.prompts import PromptTemplate

      @tool
      def calculate(expression: str) -> float:
          "Evaluate a mathematical expression"
          return eval(expression)

      @tool
      def get_weather(city: str) -> str:
          "Get current weather for a city"
          return f"Weather in {city}: 22°C, partly cloudy"

      model = ChatOpenAI(model="gpt-4o", temperature=0)
      prompt = PromptTemplate.from_template(
          "You are a helpful assistant with tools.\\n\\n{input}\\n\\n{agent_scratchpad}"
      )
      agent = create_react_agent(model, [calculate, get_weather], prompt)
      executor = AgentExecutor(agent=agent, tools=[calculate, get_weather])
      result = executor.invoke({"input": "What is 2+2 and weather in Paris?"})
      ```

      Set `handle_parsing_errors=True` on AgentExecutor to gracefully recover when the model produces malformed tool calls. Set `max_iterations=10` to prevent infinite loops.
  - title: Streaming and Observability
    id: section:langchain/streaming
    content: |
      LCEL chains stream token by token by default:

      ```python
      for chunk in chain.stream("Tell me a story"):
          print(chunk, end="", flush=True)
      ```

      For tracing via LangSmith, set environment variables and every chain call is automatically traced:

      ```python
      import os
      os.environ["LANGCHAIN_TRACING_V2"] = "true"
      os.environ["LANGCHAIN_API_KEY"] = "ls_..."
      os.environ["LANGCHAIN_PROJECT"] = "my-project"
      ```

      Custom callbacks provide granular control for logging, metrics, or cost tracking:

      ```python
      from langchain_core.callbacks import BaseCallbackHandler

      class CostTracker(BaseCallbackHandler):
          def on_llm_start(self, serialized, prompts, **kwargs):
              self.start_time = time.time()
          def on_llm_end(self, response, **kwargs):
              tokens = response.llm_output["token_usage"]["total_tokens"]
              print(f"Used {tokens} tokens in {time.time()-self.start_time:.2f}s")
      ```
---
