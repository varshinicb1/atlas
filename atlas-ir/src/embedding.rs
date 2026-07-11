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

#[cfg(test)]
mod tests {
    use super::*;

    fn norm(v: &[f32]) -> f32 {
        v.iter().map(|x| x * x).sum::<f32>().sqrt()
    }

    #[test]
    fn test_compute_shape_and_count() {
        let e = compute(&["a b c".to_string(), "x y".to_string()], 64);
        assert_eq!(e.len(), 2);
        assert_eq!(e[0].len(), 64);
        assert_eq!(e[1].len(), 64);
    }

    #[test]
    fn test_determinism() {
        let a = compute(&["hello world foo".to_string()], 128);
        let b = compute(&["hello world foo".to_string()], 128);
        assert_eq!(a, b);
    }

    #[test]
    fn test_normalization() {
        let e = compute(&["the quick brown fox jumps".to_string()], 128);
        assert!((norm(&e[0]) - 1.0).abs() < 1e-5, "norm = {}", norm(&e[0]));
    }

    #[test]
    fn test_empty_text_normalizes_to_zero() {
        let e = compute(&["".to_string()], 128);
        assert!(norm(&e[0]).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_identical() {
        let e = compute(&["hello world".to_string()], 128);
        assert!((cosine_similarity(&e[0], &e[0]) - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_cosine_similarity_orthogonal_clamped() {
        let a = vec![1.0f32, 0.0, 0.0];
        let b = vec![0.0f32, 1.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_nearest_returns_best_match() {
        let embs = compute(
            &["alpha beta gamma".to_string(), "delta epsilon zeta".to_string()],
            128,
        );
        let q = compute(&["alpha beta gamma".to_string()], 128);
        let res = nearest(&embs, &q[0], 1);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].0, 0);
        assert!(res[0].1 >= 0.0);
    }

    #[test]
    fn test_nearest_top_k_and_threshold() {
        let embs = compute(
            &[
                "alpha beta gamma".to_string(),
                "totally unrelated words here".to_string(),
            ],
            128,
        );
        let q = compute(&["alpha beta gamma".to_string()], 128);
        let res = nearest_with_threshold(&embs, &q[0], 5, 0.99);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].0, 0);
        assert!(res[0].1 > 0.99);
    }

    #[test]
    fn test_nearest_empty() {
        let embs: Vec<Embedding> = Vec::new();
        let q = compute(&["x".to_string()], 128);
        assert!(nearest(&embs, &q[0], 3).is_empty());
    }

    #[test]
    fn test_embedding_index_struct() {
        let idx = EmbeddingIndex {
            embeddings: vec![vec![0.0; 4]],
            node_ids: vec!["n1".into()],
        };
        assert_eq!(idx.embeddings.len(), 1);
        assert_eq!(idx.node_ids[0], "n1");
    }
}
