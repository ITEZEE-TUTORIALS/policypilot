use crate::answer::AnswerReport;
use crate::ingest::Document;
use crate::retrieve::RetrievedChunk;

#[derive(Debug, Clone)]
pub struct RigPack {
    pub uses_full_policy_bundle: bool,
    pub system_prompt: String,
    pub user_prompt: String,
    pub prompt_text: String,
    pub curl_command: String,
}

pub fn build_rig_pack(report: &AnswerReport, policies: &[Document]) -> RigPack {
    let uses_full_policy_bundle = should_use_full_policy_bundle(report);
    let question = report.question.trim();
    let source_summary = render_source_summary(&report.sources);
    let policy_context = if uses_full_policy_bundle {
        render_full_policy_bundle(policies)
    } else {
        render_relevant_policy_context(report)
    };

    let system_prompt = [
        "You are PolicyPilot, a careful policy assistant.",
        "Answer only from the provided policy context.",
        "If the context is weak or unrelated, say you could not find a grounded policy answer.",
        "Prefer exact policy language, include limits and exceptions, and keep the answer concise.",
    ]
    .join(" ");

    let user_prompt = [
        format!(
            "Question: {}",
            if question.is_empty() {
                "Unknown question"
            } else {
                question
            }
        ),
        String::new(),
        "Retrieved sources:".to_string(),
        if source_summary.is_empty() {
            "No sources returned.".to_string()
        } else {
            source_summary
        },
        String::new(),
        if uses_full_policy_bundle {
            "Full policy bundle:".to_string()
        } else {
            "Relevant policy excerpt:".to_string()
        },
        policy_context,
        String::new(),
        "Answer format:".to_string(),
        "- Start with the direct answer.".to_string(),
        "- Quote the exact supporting line or clause.".to_string(),
        "- If there is a limit or exception, state it clearly.".to_string(),
        "- If the policy evidence is not strong enough, say so instead of guessing.".to_string(),
    ]
    .join("\n");

    let prompt_text = ["System:", &system_prompt, "", "User:", &user_prompt].join("\n");

    let curl_command = build_curl_command(&system_prompt, &user_prompt);

    RigPack {
        uses_full_policy_bundle,
        system_prompt,
        user_prompt,
        prompt_text,
        curl_command,
    }
}

fn should_use_full_policy_bundle(report: &AnswerReport) -> bool {
    let excerpt = report.relevant_excerpt.trim();
    if excerpt.is_empty() {
        return true;
    }

    let sources = &report.sources;
    let top_score = sources.first().map(|source| source.score).unwrap_or(0.0);
    !top_score.is_finite() || top_score < 0.24
}

fn render_source_summary(sources: &[RetrievedChunk]) -> String {
    if sources.is_empty() {
        return String::new();
    }

    sources
        .iter()
        .map(|source| {
            let score_text = if source.score.is_finite() {
                format!("{:.3}", source.score)
            } else {
                "n/a".to_string()
            };
            let section = source
                .section
                .as_ref()
                .map(|section| format!(" | {}", section))
                .unwrap_or_default();

            format!(
                "- {} ({}{}) similarity {}",
                source.title, source.document_id, section, score_text
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_relevant_policy_context(report: &AnswerReport) -> String {
    let excerpt = report.relevant_excerpt.trim();
    if excerpt.is_empty() {
        "No relevant excerpt was returned.".to_string()
    } else {
        excerpt.to_string()
    }
}

fn render_full_policy_bundle(policies: &[Document]) -> String {
    if policies.is_empty() {
        return "Policy bundle unavailable.".to_string();
    }

    policies
        .iter()
        .map(|policy| {
            [
                format!("# {} ({})", policy.title, policy.id),
                policy.body.trim().to_string(),
            ]
            .join("\n")
        })
        .collect::<Vec<_>>()
        .join("\n\n---\n\n")
}

fn build_curl_command(system_prompt: &str, user_prompt: &str) -> String {
    let system_json = json_string(system_prompt);
    let user_json = json_string(user_prompt);
    let body = format!(
        "{{\"model\":\"gpt-4.1-mini\",\"messages\":[{{\"role\":\"system\",\"content\":{system_json}}},{{\"role\":\"user\",\"content\":{user_json}}}]}}"
    );

    vec![
        "export OPENAI_API_KEY=sk-your-openai-token-here".to_string(),
        String::new(),
        "curl https://api.openai.com/v1/chat/completions \\".to_string(),
        "  -H \"Content-Type: application/json\" \\".to_string(),
        "  -H \"Authorization: Bearer $OPENAI_API_KEY\" \\".to_string(),
        "  -d @- <<'JSON'".to_string(),
        body,
        "JSON".to_string(),
    ]
    .join("\n")
}

fn json_string(value: &str) -> String {
    let mut escaped = String::from("\"");

    for ch in value.chars() {
        match ch {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            c if c.is_control() => escaped.push_str(&format!("\\u{:04x}", c as u32)),
            c => escaped.push(c),
        }
    }

    escaped.push('"');
    escaped
}

#[cfg(test)]
mod tests {
    use super::build_rig_pack;
    use crate::answer::AnswerReport;
    use crate::ingest::Document;
    use crate::retrieve::RetrievedChunk;

    fn sample_report(question: &str, excerpt: &str, score: f32) -> AnswerReport {
        AnswerReport {
            question: question.to_string(),
            answer: "placeholder".to_string(),
            relevant_excerpt: excerpt.to_string(),
            sources: vec![RetrievedChunk {
                document_id: "policy-1".to_string(),
                title: "Expense Policy".to_string(),
                section: Some("chunk-1".to_string()),
                text: excerpt.to_string(),
                score,
            }],
            top_match_title: Some("Expense Policy".to_string()),
            rig_pack: None,
        }
    }

    #[test]
    fn focused_handoff_keeps_the_relevant_excerpt() {
        let report = sample_report(
            "Can I expense a hotel minibar?",
            "- Hotel minibar charges are not reimbursable.",
            0.91,
        );
        let policies = vec![Document {
            id: "policy-1",
            title: "Expense Policy",
            body: "- Hotel minibar charges are not reimbursable.",
        }];

        let pack = build_rig_pack(&report, &policies);

        assert!(!pack.uses_full_policy_bundle);
        assert!(pack.prompt_text.contains("Relevant policy excerpt"));
        assert!(pack
            .prompt_text
            .contains("Hotel minibar charges are not reimbursable"));
        assert!(pack.curl_command.contains("OPENAI_API_KEY"));
    }

    #[test]
    fn low_confidence_handoff_expands_to_full_bundle() {
        let report = sample_report("What is the policy?", "", 0.05);
        let policies = vec![Document {
            id: "policy-1",
            title: "Expense Policy",
            body: "Receipts are required for reimbursements over 25 dollars.",
        }];

        let pack = build_rig_pack(&report, &policies);

        assert!(pack.uses_full_policy_bundle);
        assert!(pack.prompt_text.contains("Full policy bundle"));
        assert!(pack
            .prompt_text
            .contains("Receipts are required for reimbursements over 25 dollars."));
    }
}
