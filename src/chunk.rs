use crate::ingest::Document;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub document_id: String,
    pub title: String,
    pub section: Option<String>,
    pub text: String,
}

pub fn split_documents(documents: &[Document]) -> Vec<Chunk> {
    let mut chunks = Vec::new();

    for document in documents {
        let sections: Vec<&str> = document
            .body
            .split("\n\n")
            .map(str::trim)
            .filter(|part| !part.is_empty())
            .collect();

        let mut body_sections = sections.as_slice();
        let heading = if body_sections
            .first()
            .map(|section| section.starts_with('#'))
            .unwrap_or(false)
        {
            body_sections = &body_sections[1..];
            Some(sections[0])
        } else {
            None
        };

        for (index, section) in body_sections.iter().enumerate() {
            let text = if let Some(heading) = heading {
                format!("{heading}\n\n{section}")
            } else {
                (*section).to_string()
            };

            chunks.push(Chunk {
                document_id: document.id.to_string(),
                title: document.title.to_string(),
                section: Some(format!("chunk-{}", index + 1)),
                text,
            });
        }
    }

    chunks
}
