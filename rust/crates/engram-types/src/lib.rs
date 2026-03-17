#![recursion_limit = "2048"]
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fact {
    pub id: String,
    pub content: String,
    pub factual_summary: Option<String>,
    pub timestamp: i64,
    pub updated_at: Option<i64>,
    pub vault_path: Option<String>,
    pub vault_synced_at: Option<i64>,
    pub confidence: Option<f64>,
    pub importance_tier: Option<String>,
    pub observation_tier: Option<String>,
    pub observation_compressed: Option<bool>,
    pub observation_generation: Option<i32>,
    pub observation_session_id: Option<String>,
    pub assertion_type: Option<String>,
    pub referenced_date: Option<i64>,
    pub observation_original_content: Option<String>,
    pub source: String,
    pub entity_ids: Vec<String>,
    pub relevance_score: f64,
    pub accessed_count: i32,
    pub importance_score: f64,
    pub merged_content: Option<String>,
    pub outcome_score: Option<f64>,
    pub created_by: String,
    pub contributing_agents: Option<Vec<String>>,
    pub conversation_id: Option<String>,
    pub scope_id: String,
    pub tags: Vec<String>,
    pub fact_type: String,
    pub embedding: Option<Vec<f64>>,
    pub compact_embedding: Option<Vec<f64>>,
    pub lifecycle_state: String,
    pub merged_into: Option<String>,
    pub consolidated_from: Option<Vec<String>>,
    pub superseded_by: Option<String>,
    pub forget_score: Option<f64>,
    pub token_estimate: Option<i32>,
    pub pinned: Option<bool>,
    pub summary: Option<String>,
    pub emotional_context: Option<String>,
    pub emotional_weight: Option<f64>,
    pub last_contradiction_check: Option<i64>,
    pub contradicts_with: Option<Vec<String>>,
    pub qa_question: Option<String>,
    pub qa_answer: Option<String>,
    pub qa_entities: Option<Vec<String>>,
    pub qa_confidence: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub entity_id: String,
    pub name: String,
    pub entity_type: String,
    pub first_seen: i64,
    pub last_seen: i64,
    pub metadata: Value,
    pub backlinks: Option<Vec<String>>,
    pub relationships: Vec<Relationship>,
    pub importance_score: f64,
    pub access_count: i32,
    pub created_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    pub target_id: String,
    pub relation_type: String,
    pub since: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub session_id: String,
    pub participants: Vec<String>,
    pub thread_facts: Vec<String>,
    pub context_summary: String,
    pub importance: f64,
    pub tags: Vec<String>,
    pub handoffs: Vec<Handoff>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    pub id: String,
    pub title: String,
    pub summary: Option<String>,
    pub agent_id: String,
    pub scope_id: String,
    pub fact_ids: Vec<String>,
    pub start_time: i64,
    pub end_time: Option<i64>,
    pub tags: Vec<String>,
    pub importance_score: f64,
    pub created_at: i64,
    pub embedding: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Handoff {
    pub from_agent: String,
    pub to_agent: String,
    pub timestamp: i64,
    pub context_summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryScope {
    pub name: String,
    pub description: String,
    pub members: Vec<String>,
    pub read_policy: String,
    pub write_policy: String,
    pub admin_policy: Option<String>,
    pub retention_days: Option<i32>,
    pub memory_policy: Option<MemoryPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryPolicy {
    pub max_facts: Option<i32>,
    pub decay_rate: Option<f64>,
    pub prioritize_types: Option<Vec<String>>,
    pub auto_forget: Option<bool>,
    pub compression_strategy: Option<String>,
    pub compaction_threshold_bytes: Option<i32>,
    pub dedup_threshold: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Signal {
    pub fact_id: Option<String>,
    pub session_id: Option<String>,
    pub agent_id: String,
    pub signal_type: String,
    pub value: f64,
    pub comment: Option<String>,
    pub confidence: Option<f64>,
    pub context: Option<String>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub name: String,
    pub description: String,
    pub fact_ids: Vec<String>,
    pub entity_ids: Vec<String>,
    pub scope_id: String,
    pub importance: f64,
    pub last_updated: i64,
    pub embedding: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgeSubspace {
    pub name: String,
    pub description: Option<String>,
    pub agent_id: String,
    pub scope_id: String,
    pub fact_ids: Vec<String>,
    pub centroid: Option<Vec<f64>>,
    pub principal_vectors: Option<Vec<Vec<f64>>>,
    pub k: Option<i32>,
    pub version: Option<i32>,
    pub novelty_threshold: Option<f64>,
    pub dimensionality: Option<i32>,
    pub variance: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryBlock {
    pub label: String,
    pub value: String,
    pub character_limit: i32,
    pub scope_id: String,
    pub created_by: String,
    pub version: i32,
    pub shared: Option<bool>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentPerformance {
    pub agent_id: String,
    pub task_type: String,
    pub start_time: i64,
    pub end_time: i64,
    pub success: bool,
    pub lines_changed: i32,
    pub tests_added: i32,
    pub violations: Vec<String>,
    pub quality_grade_before: String,
    pub quality_grade_after: String,
    pub template_used: Option<String>,
    pub patterns_followed: Vec<String>,
    pub patterns_violated: Vec<String>,
    pub rollback_required: bool,
    pub was_helpful: Option<bool>,
    pub reused_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FactVersion {
    pub fact_id: String,
    pub previous_content: String,
    pub previous_importance: Option<f64>,
    pub previous_tags: Option<Vec<String>>,
    pub changed_by: String,
    pub change_type: String,
    pub reason: Option<String>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdapterModule {
    pub agent_id: String,
    pub scope_id: String,
    pub name: String,
    pub mode: String,
    pub status: String,
    pub document: Option<String>,
    pub document_checksum: Option<String>,
    pub fact_count: Option<i32>,
    pub token_estimate: Option<i32>,
    pub base_model: Option<String>,
    pub lora_config: Option<LoraConfig>,
    pub last_generated_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoraConfig {
    pub rank: i32,
    pub target_modules: Vec<String>,
    pub alpha: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncLog {
    pub node_id: String,
    pub last_sync_timestamp: i64,
    pub facts_synced: i32,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub agent_id: String,
    pub fact_id: String,
    pub reason: String,
    pub read: bool,
    pub created_at: i64,
    pub expires_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemConfig {
    pub key: String,
    pub value: Value,
    pub category: String,
    pub description: String,
    pub version: i32,
    pub updated_at: i64,
    pub updated_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryEvent {
    pub event_type: String,
    pub fact_id: Option<String>,
    pub scope_id: Option<String>,
    pub agent_id: Option<String>,
    pub payload: Option<Value>,
    pub watermark: i64,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyValueFact {
    pub key: String,
    pub value: String,
    pub agent_id: String,
    pub scope_id: String,
    pub category: Option<String>,
    pub metadata: Option<Value>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservationSession {
    pub scope_id: String,
    pub agent_id: String,
    pub pending_token_estimate: i32,
    pub summary_token_estimate: i32,
    pub observer_threshold: i32,
    pub reflector_threshold: i32,
    pub last_observer_run: Option<i64>,
    pub last_reflector_run: Option<i64>,
    pub observer_generation: i32,
    pub reflector_generation: i32,
    pub compression_level: i32,
    pub buffer_fact_id: Option<String>,
    pub buffer_ready: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopeMembership {
    pub agent_id: String,
    pub scope_id: String,
    pub role: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockVersion {
    pub block_id: String,
    pub previous_value: String,
    pub change_type: String,
    pub changed_by: String,
    pub reason: Option<String>,
    pub created_at: i64,
}
