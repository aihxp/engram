/**
 * embeddings.ts - Local-only embedding client using node-llama-cpp
 *
 * Replaces Cohere + Ollama with direct node-llama-cpp inference.
 * Uses a local GGUF embedding model (e.g. mxbai-embed-large, embeddinggemma).
 * Fully self-hosted — no external APIs.
 */

import { getLlama, resolveModelFile, type LlamaEmbeddingContext } from "node-llama-cpp";
import { homedir } from "os";
import { join } from "path";

// ── Config ───────────────────────────────────────────

const EMBED_MODEL = process.env.ENGRAM_EMBED_MODEL ||
  "hf:ggml-org/embeddinggemma-300M-GGUF/embeddinggemma-300M-Q8_0.gguf";

const MODEL_CACHE_DIR = process.env.ENGRAM_MODEL_CACHE ||
  join(homedir(), ".cache", "qmd", "models");

const DIMENSIONS = parseInt(process.env.ENGRAM_EMBED_DIM || "768", 10);

// ── Lazy-loaded state ────────────────────────────────

let _llama: Awaited<ReturnType<typeof getLlama>> | null = null;
let _embedContext: LlamaEmbeddingContext | null = null;
let _modelReady: boolean | null = null;

async function getEmbedContext(): Promise<LlamaEmbeddingContext | null> {
  if (_embedContext) return _embedContext;
  if (_modelReady === false) return null;

  try {
    console.error(`[embeddings] Loading model: ${EMBED_MODEL}`);
    const llama = await getLlama();
    _llama = llama;

    const modelPath = await resolveModelFile(EMBED_MODEL, MODEL_CACHE_DIR);
    const model = await llama.loadModel({ modelPath });

    console.error(`[embeddings] Model loaded: ${modelPath}`);

    _embedContext = await model.createEmbeddingContext();
    _modelReady = true;
    return _embedContext;
  } catch (err: any) {
    console.error(`[embeddings] Failed to load model: ${err.message}`);
    _modelReady = false;
    return null;
  }
}

// ── Public API ───────────────────────────────────────

export async function generateEmbedding(
  text: string,
  _inputType: "search_document" | "search_query" = "search_document"
): Promise<number[]> {
  const ctx = await getEmbedContext();
  if (!ctx) {
    console.error("[embeddings] Model unavailable, returning zero vector");
    return new Array(DIMENSIONS).fill(0);
  }

  try {
    const embedding = await ctx.getEmbeddingFor(text);
    const arr = Array.from(embedding.vector);

    // Validate dimensionality
    if (arr.length !== DIMENSIONS) {
      console.warn(
        `[embeddings] Dimension mismatch: expected ${DIMENSIONS}, got ${arr.length}`
      );
      // Pad or truncate to match expected dimensions
      if (arr.length < DIMENSIONS) {
        return [...arr, ...new Array(DIMENSIONS - arr.length).fill(0)];
      }
      return arr.slice(0, DIMENSIONS);
    }

    return arr;
  } catch (err: any) {
    console.error(`[embeddings] embed() failed: ${err.message}`);
    return new Array(DIMENSIONS).fill(0);
  }
}

export async function generateBatchEmbeddings(
  texts: string[],
  _inputType: "search_document" | "search_query" = "search_document"
): Promise<number[][]> {
  if (texts.length === 0) return [];

  const ctx = await getEmbedContext();
  if (!ctx) {
    return texts.map(() => new Array(DIMENSIONS).fill(0));
  }

  const results: number[][] = [];
  for (const text of texts) {
    try {
      const embedding = await ctx.getEmbeddingFor(text);
      const arr = Array.from(embedding.vector);

      if (arr.length !== DIMENSIONS) {
        if (arr.length < DIMENSIONS) {
          results.push([...arr, ...new Array(DIMENSIONS - arr.length).fill(0)]);
        } else {
          results.push(arr.slice(0, DIMENSIONS));
        }
      } else {
        results.push(arr);
      }
    } catch (err: any) {
      console.error(`[embeddings] Batch embed failed for "${text.slice(0, 50)}...": ${err.message}`);
      results.push(new Array(DIMENSIONS).fill(0));
    }
  }

  return results;
}

/** Check which embedding provider is active (for health/diagnostics) */
export function getEmbeddingProvider(): string {
  if (_modelReady === true) return `node-llama-cpp (${EMBED_MODEL})`;
  if (_modelReady === false) return "none (model failed to load)";
  return "unknown (not probed yet)";
}
