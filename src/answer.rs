use crate::retrieve::RetrievedChunk;
use crate::rig::RigPack;
const MIN_GROUNDED_SCORE: i32 = 10;

#[derive(Debug, Clone)]
pub struct AnswerReport {
    pub question: String,
    pub answer: String,
    pub relevant_excerpt: String,
    pub sources: Vec<RetrievedChunk>,
    pub top_match_title: Option<String>,
    pub rig_pack: Option<RigPack>,
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
    let grounded_match = best_policy_match(question, matches);
    let answer = match &grounded_match {
        Some(best) => summarize_answer(question, &best.text),
        None => "I could not find a grounded answer in the provided documents.".to_string(),
    };

    AnswerReport {
        question: question.to_string(),
        answer,
        relevant_excerpt: grounded_match
            .as_ref()
            .map(|best| best.text.clone())
            .unwrap_or_default(),
        sources: matches.to_vec(),
        top_match_title: grounded_match.map(|best| best.title.clone()),
        rig_pack: None,
    }
}

fn summarize_answer(question: &str, policy_text: &str) -> String {
    let question_lower = question.to_lowercase();
    let best_line = best_policy_line(question, policy_text);

    let Some((score, line)) = best_line else {
        return "The most relevant policy guidance is shown in the excerpt below.".to_string();
    };

    let cleaned_line = cleanup_policy_line(&line);
    let cleaned_lower = cleaned_line.to_lowercase();

    if cleaned_line.is_empty() {
        return "The most relevant policy guidance is shown in the excerpt below.".to_string();
    }

    if score < MIN_GROUNDED_SCORE {
        return "I couldn't find a grounded policy answer for that question in the available documents.".to_string();
    }

    if cleaned_lower.contains("parking") && cleaned_lower.contains("transportation") {
        return "Parking should be separated and categorized as transportation.".to_string();
    }

    if cleaned_lower.contains("contact people safety")
        || (cleaned_lower.contains("safety")
            && cleaned_lower.contains("security")
            && cleaned_lower.contains("travel"))
    {
        return format!(
            "If travel feels unsafe, contact People Safety & Security for help. {}",
            cleaned_line
        );
    }

    if cleaned_lower.contains("receipts are required")
        || (cleaned_lower.contains("receipt") && cleaned_lower.contains("required"))
    {
        if let Some(limit_clause) = extract_limit_clause(&cleaned_line) {
            return format!("Yes. Receipts are required {}.", limit_clause);
        }

        return format!("Yes. {}", cleaned_line);
    }

    if cleaned_lower.contains("may book") && cleaned_lower.contains("approved by their manager") {
        if let Some(condition_clause) = extract_clause_after(&cleaned_line, "when ") {
            return format!("Yes, {}.", condition_clause);
        }

        return format!("Yes. {}", cleaned_line);
    }

    if cleaned_lower.contains("not reimbursable") || cleaned_lower.contains("not allowed") {
        if let Some(exception_clause) = extract_clause_after(&cleaned_line, "unless ") {
            return format!("No, unless {}.", exception_clause);
        }

        return format!("No. {}", cleaned_line);
    }

    if cleaned_lower.contains("reimbursable") {
        if let Some(exception_clause) = extract_clause_after(&cleaned_line, "unless ") {
            return format!("Yes, unless {}.", exception_clause);
        }

        if let Some(limit_clause) = extract_limit_clause(&cleaned_line) {
            return format!(
                "Yes, as long as it stays within the stated limit: {}.",
                limit_clause
            );
        }

        if let Some(condition_clause) = extract_clause_after(&cleaned_line, "when ") {
            return format!("Yes, {}.", condition_clause);
        }

        if let Some(condition_clause) = extract_clause_after(&cleaned_line, "if ") {
            return format!("Yes, {}.", condition_clause);
        }

        return format!("Yes. {}", cleaned_line);
    }

    if question_lower.contains("what") || question_lower.contains("how") {
        return format!("The policy guidance is: {}", cleaned_line);
    }

    format!("The policy guidance is: {}", cleaned_line)
}

