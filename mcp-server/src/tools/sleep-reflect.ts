/**
 * memory_sleep_reflect — Sleep-time reflection tool
 * 
 * Triggers a consolidation and reflection process for a memory scope.
 * Analyzes recent facts to generate high-level insights and patterns.
 */

import { z } from "zod";
import * as convex from "../lib/convex-client.js";

export const sleepReflectSchema = z.object({
  scopeId: z.string().describe("Scope ID to reflect upon"),
});

export async function sleepReflect(
  input: z.infer<typeof sleepReflectSchema>,
  agentId: string,
) {
  try {
    // Check if scope exists and agent has access
    const scope = await convex.getScope(input.scopeId);
    if (!scope) {
      return { isError: true, message: `Scope not found: ${input.scopeId}` };
    }

    // Trigger reflection action in Convex
    const result = await convex.sleepReflect({
      scopeId: input.scopeId,
      agentId,
    });

    return {
      success: true,
      scopeId: input.scopeId,
      summary: (result as any)?.report || "Reflection completed successfully.",
      insights_generated: (result as any)?.insightCount || 0,
    };
  } catch (error: any) {
    return { isError: true, message: error.message ?? String(error) };
  }
}
