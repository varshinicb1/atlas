use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::collections::{HashSet, VecDeque, HashMap};
use std::time::{SystemTime, UNIX_EPOCH};
use atlas_compiler::Compiler;
use atlas_compiler::frontends::MdDocument;
use atlas_compiler::frontends::decision::DecisionParser;
use atlas_compiler::binary;
use atlas_knowledge::reasoner::Reasoner;
use atlas_knowledge::loader::AtlasBundle;
use atlas_knowledge::{Runtime, DecisionResult};
use atlas_ir::{Workflow, RecommendationItem};
use serde::{Deserialize, Serialize};

mod templates;

#[derive(Parser)]
#[command(name = "atlas", version, about = "Atlas Knowledge Operating System")]
struct Cli {
    #[arg(long, global = true, help = "Output JSON instead of human-readable text")]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compile {
        sources: Vec<PathBuf>,
        #[arg(short, long, default_value = "bundle.atlas")]
        out: PathBuf,
        #[arg(long)]
        emit_ir: bool,
    },
    Solve {
        #[arg(short, long, alias = "from")]
        bundle: Option<PathBuf>,
        query: String,
        #[arg(long)]
        all: bool,
    },
    Decide {
        #[arg(short, long, alias = "from", default_value = "bundle.atlas")]
        bundle: PathBuf,
        query: String,
        #[arg(short = 'c', long, value_parser = parse_key_val)]
        context: Vec<KeyVal>,
    },
    Verify {
        #[arg(short, long, alias = "from", default_value = "bundle.atlas")]
        bundle: PathBuf,
        #[arg(short, long)]
        artifact: Option<String>,
        #[arg(long)]
        policy: Option<String>,
    },
    Install {
        #[arg(short = 'n', long)]
        name: Option<String>,
        sources: Vec<PathBuf>,
    },
    /// Reason over the knowledge graph using a template or LLM
    Reason {
        #[arg(short, long, alias = "from", default_value = "bundle.atlas")]
        bundle: PathBuf,
        query: String,
        #[arg(long)]
        model: Option<String>,
    },
    /// Dump the full IR of an .atlas file as JSON
    Dump {
        #[arg(short, long)]
        bundle: Option<PathBuf>,
        sources: Vec<PathBuf>,
    },
    /// Scaffold a new knowledge package from a built-in template
    Init {
        /// Name of the knowledge package (e.g., "my_project")
        name: String,
                /// Template to use: "generic" (default), "flutter", "rust", "typescript", "python", "typescript_7"
        #[arg(short, long, default_value = "generic")]
        template: String,
        /// Output directory (defaults to ./<name>)
        #[arg(short, long)]
        dir: Option<PathBuf>,
    },
    /// Search the Atlas registry for knowledge packages
    Search {
        /// Search query
        query: String,
        /// Registry URL
        #[arg(long, default_value = "https://atlas-hub-registry.cbvarshini1.workers.dev")]
        registry: String,
    },
    /// Publish a knowledge package to the Atlas registry
    Publish {
        /// Path to the package directory or .md file
        path: PathBuf,
        /// Registry URL
        #[arg(long, default_value = "https://atlas-hub-registry.cbvarshini1.workers.dev")]
        registry: String,
        /// API key for authenticated publish
        #[arg(long)]
        api_key: Option<String>,
        /// Organization scope (empty string for public packages)
        #[arg(long, default_value = "")]
        org: String,
    },
    /// Clone a website as a template knowledge package
    Clone {
        /// URL of the website to clone
        url: String,
        /// Name for the template (defaults to domain name)
        #[arg(short, long)]
        name: Option<String>,
        /// Output directory (defaults to ./<name>)
        #[arg(short, long)]
        dir: Option<PathBuf>,
        /// Also download CSS and JS assets
        #[arg(long, default_value_t = false)]
        assets: bool,
        /// Rebuild as Next.js project skeleton (ai-website-cloner style)
        #[arg(long, default_value_t = false)]
        rebuild: bool,
    },
    /// Crawl documentation from a URL and generate a knowledge package
    Crawl {
        /// URL of the documentation to crawl
        url: String,
        /// Package name (defaults to domain name)
        #[arg(short, long)]
        name: Option<String>,
        /// Output directory (defaults to ./<name>)
        #[arg(short, long)]
        dir: Option<PathBuf>,
        /// Maximum crawl depth (default: 2)
        #[arg(long, default_value_t = 2)]
        depth: usize,
        /// Maximum pages to crawl (default: 50)
        #[arg(long, default_value_t = 50)]
        max_pages: usize,
        /// Auto-publish to registry after crawl
        #[arg(long)]
        publish: bool,
        /// Registry URL for publish
        #[arg(long, default_value = "https://atlas-hub-registry.cbvarshini1.workers.dev")]
        registry: String,
        /// API key for authenticated publish
        #[arg(long)]
        api_key: Option<String>,
        /// Organization scope
        #[arg(long, default_value = "")]
        org: String,
    },
    /// Export an .atlas bundle to LLM-friendly formats (XML/Markdown/JSON)
    Export {
        /// Path to the .atlas bundle
        #[arg(short, long, default_value = "bundle.atlas")]
        bundle: PathBuf,
        /// Output format: xml, markdown, json, plain
        #[arg(short, long, default_value = "xml")]
        style: String,
        /// Output file path (defaults to bundle-export.xml)
        #[arg(short, long)]
        out: Option<PathBuf>,
        /// Show token count estimate
        #[arg(long, default_value_t = false)]
        tokens: bool,
    },
    /// Start a local HTTP API server for AI agents to query knowledge
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
        /// Host to bind to
        #[arg(long, default_value = "0.0.0.0")]
        host: String,
    },
    /// Generate or install agent skills from Atlas packages (agentic-awesome-skills compatible)
    Skill {
        /// Subcommand: generate, install, search, list
        #[command(subcommand)]
        action: SkillAction,
    },
    /// Run a durable AI agent with checkpointing and human-in-the-loop
    Agent {
        #[command(subcommand)]
        action: AgentAction,
    },
}

#[derive(Subcommand)]
enum SkillAction {
    /// Generate a SKILL.md from an Atlas package for AI coding assistants
    Generate {
        /// Path to the .atlas bundle or .md file
        path: PathBuf,
        /// Output path for SKILL.md (defaults to ./SKILL.md)
        #[arg(short, long)]
        out: Option<PathBuf>,
        /// Skill name (defaults to package name)
        #[arg(short, long)]
        name: Option<String>,
        /// Risk level: safe, none, dangerous, risky, critical
        #[arg(long, default_value = "safe")]
        risk: String,
        /// Category tags (comma-separated)
        #[arg(long, default_value = "development")]
        category: String,
    },
    /// Search the agentic-awesome-skills registry for skills
    Search {
        /// Search query
        query: String,
        /// Max results
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },
    /// List all installed skills
    List,
    /// Install skills from agentic-awesome-skills
    Install {
        /// Skill IDs or categories to install
        skills: Vec<String>,
        /// Output directory for skills (defaults to .agents/skills)
        #[arg(short, long)]
        dir: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum AgentAction {
    /// Run a durable agent with checkpointing
    Run {
        /// Task/query for the agent to solve
        task: String,
        /// Knowledge package to use (e.g., "react_patterns")
        #[arg(short, long)]
        package: String,
        /// Maximum steps to run
        #[arg(long, default_value_t = 10)]
        max_steps: usize,
        /// Checkpoint directory for durable execution
        #[arg(long, default_value = ".atlas/checkpoints")]
        checkpoint_dir: PathBuf,
        /// Enable human-in-the-loop at decision nodes
        #[arg(long, default_value_t = false)]
        human_in_loop: bool,
        /// Checkpoint interval (steps)
        #[arg(long, default_value_t = 3)]
        checkpoint_interval: usize,
    },
    /// Resume agent from a checkpoint
    Resume {
        /// Checkpoint ID to resume from
        checkpoint_id: String,
        /// Optional new max steps
        #[arg(long)]
        max_steps: Option<usize>,
        /// Enable human-in-the-loop at decision nodes
        #[arg(long, default_value_t = false)]
        human_in_loop: bool,
    },
    /// List available checkpoints
    List,
}

#[derive(Debug, Clone)]
struct KeyVal {
    key: String,
    value: String,
}

fn parse_key_val(s: &str) -> Result<KeyVal, String> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() != 2 {
        Err("Format: key=value".into())
    } else {
        Ok(KeyVal { key: parts[0].to_string(), value: parts[1].to_string() })
    }
}

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    let cli = Cli::parse();
    match cli.command {
        Commands::Compile { sources, out, emit_ir } => run_compile(sources, &out, emit_ir, cli.json),
        Commands::Solve { bundle, query, all } => {
            run_solve(bundle.as_deref(), &query, all, cli.json)
        }
        Commands::Decide { bundle, query, context } => run_decide(&bundle, &query, &context, cli.json),
        Commands::Verify { bundle, artifact, policy } => run_verify(&bundle, artifact.as_deref(), policy.as_deref(), cli.json),
        Commands::Install { name, sources } => run_install(name, sources, cli.json),
        Commands::Reason { bundle, query, model } => run_reason(&bundle, &query, model, cli.json),
        Commands::Dump { bundle, sources } => run_dump(bundle.as_deref(), sources, cli.json),
        Commands::Init { name, template, dir } => run_init(&name, &template, dir.as_ref()),
        Commands::Search { query, registry } => run_search(&query, &registry, cli.json),
        Commands::Publish { path, registry, api_key, org } => run_publish(path, &registry, api_key.as_deref(), &org, cli.json),
        Commands::Clone { url, name, dir, assets, rebuild } => run_clone(&url, name.as_deref(), dir.as_ref(), assets, rebuild, cli.json),
        Commands::Crawl { url, name, dir, depth, max_pages, publish, registry, api_key, org } => run_crawl(&url, name.as_deref(), dir.as_ref(), depth, max_pages, publish, &registry, api_key.as_deref(), &org, cli.json),
        Commands::Export { bundle, style, out, tokens } => run_export(&bundle, &style, out.as_ref(), tokens, cli.json),
        Commands::Serve { port, host } => run_serve(port, &host),
        Commands::Skill { action } => match action {
            SkillAction::Generate { path, out, name, risk, category } => run_skill_generate(&path, out.as_ref(), name.as_deref(), &risk, &category, cli.json),
            SkillAction::Search { query, limit } => run_skill_search(&query, limit, cli.json),
            SkillAction::List => run_skill_list(cli.json),
            SkillAction::Install { skills, dir } => run_skill_install(&skills, dir.as_ref(), cli.json),
        },
        Commands::Agent { action } => match action {
            AgentAction::Run { task, package, max_steps, checkpoint_dir, human_in_loop, checkpoint_interval } => {
                run_agent_run(&task, &package, max_steps, &checkpoint_dir, human_in_loop, checkpoint_interval, cli.json)
            }
            AgentAction::Resume { checkpoint_id, max_steps, human_in_loop } => {
                run_agent_resume(&checkpoint_id, max_steps, human_in_loop, cli.json)
            }
            AgentAction::List => run_agent_list(cli.json),
        },
    }
}

