import { defineStore } from "pinia";
import { computed, ref } from "vue";
import type { SessionSummary } from "@/types/generated";
import {
  invokeApprovePermission,
  invokeDenyPermission,
  invokeGetSessions,
  invokeSendToTerminal,
} from "@/lib/tauri";

export const useSessionsStore = defineStore("sessions", () => {
  const list = ref<SessionSummary[]>([]);

  const hasAttention = computed(() => list.value.some((s) => s.needs_attention));

  const overallStatus = computed<"idle" | "processing" | "attention">(() => {
    if (list.value.some((s) => s.phase === "waitingForApproval")) return "attention";
    if (list.value.some((s) => s.phase === "processing" || s.phase === "compacting")) {
      return "processing";
    }
    return "idle";
  });

  async function refresh(): Promise<void> {
    try {
      list.value = await invokeGetSessions();
    } catch (e) {
      console.error("[sessions] refresh 失败:", e);
    }
  }

  async function approve(sessionId: string): Promise<void> {
    await invokeApprovePermission(sessionId);
  }

  async function deny(sessionId: string, reason?: string): Promise<void> {
    await invokeDenyPermission(sessionId, reason);
  }

  async function sendToTerminal(sessionId: string, text: string): Promise<void> {
    await invokeSendToTerminal(sessionId, text);
  }

  return {
    list,
    hasAttention,
    overallStatus,
    refresh,
    approve,
    deny,
    sendToTerminal,
  };
});
