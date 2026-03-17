---
date: 2026-03-17
topic: unified-local-first-memory-plugin
---

# Unified Local-First Memory Plugin for AI Agents

## What We're Building

A **unified local-first memory plugin** that works across all AI agents (Claude Code, Codex, Cursor, Windsurf, OpenClaw, etc.). Built on Rust binaries + LanceDB for zero-cloud-dependency, high-performance, embedded memory storage.

Key features:
- **Codebase context** — Real-time code indexing with Tree-sitter (CocoIndex-inspired)
- **Agent memory** — Conversation context, preferences, decisions, session history
- **Knowledge graph** — GraphRAG for hierarchical reasoning (lance-graph)
- **Versioned storage** — Time-travel through memory states (lance-context)
- **Universal plugin** — Works with any agent via MCP

---

## Why This Approach

### What We Considered

1. **Engram optimization** — Add LanceDB as local cache to existing TypeScript/Convex stack
   - Pros: Incremental change, keeps existing functionality
   - Cons: Still requires Convex, complex dual-backend architecture

2. **Clean-slate Rust/LanceDB project** (chosen)
   - Pros: Zero cloud dependency, blazing fast (Rust), unified architecture, works everywhere
   - Cons: Starting from scratch

### Why LanceDB Ecosystem?

The LanceDB ecosystem provides exactly what we need:

| Component | Library | Why It Fits |
|-----------|---------|-------------|
| Storage | **lance-context** | Versioned, multimodal, time-travel |
| Knowledge | **lance-graph** | GraphRAG, Cypher queries, 1.35 Gelem/s |
| Code Indexing | **CocoIndex** pattern | Tree-sitter, incremental delta processing |
| Retrieval | **LanceDB** | Vector + FTS + structured filtering unified |

---

## Key Decisions

- **Storage**: LanceDB (embedded, no server needed)
- **Language**: Rust core + Python bindings + MCP server
- **Code Indexing**: Tree-sitter for syntax-aware chunking, incremental delta processing
- **Knowledge Graph**: lance-graph for GraphRAG workflows
- **Agent Integration**: MCP protocol for universal compatibility
- **Deployment**: Single binary, runs locally, zero config

---

## Reference Links

### Core Technologies
- **LanceDB**: https://lancedb.com
- **lance-context**: https://github.com/lance-format/lance-context
- **lance-graph**: https://github.com/lance-format/lance-graph
- **Lance format**: https://lance.org

### Research & Inspiration
- **OpenClaw + LanceDB Memory Layer**: https://lancedb.com/blog/openclaw-lancedb-memory-layer/
- **CocoIndex (incremental data pipeline)**: https://cocoindex.io
- **CocoIndex realtime-codebase-indexing**: https://github.com/cocoindex-io/realtime-codebase-indexing
- **CocoIndex + LanceDB integration**: https://lancedb.com/blog/keep-your-data-fresh-with-cocoindex-and-lancedb/
- **RAG optimization guide**: https://lancedb.com/blog/rag-isnt-one-size-fits-all/
- **GraphRAG hierarchical approach**: https://lancedb.com/blog/graphrag-hierarchical-approach-to-retrieval-augmented-generation/

### Similar Projects (for reference)
- **OpenClaw**: https://github.com/openclaw/openclaw
- **memory-lancedb-pro**: https://github.com/CortexReach/memory-lancedb-pro
- **lancedb-claw**: https://github.com/lancedb/lancedb-claw (official LanceDB OpenClaw plugin)
- **openclaw-lancedb-demo**: https://github.com/lancedb/openclaw-lancedb-demo (working demo)

### Architecture Patterns
- **Agent-Native Architectures**: https://every.to/guides/agent-native
  - Key principles: Parity, Granularity, Composability, Emergent capability
  - Files as universal interface
  - Context.md pattern for session state
- **Code Execution with MCP**: https://www.anthropic.com/engineering/code-execution-with-mcp
  - Present MCP servers as code APIs, not direct tool calls
  - 98.7% token reduction through on-demand tool loading
  - Progressive disclosure, privacy-preserving operations

### Distribution & Skill Management
- **The Library** (155 ⭐): https://github.com/disler/the-library
  - Meta-skill for private-first distribution of agentics
  - Catalog stores references (pointers), not copies
  - "package.json for agent capabilities" — manages skills/agents/prompts
  - Pure SKILL.md (no scripts) — agent IS the runtime
  - Agent-agnostic (Claude Code, Pi, etc.)
  - Supports local paths + GitHub private repos

---

## Comparison to Engram

| Aspect | Engram (current) | New Project |
|--------|------------------|-------------|
| Backend | Convex (cloud required) | LanceDB (local) |
| Language | TypeScript | Rust |
| Code Indexing | QMD (optional) | Tree-sitter + incremental |
| Knowledge Graph | Basic | lance-graph (GraphRAG) |
| Deployment | Convex + MCP server | Single binary |
| Cost | Cloud hosting | $0 (local) |
| Cross-device | Via Convex sync | Manual export/import |

