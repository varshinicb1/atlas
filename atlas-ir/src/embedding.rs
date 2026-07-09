use serde::{Deserialize, Serialize};

pub type Embedding = Vec<f32>;

pub const EMBEDDING_DIM: usize = 128;

pub fn compute(texts: &[String], dim: usize) -> Vec<Embedding> {
    texts.iter().map(|t| embed_one(t, dim)).collect()
}

fn embed_one(text: &str, dim: usize) -> Embedding {
    let mut vec = vec![0.0f32; dim];
    let lower = text.to_lowercase();
    let words: Vec<&str> = lower.split_whitespace().collect();

    for pair in words.windows(2) {
        let key = format!("{} {}", pair[0], pair[1]);
        let hash = blake3::hash(key.as_bytes());
        let bucket = u64::from_le_bytes(hash.as_bytes()[..8].try_into().expect("BLAKE3 output must be >= 8 bytes")) % dim as u64;
        vec[bucket as usize] += 1.0;
    }

    for word in &words {
        let hash = blake3::hash(word.as_bytes());
        let bucket = u64::from_le_bytes(hash.as_bytes()[..8].try_into().expect("BLAKE3 output must be >= 8 bytes")) % dim as u64;
        vec[bucket as usize] += 1.0;
    }

    let norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for v in &mut vec {
            *v /= norm;
        }
    }
    vec
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    dot.clamp(0.0, 1.0)
}

pub fn nearest(embeddings: &[Embedding], query: &Embedding, top_k: usize) -> Vec<(usize, f32)> {
    nearest_with_threshold(embeddings, query, top_k, 0.1)
}

pub fn nearest_with_threshold(embeddings: &[Embedding], query: &Embedding, top_k: usize, threshold: f32) -> Vec<(usize, f32)> {
    let mut scores: Vec<(usize, f32)> = embeddings
        .iter()
        .enumerate()
        .map(|(i, e)| (i, cosine_similarity(e, query)))
        .collect();
    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    scores.into_iter().take(top_k).filter(|(_, s)| *s > threshold).collect()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingIndex {
    pub embeddings: Vec<Embedding>,
    pub node_ids: Vec<String>,
}