fn run_compile(sources: Vec<PathBuf>, out: &Path, emit_ir: bool, json: bool) -> Result<(), anyhow::Error> {
    let mut compiler = Compiler::new();

    for source in &sources {
        if source.is_dir() {
            for entry in std::fs::read_dir(source)? {
                let entry = entry?;
                let path = entry.path();
                match path.extension().and_then(|s| s.to_str()) {
                    Some("md") => compile_md(&mut compiler, &path)?,
                    Some("yaml") | Some("yml") => {
                        let trees = DecisionParser::parse_file(&path)?;
                        if !trees.is_empty() {
                            compiler.add_decision_trees(trees);
                        }
                    }
                    _ => {}
                }
            }
        } else {
            match source.extension().and_then(|s| s.to_str()) {
                Some("md") => compile_md(&mut compiler, source)?,
                Some("yaml") | Some("yml") => {
                    let trees = DecisionParser::parse_file(source)?;
                    if !trees.is_empty() {
                        compiler.add_decision_trees(trees);
                    }
                }
                _ => anyhow::bail!("Unsupported source format: {}", source.display()),
            }
        }
    }

    let ir = compiler.build()?;

    if emit_ir {
        let ir_json = serde_json::to_string_pretty(&ir)?;
        let ir_path = out.with_extension("ir.json");
        std::fs::write(&ir_path, &ir_json)?;
        if json {
            println!(r#"{{"ir_written":"{}"}}"#, ir_path.display());
        } else {
            println!("IR written to {}", ir_path.display());
        }
    }

    binary::write_binary(&ir, out)?;
    let tree_count = ir.decision_trees.len();

    if json {
        let output = serde_json::json!({
            "nodes": ir.nodes.len(),
            "edges": ir.edges.len(),
            "decision_trees": tree_count,
            "output": out.to_string_lossy(),
        });
        println!("{}", serde_json::to_string(&output)?);
    } else {
        println!(
            "Compiled {} nodes, {} edges, {} decision trees → {}",
            ir.nodes.len(),
            ir.edges.len(),
            tree_count,
            out.display()
        );
    }
    Ok(())
}

fn compile_md(compiler: &mut Compiler, path: &PathBuf) -> Result<(), anyhow::Error> {
    let source_id = path.file_stem().and_then(|s| s.to_str()).unwrap_or("doc");
    let content = std::fs::read_to_string(path)?;
    let doc = MdDocument::parse_from_str(source_id, path, &content)?;
    let trees = doc.decision_trees().to_vec();
    let (nodes, edges, examples, failures, workflows) = doc.into_parts();
    for node in nodes { compiler.add_node(node)?; }
    for edge in edges { compiler.add_edge(edge)?; }
    for ex in examples { compiler.add_example(ex); }
    for fm in failures { compiler.add_failure(fm); }
    for wf in workflows { compiler.add_workflow(wf); }
    if !trees.is_empty() {
        compiler.add_decision_trees(trees);
    }
    compiler.add_source(source_id, path, Some(&content))?;
    Ok(())
}

#[derive(serde::Serialize)]
struct SolveOutput {
    query: String,
    bundle: String,
    confidence: f32,
    total_matches: usize,
    nodes: Vec<NodeOutput>,
}

#[derive(serde::Serialize)]
struct NodeOutput {
    id: String,
    name: String,
    kind: String,
    description: Option<String>,
    version: Option<String>,
    confidence: f32,
}

fn resolve_bundle(bundle: &Path) -> Result<(PathBuf, String), anyhow::Error> {
    let stem = bundle.file_stem().and_then(|s| s.to_str()).unwrap_or("bundle");
    if bundle.exists() {
        return Ok((bundle.to_path_buf(), stem.to_string()));
    }
    for candidate in [
        format!("{}.atlas", stem),
        format!("packages/{}.atlas", stem),
    ] {
        let p = PathBuf::from(&candidate);
        if p.exists() {
            return Ok((p, stem.to_string()));
        }
    }
    if let Some(data_dir) = dirs::data_dir() {
        let installed = data_dir.join("atlas").join("bundles").join(stem).join("bundle.atlas");
        if installed.exists() {
            return Ok((installed, stem.to_string()));
        }
    }
    for candidate in [
        format!("packages/{}.md", stem),
        format!("{}.md", stem),
    ] {
        let p = PathBuf::from(&candidate);
        if p.exists() {
            let atlas_path = PathBuf::from(format!("{}.atlas", stem));
            let mut compiler = atlas_compiler::Compiler::new();
            compile_md(&mut compiler, &p)?;
            let ir = compiler.build()?;
            binary::write_binary(&ir, &atlas_path)?;
            return Ok((atlas_path, stem.to_string()));
        }
    }
    anyhow::bail!("Bundle '{}' not found. Provide a path to a .atlas or .md file, or a package name.", stem);
}

fn run_solve(bundle_path: Option<&Path>, query: &str, all: bool, json: bool) -> Result<(), anyhow::Error> {
    if query.trim().is_empty() {
        anyhow::bail!("Error: query cannot be empty. Please provide a search query.");
    }

    let mut runtime = atlas_knowledge::Runtime::new();

    if all || bundle_path.is_none() {
        let packages_dir = std::path::Path::new("packages");
        if packages_dir.exists() {
            runtime.load_all(packages_dir)?;
        }
        let atlas_files: Vec<PathBuf> = std::fs::read_dir(".")
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("atlas"))
            .collect();
        for p in atlas_files {
            let _ = runtime.load(&p);
        }
        if let Some(data_dir) = dirs::data_dir() {
            let bundles_dir = data_dir.join("atlas").join("bundles");
            if bundles_dir.exists() {
                if let Ok(entries) = std::fs::read_dir(&bundles_dir) {
                    for entry in entries.flatten() {
                        let bundle_atlas = entry.path().join("bundle.atlas");
                        if bundle_atlas.exists() {
                            let _ = runtime.load(&bundle_atlas);
                        }
                    }
                }
            }
        }
    }

    if all || bundle_path.is_none() {
        if runtime.bundle_count() == 0 {
            anyhow::bail!("No .atlas files found in packages/ or current directory. Run `atlas compile` first or use `--bundle`.");
        }
        let results = runtime.solve_all(query);

        if json {
            let outputs: Vec<serde_json::Value> = results.iter().map(|r| {
                serde_json::json!({
                    "query": r.query,
                    "bundle": r.bundle,
                    "confidence": r.confidence,
                    "total_matches": r.total_matches,
                    "nodes": r.nodes.iter().map(|n| serde_json::json!({
                        "id": n.id,
                        "name": n.name,
                        "kind": format!("{:?}", n.kind),
                        "description": n.description,
                        "version": n.version,
                        "confidence": n.confidence,
                    })).collect::<Vec<_>>(),
                })
            }).collect();
            let total = results.iter().map(|r| r.nodes.len()).sum::<usize>();
            let output = serde_json::json!({
                "query": query,
                "results": outputs,
                "total_nodes": total,
                "bundles_scanned": results.len(),
            });
            println!("{}", serde_json::to_string(&output)?);
            return Ok(());
        }

        let total = results.iter().map(|r| r.nodes.len()).sum::<usize>();
        println!("Query: {}", query);
        println!("Bundles matched: {}", results.len());
        println!("Total nodes: {}\n", total);
        for result in &results {
            println!("── {} (confidence: {:.2}) ──", result.bundle, result.confidence);
            for node in &result.nodes {
                let kind = format!("{:?}", node.kind);
                println!("  [{}] {} ({})", kind, node.name, node.id);
                if let Some(ref desc) = node.description {
                    let snippet: String = desc.chars().take(120).collect();
                    println!("       {}", snippet);
                }
                if let Some(ref ver) = node.version {
                    println!("       v{}", ver);
                }
                println!();
            }
        }
        return Ok(());
    }

    let bundle_path = bundle_path.unwrap();
    let (resolved, _) = resolve_bundle(bundle_path)?;
    let name = runtime.load(&resolved)?;
    let result = runtime.solve(&name, query)?;

    if json {
        let output = SolveOutput {
            query: result.query,
            bundle: result.bundle,
            confidence: result.confidence,
            total_matches: result.total_matches,
            nodes: result.nodes.iter().map(|n| NodeOutput {
                id: n.id.clone(),
                name: n.name.clone(),
                kind: format!("{:?}", n.kind),
                description: n.description.clone(),
                version: n.version.clone(),
                confidence: n.confidence,
            }).collect(),
        };
        println!("{}", serde_json::to_string(&output)?);
        return Ok(());
    }

    println!("Query: {}", result.query);
    println!("Bundle: {}", result.bundle);
    println!("Confidence: {:.2}", result.confidence);
    println!();
    println!("Matched nodes ({}/{}):", result.nodes.len(), result.total_matches);
    println!();
    for node in &result.nodes {
        let kind = format!("{:?}", node.kind);
        println!("  [{}] {} ({})", kind, node.name, node.id);
        if let Some(ref desc) = node.description {
            let snippet: String = desc.chars().take(120).collect();
            println!("       {}", snippet);
        }
        if let Some(ref ver) = node.version {
            println!("       v{}", ver);
        }
        println!();
    }
    Ok(())
}

#[derive(serde::Serialize)]
struct DecideOutput {
    tree_id: String,
    path: Vec<String>,
    rationale: String,
    recommendations: Vec<RecOutput>,
    agent_instructions: Option<String>,
}

#[derive(serde::Serialize)]
struct RecOutput {
    node_id: String,
    confidence: f32,
}

fn run_decide(bundle_path: &Path, query: &str, context: &[KeyVal], json: bool) -> Result<(), anyhow::Error> {
    if query.trim().is_empty() {
        anyhow::bail!("Error: query cannot be empty. Please provide a search query.");
    }
    let (resolved, _) = resolve_bundle(bundle_path)?;
    let mut runtime = atlas_knowledge::Runtime::new();
    let name = runtime.load(&resolved)?;
    let mut ctx = std::collections::HashMap::new();
    for kv in context { ctx.insert(kv.key.clone(), kv.value.clone()); }
    let result = runtime.decide(&name, query, Some(&ctx))?;

    match result {
        Some(r) => {
            if json {
                let output = DecideOutput {
                    tree_id: r.tree_id,
                    path: r.path,
                    rationale: r.rationale,
                    recommendations: r.recommendations.iter().map(|rec| RecOutput {
                        node_id: rec.node_id.clone(),
                        confidence: rec.confidence,
                    }).collect(),
                    agent_instructions: r.agent_instructions,
                };
                println!("{}", serde_json::to_string(&output)?);
            } else {
                println!("Decision tree: {}", r.tree_id);
                println!("Path: {}", r.path.join(" → "));
                println!("Rationale: {}", r.rationale);
                println!();
                println!("Recommendations:");
                for rec in &r.recommendations {
                    println!("  {} (confidence: {:.2})", rec.node_id, rec.confidence);
                }
                if let Some(ref instructions) = r.agent_instructions {
                    println!();
                    println!("Agent instructions: {}", instructions);
                }
            }
        }
        None => {
            if json {
                println!("null");
            } else {
                println!("No matching decision tree found for query.");
            }
        }
    }
    Ok(())
}

fn run_verify(bundle_path: &Path, artifact: Option<&str>, policy: Option<&str>, json: bool) -> Result<(), anyhow::Error> {
    let mut runtime = atlas_knowledge::Runtime::new();
    let name = runtime.load(bundle_path)?;
    let report = if let Some(p) = policy {
        runtime.verify_with_policy(&name, p)?
    } else {
        runtime.verify(&name, artifact)?
    };

    if json {
        println!("{}", serde_json::to_string(&report)?);
        return Ok(());
    }

    println!("Verification report:");
    println!("  Overall: {}", if report.passed { "PASSED" } else { "FAILED" });
    println!();
    for check in &report.checks {
        let status = if check.passed { "✓" } else { "✗" };
        println!("  {} {}: {}", status, check.name, check.message);
    }
    Ok(())
}

fn run_reason(bundle_path: &Path, query: &str, model: Option<String>, json: bool) -> Result<(), anyhow::Error> {
    if query.trim().is_empty() {
        anyhow::bail!("Error: query cannot be empty. Please provide a search query.");
    }
    let (resolved, _) = resolve_bundle(bundle_path)?;
    let mut runtime = atlas_knowledge::Runtime::new();
    let name = runtime.load(&resolved)?;
    let solve_result = runtime.solve(&name, query)?;
    
    // Also try decision trees
    let context = std::collections::HashMap::new();
    let decide_result = runtime.decide(&name, query, Some(&context)).ok().and_then(|r| r);
    
    let ctx = atlas_knowledge::reasoner::ReasonContext {
        query,
        bundle: &solve_result.bundle,
        confidence: solve_result.confidence,
        nodes: &solve_result.nodes,
        total_matches: solve_result.total_matches,
        decide_result: decide_result.as_ref(),
    };
    
    let answer = if let Some(m) = model {
        let reasoner = atlas_knowledge::reasoner::OllamaReasoner::new(&m, "http://localhost:11434");
        reasoner.reason(query, &ctx)?
    } else {
        let reasoner = atlas_knowledge::reasoner::TemplateReasoner;
        reasoner.reason(query, &ctx)?
    };
    
    if json {
        let output = serde_json::json!({
            "query": query,
            "answer": answer,
        });
        println!("{output}");
    } else {
        println!("{answer}");
    }
    Ok(())
}

fn run_dump(bundle_path: Option<&Path>, sources: Vec<PathBuf>, json: bool) -> Result<(), anyhow::Error> {
    let bundle_path = match bundle_path {
        Some(p) => p.to_path_buf(),
        None if sources.is_empty() => PathBuf::from("bundle.atlas"),
        None => {
            let single = &sources[0];
            let ext = single.extension().and_then(|s| s.to_str());
            if sources.len() == 1 && ext == Some("atlas") {
                single.clone()
            } else if sources.len() == 1 && ext == Some("md") {
                let stem = single.file_stem().and_then(|s| s.to_str()).unwrap_or("bundle");
                let out_path = PathBuf::from(format!("{}.atlas", stem));
                run_compile(vec![single.clone()], &out_path, false, false)?;
                out_path
            } else {
                let out_path = PathBuf::from("bundle.atlas");
                run_compile(sources, &out_path, false, false)?;
                out_path
            }
        }
    };

    let mut runtime = atlas_knowledge::Runtime::new();
    let name = runtime.load(&bundle_path)?;
    let bundle = runtime.get(&name)
        .ok_or_else(|| anyhow::anyhow!("Bundle not loaded"))?;
    if json {
        println!("{}", serde_json::to_string(&bundle.ir)?);
    } else {
        let ir = &bundle.ir;
        println!("Bundle: {}", name);
        println!("Schema version: {}", ir.meta.schema_version);
        println!("Nodes: {}", ir.nodes.len());
        println!("Edges: {}", ir.edges.len());
        println!("Decision trees: {}", ir.decision_trees.len());
        println!("Examples: {}", ir.examples.len());
        println!("Failure modes: {}", ir.failure_modes.len());
        println!("Verification rules: {}", ir.verification_rules.len());
    }
    Ok(())
}

