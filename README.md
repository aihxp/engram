# Engram Fork — Self-Hosted with node-llama-cpp

Fork of [RyanLisse/engram](https://github.com/RyanLisse/engram) with local-only embeddings.

## What's Changed

- **No Cohere** — Replaced cloud embedding API with `node-llama-cpp` + local GGUF model
- **No Ollama** — Direct inference via llama.cpp, no wrapper daemon needed
- **Fully self-hosted** — Everything runs on your VPS, zero external AI APIs for memory

## Embedding Model

Defaults to `mxbai-embed-large-v1-q8_0.gguf` (1024-dim, downloaded automatically via HuggingFace).

Override via env:
```bash
ENGRAM_EMBED_MODEL="hf:..."      # HuggingFace GGUF path
ENGRAM_MODEL_CACHE="/path"       # Where to cache models
ENGRAM_EMBED_DIM=1024            # Expected dimensions
```

## Architecture

```
OpenClaw → MCP config → engram MCP server → node-llama-cpp (local GGUF)
                                              ↓
                                         Convex (self-hosted Docker)
```

## Setup

1. Deploy Convex (self-hosted):
   ```bash
   cd convex && docker compose up -d
   ```

2. Build MCP server:
   ```bash
   cd mcp-server && npm install && npm run build
   ```

3. Add to OpenClaw MCP config:
   ```json
   {
     "mcpServers": {
       "engram": {
         "command": "node",
         "args": ["/home/ubuntu/engram-fork/mcp-server/dist/index.js"],
         "env": {
           "CONVEX_URL": "http://localhost:3210",
           "ENGRAM_AGENT_ID": "vesper"
         }
       }
     }
   }
   ```

## Original

See upstream at https://github.com/RyanLisse/engram for full docs.