---

## Key Learnings from Engram

### 1. Tool Registry Architecture (17 categories, 77 tools)

Engram organizes tools into modular sub-registries:
- **core-entries.ts**: store_fact, recall, search, get_context, observe
- **retrieval-entries.ts**: vector_search, text_search, hierarchical_recall
- **lifecycle-entries.ts**: update_fact, archive_fact, boost_relevance
- **context-entries.ts**: resolve_scopes, load_budgeted_facts
- **episodes-entries.ts**: create_episode, search_episodes, close_episode

**Pattern to adopt**: Declarative tool registry with Zod schemas + handlers, split by concern.

### 2. Fact Schema (55+ fields)

Engram's fact schema is extremely comprehensive:
- **Identity**: content, factType, tags, entityIds
- **Temporal**: timestamp, updatedAt, referencedDate, temporalLinks
- **Lifecycle**: lifecycleState, forgetScore, mergedInto, supersededBy
- **Richness**: qaQuestion, qaAnswer (QA-pair representation)
- **Emotional**: emotionalContext, emotionalWeight (GIZIN-inspired)
- **Richness**: importanceScore, confidence, outcomeScore (MemRL)
- **Subspace**: embedding, compactEmbedding (SVD compression)

**Pattern to adopt**: LanceDB can store all this; simplify but keep key richness fields.

### 3. Embedding Fallback Chain

```typescript
// 1. Cohere Embed v4 (cloud, best)
// 2. Ollama mxbai-embed-large (local)
// 3. Zero vector (fallback)
```

**Pattern to adopt**: Graceful degradation — cloud → local → none.

### 4. Async Enrichment Pipeline

Facts stored immediately (<50ms), then async:
- Embedding generation
- QA extraction
- Entity linking (autoLinkEntities)
- SVD compact projection

**Pattern to adopt**: Synchronous write, async enrichment via background tasks.

### 5. Active Forgetting (ALMA Pattern)

```typescript
shouldForget criteria:
- forgetScore > 0.8 (learned decay score)
- accessedCount < 2
- age > 30 days
- no temporal links
- not in active episode
```

**Pattern to adopt**: Memory that learns what to forget over time.

### 6. Token Budget for Recall

```typescript
applyTokenBudget(facts, maxTokens):
- Facts ordered by relevance
- Accumulate until budget exceeded
- Return subset + tokenUsage
```

**Pattern to adopt**: Pre-computed tokenEstimate field, budget-aware retrieval.

### 7. Subspace SVD Consolidation

```typescript
consolidateEmbeddings(embeddings, k):
- Compute centroid + principal components
- projectToCompact(embedding, centroid, components)
- k-dim compact representation
```

**Pattern to adopt**: Semantic clustering for memory compression.

### 8. Entity Auto-Linking

```typescript
autoLinkEntities(content, entityNames):
- Replace entity mentions with [[wiki-links]]
- Enables graph traversal
```

**Pattern to adopt**: Content → knowledge graph bridge.

### 9. Episodes (Episodic Memory)

```typescript
create_episode(factIds, title, tags):
search_episodes(query):
close_episode(episodeId):
```

**Pattern to adopt**: Group related facts into sessions/episodes.

### 10. Observer/Reflector (Observation Memory)

```typescript
omStatus(): Current state
observeCompress(scopeId): Run background compression
reflect(scopeId, depth): Targeted reflection
```

**Pattern to adopt**: Self-monitoring memory that evolves over time.

---

## Engram Patterns to Borrow

| Pattern | Source | Value |
|---------|--------|-------|
| Tool registry modularity | Engram | Composable, testable tools |
| Fact richness | Engram | QA-pairs, emotional weight, outcome scores |
| Async enrichment | Engram | Fast writes, background processing |
| Active forgetting | Engram | Self-maintaining memory |
| Token budgets | Engram | Controlled context injection |
| Subspace compression | Engram | Memory efficiency |
| Episodes | Engram | Session-level grouping |

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                    Unified Memory Plugin                       │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐  │
│  │   Storage    │  │   Knowledge  │  │   Code Indexing      │  │
│  │  (context)   │  │   (graph)    │  │   (tree-sitter)      │  │
│  └──────────────┘  └──────────────┘  └──────────────────────┘  │
├─────────────────────────────────────────────────────────────────┤
│                      MCP Server (Rust)                         │
├─────────────────────────────────────────────────────────────────┤
│    Claude Code    │    Codex    │   Cursor   │   Windsurf    │
└─────────────────────────────────────────────────────────────────┘
```

---

## Next Steps

→ `/ce:plan` for implementation details
