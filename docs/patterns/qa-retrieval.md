# Pattern: QA-Pair Retrieval

Structured memory representation optimized for precise retrieval and multi-hop reasoning.

## Context

Raw text facts are often ambiguous for semantic search. Representing knowledge as Question-Answer (QA) pairs ("Panini" pattern) dramatically improves retrieval accuracy by matching the *intent* of the agent's query.

## Implementation

### 1. Fact Transformations
The enrichment pipeline (Phase 3+) can transform a single observation into multiple QA pairs.
- **Observation**: "The server is running on port 8080."
- **QA Pair**: `Q: What port is the server running on? A: 8080`

### 2. Intent Matching
When an agent asks a question, Engram ranks QA pairs higher than raw text if the query matches the "Q" component of the pair.

### 3. Multi-Hop Chains
QA pairs enable agents to "follow the trail":
1. Agent asks Q1.
2. Recall finds Fact A (QA pair).
3. Fact A triggers a follow-up Q2 embedded in its metadata.
4. Agent asks Q2 to find Fact B.

## Best Practices
- **Variety**: Generate multiple questions for the same piece of information (e.g., "Who", "Where", "How").
- **Integration**: Use `record_feedback` to signal which QA pairs successfully resolved ambiguities.
- **Synthetic QA**: Use the `memory_synthesize_qa` tool during sleep-time reflection to build these pairs from session logs.