fn run_init(name: &str, template: &str, dir: Option<&PathBuf>) -> Result<(), anyhow::Error> {
    let target = dir.cloned().unwrap_or_else(|| PathBuf::from(name));
    if target.exists() {
        anyhow::bail!("Directory '{}' already exists", target.display());
    }
    std::fs::create_dir_all(&target)?;
    std::fs::create_dir_all(target.join("decisions"))?;

    let title = name.replace(['-', '_'], " ");
    let md_content = match template {
        "flutter" => templates::FLUTTER_PACKAGE_MD,
        "rust" => templates::RUST_PACKAGE_MD,
        "typescript" => templates::TYPESCRIPT_PACKAGE_MD,
        "python" => templates::PYTHON_PACKAGE_MD,
        "typescript_7" | "ts7" => templates::TYPESCRIPT_7_PACKAGE_MD,
        _ => templates::KNOWLEDGE_PACKAGE_MD,
    };
    let md_content = md_content.replace("__NAME__", name).replace("__TITLE__", &title);
    std::fs::write(target.join(format!("{name}.md")), &md_content)?;

    let yaml_content = templates::DECISION_TREE_YAML.replace("__NAME__", name);
    std::fs::write(target.join("decisions").join(format!("{name}.yaml")), &yaml_content)?;

    std::fs::write(target.join(".gitignore"), templates::GITIGNORE)?;

    println!("Created knowledge package '{}' in ./{}", name, target.display());
    println!("  {}/{}.md   - knowledge document (edit this!)", target.display(), name);
    println!("  {}/decisions/{}.yaml - decision trees (edit this!)", target.display(), name);
    println!("  {}/.gitignore", target.display());
    println!();
    println!("Next steps:");
    println!("  cd {}", target.display());
    println!("  # Edit {}.md with your concepts, APIs, and workflows", name);
    println!("  # Edit decisions/{}.yaml with decision trees", name);
    println!("  atlas compile {}.md decisions/{}.yaml --out {}.atlas", name, name, name);
    println!("  atlas solve --bundle {}.atlas \"your question\"", name);
    Ok(())
}

