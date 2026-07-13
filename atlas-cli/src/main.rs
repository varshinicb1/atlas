use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use atlas_compiler::Compiler;
use atlas_compiler::frontends::MdDocument;
use atlas_compiler::frontends::decision::DecisionParser;
use atlas_compiler::binary;
use atlas_runtime::reasoner::Reasoner;
use atlas_runtime::loader::AtlasBundle;

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
    /// Generate or install agent skills from Atlas packages (agentic-awesome-skills compatible)
    Skill {
        /// Subcommand: generate, install, search, list
        #[command(subcommand)]
        action: SkillAction,
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
        Commands::Publish { path, registry, api_key, org } => run_publish(path, &registry, api_key.as_deref(), &org, cli.json),
        Commands::Clone { url, name, dir, assets, rebuild } => run_clone(&url, name.as_deref(), dir.as_ref(), assets, rebuild, cli.json),
        Commands::Export { bundle, style, out, tokens } => run_export(&bundle, &style, out.as_ref(), tokens, cli.json),
        Commands::Skill { action } => match action {
            SkillAction::Generate { path, out, name, risk, category } => run_skill_generate(&path, out.as_ref(), name.as_deref(), &risk, &category, cli.json),
            SkillAction::Search { query, limit } => run_skill_search(&query, limit, cli.json),
            SkillAction::List => run_skill_list(cli.json),
            SkillAction::Install { skills, dir } => run_skill_install(&skills, dir.as_ref(), cli.json),
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

fn run_solve(bundle_path: &Path, query: &str, json: bool) -> Result<(), anyhow::Error> {
    if query.trim().is_empty() {
        anyhow::bail!("Error: query cannot be empty. Please provide a search query.");
    }
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
    if query.trim().is_empty() {
        anyhow::bail!("Error: query cannot be empty. Please provide a search query.");
    }
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
            Commands::Solve { bundle, query } => {
                assert_eq!(bundle, std::path::PathBuf::from("b.atlas"));
                assert_eq!(query, "my query");
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

