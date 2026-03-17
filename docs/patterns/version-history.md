# Pattern: Version History

Non-destructive mutations ensure auditability, safety, and the ability to revert memory changes.

## Context

In multi-agent environments, agents may overwrite each other's work or make incorrect updates. Engram implements a "Copy-on-Write" style versioning system for facts.

## Implementation

### 1. `fact_versions` Table
Every update to a fact creates a new entry in the `fact_versions` table.
- **Source Fact**: Pointer to the original `factId`.
- **Diff/Content**: The new state of the fact.
- **Provenance**: Which agent made the change and `reason: string` (mandatory).

### 2. Mandatory `reason`
Tools like `memory_update_fact` must include a `reason` parameter. This reason is stored in the version history, providing a narrative trail of *why* memory evolved.

### 3. Rollback Capability
The `memory_rollback_fact` tool (coming in P4/P5) allows reverting a fact to a previous `versionId`.

### 4. Convergence via Reflection
The "Sleep-Time Reflection" agent reviews version histories to resolve contradictions between agents that occurred during high-concurrency periods.

## Best Practices
- **Semantic Diffs**: When updating, agents should provide a reason that describes the *delta* (e.g., "Updated project deadline based on recent meeting notes").
- **Deduplication**: Frequent updates should be consolidated by the reflection agent to prevent version bloat.
- **View History**: Agents can use `memory_get_fact_history` to understand the evolution of a specific piece of knowledge.
