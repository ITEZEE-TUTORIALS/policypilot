use crate::answer::AnswerReport;
use crate::{answer, chunk, embed, ingest, retrieve, rig, store};

pub const SAMPLE_QUESTIONS: [&str; 3] = [
    "Can I expense a hotel minibar?",
    "Are hotel stays reimbursable?",
    "What happens if travel feels unsafe?",
];

pub fn answer_question(question: &str) -> AnswerReport {
    let documents = ingest::load_demo_documents();
    let chunks = chunk::split_documents(&documents);
    let query_embedding = embed::embed_query_text(question);
    let store = store::VectorStore::from_chunks(chunks);
    let matches = retrieve::search(&store, &query_embedding, usize::MAX);
    let mut report = answer::draft_answer(question, &matches);
    report.rig_pack = Some(rig::build_rig_pack(&report, &documents));
    report
}
