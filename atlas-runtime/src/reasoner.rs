pub trait Reasoner {
    fn reason(&self, query: &str, context: &ReasonContext) -> Result<String, anyhow::Error>;
}

pub struct ReasonContext<'a> {
    pub query: &'a str,
    pub bundle: &'a str,
    pub confidence: f32,
    pub nodes: &'a [&'a atlas_ir::Node],
    pub total_matches: usize,
    pub decide_result: Option<&'a crate::DecisionResult>,
}

pub struct TemplateReasoner;

impl Reasoner for TemplateReasoner {
    fn reason(&self, query: &str, ctx: &ReasonContext) -> Result<String, anyhow::Error> {
        let mut answer = String::new();
        
        // Header
        answer.push_str(&format!("## Answer for: {}\n\n", query));
        
        // Confidence summary
        let confidence_label = if ctx.confidence > 0.9 {
            "high confidence"
        } else if ctx.confidence > 0.7 {
            "good confidence" 
        } else if ctx.confidence > 0.4 {
            "moderate confidence"
        } else {
            "low confidence"
        };
        answer.push_str(&format!("**Confidence**: {} ({:.2})\n\n", confidence_label, ctx.confidence));
        
        // Decision result if available
        if let Some(decide) = &ctx.decide_result {
            answer.push_str("### Decision Path\n\n");
            answer.push_str(&format!("**Tree**: {}\n\n", decide.tree_id));
            answer.push_str(&format!("**Path**: {} → **{}**\n\n", 
                decide.path[..decide.path.len()-1].join(" → "),
                decide.path.last().unwrap_or(&"terminal".into())
            ));
            answer.push_str(&format!("**Rationale**: {}\n\n", decide.rationale));
            if !decide.recommendations.is_empty() {
                answer.push_str("**Recommendations**:\n");
                for rec in &decide.recommendations {
                    answer.push_str(&format!("- {} (confidence: {:.2})\n", rec.node_id, rec.confidence));
                }
                answer.push('\n');
            }
            if let Some(ref instructions) = decide.agent_instructions {
                answer.push_str(&format!("**Agent Instructions**: {}\n\n", instructions));
            }
        }
        
        // Knowledge nodes
        if !ctx.nodes.is_empty() {
            answer.push_str(&format!("### Matched Knowledge ({} of {})\n\n", ctx.nodes.len(), ctx.total_matches));
            for node in ctx.nodes {
                let kind = format!("{:?}", node.kind);
                answer.push_str(&format!("**{}** [{}]\n", node.name, kind));
                if let Some(ref desc) = node.description {
                    let snippet: String = desc.chars().take(200).collect();
                    answer.push_str(&format!(": {}\n", snippet));
                }
                if let Some(ref ver) = node.version {
                    answer.push_str(&format!("  _v{}_\n", ver));
                }
                answer.push('\n');
            }
        }
        
        if ctx.nodes.is_empty() {
            answer.push_str("No matching knowledge found in the graph.\n");
            answer.push_str("Try rephrasing your query or expanding the knowledge package.\n");
        }
        
        Ok(answer)
    }
}

#[cfg(feature = "ollama")]
pub struct OllamaReasoner {
    pub model: String,
    pub endpoint: String,
}

#[cfg(feature = "ollama")]
impl OllamaReasoner {
    pub fn new(model: &str, endpoint: &str) -> Self {
        Self {
            model: model.to_string(),
            endpoint: endpoint.trim_end_matches('/').to_string(),
        }
    }
}

#[cfg(feature = "ollama")]
impl Default for OllamaReasoner {
    fn default() -> Self {
        Self::new("phi3:mini", "http://localhost:11434")
    }
}

