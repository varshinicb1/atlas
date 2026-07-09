#[cfg(test)]
mod tests {
    use atlas_ir::embedding;

    #[test]
    fn test_embedding_compute() {
        let embs = embedding::compute(&["hello world".to_string(), "test".to_string()], 128);
        assert_eq!(embs.len(), 2);
        assert_eq!(embs[0].len(), 128);
    }
}