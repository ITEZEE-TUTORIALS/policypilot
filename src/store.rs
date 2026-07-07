use crate::chunk::Chunk;
use crate::embed::{cosine_similarity, embed_text, Embedding};
use crate::retrieve::RetrievedChunk;

#[derive(Debug, Clone)]
pub struct StoredChunk {
    pub document_id: String,
    pub title: String,
    pub section: Option<String>,
    pub text: String,
    pub embedding: Embedding,
}

#[derive(Debug, Default, Clone)]
pub struct VectorStore {
    items: Vec<StoredChunk>,
}

impl VectorStore {
    pub fn from_chunks(chunks: Vec<Chunk>) -> Self {
        let items = chunks
            .into_iter()
            .map(|chunk| StoredChunk {
                document_id: chunk.document_id,
                title: chunk.title,
                section: chunk.section,
                text: chunk.text.clone(),
                embedding: embed_text(&chunk.text),
            })
            .collect();

        Self { items }
    }

    pub fn search(&self, query: &Embedding, limit: usize) -> Vec<RetrievedChunk> {
        let mut scored: Vec<RetrievedChunk> = self
            .items
            .iter()
            .map(|item| RetrievedChunk {
                document_id: item.document_id.clone(),
                title: item.title.clone(),
                section: item.section.clone(),
                text: item.text.clone(),
                score: cosine_similarity(&item.embedding, query),
            })
            .collect();

        scored.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.title.cmp(&b.title))
        });
        scored.truncate(limit);
        scored
    }
}
