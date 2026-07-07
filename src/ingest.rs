#[derive(Debug, Clone)]
pub struct Document {
    pub id: &'static str,
    pub title: &'static str,
    pub body: &'static str,
}

pub fn load_demo_documents() -> Vec<Document> {
    vec![
        Document {
            id: "policy-001",
            title: "Travel Policy",
            body: include_str!("../data/policies/travel_policy.md"),
        },
        Document {
            id: "policy-002",
            title: "Expense Policy",
            body: include_str!("../data/policies/expense_policy.md"),
        },
        Document {
            id: "policy-003",
            title: "PTO Policy",
            body: include_str!("../data/policies/pto_policy.md"),
        },
    ]
}
