# Pattern: Progressive Disclosure

Tiered memory loading ensures agents remain efficient by only loading full details when necessary.

## Context

Modern LLMs have large context windows, but performance (latency, cost, and alignment) degrades as the window fills with irrelevant data. Engram uses "Progressive Disclosure" to provide a top-down view of memory.

## Implementation

### 1. Unified Manifest
Instead of loading all facts, agents first call `memory_get_context` (or the newer `memory_get_manifest`). This returns:
- **Pinned Facts**: Full content of essential memories.
- **Category Summaries**: A high-level overview of other categories (e.g., "5 facts about Project X").
- **Available Scopes**: The boundaries of searchable knowledge.

### 2. Tiered Loading
1. **Tier 0 (Pinned)**: facts marked with `pinned: true`. Always in context.
2. **Tier 1 (Summaries)**: One-line `summary` field of unpinned facts.
3. **Tier 2 (On-Demand)**: Full content retrieved via `memory_recall` or `memory_search` when the agent detects a specific needle-in-haystack need.

### 3. Summary Field
Every fact should have a `summary` (1-sentence description) distinct from its `content`.
- **UX**: Displayed in manifests and search results.
- **Retrieval**: Used as the primary lookup for high-level reasoning.

## Best Practices
- **Auto-Summarization**: Use the enrichment pipeline to generate summaries on creation.
- **Pinning Limits**: Cap pinned facts (default 20) to prevent context bloat.
- **Intent Detection**: Agents should use `load_budgeted_facts` to let Engram handle the disclosure logic automatically.
