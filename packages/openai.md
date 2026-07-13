---
kind: Package
id: package:openai
name: OpenAI API Patterns
version: "2.0"
purpose: Document OpenAI API patterns — GPT completions, embeddings, assistants, function calling, vision, and structured outputs for building LLM-powered applications.
problem_solved: Provides a reference for OpenAI's API surface — chat completions, embeddings, assistants, image generation, and audio — covering authentication, streaming, error handling, retries, and cost optimization patterns.
install: pip install openai
dependencies:
  - concept:llm-patterns
  - concept:python
  - concept:api-design
concepts:
  - name: Chat Completions
    id: concept:openai/chat-completions
    description: The primary API for text generation with GPT models. Accepts an array of messages with system, user, and assistant roles. Supports temperature, top_p, frequency_penalty, presence_penalty for output control. Models include gpt-4o, gpt-4o-mini, gpt-4-turbo, and o-series reasoning models.
  - name: Streaming
    id: concept:openai/streaming
    description: "Setting stream=True on chat completions returns an iterator of delta chunks. Each chunk contains partial content deltas, tool call deltas, and finish_reason. Server-sent events (SSE) format. Use for real-time token-by-token display in chat UIs."
  - name: Function/Tool Calling
    id: concept:openai/tool-calling
    description: "Declaring tools (functions with JSON Schema parameters) that the model can invoke. The model returns a tool_calls array with id, type, function name, and arguments (JSON string). The caller executes the function and sends the result back. Supports parallel tool calls in a single response."
  - name: Structured Outputs
    id: concept:openai/structured-outputs
    description: "Guaranteed JSON responses matching a provided schema using response_format={type: 'json_schema', json_schema: {...}}. Unlike JSON mode, the model's output is constrained by grammar to produce structurally valid JSON. Supports strict mode (strict: true) for reliable schema adherence."
  - name: Embeddings
    id: concept:openai/embeddings
    description: Converts text into 1536- or 3072-dimensional vectors using text-embedding-3-small or text-embedding-3-large. Useful for semantic search, clustering, classification, and RAG pipelines. The new models support dimensions parameter to truncate vectors, reducing storage costs.
  - name: Assistants API
    id: concept:openai/assistants
    description: A higher-level API for building stateful AI assistants with persistent threads, messages, and runs. Supports attaching files (code_interpreter, file_search), custom instructions, and tool resources. Runs execute asynchronously with polling via run status (queued, in_progress, requires_action, completed, failed).
  - name: Vision
    id: concept:openai/vision
    description: "GPT-4o and GPT-4-turbo can process images by including image_url content blocks in user messages. Accepts base64-encoded images or URL references. Supports detail: 'low', 'high', or 'auto' for resolution-dependent processing. Useful for OCR, diagram analysis, and visual QA."
  - name: Token Management
    id: concept:openai/tokens
    description: "Models have context windows (128K for gpt-4o, 1M for o1/o3). Token counts affect cost and latency. Use tiktoken to count tokens before requests. Set max_tokens to limit response length. Implement truncation strategies for long conversations (sliding window, summarization)."
  - name: Rate Limits
    id: concept:openai/rate-limits
    description: "Tiered rate limits based on usage tier: RPM (requests per minute) and TPM (tokens per minute). Exceeding returns 429 status with Retry-After header. Implement exponential backoff with jitter. The OpenAI Python client automatically retries on 429, 500, and 503 errors."
  - name: System Prompts
    id: concept:openai/system-prompts
    description: "The system message sets assistant behavior, tone, constraints, and context. It has higher influence than user messages. Best practices: be specific, provide examples, set guardrails (what not to do), and include output format instructions. System prompts are not returned in the response."
  - name: Logprobs
    id: concept:openai/logprobs
    description: "Requesting logprobs in chat completions returns token-level log probabilities, showing the model's confidence in each generated token. Useful for calibration, uncertainty estimation, and debugging unexpected outputs. Specify top_logprobs to get alternative token candidates."
  - name: Fine-tuning
    id: concept:openai/fine-tuning
    description: "Customizing a base model with training data (conversation examples) to improve performance on specific tasks. Requires uploading a JSONL training file. Fine-tuned models retain the base model's capabilities while specializing in the training domain. Supports gpt-4o-mini and gpt-3.5-turbo."
