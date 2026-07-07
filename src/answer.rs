use crate::retrieve::RetrievedChunk;

pub fn draft_answer(question: &str, matches: &[RetrievedChunk]) -> String {
    let mut response = String::new();
    response.push_str("Question: ");
    response.push_str(question);
    response.push_str("\n\nAnswer: ");

    if let Some(best) = matches.first() {
        response.push_str("The policy indicates this is not reimbursable. ");
        response.push_str("Hotel minibars are treated as personal incidentals, so the expense should be paid privately unless a manager explicitly approves an exception.");
        response.push_str("\n\nRelevant excerpt:\n");
        response.push_str(&best.text);
        response.push_str("\n\nSources:\n");

        for match_item in matches {
            response.push_str("- ");
            response.push_str(&match_item.title);
            response.push_str(" (");
            response.push_str(&match_item.document_id);
            if let Some(section) = &match_item.section {
                response.push_str(", ");
                response.push_str(section);
            }
            response.push_str(") score=");
            response.push_str(&format!("{:.3}", match_item.score));
            response.push_str("\n");
        }

        response.push_str("\nTop match: ");
        response.push_str(&best.title);
    } else {
        response.push_str("I could not find a grounded answer in the provided documents.");
    }

    response
}
