#[derive(Debug, Clone)]
pub struct Embedding {
    pub values: Vec<f32>,
}

const EMBEDDING_DIM: usize = 256;

pub fn embed_text(text: &str) -> Embedding {
    let mut values = vec![0.0_f32; EMBEDDING_DIM];

    for token in tokenize(text) {
        let index = hash_token(&token) % EMBEDDING_DIM;
        values[index] += 1.0;
    }

    normalize(&mut values);
    Embedding { values }
}

fn normalize_token(token: &str) -> String {
    let normalized = token
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .collect::<String>()
        .to_lowercase();

    if normalized.ends_with('s') && normalized.len() > 4 && !normalized.ends_with("ss") {
        normalized[..normalized.len() - 1].to_string()
    } else {
        normalized
    }
}

fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(normalize_token)
        .filter(|token| !token.is_empty())
        .collect()
}

fn hash_token(token: &str) -> usize {
    let mut hash = 0xcbf29ce484222325_u64;

    for byte in token.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }

    hash as usize
}

fn normalize(values: &mut [f32]) {
    let magnitude = values.iter().map(|value| value * value).sum::<f32>().sqrt();

    if magnitude > 0.0 {
        for value in values {
            *value /= magnitude;
        }
    }
}

pub fn cosine_similarity(left: &Embedding, right: &Embedding) -> f32 {
    left.values
        .iter()
        .zip(&right.values)
        .map(|(l, r)| l * r)
        .sum()
}