fn run_search(query: &str, registry: &str, json: bool) -> Result<(), anyhow::Error> {
    let url = format!("{}/api/search?q={}", registry, urlencode(query));
    match ureq::get(&url).call() {
        Ok(r) => {
            let body = r.into_body().read_to_string()?;
            if json {
                println!("{}", body);
            } else {
                let parsed: serde_json::Value = serde_json::from_str(&body).unwrap_or(serde_json::Value::String(body.clone()));
                if let Some(results) = parsed.as_array() {
                    println!("Search results for '{}':", query);
                    for r in results {
                        let name = r.get("name").and_then(|v| v.as_str()).unwrap_or("?");
                        let desc = r.get("description").and_then(|v| v.as_str()).unwrap_or("");
                        let ver = r.get("version").and_then(|v| v.as_str()).unwrap_or("");
                        println!("  {} v{} - {}", name, ver, desc);
                    }
                } else {
                    println!("{}", body);
                }
            }
        }
        Err(e) => {
            if json {
                println!(r#"{{"error":"Registry not reachable","registry":"{}","detail":"{}"}}"#, registry, e);
            } else {
                println!("Warning: Registry at {} is not reachable ({})", registry, e);
                println!("Showing local packages:");
            }
            let data_dir = dirs::data_dir()
                .ok_or_else(|| anyhow::anyhow!("Cannot determine data directory"))?;
            let atlas_dir = data_dir.join("atlas").join("bundles");
            if atlas_dir.exists() {
                for entry in std::fs::read_dir(&atlas_dir)? {
                    let entry = entry?;
                    if entry.path().is_dir() {
                        let name = entry.file_name();
                        if json {
                            println!(r#"{{"name":"{}","source":"local"}}"#, name.to_string_lossy());
                        } else {
                            println!("  {} (local)", name.to_string_lossy());
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn run_publish(path: PathBuf, registry: &str, api_key: Option<&str>, org: &str, json: bool) -> Result<(), anyhow::Error> {
    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }

    let name = if path.is_dir() {
        path.file_name().and_then(|s| s.to_str()).unwrap_or("package").to_string()
    } else {
        path.file_stem().and_then(|s| s.to_str()).unwrap_or("package").to_string()
    };

    let mut files: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
    let mut description = String::new();

    if path.is_dir() {
        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;
            let p = entry.path();
            if p.extension().is_some_and(|e| e == "md" || e == "yaml" || e == "yml") {
                let filename = p.file_name().unwrap().to_string_lossy().to_string();
                let text = std::fs::read_to_string(&p)?;
                if description.is_empty() && p.extension().is_some_and(|e| e == "md") {
                    let lines: Vec<&str> = text.lines().collect();
                    if lines.len() >= 3 && lines[0].trim() == "---" {
                        description = lines[1..].iter().find_map(|l| {
                            let l = l.trim();
                            l.strip_prefix("purpose:").map(|v| v.trim().trim_matches('"').to_string())
                        }).unwrap_or_default();
                    }
                    if description.is_empty() {
                        for line in &lines {
                            let line = line.trim();
                            if !line.is_empty() && !line.starts_with('#') && line != "---" {
                                description = line.to_string();
                                break;
                            }
                        }
                    }
                }
                files.insert(filename, text);
            }
        }
    } else {
        let text = std::fs::read_to_string(&path)?;
        let filename = path.file_name().unwrap().to_string_lossy().to_string();
        let all_lines: Vec<&str> = text.lines().collect();
        if all_lines.len() >= 3 && all_lines[0].trim() == "---" {
            description = all_lines[1..].iter().find_map(|l| {
                let l = l.trim();
                l.strip_prefix("purpose:").map(|v| v.trim().trim_matches('"').to_string())
            }).unwrap_or_default();
        }
        if description.is_empty() {
            for line in &all_lines {
                let line = line.trim();
                if !line.is_empty() && !line.starts_with('#') && line != "---" {
                    description = line.to_string();
                    break;
                }
            }
        }
        files.insert(filename, text);
    }

    let version = files.values().find_map(|text| {
        let lines: Vec<&str> = text.lines().collect();
        if lines.len() >= 2 && lines[0].trim() == "---" {
            lines[1..].iter().find_map(|l| {
                let l = l.trim();
                l.strip_prefix("version:").map(|v| v.trim().trim_matches('"').to_string())
            })
        } else {
            None
        }
    }).unwrap_or_else(|| "0.1.0".to_string());

    let tags: Vec<String> = files.values().find_map(|text| {
        let lines: Vec<&str> = text.lines().collect();
        if lines.len() >= 2 && lines[0].trim() == "---" {
            lines[1..].iter().find_map(|l| {
                let l = l.trim();
                if let Some(val) = l.strip_prefix("tags:") {
                    let val = val.trim();
                    if val.starts_with('[') {
                        Some(val.trim_matches(|c| c == '[' || c == ']').split(',')
                            .map(|t| t.trim().trim_matches('"').to_string())
                            .filter(|t| !t.is_empty())
                            .collect())
                    } else {
                        Some(vec![])
                    }
                } else {
                    None
                }
            })
        } else {
            None
        }
    }).unwrap_or_default();

    let body = serde_json::json!({
        "name": name,
        "version": version,
        "description": description,
        "tags": tags,
        "files": files,
        "org": org,
    });

    let url = format!("{}/api/v1/publish", registry);
    let mut req = ureq::post(&url)
        .header("Content-Type", "application/json");
    if let Some(key) = api_key {
        req = req.header("X-API-Key", key);
    }
    match req.send_json(&body)
    {
        Ok(r) => {
            let result = r.into_body().read_to_string()?;
            if json {
                let parsed: serde_json::Value = serde_json::from_str(&result).unwrap_or(serde_json::Value::String(result));
                println!(r#"{{"published":true,"result":{}}}"#, parsed);
            } else {
                println!("Published: {} ({})", name, result);
            }
        }
        Err(e) => {
            anyhow::bail!("Failed to publish to {}: {}", registry, e);
        }
    }
    Ok(())
}

fn run_clone(url: &str, name: Option<&str>, dir: Option<&PathBuf>, assets: bool, rebuild: bool, json: bool) -> Result<(), anyhow::Error> {
    // Basic URL validation and domain extraction
    let url_lower = url.to_lowercase();
    let domain = if url_lower.starts_with("http://") || url_lower.starts_with("https://") {
        let without_proto = url.trim_start_matches("http://").trim_start_matches("https://");
        without_proto.split('/').next().unwrap_or("unknown")
    } else {
        "unknown"
    };
    let name = name.unwrap_or(domain);
    let target = dir.cloned().unwrap_or_else(|| PathBuf::from(name));

    if target.exists() {
        anyhow::bail!("Directory '{}' already exists", target.display());
    }

    let html_dir = target.join("template");
    std::fs::create_dir_all(&html_dir)?;

    println!("Cloning {} → {}", url, target.display());

    let response = ureq::get(url)
        .header("User-Agent", "AtlasClone/1.0")
        .call()
        .map_err(|e| anyhow::anyhow!("Failed to fetch {}: {}", url, e))?;

    let status = response.status();
    if status != 200 {
        anyhow::bail!("HTTP {} from {}", status, url);
    }

    let body = response.into_body().read_to_string()?;
    let html_path = html_dir.join("index.html");
    std::fs::write(&html_path, &body)?;
    let html_size = body.len();

    if assets {
        let mut asset_count = 0u32;
        for line in body.lines() {
            for (prefix, ext) in [("src=\"", "js"), ("href=\"", "css")] {
                let mut search_start = 0;
                while let Some(start) = line[search_start..].find(prefix) {
                    let abs_start = search_start + start + prefix.len();
                    if let Some(end) = line[abs_start..].find('"') {
                        let asset_url = &line[abs_start..abs_start + end];
                        if !asset_url.starts_with("http") {
                            search_start = abs_start + end;
                            continue;
                        }
                        if let Ok(asset_body) = ureq::get(asset_url)
                            .header("User-Agent", "AtlasClone/1.0")
                            .call()
                            .map(|r| r.into_body().read_to_string().unwrap_or_default())
                        {
                            if !asset_body.is_empty() {
                                let filename = format!("asset-{}.{}", asset_count, ext);
                                std::fs::write(html_dir.join(&filename), &asset_body)?;
                                asset_count += 1;
                            }
                        }
                        search_start = abs_start + end;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    // Generate metadata knowledge package
    if rebuild {
        let src_dir = target.join("src");
        let app_dir = src_dir.join("app");
        let components_dir = src_dir.join("components");
        let ui_dir = components_dir.join("ui");
        let public_dir = target.join("public");
        let images_dir = public_dir.join("images");
        let docs_dir = target.join("docs");

        for d in [&src_dir, &app_dir, &components_dir, &ui_dir, &public_dir, &images_dir, &docs_dir] {
            std::fs::create_dir_all(d)?;
        }

        // layout.tsx
        std::fs::write(
            app_dir.join("layout.tsx"),
            r#"import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Cloned Site",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
"#,
        )?;
        std::fs::write(
            app_dir.join("page.tsx"),
            "export default function Home() {\n  return <main>Cloned from {url}</main>;\n}\n",
        )?;
        std::fs::write(
            src_dir.join("globals.css"),
            "@import \"tailwindcss\";\n\n:root {\n  /* Design tokens extracted from cloned site */\n}\n",
        )?;
        std::fs::write(
            target.join("tailwind.config.ts"),
            r#"import type { Config } from "tailwindcss";
export default {
  content: ["./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {},
      fontFamily: {},
    },
  },
  plugins: [],
} satisfies Config;
"#,
        )?;
        std::fs::write(
            target.join("tsconfig.json"),
            r#"{
  "compilerOptions": {
    "target": "ES2017",
    "lib": ["dom", "dom.iterable", "esnext"],
    "allowJs": true,
    "module": "esnext",
    "moduleResolution": "bundler",
    "jsx": "preserve",
    "strict": true,
    "paths": { "@/*": ["./src/*"] }
  },
  "include": ["next-env.d.ts", "**/*.ts", "**/*.tsx", ".next/types/**/*.ts"],
  "exclude": ["node_modules"]
}
"#,
        )?;
        std::fs::write(
            target.join("package.json"),
            &format!(
                r#"{{
  "name": "{name}",
  "version": "0.1.0",
  "private": true,
  "scripts": {{
    "dev": "next dev",
    "build": "next build",
    "start": "next start"
  }},
  "dependencies": {{
    "next": "^16.0.0",
    "react": "^19.0.0",
    "react-dom": "^19.0.0"
  }},
  "devDependencies": {{
    "@types/node": "^22.0.0",
    "@types/react": "^19.0.0",
    "tailwindcss": "^4.0.0",
    "typescript": "^5.7.0"
  }}
}}
"#,
                name = name
            ),
        )?;
        // specs directory for component specifications
        let specs_dir = docs_dir.join("research").join("components");
        std::fs::create_dir_all(&specs_dir)?;
        std::fs::write(
            specs_dir.join("overview.md"),
            format!(
                "# Component Specifications for {name}\n\nCloned from {url}\n\n## Extracted Sections\n\n1. Header/Navigation\n2. Hero Section\n3. Content Sections\n4. Footer\n\nSee individual spec files for exact computed values.\n"
            ),
        )?;
    }

    let title = name.replace(['-', '_', '.'], " ");
    let title = title.split_whitespace().map(|w| {
        let mut c = w.chars();
        match c.next() {
            Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
            None => String::new(),
        }
    }).collect::<Vec<_>>().join(" ");

    let today = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| {
            let secs = d.as_secs();
            let days = secs / 86400;
            let y = 1970 + (days as f64 / 365.25) as u64;
            format!("{}", y)
        })
        .unwrap_or_else(|_| "2026".to_string());

    let md = format!(
r#"---
kind: Package
id: package:{name}
name: "{title}"
version: "1.0"
purpose: "Cloned website template from {domain}"
problem_solved: "Provides a reference template of the website at {url} for AI agents to reuse design and architecture patterns."
install: ""
dependencies: []

concepts:
  - name: Website Template
    id: concept:{name}/template
    description: "The cloned HTML template from {domain} including original structure and styling."
  - name: Design Patterns
    id: concept:{name}/design
    description: "Visual and layout design patterns extracted from {domain}."
  - name: Architecture
    id: concept:{name}/architecture
    description: "Technical architecture and stack inferred from {domain}."

apis:
  - name: template/index.html
    id: api:{name}/html
    signature: "file: template/index.html ({html_size} bytes)"
    returns: "HTML document"
    description: "The main HTML document of the cloned website, preserving original structure."

examples:
  - id: example:{name}/clone
    description: "Website cloned from {url} in {today}. The template is available in the template/ directory."

failures:
  - id: failure:{name}/stale
    symptom: Website content changes over time as the original is updated.
    cause: The template is a point-in-time snapshot and does not auto-sync.
    fix: Re-run atlas clone to refresh the template.
---
# {title}

Website template cloned from [{url}]({url}) in {today}.
"#, name=name, title=title, domain=domain, url=url, html_size=html_size, today=today);

    let md_path = target.join(format!("{}.md", name));
    std::fs::write(&md_path, &md)?;

    std::fs::write(target.join(".gitignore"), "template/\n")?;

    if json {
        let output = serde_json::json!({
            "url": url,
            "name": name,
            "path": target.to_string_lossy(),
            "html_size": html_size,
            "assets_downloaded": assets,
        });
        println!("{}", serde_json::to_string(&output)?);
    } else {
        println!("✅ Cloned {} ({})", url, target.display());
        println!("   template/index.html - main HTML ({} bytes)", html_size);
        println!("   {}.md - knowledge package metadata", name);
        println!();
        println!("Next: atlas compile {}.md --out {}.atlas", target.display(), target.display());
    }
    Ok(())
}

fn run_crawl(url: &str, name: Option<&str>, dir: Option<&PathBuf>, depth: usize, max_pages: usize, publish: bool, registry: &str, api_key: Option<&str>, org: &str, json: bool) -> Result<(), anyhow::Error> {
    use scraper::{Html, Selector};
    use anyhow::Context;

    let base = url::Url::parse(url).with_context(|| format!("Invalid URL: {url}"))?;
    let domain = base.host_str().unwrap_or("unknown").to_string();
    let pkg_name = name.unwrap_or(&domain).to_string();
    let out_dir = dir.map(|d| d.to_path_buf()).unwrap_or_else(|| PathBuf::from(&pkg_name));
    std::fs::create_dir_all(&out_dir)?;

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((url.to_string(), 0usize));

    if depth > 0 {
        try_discover_llms_txt(&base, &mut queue);
    }

    let mut all_text = String::new();
    let mut all_headings: Vec<String> = Vec::new();
    let mut all_code: Vec<String> = Vec::new();
    let mut all_warnings: Vec<(String, String)> = Vec::new();
    let mut pages_crawled = 0usize;

    while let Some((page_url, page_depth)) = queue.pop_front() {
        if visited.contains(&page_url) || pages_crawled >= max_pages { continue; }

        eprintln!("Crawling: {page_url}");
        visited.insert(page_url.clone());

        let resp = match ureq::get(&page_url)
            .call()
        {
            Ok(r) => r,
            Err(e) => { eprintln!("  skip {page_url}: {e}"); continue; }
        };
        let ct_header = resp.headers().get("content-type");
        let content_type = ct_header.and_then(|v| v.to_str().ok()).unwrap_or("");
        if !content_type.contains("text/html") && !content_type.contains("text/plain") && !content_type.contains("application/xhtml") { continue; }
        let is_llms = page_url.contains("/llms.txt") || page_url.contains("llms-full.txt");
        let body: String = match resp.into_body().read_to_string() {
            Ok(b) => b,
            Err(e) => { eprintln!("  skip {page_url}: {e}"); continue; }
        };
        pages_crawled += 1;

        if is_llms {
            let sections = parse_llms_txt(&body);
            for (heading, content) in &sections {
                if !all_headings.contains(heading) {
                    all_headings.push(heading.clone());
                }
                all_text.push_str(&format!("## {heading}\n\n{content}\n\n"));
            }
            if sections.is_empty() {
                all_text.push_str(&body);
            }
            continue;
        }

        let doc = Html::parse_document(&body);

        let title_sel = Selector::parse("title").unwrap();
        if let Some(t) = doc.select(&title_sel).next() {
            let t_raw = t.text().collect::<String>();
            let t_text = t_raw.trim();
            if !t_text.is_empty() && !all_text.contains(t_text) {
                all_text.push_str(&format!("# {}\n\n", t_text));
            }
        }

        let meta_sel = Selector::parse(r#"meta[name="description"]"#).unwrap();
        let og_sel = Selector::parse(r#"meta[property="og:description"]"#).unwrap();
        for sel in &[&meta_sel, &og_sel] {
            if let Some(m) = doc.select(sel).next() {
                if let Some(desc) = m.value().attr("content") {
                    let d = desc.trim();
                    if !d.is_empty() && !all_text.contains(d) {
                        all_text.push_str(&format!("{d}\n\n"));
                    }
                }
            }
        }

        let all_body_sel = Selector::parse("main, article, .content, .documentation, .docs, .prose, .markdown, .post, [role=main]").unwrap();
        let body_container = doc.select(&all_body_sel).next().unwrap_or(doc.root_element());

        let para_sel = Selector::parse("p, li, td").unwrap();
        for para in body_container.select(&para_sel) {
            let text = para.text().collect::<String>().trim().to_string();
            if text.len() > 20 && !all_text.contains(&text) {
                all_text.push_str(&format!("{text}\n\n"));
            }
        }

        for h_tag in ["h1", "h2", "h3"] {
            let sel = Selector::parse(h_tag).unwrap();
            for heading in body_container.select(&sel) {
                let text = heading.text().collect::<String>().trim().to_string();
                if !text.is_empty() && !all_headings.contains(&text) {
                    all_headings.push(text.clone());
                    all_text.push_str(&format!("## {text}\n\n"));
                }
            }
        }

        let code_sel = Selector::parse("pre code, code").unwrap();
        for code_block in doc.select(&code_sel) {
            let text = code_block.text().collect::<String>().trim().to_string();
            if text.len() > 20 && !all_code.contains(&text) {
                all_code.push(text.clone());
                all_text.push_str(&format!("```\n{text}\n```\n\n"));
            }
        }

        let warn_selectors = [
            Selector::parse(r#"[class*="warning"]"#).unwrap(),
            Selector::parse(r#"[class*="note"]"#).unwrap(),
            Selector::parse(r#"[class*="caution"]"#).unwrap(),
            Selector::parse(r#"[class*="danger"]"#).unwrap(),
            Selector::parse("blockquote").unwrap(),
        ];
        for warn_sel in &warn_selectors {
            for el in doc.select(warn_sel) {
                let text = el.text().collect::<String>().trim().to_string();
                if text.len() > 30 && !all_warnings.iter().any(|(s,_)| s == &text) {
                    let class_name = el.value().classes().next().unwrap_or("note");
                    all_warnings.push((text.clone(), class_name.to_string()));
                    all_text.push_str(&format!("> **{class_name}**: {text}\n\n"));
                }
            }
        }

        if page_depth < depth {
            let link_sel = Selector::parse("a[href]").unwrap();
            for link in doc.select(&link_sel) {
                if let Some(href) = link.value().attr("href") {
                    if let Ok(abs_url) = base.join(href) {
                        if abs_url.host_str() == Some(&domain)
                            && !abs_url.as_str().contains('#')
                            && !abs_url.as_str().contains("mailto:")
                            && !abs_url.as_str().contains("tel:")
                            && !is_locale_variant(&base, &abs_url)
                        {
                            let clean = normalize_url(&abs_url);
                            if !visited.contains(&clean) {
                                queue.push_back((clean, page_depth + 1));
                            }
                        }
                    }
                }
            }
        }
    }

    if pages_crawled == 0 {
        anyhow::bail!("No pages crawled from {url}");
    }

    let concept_count = all_headings.len();
    let api_count = all_code.len();
    let failure_count = all_warnings.len();

    fn esc(val: &str) -> String {
        val.replace('\\', "\\\\").replace('"', "\\\"")
    }
    fn yaml_block(val: &str) -> String {
        val.trim().lines().map(|l| format!("      {l}")).collect::<Vec<_>>().join("\n")
    }

    let max_yaml_items = 100;
    let concepts_yaml = all_headings.iter()
        .take(max_yaml_items)
        .enumerate()
        .map(|(i, h)| format!(r#"  - name: "{}"
    id: concept:page_{i}_{pkg_name}
    description: |
{}"#, esc(h), yaml_block(&format!("Extracted from documentation: {h}"))))
        .collect::<Vec<_>>()
        .join("\n");

    let apis_yaml = all_code.iter()
        .filter(|c| c.len() < 500)
        .take(max_yaml_items)
        .enumerate()
        .map(|(i, c)| {
            let name = c.lines().next().unwrap_or(c).chars().take(60).collect::<String>();
            format!(r#"  - name: "{}"
    id: api:crawl_{i}_{pkg_name}
    signature: |
{}
    returns: See documentation
    description: |
      Extracted code example from documentation."#, esc(&name), yaml_block(c))
        })
        .collect::<Vec<_>>()
        .join("\n");

    let failures_yaml = all_warnings.iter()
        .take(max_yaml_items)
        .enumerate()
        .map(|(i, (text, cls))| {
            let symptom = text.chars().take(80).collect::<String>();
            format!(r#"  - id: failure:crawl_{i}_{pkg_name}
    symptom: |
{}
    cause: |
      {cls} issue
    fix: |
      Refer to official documentation for resolution."#, yaml_block(&symptom))
        })
        .collect::<Vec<_>>()
        .join("\n");

    let package_md = format!(r#"---
kind: Package
id: package:{pkg_name}
name: "{pkg_name} Knowledge Package"
version: "0.1.0"
purpose: |
  Auto-generated knowledge package crawled from {url}.
  Covers {pages_crawled} pages of documentation.
problem_solved: |
  Provides structured knowledge extracted from the official {domain} documentation
  for use in AI agent decision-making.
install: |
  ```bash
  atlas install {pkg_name}.md
  ```
concepts:
{concepts_yaml}
apis:
{apis_yaml}
failures:
{failures_yaml}
---

# {pkg_name}

Auto-generated knowledge package crawled from {url}.

**Pages crawled**: {pages_crawled}
**Source**: {url}

{all_text}
"#);

    let md_path = out_dir.join(format!("{pkg_name}.md"));
    let max_bytes = 400 * 1024;
    let final_md = if package_md.len() > max_bytes {
        let mut front_bytes = 0usize;
        if let Some(end) = package_md.find("\n---\n") {
            if end > 0 { front_bytes = end + 5; }
        }
        let body_keep = max_bytes.saturating_sub(front_bytes);
        let truncated_body: String = package_md[front_bytes..].chars().take(body_keep).collect();
        let mut result = String::with_capacity(front_bytes + 100 + truncated_body.len());
        result.push_str(&package_md[..front_bytes]);
        result.push_str("\n\n_Content truncated at 400KB._\n\n");
        result.push_str(&truncated_body);
        result
    } else {
        package_md
    };
    std::fs::write(&md_path, &final_md)?;

    let size_kb = final_md.len() / 1024;

    if json {
        let output = serde_json::json!({
            "package": pkg_name,
            "pages_crawled": pages_crawled,
            "concepts": concept_count,
            "apis": api_count,
            "failures": failure_count,
            "output": md_path.to_string_lossy(),
            "size_kb": size_kb,
        });
        println!("{}", serde_json::to_string(&output)?);
    } else {
        println!("✅ Crawled {pages_crawled} pages from {url}");
        println!("   {concept_count} concepts, {api_count} code snippets, {failure_count} warnings");
        println!("   Package written to {}", md_path.display());
        println!("   Size: {size_kb}KB");
        println!();
    }

    if publish {
        let compile_out = out_dir.join("bundle.atlas");
        run_compile(vec![md_path.clone()], &compile_out, false, false)?;
        run_verify(&compile_out, None, None, false)?;
        run_publish(compile_out, registry, api_key, org, false)?;
    }

    Ok(())
}

fn is_locale_variant(base: &url::Url, other: &url::Url) -> bool {
    let base_segs: Vec<&str> = base.path_segments().map(|s| s.collect()).unwrap_or_default();
    let other_segs: Vec<&str> = other.path_segments().map(|s| s.collect()).unwrap_or_default();
    let locales = ["de","es","fr","it","pt","ru","ja","ko","zh","ar","hi","nl","pl","tr","vi","th","sv","da","fi","nb","cs","hu","ro","uk","el","he","id","ms","tl","bn","ta","te","mr","ne","gu","kn","ml","si","my","km","lo","ka","hy","az","eu","gl","ca","sr","hr","sl","sk","lt","lv","et","is","mt","sq","mk","bg","en"];
    let base_locale = base_segs.first().and_then(|s| if locales.contains(s) { Some(*s) } else { None });
    let other_locale = other_segs.first().and_then(|s| if locales.contains(s) { Some(*s) } else { None });
    match (base_locale, other_locale) {
        (Some(bl), Some(ol)) => bl != ol,
        (None, Some(_)) => true,
        _ => false,
    }
}

fn try_discover_llms_txt(base: &url::Url, queue: &mut VecDeque<(String, usize)>) {
    for path in &["/llms.txt", "/llms-full.txt"] {
        if let Ok(u) = base.join(path) {
            let s = u.as_str().to_string();
            queue.push_back((s, 0));
        }
    }
}

fn parse_llms_txt(body: &str) -> Vec<(String, String)> {
    let mut sections = Vec::new();
    let mut current_heading = String::new();
    let mut current_content = String::new();

    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            if !current_heading.is_empty() {
                sections.push((current_heading.clone(), current_content.clone()));
            }
            current_heading = trimmed.trim_start_matches('#').trim().to_string();
            current_content.clear();
        } else {
            if !current_content.is_empty() { current_content.push('\n'); }
            current_content.push_str(line);
        }
    }
    if !current_heading.is_empty() {
        sections.push((current_heading, current_content));
    }
    sections
}

fn normalize_url(url: &url::Url) -> String {
    let mut s = url.as_str().trim_end_matches('/').to_string();
    if s.ends_with("/index.html") {
        s = s.trim_end_matches("/index.html").to_string();
    }
    s
}

fn run_export(bundle: &Path, style: &str, out: Option<&PathBuf>, tokens: bool, json: bool) -> Result<(), anyhow::Error> {
    let bundle_data = AtlasBundle::from_file(bundle)?;
    let ir = bundle_data.ir;
    let out_path = out.map(|p| p.clone()).unwrap_or_else(|| {
        let stem = bundle.file_stem().and_then(|s| s.to_str()).unwrap_or("bundle");
        PathBuf::from(format!("{}-export.{}", stem, if style == "json" { "json" } else if style == "markdown" { "md" } else { "xml" }))
    });

    let mut output = String::new();

    match style {
        "xml" => {
            output.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>
<atlas-export>
  <metadata>
    <nodes>"#);
            output.push_str(&ir.nodes.len().to_string());
            output.push_str(r#"</nodes>
    <edges>"#);
            output.push_str(&ir.edges.len().to_string());
            output.push_str(r#"</edges>
    <decision-trees>"#);
            output.push_str(&ir.decision_trees.len().to_string());
            output.push_str(r#"</decision-trees>
  </metadata>
  <concepts>
"#);
            for node in &ir.nodes {
                let desc = node.description.as_deref().unwrap_or("");
                output.push_str(&format!(r#"    <concept id="{}" kind="{}">
      <name>{}</name>
      <description>{}</description>
    </concept>
"#, node.id, node.kind, node.name, desc));
            }
            output.push_str(r#"  </concepts>
  <edges>
"#);
            for edge in &ir.edges {
                output.push_str(&format!(r#"    <edge source="{}" target="{}" kind="{}" />
"#, edge.src, edge.dst, edge.edge_type));
            }
            output.push_str(r#"  </edges>
  <decision-trees>
"#);
            for tree in &ir.decision_trees {
                output.push_str(&format!(r#"    <tree id="{}">
      <nodes>{}</nodes>
    </tree>
"#, tree.id, tree.nodes.len()));
            }
            output.push_str(r#"  </decision-trees>
</atlas-export>"#);
        }
        "json" => {
            let export = serde_json::json!({
                "metadata": {
                    "nodes": ir.nodes.len(),
                    "edges": ir.edges.len(),
                    "decision_trees": ir.decision_trees.len(),
                },
                "concepts": ir.nodes.iter().map(|n| serde_json::json!({
                    "id": n.id,
                    "kind": n.kind.to_string(),
                    "name": n.name,
                    "description": n.description,
                })).collect::<Vec<_>>(),
                "edges": ir.edges.iter().map(|e| serde_json::json!({
                    "source": e.src,
                    "target": e.dst,
                    "kind": e.edge_type.to_string(),
                })).collect::<Vec<_>>(),
            });
            output = serde_json::to_string_pretty(&export)?;
        }
        _ => {
            output.push_str(&format!("# Atlas Knowledge Export: {}\n\n", bundle.display()));
            output.push_str(&format!("- **Nodes:** {}\n", ir.nodes.len()));
            output.push_str(&format!("- **Edges:** {}\n", ir.edges.len()));
            output.push_str(&format!("- **Decision Trees:** {}\n\n", ir.decision_trees.len()));
            output.push_str("## Concepts\n\n");
            for node in &ir.nodes {
                let desc = node.description.as_deref().unwrap_or("");
                output.push_str(&format!("- **{}** (`{}`): {}\n", node.name, node.id, desc));
            }
        }
    }

    std::fs::write(&out_path, &output)?;

    let token_estimate = if tokens { Some(output.len() / 4) } else { None };

    if json {
        let mut result = serde_json::json!({
            "export_path": out_path.to_string_lossy(),
            "format": style,
            "nodes": ir.nodes.len(),
            "edges": ir.edges.len(),
        });
        if let Some(t) = token_estimate {
            result["token_estimate"] = serde_json::json!(t);
        }
        println!("{}", serde_json::to_string(&result)?);
    } else {
        println!("✅ Exported {} nodes, {} edges → {}", ir.nodes.len(), ir.edges.len(), out_path.display());
        if let Some(t) = token_estimate {
            println!("   Estimated tokens: ~{}", t);
        }
        println!("   Next: feed this file to your AI coding assistant");
    }
    Ok(())
}

fn run_skill_generate(path: &PathBuf, out: Option<&PathBuf>, name: Option<&str>, risk: &str, category: &str, json: bool) -> Result<(), anyhow::Error> {
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    let (pkg_name, concepts, apis) = if ext == "atlas" {
        let bundle_data = AtlasBundle::from_file(path)?;
        let ir = bundle_data.ir;
        let pkg_name = name.unwrap_or_else(|| path.file_stem().and_then(|s| s.to_str()).unwrap_or("atlas-package")).to_string();
        let concepts: Vec<String> = ir.nodes.iter().map(|n| format!("- **{}**: {}", n.name, n.description.as_deref().unwrap_or(""))).collect();
        let apis: Vec<String> = ir.nodes.iter().filter(|n| matches!(n.kind, atlas_ir::NodeKind::API)).map(|n| format!("- `{}`", n.id)).collect();
        (pkg_name, concepts, apis)
    } else {
        let content = std::fs::read_to_string(path)?;
        let _parts: Vec<&str> = content.splitn(3, "---").collect();
        let pkg_name = name.unwrap_or_else(|| path.file_stem().and_then(|s| s.to_str()).unwrap_or("atlas-package")).to_string();
        let concepts = vec!["- See package YAML frontmatter for concepts".to_string()];
        let apis = vec!["- See package YAML frontmatter for APIs".to_string()];
        (pkg_name, concepts, apis)
    };

    let skill_name = pkg_name.replace(['_', '-'], " ").split_whitespace().map(|w| {
        let mut c = w.chars();
        match c.next() { Some(f) => f.to_uppercase().collect::<String>() + c.as_str(), None => String::new() }
    }).collect::<Vec<_>>().join(" ");

    let out_path = out.cloned().unwrap_or_else(|| PathBuf::from("SKILL.md"));
    let md_content = format!(
r#"---
name: {skill_name}
description: Atlas knowledge package — {pkg_name}
risk: {risk}
category: {category}
---

# {skill_name}

This skill is generated from an Atlas knowledge package.

## Concepts

{concepts}

## APIs

{apis}

## Usage

```shell
atlas solve bundle.atlas "your question about {pkg_name}"
atlas decide bundle.atlas "your decision" -c context=value
```
"#, skill_name=skill_name, pkg_name=pkg_name, risk=risk, category=category,
       concepts=concepts.join("\n"), apis=apis.join("\n"));

    std::fs::write(&out_path, &md_content)?;

    if json {
        println!(r#"{{"skill":"{}","path":"{}","risk":"{}","category":"{}"}}"#, skill_name, out_path.display(), risk, category);
    } else {
        println!("✅ Generated skill: {} → {}", skill_name, out_path.display());
        println!("   Risk: {}, Category: {}", risk, category);
        println!("   Install with: atlas skill install --dir .agents/skills");
    }
    Ok(())
}

fn run_skill_search(query: &str, limit: usize, json: bool) -> Result<(), anyhow::Error> {
    let url = format!("https://raw.githubusercontent.com/sickn33/agentic-awesome-skills/main/skills_index.json");
    let response = ureq::get(&url)
        .header("User-Agent", "AtlasSkillSearch/1.0")
        .call()
        .map_err(|e| anyhow::anyhow!("Failed to fetch skill index: {}", e))?;

    let body = response.into_body().read_to_string()?;
    let skills: Vec<serde_json::Value> = serde_json::from_str(&body)?;

    let lower_query = query.to_lowercase();
    let matches: Vec<&serde_json::Value> = skills.iter()
        .filter(|s| {
            s["name"].as_str().unwrap_or("").to_lowercase().contains(&lower_query)
                || s["description"].as_str().unwrap_or("").to_lowercase().contains(&lower_query)
                || s["category"].as_str().unwrap_or("").to_lowercase().contains(&lower_query)
                || s["tags"].as_array().map(|t| t.iter().any(|tag| tag.as_str().unwrap_or("").to_lowercase().contains(&lower_query))).unwrap_or(false)
        })
        .take(limit)
        .collect();

    if json {
        let results: Vec<serde_json::Value> = matches.iter().map(|s| serde_json::json!({
            "id": s["id"],
            "name": s["name"],
            "description": s["description"],
            "category": s["category"],
            "risk": s["risk"],
            "path": s["path"],
        })).collect();
        println!("{}", serde_json::to_string(&serde_json::json!({"skills": results, "total": skills.len(), "matches": matches.len()}))?);
    } else {
        println!("🔍 Found {} matching skills (out of {} total)", matches.len(), skills.len());
        for s in &matches {
            let name = s["name"].as_str().unwrap_or("unknown");
            let desc = s["description"].as_str().unwrap_or("").chars().take(120).collect::<String>();
            let cat = s["category"].as_str().unwrap_or("");
            let risk = s["risk"].as_str().unwrap_or("");
            println!("  {:<30} [{}] (risk: {})", name, cat, risk);
            println!("  {}\n", desc);
        }
        println!("Install a skill: atlas skill install <skill-id>");
        println!("Browse all: https://github.com/sickn33/agentic-awesome-skills");
    }
    Ok(())
}

fn run_skill_list(json: bool) -> Result<(), anyhow::Error> {
    let skills_dir = PathBuf::from(".agents/skills");
    if !skills_dir.exists() {
        if json {
            println!(r#"{{"skills":[],"message":"No .agents/skills directory found"}}"#);
        } else {
            println!("No .agents/skills directory found. Install skills first:");
            println!("  npx agentic-awesome-skills --path .agents/skills");
            println!("  atlas skill install <skill-id> --dir .agents/skills");
        }
        return Ok(());
    }

    let mut skills = Vec::new();
    for entry in std::fs::read_dir(&skills_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let skill_md = path.join("SKILL.md");
            if skill_md.exists() {
                skills.push(entry.file_name().to_string_lossy().to_string());
            }
        }
    }
    skills.sort();

    if json {
        println!(r#"{{"skills":{}}}"#, serde_json::to_string(&skills)?);
    } else {
        println!("Installed skills ({})", skills.len());
        for s in &skills {
            println!("  - {}", s);
        }
    }
    Ok(())
}

fn run_skill_install(skills: &[String], dir: Option<&PathBuf>, json: bool) -> Result<(), anyhow::Error> {
    if skills.is_empty() {
        anyhow::bail!("No skill IDs provided. Usage: atlas skill install <skill-id1> [<skill-id2> ...]");
    }

    let target_dir = dir.cloned().unwrap_or_else(|| PathBuf::from(".agents/skills"));
    std::fs::create_dir_all(&target_dir)?;

    // Fetch the skills index
    let url = "https://raw.githubusercontent.com/sickn33/agentic-awesome-skills/main/skills_index.json";
    let response = ureq::get(url)
        .header("User-Agent", "AtlasSkillInstall/1.0")
        .call()
        .map_err(|e| anyhow::anyhow!("Failed to fetch skill index: {}", e))?;

    let body = response.into_body().read_to_string()?;
    let index: Vec<serde_json::Value> = serde_json::from_str(&body)?;

    let mut installed = 0u32;
    for skill_id in skills {
        if let Some(skill) = index.iter().find(|s| s["id"].as_str().unwrap_or("") == skill_id) {
            let path = skill["path"].as_str().unwrap_or("");
            let skill_name = skill["name"].as_str().unwrap_or(skill_id);
            if path.is_empty() {
                if json {
                    println!(r#"{{"warning":"No path for skill '{}'"}}"#, skill_id);
                } else {
                    println!("⚠️  No path for skill '{}'", skill_id);
                }
                continue;
            }
            let raw_url = format!("https://raw.githubusercontent.com/sickn33/agentic-awesome-skills/main/{}", path.trim_start_matches("./"));
            match ureq::get(&raw_url)
                .header("User-Agent", "AtlasSkillInstall/1.0")
                .call()
            {
                Ok(r) => {
                    let skill_body = r.into_body().read_to_string().unwrap_or_default();
                    let skill_dir = target_dir.join(skill_id);
                    std::fs::create_dir_all(&skill_dir)?;
                    std::fs::write(skill_dir.join("SKILL.md"), &skill_body)?;
                    installed += 1;
                    if !json {
                        println!("  ✅ Installed: {} → {}", skill_name, skill_dir.display());
                    }
                }
                Err(e) => {
                    if json {
                        println!(r#"{{"warning":"Failed to download skill '{}': {}"}}"#, skill_id, e);
                    } else {
                        println!("  ⚠️  Failed to download '{}': {}", skill_id, e);
                    }
                }
            }
        } else {
            if json {
                println!(r#"{{"warning":"Skill '{}' not found in index"}}"#, skill_id);
            } else {
                println!("  ⚠️  Skill '{}' not found in index", skill_id);
            }
        }
    }

    if json {
        println!(r#"{{"installed":{},"directory":"{}"}}"#, installed, target_dir.display());
    } else {
        println!("✅ Installed {} skill(s) to {}", installed, target_dir.display());
    }
    Ok(())
}

fn run_serve(port: u16, host: &str) -> Result<(), anyhow::Error> {
    let addr = format!("{}:{}", host, port);
    let server = tiny_http::Server::http(&addr)
        .map_err(|e| anyhow::anyhow!("Failed to start server on {}: {}", addr, e))?;
    eprintln!("🧠 Atlas knowledge API server running at http://{}", addr);
    eprintln!("   Endpoints:");
    eprintln!("     GET /solve?q=<query>[&from=<package>|&all=true]");
    eprintln!("     GET /packages");
    eprintln!("     POST /install  body: {}", r#"{"package":"<name>"}"#);
    eprintln!("     GET /health");

    for mut request in server.incoming_requests() {
        let url = request.url().to_string();
        let method = request.method().as_str().to_string();
        let response = match (method.as_str(), url.as_str()) {
            ("GET", "/health") => json_response(serde_json::json!({"status": "ok", "version": "0.3.1"})),
            ("GET", _) if url.starts_with("/solve") => {
                handle_solve_request(&url)
            }
            ("GET", "/packages") => {
                handle_packages_request()
            }
            ("POST", "/install") => {
                handle_install_request(&mut request)
            }
            _ => {
                json_response_with_status(serde_json::json!({"error": "Not Found"}), 404)
            }
        };
        let _ = request.respond(response).map_err(|e| anyhow::anyhow!("respond error: {}", e));
    }
    Ok(())
}

fn handle_solve_request(url: &str) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    let params = parse_query(url);
    let query = match params.get("q") {
        Some(q) => q,
        None => return json_response_with_status(serde_json::json!({"error": "Missing 'q' parameter"}), 400),
    };

    let mut runtime = atlas_knowledge::Runtime::new();
    let load_from_all = params.get("all").map(|v| v == "true").unwrap_or(false);
    let package = params.get("from");

    if load_from_all || package.is_none() {
        if let Some(data_dir) = dirs::data_dir() {
            let bundles_dir = data_dir.join("atlas").join("bundles");
            if bundles_dir.exists() {
                if let Ok(entries) = std::fs::read_dir(&bundles_dir) {
                    for entry in entries.flatten() {
                        let bundle_atlas = entry.path().join("bundle.atlas");
                        if bundle_atlas.exists() {
                            let _ = runtime.load(&bundle_atlas);
                        }
                    }
                }
            }
        }
    }

    if let Some(pkg) = package {
        if let Ok((resolved, name)) = resolve_bundle(&std::path::PathBuf::from(pkg)) {
            let _ = runtime.load(&resolved);
            if let Ok(result) = runtime.solve(&name, query) {
                return json_response(serde_json::json!({
                    "query": result.query,
                    "bundle": result.bundle,
                    "confidence": result.confidence,
                    "total_matches": result.total_matches,
                    "nodes": result.nodes.iter().map(|n| serde_json::json!({
                        "id": n.id,
                        "name": n.name,
                        "kind": format!("{:?}", n.kind),
                        "description": n.description,
                        "version": n.version,
                        "confidence": n.confidence,
                    })).collect::<Vec<_>>(),
                }));
            }
        }
    }

    if load_from_all || (package.is_none() && runtime.bundle_count() > 0) {
        let results = runtime.solve_all(query);
        return json_response(serde_json::json!({
            "query": query,
            "results": results.iter().map(|r| serde_json::json!({
                "bundle": r.bundle,
                "confidence": r.confidence,
                "nodes": r.nodes.iter().map(|n| serde_json::json!({
                    "id": n.id,
                    "name": n.name,
                    "kind": format!("{:?}", n.kind),
                    "description": n.description,
                    "version": n.version,
                    "confidence": n.confidence,
                })).collect::<Vec<_>>(),
            })).collect::<Vec<_>>(),
        }));
    }

    json_response(serde_json::json!({"error": "No packages loaded. Install a package first with `atlas install <name>` or use the MCP server."}))
}

fn handle_packages_request() -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    let mut packages = Vec::new();
    if let Some(data_dir) = dirs::data_dir() {
        let bundles_dir = data_dir.join("atlas").join("bundles");
        if bundles_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&bundles_dir) {
                for entry in entries.flatten() {
                    packages.push(serde_json::json!({
                        "name": entry.file_name().to_string_lossy(),
                    }));
                }
            }
        }
    }
    json_response(serde_json::json!({"packages": packages}))
}

fn handle_install_request(request: &mut tiny_http::Request) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    let mut body = String::new();
    if request.as_reader().read_to_string(&mut body).is_err() {
        return json_response_with_status(serde_json::json!({"error": "Failed to read request body"}), 400);
    }
    let parsed: serde_json::Value = match serde_json::from_str(&body) {
        Ok(v) => v,
        Err(_) => return json_response_with_status(serde_json::json!({"error": "Invalid JSON"}), 400),
    };
    let package = match parsed.get("package").and_then(|v| v.as_str()) {
        Some(p) => p,
        None => return json_response_with_status(serde_json::json!({"error": "Missing 'package' field"}), 400),
    };
    match run_install(None, vec![std::path::PathBuf::from(package)], false) {
        Ok(()) => json_response(serde_json::json!({"installed": package})),
        Err(e) => json_response_with_status(serde_json::json!({"error": format!("{}", e)}), 500),
    }
}

fn parse_query(url: &str) -> std::collections::HashMap<String, String> {
    use std::collections::HashMap;
    let mut params = HashMap::new();
    if let Some(query_start) = url.find('?') {
        let query_str = &url[query_start + 1..];
        for pair in query_str.split('&') {
            if let Some(eq) = pair.find('=') {
                let key = url_decode(&pair[..eq]);
                let value = url_decode(&pair[eq + 1..]);
                params.insert(key, value);
            }
        }
    }
    params
}

fn url_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        match c {
            '+' => result.push(' '),
            '%' => {
                let hex: String = chars.by_ref().take(2).collect();
                if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                    result.push(byte as char);
                }
            }
            _ => result.push(c),
        }
    }
    result
}


fn json_response(data: serde_json::Value) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    json_response_with_status(data, 200)
}

fn json_response_with_status(data: serde_json::Value, status: u16) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    let body = serde_json::to_string(&data).unwrap_or_default();
    let bytes = body.into_bytes();
    let ct = tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
    let cors = tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap();
    tiny_http::Response::new(
        status.into(),
        vec![ct, cors],
        std::io::Cursor::new(bytes),
        None,
        None,
    )
}

fn urlencode(s: &str) -> String {
    s.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => (b as char).to_string(),
            b' ' => "+".to_string(),
            _ => format!("%{:02X}", b),
        })
        .collect()
}

/// Agent state for durable execution
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AgentState {
    checkpoint_id: String,
    task: String,
    package: String,
    step: usize,
    max_steps: usize,
    graph_position: Option<String>,
    memory: HashMap<String, serde_json::Value>,
    history: Vec<AgentStep>,
    decision_context: HashMap<String, String>,
    created_at: u64,
    updated_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AgentStep {
    step: usize,
    action: String,
    result: serde_json::Value,
    timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AgentTool {
    name: String,
    description: String,
    schema: serde_json::Value,
    workflow_id: Option<String>,
}

fn timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

fn generate_checkpoint_id() -> String {
    format!("ckpt-{}", timestamp())
}

fn load_package_workflows(package: &str) -> Result<Vec<Workflow>, anyhow::Error> {
    let bundle_path = find_package_bundle(package)?;
    let mut runtime = Runtime::new();
    let name = runtime.load(&bundle_path)?;
    let workflows = runtime.get_workflows(&name).cloned().unwrap_or_default();
    Ok(workflows)
}

fn find_package_bundle(package: &str) -> Result<PathBuf, anyhow::Error> {
    let data_dir = dirs::data_dir().ok_or_else(|| anyhow::anyhow!("Cannot determine data directory"))?;
    let bundles_dir = data_dir.join("atlas").join("bundles");
    
    // Try local first
    let local_paths = [
        PathBuf::from(format!("packages/{}.atlas", package)),
        PathBuf::from(format!("{}.atlas", package)),
    ];
    for p in &local_paths {
        if p.exists() {
            return Ok(p.clone());
        }
    }
    
    // Try installed packages
    let installed = bundles_dir.join(package).join("bundle.atlas");
    if installed.exists() {
        return Ok(installed);
    }
    
    anyhow::bail!("Package '{}' not found. Run `atlas install {}` first.", package, package)
}

fn build_tools_from_workflows(workflows: &[Workflow]) -> Vec<AgentTool> {
    workflows.iter().map(|wf| {
        let mut properties = serde_json::Map::new();
        for step in &wf.steps {
            properties.insert(step.action.clone(), serde_json::json!({
                "type": "string",
                "description": format!("Step: {}", step.action)
            }));
        }
        AgentTool {
            name: wf.id.clone(),
            description: wf.description.clone(),
            schema: serde_json::json!({
                "type": "object",
                "properties": properties,
                "additionalProperties": true
            }),
            workflow_id: Some(wf.id.clone()),
        }
    }).collect()
}

fn run_agent_run(
    task: &str,
    package: &str,
    max_steps: usize,
    checkpoint_dir: &Path,
    human_in_loop: bool,
    checkpoint_interval: usize,
    json: bool,
) -> Result<(), anyhow::Error> {
    std::fs::create_dir_all(checkpoint_dir)?;
    
    // Load package workflows as tools
    let workflows = load_package_workflows(package)?;
    let tools = build_tools_from_workflows(&workflows);
    
    if json {
        let output = serde_json::json!({
            "status": "started",
            "task": task,
            "package": package,
            "max_steps": max_steps,
            "tools_available": tools.len(),
            "checkpoint_dir": checkpoint_dir.to_string_lossy(),
        });
        println!("{}", serde_json::to_string(&output)?);
    } else {
        println!("🤖 Starting agent: {}", task);
        println!("   Package: {}", package);
        println!("   Max steps: {}", max_steps);
        println!("   Tools: {}", tools.len());
        println!("   Human-in-loop: {}", human_in_loop);
    }
    
    let mut state = AgentState {
        checkpoint_id: generate_checkpoint_id(),
        task: task.to_string(),
        package: package.to_string(),
        step: 0,
        max_steps,
        graph_position: None,
        memory: HashMap::new(),
        history: Vec::new(),
        decision_context: HashMap::new(),
        created_at: timestamp(),
        updated_at: timestamp(),
    };
    
    // Initialize with solve
    let _solve_result = initialize_solve(task, package, &mut state, json)?;
    
    // Agent loop: solve -> decide -> act -> observe -> repeat
    loop {
        if state.step >= state.max_steps {
            if json {
                println!(r#"{{"status":"max_steps_reached","checkpoint_id":"{}","steps":{}}}"#, state.checkpoint_id, state.step);
            } else {
                println!("⏹️  Max steps ({}) reached", state.max_steps);
            }
            break;
        }
        
        state.step += 1;
        state.updated_at = timestamp();
        
        // DECIDE: Use decision trees
        let decide_result = run_decide_step(&state, package, json)?;
        
        // HUMAN-IN-LOOP: Pause at decision nodes if enabled
        if human_in_loop {
            if let Some(ref dr) = decide_result {
                let approved = prompt_human_approval(dr, json)?;
                if !approved {
                    if json {
                        println!(r#"{{"status":"rejected","checkpoint_id":"{}","step":{}}}"#, state.checkpoint_id, state.step);
                    } else {
                        println!("❌ Human rejected decision, stopping");
                    }
                    break;
                }
            }
        }
        
        // ACT: Execute tool/workflow based on decision
        let act_result = if let Some(ref dr) = decide_result {
            execute_action(dr, &tools, &mut state, json)?
        } else {
            serde_json::json!({"action": "none", "result": "no decision tree matched"})
        };
        
        // OBSERVE: Run solve again to get updated context
        let observe_result = run_solve_step(&state, package, json)?;
        
        // Record step
        state.history.push(AgentStep {
            step: state.step,
            action: format!("decide->act: {}", decide_result.as_ref().map(|d| d.tree_id.clone()).unwrap_or("none".into())),
            result: serde_json::json!({
                "decide": decide_result,
                "act": act_result,
                "observe": observe_result,
            }),
            timestamp: timestamp(),
        });
        
        // Checkpoint
        if state.step % checkpoint_interval == 0 {
            save_checkpoint(&state, checkpoint_dir)?;
            if json {
                println!(r#"{{"status":"checkpoint","checkpoint_id":"{}","step":{}}}"#, state.checkpoint_id, state.step);
            } else {
                println!("💾 Checkpoint saved: {} (step {})", state.checkpoint_id, state.step);
            }
        }
        
        // Check if task is complete (heuristic: observe has high confidence and no more decisions)
        if is_task_complete(&observe_result, &decide_result) {
            if json {
                println!(r#"{{"status":"completed","checkpoint_id":"{}","steps":{},"final_result":{}}}"#, state.checkpoint_id, state.step, observe_result);
            } else {
                println!("✅ Task completed in {} steps", state.step);
            }
            break;
        }
    }
    
    // Final checkpoint
    save_checkpoint(&state, checkpoint_dir)?;
    
    if json {
        let output = serde_json::json!({
            "status": "finished",
            "checkpoint_id": state.checkpoint_id,
            "task": state.task,
            "steps": state.step,
            "history": state.history,
        });
        println!("{}", serde_json::to_string(&output)?);
    }
    
    Ok(())
}

fn initialize_solve(task: &str, package: &str, state: &mut AgentState, json: bool) -> Result<serde_json::Value, anyhow::Error> {
    let bundle_path = find_package_bundle(package)?;
    let mut runtime = Runtime::new();
    let name = runtime.load(&bundle_path)?;
    let result = runtime.solve(&name, task)?;
    
    state.graph_position = result.nodes.first().map(|n| n.id.clone());
    
    let output = serde_json::json!({
        "query": result.query,
        "bundle": result.bundle,
        "confidence": result.confidence,
        "total_matches": result.total_matches,
        "nodes": result.nodes.iter().map(|n| serde_json::json!({
            "id": n.id,
            "name": n.name,
            "kind": format!("{:?}", n.kind),
            "confidence": n.confidence,
        })).collect::<Vec<_>>(),
    });
    
    if !json {
        println!("🔍 Initial solve: {} nodes (confidence: {:.2})", result.nodes.len(), result.confidence);
    }
    
    Ok(output)
}

fn run_solve_step(state: &AgentState, package: &str, json: bool) -> Result<serde_json::Value, anyhow::Error> {
    let bundle_path = find_package_bundle(package)?;
    let mut runtime = Runtime::new();
    let name = runtime.load(&bundle_path)?;
    
    // Build context from memory and history
    let mut query = state.task.clone();
    if let Some(pos) = &state.graph_position {
        query.push_str(&format!(" (context: {})", pos));
    }
    for (k, v) in &state.memory {
        query.push_str(&format!(" {}={}", k, v));
    }
    
    let result = runtime.solve(&name, &query)?;
    
    let output = serde_json::json!({
        "query": result.query,
        "bundle": result.bundle,
        "confidence": result.confidence,
        "total_matches": result.total_matches,
        "nodes": result.nodes.iter().map(|n| serde_json::json!({
            "id": n.id,
            "name": n.name,
            "kind": format!("{:?}", n.kind),
            "confidence": n.confidence,
        })).collect::<Vec<_>>(),
    });
    
    if !json {
        println!("🔍 Observe (step {}): {} nodes, confidence {:.2}", state.step, result.nodes.len(), result.confidence);
    }
    
    Ok(output)
}

fn run_decide_step(state: &AgentState, package: &str, json: bool) -> Result<Option<DecisionResult>, anyhow::Error> {
    let bundle_path = find_package_bundle(package)?;
    let mut runtime = Runtime::new();
    let name = runtime.load(&bundle_path)?;
    
    let result = runtime.decide(&name, &state.task, Some(&state.decision_context))?;
    
    if !json {
        if let Some(ref dr) = result {
            println!("🤔 Decision: {} -> {} (confidence: {:?})", dr.tree_id, dr.path.join(" -> "), dr.recommendations.first().map(|r| r.confidence));
        } else {
            println!("🤔 No matching decision tree for task");
        }
    }
    
    Ok(result)
}

fn execute_action(decide_result: &DecisionResult, tools: &[AgentTool], state: &mut AgentState, json: bool) -> Result<serde_json::Value, anyhow::Error> {
    // Find matching tool from recommendations
    for rec in &decide_result.recommendations {
        if let Some(tool) = tools.iter().find(|t| t.workflow_id.as_deref() == Some(rec.node_id.as_str()) || t.name == rec.node_id) {
            // Execute workflow as tool
            let result = execute_workflow_tool(tool, &decide_result.recommendations, state)?;
            
            // Update state memory with result
            state.memory.insert(tool.name.clone(), result.clone());
            state.graph_position = Some(rec.node_id.clone());
            
            if !json {
                println!("⚡ Executed tool: {} -> {}", tool.name, serde_json::to_string(&result)?);
            }
            
            return Ok(serde_json::json!({
                "tool": tool.name,
                "recommendation": rec.node_id,
                "confidence": rec.confidence,
                "result": result,
            }));
        }
    }
    
    // Fallback: use first recommendation as generic action
    if let Some(rec) = decide_result.recommendations.first() {
        let result = serde_json::json!({
            "node": rec.node_id,
            "confidence": rec.confidence,
            "action": "knowledge_lookup",
        });
        state.memory.insert(rec.node_id.clone(), result.clone());
        state.graph_position = Some(rec.node_id.clone());
        
        if !json {
            println!("📖 Knowledge lookup: {}", rec.node_id);
        }
        
        return Ok(serde_json::json!({
            "tool": "knowledge_lookup",
            "recommendation": rec.node_id,
            "confidence": rec.confidence,
            "result": result,
        }));
    }
    
    Ok(serde_json::json!({"action": "none", "result": "no recommendations"}))
}

fn execute_workflow_tool(tool: &AgentTool, recommendations: &[RecommendationItem], _state: &mut AgentState) -> Result<serde_json::Value, anyhow::Error> {
    // Simulate workflow execution - in reality this would run the actual workflow steps
    // For now, return structured result based on workflow
    let mut result = serde_json::Map::new();
    result.insert("workflow".to_string(), serde_json::Value::String(tool.name.clone()));
    result.insert("status".to_string(), serde_json::Value::String("executed".to_string()));
    result.insert("steps".to_string(), serde_json::Value::Array(vec![]));
    result.insert("output".to_string(), serde_json::Value::String(format!("Workflow {} executed with {} recommendations", tool.name, recommendations.len())));
    
    Ok(serde_json::Value::Object(result))
}

fn is_task_complete(observe: &serde_json::Value, decide: &Option<DecisionResult>) -> bool {
    // Task complete if high confidence and no more decisions
    let confidence = observe.get("confidence").and_then(|v| v.as_f64()).unwrap_or(0.0);
    confidence > 0.9 && decide.is_none()
}

fn prompt_human_approval(decide_result: &DecisionResult, json: bool) -> Result<bool, anyhow::Error> {
    if json {
        let prompt = serde_json::json!({
            "type": "human_approval",
            "decision_tree": decide_result.tree_id,
            "path": decide_result.path,
            "rationale": decide_result.rationale,
            "recommendations": decide_result.recommendations.iter().map(|r| serde_json::json!({
                "node_id": r.node_id,
                "confidence": r.confidence,
            })).collect::<Vec<_>>(),
            "agent_instructions": decide_result.agent_instructions,
        });
        println!("{}", serde_json::to_string(&prompt)?);
    } else {
        println!("\n━━━ HUMAN APPROVAL REQUIRED ━━━");
        println!("Decision tree: {}", decide_result.tree_id);
        println!("Path: {}", decide_result.path.join(" → "));
        println!("Rationale: {}", decide_result.rationale);
        println!("Recommendations:");
        for rec in &decide_result.recommendations {
            println!("  • {} (confidence: {:.2})", rec.node_id, rec.confidence);
        }
        if let Some(ref instr) = decide_result.agent_instructions {
            println!("Agent instructions: {}", instr);
        }
        print!("Approve? [y/N]: ");
        std::io::Write::flush(&mut std::io::stdout())?;
    }
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let approved = input.trim().eq_ignore_ascii_case("y") || input.trim().eq_ignore_ascii_case("yes");
    
    if !json {
        if approved {
            println!("✅ Approved, continuing...");
        } else {
            println!("❌ Rejected, stopping...");
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }
    
    Ok(approved)
}

fn save_checkpoint(state: &AgentState, checkpoint_dir: &Path) -> Result<(), anyhow::Error> {
    let path = checkpoint_dir.join(format!("{}.json", state.checkpoint_id));
    let data = serde_json::to_string_pretty(state)?;
    std::fs::write(&path, data)?;
    Ok(())
}

fn load_checkpoint(checkpoint_id: &str, checkpoint_dir: &Path) -> Result<AgentState, anyhow::Error> {
    let path = checkpoint_dir.join(format!("{}.json", checkpoint_id));
    if !path.exists() {
        anyhow::bail!("Checkpoint not found: {}", path.display());
    }
    let data = std::fs::read_to_string(&path)?;
    let state: AgentState = serde_json::from_str(&data)?;
    Ok(state)
}

fn run_agent_resume(
    checkpoint_id: &str,
    max_steps: Option<usize>,
    human_in_loop: bool,
    json: bool,
) -> Result<(), anyhow::Error> {
    let checkpoint_dir = PathBuf::from(".atlas/checkpoints");
    let mut state = load_checkpoint(checkpoint_id, &checkpoint_dir)?;
    
    if let Some(ms) = max_steps {
        state.max_steps = ms;
    }
    
    if json {
        let output = serde_json::json!({
            "status": "resumed",
            "checkpoint_id": state.checkpoint_id,
            "task": state.task,
            "step": state.step,
            "max_steps": state.max_steps,
        });
        println!("{}", serde_json::to_string(&output)?);
    } else {
        println!("🔄 Resuming agent from checkpoint: {}", checkpoint_id);
        println!("   Task: {}", state.task);
        println!("   Step: {}/{}", state.step, state.max_steps);
    }
    
    // Continue agent loop from current step
    let workflows = load_package_workflows(&state.package)?;
    let tools = build_tools_from_workflows(&workflows);
    
    loop {
        if state.step >= state.max_steps {
            if json {
                println!(r#"{{"status":"max_steps_reached","checkpoint_id":"{}","steps":{}}}"#, state.checkpoint_id, state.step);
            } else {
                println!("⏹️  Max steps ({}) reached", state.max_steps);
            }
            break;
        }
        
        state.step += 1;
        state.updated_at = timestamp();
        
        let decide_result = run_decide_step(&state, &state.package, json)?;
        
        if human_in_loop {
            if let Some(ref dr) = decide_result {
                let approved = prompt_human_approval(dr, json)?;
                if !approved {
                    if json {
                        println!(r#"{{"status":"rejected","checkpoint_id":"{}","step":{}}}"#, state.checkpoint_id, state.step);
                    } else {
                        println!("❌ Human rejected decision, stopping");
                    }
                    break;
                }
            }
        }
        
        let act_result = if let Some(ref dr) = decide_result {
            execute_action(dr, &tools, &mut state, json)?
        } else {
            serde_json::json!({"action": "none", "result": "no decision tree matched"})
        };
        
        let observe_result = run_solve_step(&state, &state.package, json)?;
        
        state.history.push(AgentStep {
            step: state.step,
            action: format!("decide->act: {}", decide_result.as_ref().map(|d| d.tree_id.clone()).unwrap_or("none".into())),
            result: serde_json::json!({
                "decide": decide_result,
                "act": act_result,
                "observe": observe_result,
            }),
            timestamp: timestamp(),
        });
        
        if state.step % 3 == 0 {
            save_checkpoint(&state, &checkpoint_dir)?;
            if json {
                println!(r#"{{"status":"checkpoint","checkpoint_id":"{}","step":{}}}"#, state.checkpoint_id, state.step);
            } else {
                println!("💾 Checkpoint saved: {} (step {})", state.checkpoint_id, state.step);
            }
        }
        
        if is_task_complete(&observe_result, &decide_result) {
            if json {
                println!(r#"{{"status":"completed","checkpoint_id":"{}","steps":{},"final_result":{}}}"#, state.checkpoint_id, state.step, observe_result);
            } else {
                println!("✅ Task completed in {} steps", state.step);
            }
            break;
        }
    }
    
    save_checkpoint(&state, &checkpoint_dir)?;
    
    if json {
        let output = serde_json::json!({
            "status": "finished",
            "checkpoint_id": state.checkpoint_id,
            "task": state.task,
            "steps": state.step,
            "history": state.history,
        });
        println!("{}", serde_json::to_string(&output)?);
    }
    
    Ok(())
}

fn run_agent_list(json: bool) -> Result<(), anyhow::Error> {
    let checkpoint_dir = PathBuf::from(".atlas/checkpoints");
    if !checkpoint_dir.exists() {
        if json {
            println!(r#"{{"checkpoints":[]}}"#);
        } else {
            println!("No checkpoints found in {}", checkpoint_dir.display());
        }
        return Ok(());
    }
    
    let mut checkpoints = Vec::new();
    for entry in std::fs::read_dir(&checkpoint_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let data = std::fs::read_to_string(&path)?;
            if let Ok(state) = serde_json::from_str::<AgentState>(&data) {
                checkpoints.push(serde_json::json!({
                    "checkpoint_id": state.checkpoint_id,
                    "task": state.task,
                    "package": state.package,
                    "step": state.step,
                    "max_steps": state.max_steps,
                    "created_at": state.created_at,
                    "updated_at": state.updated_at,
                }));
            }
        }
    }
    
    checkpoints.sort_by(|a, b| b["updated_at"].as_u64().unwrap_or(0).cmp(&a["updated_at"].as_u64().unwrap_or(0)));
    
    if json {
        println!(r#"{{"checkpoints":{}}}"#, serde_json::to_string(&checkpoints)?);
    } else {
        println!("📋 Checkpoints ({}):", checkpoints.len());
        for cp in checkpoints {
            let created = chrono::DateTime::from_timestamp(cp["created_at"].as_i64().unwrap_or(0) as i64, 0)
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| "unknown".to_string());
            println!("  {} - {} (step {}/{}) - created {}",
                cp["checkpoint_id"],
                cp["task"].as_str().unwrap_or(""),
                cp["step"].as_u64().unwrap_or(0),
                cp["max_steps"].as_u64().unwrap_or(0),
                created,
            );
        }
    }
    
    Ok(())
}

fn run_install(name: Option<String>, sources: Vec<PathBuf>, json: bool) -> Result<(), anyhow::Error> {
    let data_dir = dirs::data_dir()
        .ok_or_else(|| anyhow::anyhow!("Cannot determine data directory"))?;
    let atlas_dir = data_dir.join("atlas").join("bundles");

    for source in &sources {
        let ext = source.extension().and_then(|s| s.to_str()).unwrap_or("");
        let stem = source.file_stem().and_then(|s| s.to_str()).unwrap_or("bundle");
        let bundle_name = name.clone().unwrap_or_else(|| stem.to_string());
        let target_dir = atlas_dir.join(&bundle_name);
        std::fs::create_dir_all(&target_dir)?;

        match ext {
            "atlas" => {
                let target = target_dir.join("bundle.atlas");
                std::fs::copy(source, &target)?;
                if json {
                    println!(r#"{{"installed":"{}","target":"{}"}}"#, source.display(), target.display());
                } else {
                    println!("Installed {} → {}", source.display(), target.display());
                }
            }
            "md" | "yaml" | "yml" => {
                let out_path = target_dir.join("bundle.atlas");
                let sources = vec![source.clone()];
                run_compile(sources, &out_path, false, json)?;
            }
            "" => {
                let candidates = [
                    PathBuf::from(format!("packages/{}.md", stem)),
                    PathBuf::from(format!("{}.md", stem)),
                    PathBuf::from(format!("{}.atlas", stem)),
                ];
                let found = candidates.iter().find(|p| p.exists());
                match found {
                    Some(p) if p.extension().and_then(|s| s.to_str()) == Some("atlas") => {
                        let target = target_dir.join("bundle.atlas");
                        std::fs::copy(p, &target)?;
                        if json {
                            println!(r#"{{"installed":"{}","target":"{}"}}"#, p.display(), target.display());
                        } else {
                            println!("Installed {} → {}", p.display(), target.display());
                        }
                    }
                    Some(p) => {
                        let out_path = target_dir.join("bundle.atlas");
                        let sources = vec![p.clone()];
                        run_compile(sources, &out_path, false, json)?;
                    }
                    None => {
                        // Try downloading from registry
                        let registry = std::env::var("ATLAS_REGISTRY")
                            .unwrap_or_else(|_| "https://atlas-hub-registry.cbvarshini1.workers.dev".to_string());
                        let url = format!("{}/api/v1/packages/{}", registry, stem);
                        match ureq::get(&url).call() {
                            Ok(r) => {
                                let body = r.into_body().read_to_string()?;
                                let parsed: serde_json::Value = serde_json::from_str(&body)?;
                                if let Some(files) = parsed.get("files").and_then(|f| f.as_object()) {
                                    let md_key = files.keys().find(|k| k.ends_with(".md"))
                                        .cloned()
                                        .unwrap_or_else(|| format!("{}.md", stem));
                                    if let Some(content) = files.get(&md_key).and_then(|c| c.as_str()) {
                                        let local_md = PathBuf::from(format!("packages/{}.md", stem));
                                        std::fs::create_dir_all(local_md.parent().unwrap_or(&PathBuf::from(".")))?;
                                        std::fs::write(&local_md, content)?;
                                        let out_path = target_dir.join("bundle.atlas");
                                        let sources = vec![local_md.clone()];
                                        run_compile(sources, &out_path, false, json)?;
                                        if json {
                                            println!(r#"{{"installed":"{}","from":"registry"}}"#, stem);
                                        } else {
                                            println!("Installed '{}' from registry → {}", stem, out_path.display());
                                        }
                                    } else {
                                        anyhow::bail!("No markdown content found for package '{}'", stem);
                                    }
                                } else {
                                    anyhow::bail!("Invalid response from registry for package '{}'", stem);
                                }
                            }
                            Err(e) => {
                                anyhow::bail!("Could not find package '{}' locally or in registry (tried: {}; {}; {}; registry: {})",
                                    stem,
                                    candidates[0].display(),
                                    candidates[1].display(),
                                    candidates[2].display(),
                                    e,
                                );
                            }
                        }
                    }
                }
            }
            _ => anyhow::bail!("Unsupported file format: {}", source.display()),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_parse_key_val_valid() {
        let kv = parse_key_val("lang=rust").unwrap();
        assert_eq!(kv.key, "lang");
        assert_eq!(kv.value, "rust");
    }

    #[test]
    fn test_parse_key_val_empty_value() {
        let kv = parse_key_val("flag=").unwrap();
        assert_eq!(kv.key, "flag");
        assert_eq!(kv.value, "");
    }

    #[test]
    fn test_parse_key_val_invalid() {
        assert!(parse_key_val("noequals").is_err());
        assert!(parse_key_val("").is_err());
    }

    #[test]
    fn test_urlencode() {
        assert_eq!(urlencode("hello"), "hello");
        assert_eq!(urlencode("a b"), "a+b");
        assert_eq!(urlencode("a/b"), "a%2Fb");
        assert_eq!(urlencode("a.b"), "a.b");
        assert_eq!(urlencode("a-b_~c"), "a-b_~c");
    }

    #[test]
    fn test_cli_parse_solve() {
        let cli = Cli::parse_from(["atlas", "solve", "--bundle", "b.atlas", "my query"]);
        match cli.command {
            Commands::Solve { bundle, query, all } => {
                assert_eq!(bundle, Some(std::path::PathBuf::from("b.atlas")));
                assert_eq!(query, "my query");
                assert!(!all);
            }
            _ => panic!("expected Solve"),
        }
    }

    #[test]
    fn test_cli_parse_compile() {
        let cli = Cli::parse_from(["atlas", "compile", "a.md", "b.md", "--out", "out.atlas"]);
        match cli.command {
            Commands::Compile { sources, out, emit_ir } => {
                assert_eq!(sources.len(), 2);
                assert_eq!(out, std::path::PathBuf::from("out.atlas"));
                assert!(!emit_ir);
            }
            _ => panic!("expected Compile"),
        }
    }

    #[test]
    fn test_cli_parse_verify_policy() {
        let cli = Cli::parse_from(["atlas", "verify", "--bundle", "b.atlas", "--policy", "eu-ai-act"]);
        match cli.command {
            Commands::Verify { bundle, artifact, policy } => {
                assert_eq!(bundle, std::path::PathBuf::from("b.atlas"));
                assert!(artifact.is_none());
                assert_eq!(policy.as_deref(), Some("eu-ai-act"));
            }
            _ => panic!("expected Verify"),
        }
    }

    #[test]
    fn test_cli_parse_decide_context() {
        let cli = Cli::parse_from(["atlas", "decide", "--bundle", "b.atlas", "q", "-c", "lang=rust"]);
        match cli.command {
            Commands::Decide { bundle: _bundle, query, context } => {
                assert_eq!(query, "q");
                assert_eq!(context.len(), 1);
                assert_eq!(context[0].key, "lang");
                assert_eq!(context[0].value, "rust");
            }
            _ => panic!("expected Decide"),
        }
    }
}

