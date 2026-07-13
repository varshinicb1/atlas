---
kind: Package
id: package:huggingface
name: HuggingFace Ecosystem
version: "4.47"
purpose: Document HuggingFace ecosystem patterns — Transformers, Datasets, tokenizers, model hub, inference API, and fine-tuning workflows.
problem_solved: Provides a unified interface for thousands of pre-trained models across NLP, vision, and audio domains, eliminating the need to implement architectures from scratch or manage model weights manually by centralizing model discovery, loading, and sharing.
install: pip install transformers torch datasets accelerate
dependencies:
  - concept:llm-patterns
  - concept:python
  - concept:deep-learning
concepts:
  - name: Pipeline API
    id: concept:hf/pipeline
    description: "The highest-level abstraction — pipeline('sentiment-analysis'), pipeline('text-generation'), pipeline('image-classification') auto-loads the appropriate model, tokenizer, and preprocessor. Supports 30+ tasks including text, image, audio, and multimodal. Configure with model=, device=, and batch_size=."
  - name: Model Classes
    id: concept:hf/model-classes
    description: "Architecture-specific classes (AutoModelForCausalLM, AutoModelForSequenceClassification, AutoModelForImageClassification) that load pretrained weights from the Hub. The AutoModel variants auto-detect the architecture from the config. Models are PyTorch nn.Module or TensorFlow Keras instances."
  - name: Tokenizers
    id: concept:hf/tokenizers
    description: "Fast tokenizers implemented in Rust with Python bindings — AutoTokenizer handles BPE, WordPiece, and SentencePiece. Methods: tokenize(), encode(), decode(), encode_plus(), batch_encode_plus(). Returns input_ids, attention_mask, and token_type_ids tensors."
  - name: Model Hub
    id: concept:hf/model-hub
    description: "A central repository with 500K+ models, 100K+ datasets, and 200K+ demos. Models are versioned with git (git-lfs for weights). Each model has a model card (README.md), config files, weights, and usage examples. Upload via push_to_hub() or the web UI."
  - name: Datasets Library
    id: concept:hf/datasets
    description: "A library for loading, processing, and streaming datasets from the Hub. Supports map(), filter(), select(), shuffle(), train_test_split(). Arrow-based columnar format enables memory-mapped access for datasets larger than RAM without loading entirely into memory."
  - name: Fine-tuning
    id: concept:hf/fine-tuning
    description: Training loop using Trainer or SFTTrainer for supervised fine-tuning on custom datasets. The Trainer handles batching, gradient accumulation, logging, evaluation, and checkpointing. TrainingArguments configure learning rate, warmup, weight decay, fp16/bf16, deepspeed, and LoRA adapters.
  - name: PEFT & LoRA
    id: concept:hf/peft
    description: "Parameter-Efficient Fine-Tuning (PEFT) using LoRA (Low-Rank Adaptation) — training small rank-decomposition matrices instead of full model weights. Reduces VRAM requirements by 8-16x. peft_config = LoraConfig(r=8, lora_alpha=32, target_modules=['q_proj', 'v_proj'])."
  - name: Quantization
    id: concept:hf/quantization
    description: "Reducing model precision for memory-efficient inference. bitsandbytes provides 4-bit and 8-bit quantization via load_in_4bit=True. GPTQ and AWQ support for GPU, GGUF for CPU. Quanto for uniform quantization. Enables running 70B models on consumer GPUs."
  - name: Inference API
    id: concept:hf/inference-api
    description: "A serverless API for HuggingFace models without self-hosting. POST to https://api-inference.huggingface.co/models/{model} with API key. Supports all pipeline tasks. Serverless (cold starts) or dedicated inference endpoints for production with auto-scaling."
  - name: Gradio Spaces
    id: concept:hf/spaces
    description: "Hosted demos for ML models using Gradio or Streamlit. Spaces are Docker containers on HuggingFace infrastructure. Each Space has an SDK (static, Gradio, Streamlit, Docker), hardware options (CPU, GPU, T4, A10G, A100), and zero-downtime deploys."
  - name: Accelerate
    id: concept:hf/accelerate
    description: Zero-boilerplate distributed training across multiple GPUs and TPUs. A single accelerate config handles mixed precision, gradient accumulation, DeepSpeed, FSDP, and device placement. Replace .to(device) with accelerate's automatic device_map.
  - name: Safetensors
    id: concept:hf/safetensors
    description: "A safe serialization format for ML model weights — no pickle, no arbitrary code execution. Safetensors files load faster than PyTorch .bin files and are the default format for models on the Hub. Use safetensors.torch.load_file() and save_file()."
  - name: TRL (Transformer Reinforcement Learning)
    id: concept:hf/trl
    description: "A library for alignment training using RLHF, DPO, and PPO. SFTTrainer for supervised fine-tuning, DPOTrainer for direct preference optimization, PPOTrainer for reinforcement learning from human feedback. Integrates with PEFT for memory-efficient alignment."