#[cfg(feature = "ollama")]
impl Reasoner for OllamaReasoner {
    fn reason(&self, query: &str, ctx: &ReasonContext) -> Result<String, anyhow::Error> {
        let template = TemplateReasoner;
        let facts = template.reason(query, ctx)?;

        let prompt = format!(
            r#"You are an expert engineering knowledge assistant. Given a query and relevant knowledge graph facts, synthesize a clear, accurate answer. Use only the facts provided. Do not fabricate APIs, versions, or concepts.

Query: {query}

Knowledge graph facts:
{facts}

Answer concisely and accurately:"#
        );

        let body = serde_json::json!({
            "model": self.model,
            "prompt": prompt,
            "stream": false,
            "options": {
                "temperature": 0.1,
                "num_predict": 512,
            }
        });

        let resp = ureq::post(&format!("{}/api/generate", self.endpoint))
            .header("Content-Type", "application/json")
            .send_json(&body)
            .map_err(|e| anyhow::anyhow!("Ollama request failed: {e}"))?;

        let data: serde_json::Value = resp.into_body().read_json()
            .map_err(|e| anyhow::anyhow!("Ollama response parse failed: {e}"))?;

        let text = data["response"].as_str().unwrap_or("").trim().to_string();
        Ok(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DecisionResult;
    use atlas_ir::{Node, NodeKind, NodeStatus, SourceRef};

    fn node(id: &str, name: &str) -> Node {
        Node {
            id: id.into(),
            kind: NodeKind::Concept,
            name: name.into(),
            version: Some("1.0".into()),
            category: None,
            provenance: vec![SourceRef {
                source_id: "s".into(),
                locator: "l".into(),
                line: None,
                hash: "h".into(),
            }],
            confidence: 1.0,
            status: NodeStatus::Verified,
            description: Some("A long description of the concept that should be truncated in output to keep it short.".into()),
            attributes: serde_json::json!({}),
        }
    }

    fn decide_result() -> DecisionResult {
        DecisionResult {
            tree_id: "tree:1".into(),
            recommendations: vec![atlas_ir::RecommendationItem {
                node_id: "concept:a".into(),
                confidence: 0.9,
            }],
            rationale: "Because it works.".into(),
            agent_instructions: Some("review manually".into()),
            path: vec!["start".into(), "end".into()],
        }
    }

    #[test]
    fn test_template_reasoner_with_nodes_and_decision() {
        let n = node("concept:a", "Alpha");
        let nodes = vec![&n];
        let dr = decide_result();
        let ctx = ReasonContext {
            query: "how do I use Alpha?",
            bundle: "demo",
            confidence: 0.95,
            nodes: &nodes,
            total_matches: 1,
            decide_result: Some(&dr),
        };
        let out = TemplateReasoner.reason("how do I use Alpha?", &ctx).unwrap();
        assert!(out.contains("## Answer for: how do I use Alpha?"));
        assert!(out.contains("high confidence"));
        assert!(out.contains("Alpha"));
        assert!(out.contains("Decision Path"));
        assert!(out.contains("tree:1"));
        assert!(out.contains("Because it works."));
        assert!(out.contains("review manually"));
        assert!(out.contains("_v1.0_"));
    }

    #[test]
    fn test_template_reasoner_low_confidence_no_nodes() {
        let nodes: Vec<&Node> = Vec::new();
        let ctx = ReasonContext {
            query: "missing thing",
            bundle: "demo",
            confidence: 0.0,
            nodes: &nodes,
            total_matches: 0,
            decide_result: None,
        };
        let out = TemplateReasoner.reason("missing thing", &ctx).unwrap();
        assert!(out.contains("low confidence"));
        assert!(out.contains("No matching knowledge found"));
    }

    #[test]
    fn test_template_reasoner_moderate_confidence() {
        let n = node("concept:b", "Beta");
        let nodes = vec![&n];
        let ctx = ReasonContext {
            query: "q",
            bundle: "demo",
            confidence: 0.5,
            nodes: &nodes,
            total_matches: 1,
            decide_result: None,
        };
        let out = TemplateReasoner.reason("q", &ctx).unwrap();
        assert!(out.contains("moderate confidence"));
    }
}

