#![recursion_limit = "2048"]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub id: String,
    pub semantic: f32,
    pub lexical: f32,
    pub importance: f32,
    pub freshness: f32,
    pub outcome: f32,
    pub emotional: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weights {
    pub semantic: f32,
    pub lexical: f32,
    pub importance: f32,
    pub freshness: f32,
    pub outcome: f32,
    pub emotional: f32,
}

impl Default for Weights {
    fn default() -> Self {
        Self {
            semantic: 0.4,
            lexical: 0.15,
            importance: 0.2,
            freshness: 0.1,
            outcome: 0.1,
            emotional: 0.05,
        }
    }
}

pub fn freshness_score(timestamp: i64, current_time: i64, relevance_score: Option<f32>) -> f32 {
    let age_ms = (current_time - timestamp).max(0) as f32;
    let age_days = age_ms / (24.0 * 60.0 * 60.0 * 1000.0);
    let time_freshness = (1.0 - age_days / 30.0).clamp(0.0, 1.0);

    if let Some(rs) = relevance_score {
        let decay_freshness = rs.clamp(0.0, 1.0);
        0.6 * time_freshness + 0.4 * decay_freshness
    } else {
        time_freshness
    }
}

pub fn lexical_score(query: &str, content: Option<&str>) -> f32 {
    let content = match content {
        Some(c) => c.to_lowercase(),
        None => return 0.0,
    };
    let query = query.to_lowercase();
    let q_trimmed = query.trim();
    if q_trimmed.is_empty() { return 0.0; }

    let tokens: Vec<&str> = q_trimmed.split_whitespace().collect();
    if tokens.is_empty() { return 0.0; }

    let hits = tokens.iter().filter(|&&t| content.contains(t)).count();
    (hits as f32 / tokens.len() as f32).clamp(0.0, 1.0)
}

pub fn score_candidate(c: &Candidate, w: &Weights) -> f32 {
    c.semantic * w.semantic +
    c.lexical * w.lexical +
    c.importance * w.importance +
    c.freshness * w.freshness +
    c.outcome * w.outcome +
    c.emotional * w.emotional
}

pub fn rrf_combine(results: Vec<Vec<String>>, k: usize) -> Vec<String> {
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    let constant = 60.0;

    for list in results {
        for (rank, id) in list.iter().enumerate() {
            let score = 1.0 / (constant + (rank + 1) as f32);
            *scores.entry(id.clone()).or_insert(0.0) += score;
        }
    }

    let mut combined: Vec<_> = scores.into_iter().collect();
    combined.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    combined.into_iter().take(k).map(|(id, _)| id).collect()
}