apis:
  - name: pipeline(task, model)
    id: api:hf/pipeline
    signature: "pipeline(task: str, model: str | None, device: int | None, batch_size: int) -> Pipeline"
    returns: A pipeline object callable on inputs.
    description: "Creates an inference pipeline for a given task. The returned pipeline is callable: result = pipe('text'). Results vary by task — classification returns label+score dicts, generation returns generated text."
  - name: AutoModel.from_pretrained()
    id: api:hf/from-pretrained
    signature: "AutoModel.from_pretrained(pretrained_model_name_or_path: str, **kwargs) -> PreTrainedModel"
    returns: A model with pretrained weights loaded.
    description: "Loads a model and its weights from the Hub or local directory. kwargs include torch_dtype, device_map, load_in_4bit, cache_dir, trust_remote_code, and use_safetensors."
  - name: AutoTokenizer.from_pretrained()
    id: api:hf/tokenizer-from-pretrained
    signature: "AutoTokenizer.from_pretrained(pretrained_model_name_or_path: str, **kwargs) -> PreTrainedTokenizerFast"
    returns: A tokenizer with the matching vocabulary.
    description: "Loads a tokenizer with the vocabulary and config matching the model. Returns input IDs and attention masks for model input. Supports padding='max_length', truncation=True, return_tensors='pt'."
  - name: Trainer.train()
    id: api:hf/trainer-train
    signature: "trainer.train(resume_from_checkpoint: str | None = None) -> TrainOutput"
    returns: Training metrics including loss and step count.
    description: "Starts the training loop. Handles automatic batching, gradient accumulation, logging, evaluation, and checkpoint saving. TrainingArguments control all hyperparameters. Supports resuming from checkpoints if training is interrupted."
  - name: datasets.load_dataset()
    id: api:hf/load-dataset
    signature: "load_dataset(path: str, split: str | None, streaming: bool, **kwargs) -> Dataset | IterableDataset"
    returns: A Dataset (Arrow-backed) or IterableDataset (streaming).
    description: "Loads a dataset from the Hub. split='train' loads training split. streaming=True loads iteratively without downloading entirely — useful for huge datasets. Supports slice splits: split='train[:10%]'."
  - name: model.push_to_hub()
    id: api:hf/push-to-hub
    signature: "model.push_to_hub(repo_id: str, commit_message: str, private: bool) -> str"
    returns: URL of the uploaded model.
    description: "Uploads model weights and config to the Hub. Requires authentication via huggingface-cli login. The repo_id is namespace/model-name. Safetensors format is used by default."
  - name: pipeline.model.to(device)
    id: api:hf/model-to-device
    signature: "model.to(device: str | torch.device) -> Self"
    returns: The model moved to the specified device.
    description: "Moves all model parameters to the specified device ('cuda:0', 'cpu', 'mps'). For large models, use device_map='auto' with accelerate instead of manual .to() to split layers across devices."
sections:
  - title: Text Classification Pipeline
    id: section:hf/text-classification
    content: |
      The pipeline API makes inference a one-liner:

      ```python
      from transformers import pipeline

      classifier = pipeline(
          "sentiment-analysis",
          model="distilbert-base-uncased-finetuned-sst-2-english",
          device=0
      )

      result = classifier(
          ["I love this product!", "This is terrible."],
          batch_size=2
      )
      # [{'label': 'POSITIVE', 'score': 0.999}, {'label': 'NEGATIVE', 'score': 0.998}]
      ```

      Pipeline handles tokenization, batching, padding, and output decoding internally. For custom models, pass model= and tokenizer= separately.
  - title: Fine-tuning with LoRA
    id: section:hf/fine-tuning-lora
    content: |
      Memory-efficient fine-tuning using LoRA adapters:

      ```python
      from transformers import AutoModelForCausalLM, TrainingArguments, Trainer
      from peft import LoraConfig, get_peft_model
      from datasets import load_dataset

      model = AutoModelForCausalLM.from_pretrained(
          "mistralai/Mistral-7B-v0.1",
          load_in_4bit=True,
          device_map="auto"
      )

      lora_config = LoraConfig(
          r=16, lora_alpha=32, target_modules=["q_proj", "v_proj"],
          lora_dropout=0.05, bias="none", task_type="CAUSAL_LM"
      )
      model = get_peft_model(model, lora_config)

      dataset = load_dataset("json", data_files="training.jsonl", split="train")
      training_args = TrainingArguments(
          output_dir="./mistral-lora",
          per_device_train_batch_size=4,
          gradient_accumulation_steps=4,
          learning_rate=2e-4,
          fp16=True,
          logging_steps=10,
          num_train_epochs=3,
      )

      trainer = Trainer(model=model, args=training_args, train_dataset=dataset)
      trainer.train()
      model.push_to_hub("my-org/mistral-lora-adapters")
      ```

      LoRA trains only the adapter weights (~2% of parameters) while the base model stays frozen in 4-bit.
  - title: Dataset Preparation
    id: section:hf/dataset-prep
    content: |
      The Datasets library provides Arrow-based high-performance processing:

      ```python
      from datasets import load_dataset, Dataset

      dataset = load_dataset("imdb", split="train[:10%]")
      tokenized = dataset.map(
          lambda x: tokenizer(x["text"], truncation=True, padding="max_length", max_length=512),
          batched=True
      )
      tokenized = tokenized.train_test_split(test_size=0.1)
      tokenized.set_format(type="torch", columns=["input_ids", "attention_mask", "label"])

      # Streaming for large datasets
      huge_dataset = load_dataset("c4", "en", split="train", streaming=True)
      for example in huge_dataset.take(100):
          print(example["text"][:200])
      ```

      The Arrow format enables zero-copy reads, memory-mapped access for datasets larger than RAM, and efficient shuffling without duplicating data.
---
