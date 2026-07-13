---
kind: Package
id: package:crewai
name: CrewAI Framework
version: "0.108"
purpose: Document CrewAI patterns — multi-agent orchestration, role-based agents, task delegation, tool integration, and workflow management for LLM agent teams.
problem_solved: Enables developers to design and orchestrate teams of AI agents that collaborate on complex tasks by assigning specialized roles, delegating sub-tasks, sharing context, and executing sequential or hierarchical workflows with structured outputs.
install: pip install crewai crewai-tools
dependencies:
  - concept:llm-patterns
  - concept:python
  - concept:agent-patterns
concepts:
  - name: Agents
    id: concept:crewai/agents
    description: "Autonomous entities with a defined role, goal, backstory, and LLM configuration. Each agent has a specific expertise area (researcher, writer, coder, reviewer). Agents can be delegated tasks, share context via the crew's shared memory, and use tools. Configured with Agent(role=, goal=, backstory=, llm=, tools=[])."
  - name: Tasks
    id: concept:crewai/tasks
    description: "Assignable units of work with a description, expected output, and assigned agent. Tasks can have dependencies (waiting for other tasks to complete), context from previous tasks, and custom output formats. Task(description=, agent=, expected_output=, context_from_child_tasks=)."
  - name: Crew
    id: concept:crewai/crew
    description: "The orchestrator that manages agents and tasks. Defines the execution flow (sequential, hierarchical), shared memory, and process-level configuration. Crew(agents=[], tasks=[], process=Process.sequential, verbose=True) runs all tasks with the defined agents."
  - name: Sequential Process
    id: concept:crewai/sequential
    description: "Tasks execute one after another, each receiving the previous task's output as context. Simple and predictable — ideal for linear workflows like research-then-write-then-review. The output of task N is automatically available to task N+1."
  - name: Hierarchical Process
    id: concept:crewai/hierarchical
    description: "A manager agent (typically a more capable model like GPT-4o) coordinates specialist agents. The manager creates tasks, delegates to specialists, reviews outputs, and assembles the final result. Requires a manager_llm. Automatically handles task decomposition and result synthesis."
  - name: Tools
    id: concept:crewai/tools
    description: "Functions agents can invoke to interact with external systems — web search, file read/write, API calls, database queries, code execution. crewai-tools provides pre-built tools (SerperDevTool, ScrapeWebsiteTool, FileReadTool, DOCXSearchTool). Custom tools use @tool decorator with type hints."
  - name: Memory & Context
    id: concept:crewai/memory
    description: "Short-term memory stores recent task outputs accessible within the crew run. Long-term memory persists across runs via embeddings in a vector store (ChromaDB). Entity memory tracks entities mentioned in tasks. User memory stores user-specific preferences."
  - name: Delegation
    id: concept:crewai/delegation
    description: "Agents can delegate work to other agents by calling the DelegateWorkTool. In hierarchical mode, the manager agent automatically delegates tasks to appropriate specialist agents. Delegation includes task context, expected output format, and deadline hints."
  - name: Callbacks & Events
    id: concept:crewai/callbacks
    description: "Hooks that fire at each stage of crew execution: on_agent_start, on_agent_end, on_task_start, on_task_end, on_step_start, on_step_end. Used for logging, monitoring, progress tracking, and result accumulation. Implemented via crew's step_callback or step_callback_after for end-of-step actions."
  - name: Output & Formatting
    id: concept:crewai/output
    description: "Tasks define expected_output (a string description of the output format). Crew results can be returned as JSON, plain text, or Pydantic models via output_json=True on Task. The crew's kickoff() returns a CrewOutput with tasks_output (list of TaskOutput) and token usage metadata."
  - name: Custom LLM Providers
    id: concept:crewai/custom-llm
    description: "Agents support any LangChain-compatible LLM — OpenAI, Anthropic, Ollama, Azure OpenAI, Google Gemini, Mistral, Groq. Configure via llm=ChatOpenAI(model='gpt-4') or llm='ollama/llama3'. Different agents can use different models (e.g., GPT-4o for manager, GPT-4o-mini for research)."
  - name: Crews as Tools
    id: concept:crewai/crew-as-tool
    description: "A complete Crew can be wrapped as a Tool for use by another Crew's agents. Enables nested multi-agent teams — a 'research department' crew can be a tool called by an 'executive' agent. Supports recursive orchestration patterns with shared context handling."