apis:
  - name: client.chat.completions.create()
    id: api:openai/chat-create
    signature: "client.chat.completions.create(model: str, messages: list[dict], temperature: float, max_tokens: int, stream: bool, tools: list[dict], response_format: dict) -> ChatCompletion | Stream[ChatCompletionChunk]"
    returns: A chat completion object or stream of chunks.
    description: "The core generation API. Returns a ChatCompletion with choices (index, message, finish_reason), usage (prompt_tokens, completion_tokens, total_tokens), and id. When stream=True, returns an iterator of ChatCompletionChunk objects."
  - name: client.embeddings.create()
    id: api:openai/embeddings-create
    signature: "client.embeddings.create(model: str, input: str | list[str], dimensions: int | None) -> CreateEmbeddingResponse"
    returns: An embedding response with vector data.
    description: "Generates embeddings for the input text. Returns an object with data (array of {object, index, embedding}) and model. Embedding vectors are normalized unit vectors for cosine similarity."
  - name: client.beta.threads.create_and_run()
    id: api:openai/assistants-create-run
    signature: "client.beta.threads.create_and_run(assistant_id: str, thread: dict, instructions: str | None) -> Run"
    returns: A run object with status and required_action.
    description: "Creates a thread and starts a run in one call. The run executes asynchronously. Poll with client.beta.threads.runs.retrieve() until status is 'completed' or 'requires_action'. Submit tool outputs via submit_tool_outputs()."
  - name: client.images.generate()
    id: api:openai/images-generate
    signature: "client.images.generate(model: str, prompt: str, n: int, size: str, quality: str, style: str) -> ImagesResponse"
    returns: An image generation response with URLs or base64 data.
    description: "Generates images from text prompts using DALL-E 3 or DALL-E 2. Supports sizes 1024x1024, 1792x1024, 1024x1792. quality can be 'standard' or 'hd'. style can be 'vivid' or 'natural'."
  - name: client.audio.transcriptions.create()
    id: api:openai/audio-transcribe
    signature: "client.audio.transcriptions.create(model: str, file: FileObject, response_format: str, language: str | None) -> Transcription"
    returns: Transcribed text or structured output.
    description: "Transcribes audio to text using Whisper. Supports mp3, wav, m4a, ogg, and other formats. response_format can be 'json', 'text', 'srt', 'vtt', or 'verbose_json'. Language auto-detects by default."
  - name: client.moderations.create()
    id: api:openai/moderations
    signature: "client.moderations.create(input: str | list[str], model: str | None) -> Moderation"
    returns: A moderation response with category flags and scores.
    description: "Classifies content against OpenAI's usage policies. Returns flagged (boolean), categories (hate, harassment, self-harm, sexual, violence, etc.), and category_scores (0-1). Use as a safety guardrail for user inputs and model outputs."
  - name: client.fine_tuning.jobs.create()
    id: api:openai/fine-tuning-create
    signature: "client.fine_tuning.jobs.create(training_file: str, model: str, hyperparameters: dict | None) -> FineTuningJob"
    returns: A fine-tuning job object.
    description: "Starts a fine-tuning job using a previously uploaded training file in JSONL format. hyperparameters includes n_epochs, batch_size, and learning_rate_multiplier. The job runs asynchronously — poll with .retrieve()."
sections:
  - title: Chat Completions with Tool Calling
    id: section:openai/tool-calling
    content: |
      Tool calling lets the model invoke external functions and use their results:

      ```python
      from openai import OpenAI
      client = OpenAI()

      tools = [{
          "type": "function",
          "function": {
              "name": "get_weather",
              "description": "Get current temperature for a city",
              "parameters": {
                  "type": "object",
                  "properties": {
                      "city": {"type": "string", "description": "City name"}
                  },
                  "required": ["city"]
              }
          }
      }]

      response = client.chat.completions.create(
          model="gpt-4o",
          messages=[{"role": "user", "content": "What's the weather in Tokyo?"}],
          tools=tools,
          tool_choice="auto"
      )

      if response.choices[0].message.tool_calls:
          for tc in response.choices[0].message.tool_calls:
              # Execute function and submit result
              result = get_weather(tc.function.arguments)
              messages.append(tc.message)
              messages.append({
                  "role": "tool",
                  "tool_call_id": tc.id,
                  "content": str(result)
              })
      ```
  - title: Structured Output Mode
    id: section:openai/structured-output
    content: |
      Guarantee JSON responses with a schema using strict structured outputs:

      ```python
      response = client.chat.completions.create(
          model="gpt-4o",
          messages=[{"role": "user", "content": "Parse this: John (35) from NYC"}],
          response_format={
              "type": "json_schema",
              "json_schema": {
                  "name": "person_schema",
                  "strict": True,
                  "schema": {
                      "type": "object",
                      "properties": {
                          "name": {"type": "string"},
                          "age": {"type": "number"},
                          "city": {"type": "string"}
                      },
                      "required": ["name", "age", "city"],
                      "additionalProperties": False
                  }
              }
          }
      )
      parsed = json.loads(response.choices[0].message.content)
      ```

      Strict mode ensures the output structurally conforms to the schema with no extra fields. The model receives a grammar constraint that prevents invalid JSON generation.
  - title: Streaming and Error Handling
    id: section:openai/streaming
    content: |
      Stream responses token by token for a real-time UX:

      ```python
      stream = client.chat.completions.create(
          model="gpt-4o",
          messages=[{"role": "user", "content": "Write a haiku"}],
          stream=True
      )

      collected = []
      for chunk in stream:
          if chunk.choices[0].delta.content:
              content = chunk.choices[0].delta.content
              print(content, end="", flush=True)
              collected.append(content)
      ```

      Implement retry logic with exponential backoff:

      ```python
      import time
      from openai import RateLimitError, APIStatusError

      def retry_create(max_retries=3):
          for attempt in range(max_retries):
              try:
                  return client.chat.completions.create(model="gpt-4o", messages=messages)
              except RateLimitError as e:
                  wait = int(e.response.headers.get("Retry-After", 2 ** attempt))
                  time.sleep(wait)
              except APIStatusError as e:
                  if e.status_code < 500:
                      raise
                  time.sleep(2 ** attempt)
          raise Exception("Max retries exceeded")
      ```
---
