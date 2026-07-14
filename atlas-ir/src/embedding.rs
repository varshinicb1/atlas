use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

pub type Embedding = Vec<f32>;

pub const EMBEDDING_DIM: usize = 128;

/// Hash-based embedding (legacy, backward-compatible)
pub fn compute_hash(texts: &[String]) -> Vec<Embedding> {
    texts.iter().map(|t| embed_hash_one(t)).collect()
}

fn embed_hash_one(text: &str) -> Embedding {
    let dim = EMBEDDING_DIM;
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

/// TF-IDF based embeddings. Significantly better than hash-based.
/// Returns (embeddings, vocab, idf_values).
pub fn compute_tfidf(texts: &[String]) -> (Vec<Embedding>, Vec<String>, Vec<f32>) {
    let num_docs = texts.len();
    if num_docs == 0 {
        return (vec![], vec![], vec![]);
    }

    // Tokenize all texts
    let tokenized: Vec<Vec<String>> = texts.iter().map(|t| tokenize(t)).collect();

    // Build vocabulary and document frequency
    let mut vocab_set = HashSet::new();
    let mut df: HashMap<String, usize> = HashMap::new();
    for tokens in &tokenized {
        let unique: HashSet<&str> = tokens.iter().map(|s| s.as_str()).collect();
        for token in unique {
            vocab_set.insert(token.to_string());
            *df.entry(token.to_string()).or_insert(0) += 1;
        }
    }

    let mut vocab: Vec<String> = vocab_set.into_iter().collect();
    vocab.sort();
    let dim = vocab.len();

    // Build vocab index
    let vocab_index: HashMap<&str, usize> = vocab.iter().enumerate().map(|(i, w)| (w.as_str(), i)).collect();

    // Compute IDF
    let mut idf = vec![0.0f32; dim];
    for (i, word) in vocab.iter().enumerate() {
        let doc_freq = df.get(word).copied().unwrap_or(1);
        idf[i] = ((num_docs as f32) / (doc_freq as f32)).ln() + 1.0;
    }

    // Compute TF-IDF vectors
    let mut embeddings: Vec<Embedding> = Vec::with_capacity(num_docs);
    for tokens in &tokenized {
        let mut vec = vec![0.0f32; dim];
        let mut tf: HashMap<usize, f32> = HashMap::new();
        for token in tokens {
            if let Some(&idx) = vocab_index.get(token.as_str()) {
                *tf.entry(idx).or_insert(0.0) += 1.0;
            }
        }
        let total = tokens.len() as f32;
        for (idx, count) in tf {
            let tf_val = count / total;
            vec[idx] = tf_val * idf[idx];
        }
        // L2 normalize
        let norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for v in &mut vec {
                *v /= norm;
            }
        }
        embeddings.push(vec);
    }

    (embeddings, vocab, idf)
}

/// Compute a TF-IDF query vector using a pre-built vocabulary and IDF values.
pub fn query_tfidf(query: &str, vocab: &[String], idf: &[f32]) -> Embedding {
    let dim = vocab.len();
    let tokens = tokenize(query);
    let vocab_index: HashMap<&str, usize> = vocab.iter().enumerate().map(|(i, w)| (w.as_str(), i)).collect();

    let mut vec = vec![0.0f32; dim];
    let mut tf: HashMap<usize, f32> = HashMap::new();
    for token in &tokens {
        if let Some(&idx) = vocab_index.get(token.as_str()) {
            *tf.entry(idx).or_insert(0.0) += 1.0;
        }
    }
    let total = tokens.len() as f32;
    for (idx, count) in tf {
        let tf_val = count / total;
        if idx < idf.len() {
            vec[idx] = tf_val * idf[idx];
        }
    }
    let norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for v in &mut vec {
            *v /= norm;
        }
    }
    vec
}

fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split_whitespace()
        .flat_map(|w| w.split(|c: char| !c.is_alphanumeric()))
        .filter(|w| !w.is_empty() && w.len() > 1)
        .map(|w| w.to_string())
        .collect()
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    dot.clamp(0.0, 1.0)
}

pub fn nearest(embeddings: &[Embedding], query: &Embedding, top_k: usize) -> Vec<(usize, f32)> {
    nearest_with_threshold(embeddings, query, top_k, 0.3)
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
    fn test_hash_shape_and_count() {
        let e = compute_hash(&["a b c".to_string(), "x y".to_string()]);
        assert_eq!(e.len(), 2);
        assert_eq!(e[0].len(), EMBEDDING_DIM);
    }

    #[test]
    fn test_hash_determinism() {
        let a = compute_hash(&["hello world foo".to_string()]);
        let b = compute_hash(&["hello world foo".to_string()]);
        assert_eq!(a, b);
    }

    #[test]
    fn test_tfidf_shape() {
        let (embs, vocab, idf) = compute_tfidf(&["hello world".to_string(), "foo bar".to_string()]);
        assert_eq!(embs.len(), 2);
        assert_eq!(vocab.len(), 4);
        assert_eq!(idf.len(), 4);
    }

    #[test]
    fn test_tfidf_normalization() {
        let (embs, _, _) = compute_tfidf(&["the quick brown fox jumps".to_string()]);
        assert!((norm(&embs[0]) - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_tfidf_similar_texts() {
        let texts = vec!["rust error handling result option".to_string(), "rust programming language systems".to_string()];
        let (embs, vocab, idf) = compute_tfidf(&texts);
        let q = query_tfidf("rust error handling", &vocab, &idf);
        // First text should be more similar than second
        let sim0 = cosine_similarity(&embs[0], &q);
        let sim1 = cosine_similarity(&embs[1], &q);
        assert!(sim0 > sim1, "sim0={} should be > sim1={}", sim0, sim1);
    }

    #[test]
    fn test_tfidf_orthogonal() {
        let (embs, _, _) = compute_tfidf(&["alpha beta".to_string(), "gamma delta".to_string()]);
        let sim = cosine_similarity(&embs[0], &embs[1]);
        assert!(sim < 0.01, "orthogonal texts should have near-zero similarity, got {}", sim);
    }

    #[test]
    fn test_tfidf_identical() {
        let (embs, vocab, idf) = compute_tfidf(&["hello world".to_string()]);
        let q = query_tfidf("hello world", &vocab, &idf);
        assert!((cosine_similarity(&embs[0], &q) - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_tfidf_empty_corpus() {
        let (embs, vocab, idf) = compute_tfidf(&[]);
        assert!(embs.is_empty());
        assert!(vocab.is_empty());
        assert!(idf.is_empty());
    }
}
