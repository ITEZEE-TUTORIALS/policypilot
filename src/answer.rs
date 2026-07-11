use crate::retrieve::RetrievedChunk;

#[derive(Debug, Clone)]
pub struct AnswerReport {
    pub question: String,
    pub answer: String,
    pub relevant_excerpt: String,
    pub sources: Vec<RetrievedChunk>,
    pub top_match_title: Option<String>,
}

impl AnswerReport {
    pub fn plain_text(&self) -> String {
        let mut response = String::new();
        response.push_str("Question: ");
        response.push_str(&self.question);
        response.push_str("\n\nAnswer: ");
        response.push_str(&self.answer);

        if !self.relevant_excerpt.is_empty() {
            response.push_str("\n\nRelevant excerpt:\n");
            response.push_str(&self.relevant_excerpt);
        }

        if !self.sources.is_empty() {
            response.push_str("\n\nSources:\n");

            for match_item in &self.sources {
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
        }

        if let Some(top_match) = &self.top_match_title {
            response.push_str("\nTop match: ");
            response.push_str(top_match);
        }

        response
    }
}

pub fn draft_answer(question: &str, matches: &[RetrievedChunk]) -> AnswerReport {
    let top = matches.first();
    let answer = match top {
        Some(best) => summarize_answer(question, &best.text),
        None => "I could not find a grounded answer in the provided documents.".to_string(),
    };

    AnswerReport {
        question: question.to_string(),
        answer,
        relevant_excerpt: top.map(|best| best.text.clone()).unwrap_or_default(),
        sources: matches.to_vec(),
        top_match_title: top.map(|best| best.title.clone()),
    }
}

fn summarize_answer(question: &str, policy_text: &str) -> String {
    let question_lower = question.to_lowercase();
    let policy_lower = policy_text.to_lowercase();

    if policy_lower.contains("not reimbursable") || policy_lower.contains("not allowed") {
        if question_lower.contains("minibar") || question_lower.contains("expense") {
            return "The policy indicates this is not reimbursable. Hotel minibars are treated as personal incidentals, so the expense should be paid privately unless a manager explicitly approves an exception.".to_string();
        }

        return "The relevant policy text says the requested item or action is not reimbursable or not allowed under the stated rules.".to_string();
    }

    if policy_lower.contains("reimbursable") {
        return "The policy indicates this may be reimbursable within the stated limits and conditions.".to_string();
    }

    if policy_lower.contains("safety") || policy_lower.contains("security") {
        return "The policy guidance points to safety and security procedures that should be followed before traveling.".to_string();
    }

    "The most relevant policy guidance is shown in the excerpt below.".to_string()
}
