use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use atlas_compiler::Compiler;
use atlas_compiler::frontends::MdDocument;
use atlas_compiler::frontends::decision::DecisionParser;
use atlas_compiler::binary;
use atlas_runtime::reasoner::Reasoner;

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
        #[arg(short, long, default_value = "bundle.atlas")]
        bundle: PathBuf,
        query: String,
    },
    Decide {
        #[arg(short, long, default_value = "bundle.atlas")]
        bundle: PathBuf,
        query: String,
        #[arg(short = 'c', long, value_parser = parse_key_val)]
        context: Vec<KeyVal>,
    },
    Verify {
        #[arg(short, long, default_value = "bundle.atlas")]
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
        #[arg(short, long, default_value = "bundle.atlas")]
        bundle: PathBuf,
        query: String,
        #[arg(long)]
        model: Option<String>,
    },
    /// Dump the full IR of an .atlas file as JSON
    Dump {
        #[arg(short, long, default_value = "bundle.atlas")]
        bundle: PathBuf,
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
    },
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
        Commands::Solve { bundle, query } => run_solve(&bundle, &query, cli.json),
        Commands::Decide { bundle, query, context } => run_decide(&bundle, &query, &context, cli.json),
        Commands::Verify { bundle, artifact, policy } => run_verify(&bundle, artifact.as_deref(), policy.as_deref(), cli.json),
        Commands::Install { name, sources } => run_install(name, sources, cli.json),
        Commands::Reason { bundle, query, model } => run_reason(&bundle, &query, model, cli.json),
        Commands::Dump { bundle } => run_dump(&bundle, cli.json),
        Commands::Init { name, template, dir } => run_init(&name, &template, dir.as_ref()),
        Commands::Search { query, registry } => run_search(&query, &registry, cli.json),
        Commands::Publish { path, registry, api_key } => run_publish(path, &registry, api_key.as_deref(), cli.json),
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
    let (nodes, edges, examples, failures, workflows) = doc.into_parts();
    for node in nodes { compiler.add_node(node)?; }
    for edge in edges { compiler.add_edge(edge)?; }
    for ex in examples { compiler.add_example(ex); }
    for fm in failures { compiler.add_failure(fm); }
    for wf in workflows { compiler.add_workflow(wf); }
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
}

fn run_solve(bundle_path: &Path, query: &str, json: bool) -> Result<(), anyhow::Error> {
    let mut runtime = atlas_runtime::Runtime::new();
    let name = runtime.load(bundle_path)?;
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
    let mut runtime = atlas_runtime::Runtime::new();
    let name = runtime.load(bundle_path)?;
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
    let mut runtime = atlas_runtime::Runtime::new();
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
    let mut runtime = atlas_runtime::Runtime::new();
    let name = runtime.load(bundle_path)?;
    let solve_result = runtime.solve(&name, query)?;
    
    // Also try decision trees
    let context = std::collections::HashMap::new();
    let decide_result = runtime.decide(&name, query, Some(&context)).ok().and_then(|r| r);
    
    let ctx = atlas_runtime::reasoner::ReasonContext {
        query,
        bundle: &solve_result.bundle,
        confidence: solve_result.confidence,
        nodes: &solve_result.nodes,
        total_matches: solve_result.total_matches,
        decide_result: decide_result.as_ref(),
    };
    
    let answer = if let Some(m) = model {
        let reasoner = atlas_runtime::reasoner::OllamaReasoner::new(&m, "http://localhost:11434");
        reasoner.reason(query, &ctx)?
    } else {
        let reasoner = atlas_runtime::reasoner::TemplateReasoner;
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

fn run_dump(bundle_path: &Path, json: bool) -> Result<(), anyhow::Error> {
    let mut runtime = atlas_runtime::Runtime::new();
    let name = runtime.load(bundle_path)?;
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

fn run_publish(path: PathBuf, registry: &str, api_key: Option<&str>, json: bool) -> Result<(), anyhow::Error> {
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
                    for line in text.lines() {
                        let line = line.trim();
                        if !line.is_empty() && !line.starts_with('#') {
                            description = line.to_string();
                            break;
                        }
                    }
                }
                files.insert(filename, text);
            }
        }
    } else {
        let text = std::fs::read_to_string(&path)?;
        let filename = path.file_name().unwrap().to_string_lossy().to_string();
        for line in text.lines() {
            let line = line.trim();
            if !line.is_empty() && !line.starts_with('#') {
                description = line.to_string();
                break;
            }
        }
        files.insert(filename, text);
    }

    let body = serde_json::json!({
        "name": name,
        "version": "0.1.0",
        "description": description,
        "tags": [],
        "files": files,
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

fn urlencode(s: &str) -> String {
    s.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => (b as char).to_string(),
            b' ' => "+".to_string(),
            _ => format!("%{:02X}", b),
        })
        .collect()
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
            _ => anyhow::bail!("Unsupported file format: {}", source.display()),
        }
    }
    Ok(())
}
