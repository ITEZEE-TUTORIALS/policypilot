#[derive(Debug, Clone)]
pub struct Document {
    pub id: &'static str,
    pub title: &'static str,
    pub body: &'static str,
}

pub fn load_demo_documents() -> Vec<Document> {
    vec![
        Document {
            id: "gitlab-policy-001",
            title: "GitLab Global Travel and Expense Policy",
            body: include_str!("../data/gitlab/global_travel_expense.md"),
        },
        Document {
            id: "gitlab-policy-002",
            title: "GitLab Travel Safety and Security",
            body: include_str!("../data/gitlab/travel_safety.md"),
        },
    ]
}
