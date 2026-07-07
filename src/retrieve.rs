use crate::embed::Embedding;
use crate::store::VectorStore;

#[derive(Debug, Clone)]
pub struct RetrievedChunk {
    pub document_id: String,
    pub title: String,
    pub section: Option<String>,
    pub text: String,
    pub score: f32,
}

pub fn search(store: &VectorStore, query: &Embedding, limit: usize) -> Vec<RetrievedChunk> {
    store.search(query, limit)
}