apis:
  - name: Crew.kickoff()
    id: api:crewai/kickoff
    signature: "crew.kickoff(inputs: dict | None = None) -> CrewOutput"
    returns: A CrewOutput with task results and usage statistics.
    description: "Starts the crew execution. inputs populate template variables in task descriptions (e.g., {topic}). Returns a CrewOutput with .tasks_output (list of TaskOutput), .token_usage, and .raw output. Tasks execute in the defined process order."
  - name: Agent()
    id: api:crewai/agent
    signature: "Agent(role: str, goal: str, backstory: str, llm: str | ChatLLM, tools: list[BaseTool], allow_delegation: bool, verbose: bool, max_iter: int) -> Agent"
    returns: An Agent instance configured for the crew.
    description: "Creates an agent with a defined persona. The role, goal, and backstory are used as system prompts. Tools define what the agent can do. allow_delegation enables peer-to-peer task handoff. max_iter limits reasoning steps (default 15)."
  - name: Task()
    id: api:crewai/task
    signature: "Task(description: str, expected_output: str, agent: Agent | None, context: list[Task] | None, output_json: bool | Type[BaseModel] | None) -> Task"
    returns: A Task instance awaiting assignment.
    description: "Defines a unit of work. agent can be assigned now or later. context makes outputs of specified tasks available. output_json=True returns structured output. output_pydantic=MyModel validates output against a Pydantic schema."
  - name: "@tool decorator"
    id: api:crewai/tool-decorator
    signature: "@tool('tool_name')\ndef my_tool(param: type) -> return_type:\n    '''Description'''"
    returns: A BaseTool instance.
    description: "Converts a function into a CrewAI tool. The name is the first argument to @tool. The docstring becomes the tool's description for LLM selection. Type hints define the parameter schema for automatic tool call generation."
  - name: Agent.kickoff()
    id: api:crewai/agent-execute
    signature: "agent.execute_task(task: Task, context: str | None) -> str"
    returns: The task output as a string.
    description: "Directly executes a single task on an agent without the crew orchestrator. Useful for testing agents in isolation. context provides additional context from previous tasks. Returns the raw output string."
sections:
  - title: Sequential Research Pipeline
    id: section:crewai/sequential-pipeline
    content: |
      A three-agent crew performing research, writing, and review in sequence:

      ```python
      from crewai import Agent, Task, Crew, Process

      researcher = Agent(
          role="Senior Research Analyst",
          goal="Find comprehensive information on the given topic",
          backstory="Expert at finding and synthesizing information from multiple sources",
          llm="gpt-4o",
          tools=[SerperDevTool(), ScrapeWebsiteTool()]
      )

      writer = Agent(
          role="Content Writer",
          goal="Write a compelling article from the research",
          backstory="Skilled at transforming research into engaging, accurate content",
          llm="gpt-4o-mini"
      )

      reviewer = Agent(
          role="Quality Reviewer",
          goal="Ensure the article meets quality standards and is factually accurate",
          backstory="Detail-oriented editor with expertise in fact-checking",
          llm="gpt-4o"
      )

      research_task = Task(
          description="Research the topic: {topic}. Gather key facts, statistics, and expert opinions.",
          expected_output="A bullet-point research brief with key findings and sources.",
          agent=researcher
      )

      write_task = Task(
          description="Write a 500-word article based on the research provided.",
          expected_output="A well-structured article with introduction, body, and conclusion.",
          agent=writer,
          context=[research_task]
      )

      review_task = Task(
          description="Review the article for accuracy, clarity, and completeness. Suggest improvements.",
          expected_output="A review report with pass/fail and revision suggestions.",
          agent=reviewer,
          context=[write_task]
      )

      crew = Crew(agents=[researcher, writer, reviewer], tasks=[research_task, write_task, review_task], process=Process.sequential, verbose=True)
      result = crew.kickoff(inputs={"topic": "Quantum Computing Applications in Drug Discovery"})
      ```
  - title: Hierarchical Team
    id: section:crewai/hierarchical-team
    content: |
      A manager agent coordinates specialist agents for complex tasks:

      ```python
      manager = Agent(
          role="Project Manager",
          goal="Coordinate specialists to deliver a comprehensive market analysis",
          backstory="Experienced PM with expertise in task breakdown and quality control",
          llm="gpt-4o",
          allow_delegation=True
      )

      analyst = Agent(
          role="Market Analyst", goal="Analyze market data",
          backstory="Data-driven analyst", llm="gpt-4o-mini",
          tools=[SerperDevTool()]
      )

      designer = Agent(
          role="Report Designer", goal="Create visual presentations",
          backstory="Expert at data visualization", llm="gpt-4o-mini"
      )

      crew = Crew(
          agents=[manager, analyst, designer],
          tasks=[Task(description="Complete market analysis for {company}", expected_output="Full analysis report")],
          process=Process.hierarchical,
          manager_llm="gpt-4o",
          verbose=True
      )
      ```

      The manager automatically decomposes the task, delegates to specialists, and synthesizes the final output.
---