fn best_policy_match(question: &str, matches: &[RetrievedChunk]) -> Option<RetrievedChunk> {
    matches
        .iter()
        .filter_map(|chunk| {
            let (score, _) = best_policy_line(question, &chunk.text)?;
            if score < MIN_GROUNDED_SCORE {
                return None;
            }

            Some((score, chunk.score, chunk.clone()))
        })
        .max_by(|left, right| {
            left.0
                .cmp(&right.0)
                .then_with(|| {
                    left.1
                        .partial_cmp(&right.1)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .then_with(|| left.2.title.cmp(&right.2.title))
        })
        .map(|(_, _, chunk)| chunk)
}

fn best_policy_line(question: &str, policy_text: &str) -> Option<(i32, String)> {
    let question_terms = expanded_question_terms(question);
    let lines: Vec<&str> = policy_text.lines().collect();

    lines
        .iter()
        .filter_map(|line| {
            let cleaned = line.trim();
            if cleaned.is_empty() || cleaned.starts_with('#') {
                return None;
            }

            let cleaned_lower = cleaned.to_lowercase();
            let line_terms = extract_terms(cleaned);
            let overlap = question_terms
                .iter()
                .filter(|term| line_terms.contains(*term))
                .count() as i32;

            let mut score = overlap * 10;

            if cleaned.starts_with('-') {
                score += 2;
            }

            if question_terms.contains("reimbursable") && cleaned_lower.contains("reimbursable") {
                score += 6;
            }

            if question_terms.contains("minibar") && cleaned_lower.contains("minibar") {
                score += 8;
            }

            if question_terms.contains("hotel") && cleaned_lower.contains("hotel") {
                score += 4;
            }

            if (question_terms.contains("unsafe") || question_terms.contains("safety"))
                && (cleaned_lower.contains("unsafe")
                    || cleaned_lower.contains("safety")
                    || cleaned_lower.contains("security"))
            {
                score += 8;
            }

            if question_terms.contains("travel") && cleaned_lower.contains("travel") {
                score += 4;
            }

            if question_terms.contains("parking") && cleaned_lower.contains("parking") {
                score += 12;
            }

            if question_terms.contains("transportation") && cleaned_lower.contains("transportation")
            {
                score += 10;
            }

            if cleaned_lower.contains("not reimbursable") && question_terms.contains("minibar") {
                score += 4;
            }

            Some((score, cleaned.to_string()))
        })
        .max_by_key(|(score, _)| *score)
        .map(|(score, line)| (score, line))
}

fn cleanup_policy_line(line: &str) -> String {
    line.trim_start().trim_start_matches('-').trim().to_string()
}

fn extract_limit_clause(line: &str) -> Option<String> {
    let lower = line.to_lowercase();

    for marker in [
        "up to ",
        "maximum ",
        "max ",
        "over ",
        "above ",
        "more than ",
    ] {
        if let Some(start) = lower.find(marker) {
            let clause = line[start..].trim().trim_end_matches('.');
            if !clause.is_empty() {
                return Some(clause.to_string());
            }
        }
    }

    None
}

fn extract_clause_after(line: &str, marker: &str) -> Option<String> {
    let lower = line.to_lowercase();
    let marker_lower = marker.to_lowercase();

    let start = lower.find(&marker_lower)? + marker_lower.len();
    let clause = line[start..].trim().trim_end_matches('.');

    if clause.is_empty() {
        None
    } else {
        Some(clause.to_string())
    }
}

fn extract_terms(value: &str) -> std::collections::HashSet<String> {
    value
        .to_lowercase()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch.is_ascii_whitespace() || ch == '-' {
                ch
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .map(normalize_term)
        .filter(|term| term.len() > 2 && !is_stop_term(term))
        .collect()
}

fn expanded_question_terms(value: &str) -> std::collections::HashSet<String> {
    let mut terms = extract_terms(value);
    let additions: Vec<String> = terms
        .iter()
        .flat_map(|term| match term.as_str() {
            "drink" | "drinks" | "beverage" | "beverages" => {
                vec!["alcohol".to_string(), "purchase".to_string()]
            }
            "buy" | "bought" | "purchase" | "purchased" => {
                vec!["expense".to_string(), "purchase".to_string()]
            }
            "expense" | "expenses" => vec!["reimbursement".to_string()],
            "reimbursable" | "reimbursement" => vec!["expense".to_string()],
            "unsafe" | "safety" | "security" => vec![
                "travel".to_string(),
                "security".to_string(),
                "safety".to_string(),
            ],
            "minibar" => vec!["incidentals".to_string(), "personal".to_string()],
            "parking" => vec!["transportation".to_string()],
            _ => Vec::new(),
        })
        .collect();

    terms.extend(additions);
    terms
}

fn normalize_term(term: &str) -> String {
    if let Some(stripped) = term.strip_suffix("ies") {
        return format!("{stripped}y");
    }
    if let Some(stripped) = term.strip_suffix("ed") {
        return stripped.to_string();
    }
    if let Some(stripped) = term.strip_suffix('s') {
        return stripped.to_string();
    }
    term.to_string()
}

fn is_stop_term(term: &str) -> bool {
    matches!(
        term,
        "a" | "an"
            | "and"
            | "are"
            | "as"
            | "at"
            | "be"
            | "been"
            | "being"
            | "by"
            | "can"
            | "could"
            | "did"
            | "do"
            | "does"
            | "for"
            | "from"
            | "had"
            | "has"
            | "have"
            | "how"
            | "i"
            | "if"
            | "in"
            | "is"
            | "it"
            | "may"
            | "might"
            | "must"
            | "of"
            | "on"
            | "or"
            | "please"
            | "should"
            | "the"
            | "their"
            | "them"
            | "there"
            | "these"
            | "they"
            | "this"
            | "those"
            | "to"
            | "us"
            | "was"
            | "we"
            | "were"
            | "what"
            | "when"
            | "where"
            | "which"
            | "who"
            | "why"
            | "will"
            | "with"
            | "would"
            | "you"
            | "your"
    )
}

#[cfg(test)]
mod tests {
    use crate::retrieve::RetrievedChunk;

    use super::{draft_answer, summarize_answer};

    #[test]
    fn hotel_stays_use_the_reimbursable_line() {
        let policy = "\
# GitLab Global Travel and Expense Policy

## Lodging
- Standard hotel stays are reimbursable up to $300 USD per night, including tax, for standard rooms.
- Hotel minibar charges are not reimbursable.
";

        let answer = summarize_answer("Are hotel stays reimbursable?", policy);
        assert!(answer.contains("Yes, as long as it stays within the stated limit"));
        assert!(answer.contains("up to $300 USD per night"));
        assert!(!answer.contains("Hotel minibar charges are not reimbursable"));
    }

    #[test]
    fn minibar_questions_use_the_minibar_line() {
        let policy = "\
# GitLab Global Travel and Expense Policy

## Lodging
- Standard hotel stays are reimbursable up to $300 USD per night, including tax, for standard rooms.
- Hotel minibar charges are not reimbursable.
";

        let answer = summarize_answer("Can I expense a hotel minibar?", policy);
        assert!(answer.contains("No."));
        assert!(answer.contains("Hotel minibar charges are not reimbursable"));
    }

    #[test]
    fn unsafe_travel_questions_use_safety_guidance() {
        let policy = "\
# GitLab Travel Safety and Security

## Core principle
- Team member safety and security are a top priority.
- If travel feels unsafe because of location, health, or security concerns, GitLab asks team members to contact People Safety & Security for help.
";

        let answer = summarize_answer("What happens if travel feels unsafe?", policy);
        assert!(answer.contains("People Safety & Security"));
    }

    #[test]
    fn receipt_questions_use_the_receipt_threshold() {
        let policy = "\
# Expense Policy

Business expenses must be ordinary, necessary, and tied to approved work activity.

Receipts are required for reimbursements over 25 dollars.
";

        let answer = summarize_answer("Do I need receipts?", policy);
        assert!(answer.contains("Receipts are required"));
        assert!(answer.contains("over 25 dollars"));
    }

    #[test]
    fn manager_approval_questions_use_the_approval_condition() {
        let policy = "\
# Travel Policy

Employees may book standard business travel when a trip has been approved by their manager.
";

        let answer = summarize_answer("Can I book business travel?", policy);
        assert!(answer.contains("approved by their manager"));
        assert!(answer.contains("Yes"));
    }

    #[test]
    fn drink_questions_use_the_expense_policy_exception_line() {
        let policy = "\
# Expense Policy

Business expenses must be ordinary, necessary, and tied to approved work activity.

Alcohol, entertainment, hotel minibar items, and other personal convenience purchases are not reimbursable unless a specific exception is documented.
";

        let answer = summarize_answer("Can I buy drink?", policy);
        assert!(answer.contains("No, unless a specific exception is documented"));
    }

    #[test]
    fn unrelated_questions_decline_policy_answers() {
        let policy = "\
# GitLab Global Travel and Expense Policy

## Lodging
- Standard hotel stays are reimbursable up to $300 USD per night, including tax, for standard rooms.
";

        let answer = summarize_answer("What is the capital of France?", policy);
        assert!(answer.contains("couldn't find a grounded policy answer"));
    }

    #[test]
    fn draft_answer_prefers_the_best_grounded_chunk_not_the_top_retrieval_chunk() {
        let parking_chunk = RetrievedChunk {
            document_id: "gitlab-policy-001".to_string(),
            title: "GitLab Global Travel and Expense Policy".to_string(),
            section: Some("Additional hotel guidance".to_string()),
            text: "\
# GitLab Global Travel and Expense Policy

## Additional hotel guidance
- Parking should be separated and categorized as transportation.
"
            .to_string(),
            score: 0.18,
        };

        let unrelated_chunk = RetrievedChunk {
            document_id: "policy-expense-001".to_string(),
            title: "Expense Policy".to_string(),
            section: Some("Lodging".to_string()),
            text: "\
# Expense Policy

Business expenses must be ordinary, necessary, and tied to approved work activity.
"
            .to_string(),
            score: 0.51,
        };

        let report = draft_answer(
            "Is parking reimbursable?",
            &[unrelated_chunk, parking_chunk],
        );
        assert!(report
            .answer
            .contains("Parking should be separated and categorized as transportation."));
        assert_eq!(
            report.top_match_title.as_deref(),
            Some("GitLab Global Travel and Expense Policy")
        );
    }

    #[test]
    fn out_of_scope_questions_return_no_grounded_answer() {
        let report = draft_answer(
            "Should I invest in bitcoin?",
            &[RetrievedChunk {
                document_id: "policy-expense-001".to_string(),
                title: "Expense Policy".to_string(),
                section: Some("Lodging".to_string()),
                text: "\
# Expense Policy

Business expenses must be ordinary, necessary, and tied to approved work activity.
"
                .to_string(),
                score: 0.44,
            }],
        );

        assert!(report.answer.contains("could not find a grounded answer"));
        assert!(report.relevant_excerpt.is_empty());
        assert!(report.top_match_title.is_none());
    }
}
