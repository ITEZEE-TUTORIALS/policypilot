mod answer;
mod chunk;
mod embed;
mod ingest;
mod retrieve;
mod store;

use std::env;

fn main() {
    let question = env::args()
        .nth(1)
        .unwrap_or_else(|| "Can I expense a hotel minibar?".to_string());

    // This is a presentation scaffold, so the data flow is intentionally simple.
    let documents = ingest::load_demo_documents();
    let chunks = chunk::split_documents(&documents);
    let query_embedding = embed::embed_text(&question);
    let store = store::VectorStore::from_chunks(chunks);
    let matches = retrieve::search(&store, &query_embedding, 3);
    let response = answer::draft_answer(&question, &matches);

    println!("{response}");
}
