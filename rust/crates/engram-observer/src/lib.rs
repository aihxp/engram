use engram_types::Fact;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TieredManifest {
    pub core_facts: Vec<Fact>,
    pub relevant_episodes: Vec<String>,
    pub recent_signals: Vec<String>,
    pub token_usage: TokenUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub total: i32,
    pub limit: i32,
    pub remains: i32,
}

pub struct Observer {
    pub budget: i32,
}

impl Observer {
    pub fn new(budget: i32) -> Self {
        Self { budget }
    }

    pub fn generate_manifest(&self, facts: Vec<Fact>, limit: i32) -> TieredManifest {
        let mut core_facts = Vec::new();
        let mut current_tokens = 0;

        // Simplified progressive disclosure logic
        for fact in facts {
            let tokens = fact.token_estimate.unwrap_or(100);
            if current_tokens + tokens <= limit {
                core_facts.push(fact);
                current_tokens += tokens;
            } else {
                 break;
            }
        }

        TieredManifest {
            core_facts,
            relevant_episodes: vec![],
            recent_signals: vec![],
            token_usage: TokenUsage {
                total: current_tokens,
                limit,
                remains: limit - current_tokens,
            },
        }
    }
}
