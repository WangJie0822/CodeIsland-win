<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useSettingsStore } from "@/stores/settings";

const settingsStore = useSettingsStore();

const hooksEnabled = computed({
  get: () => settingsStore.value?.hooks_enabled ?? true,
  set: (v: boolean) => void settingsStore.update({ hooks_enabled: v }),
});

// Hook 安装状态（占位：Stage 2 接入真实检测命令）
const hookStatus = ref<"checking" | "installed" | "not_installed" | "error">(
  "checking",
);
const hookPath = ref<string>("");

async function checkHookStatus() {
  hookStatus.value = "checking";
  try {
    // @stage2 替换：调用真实命令检测 hook 安装状态
    // 目前占位：假设已安装（由 hook/installer.rs 在启动时完成）
    await new Promise((r) => setTimeout(r, 200));
    hookPath.value = "%USERPROFILE%\\.claude\\hooks\\codeisland-state.py";
    hookStatus.value = "installed";
  } catch (e) {
    hookStatus.value = "error";
    console.error("[HooksSettings] 检测 hook 状态失败:", e);
  }
}

onMounted(checkHookStatus);
</script>

<template>
  <section class="settings-section">
    <h3 class="section-title">Hooks 设置</h3>

    <!-- 开关 -->
    <div class="setting-row">
      <div class="setting-label-group">
        <span class="setting-label">启用 Claude Hooks</span>
        <span class="setting-desc">接收 Claude Code 会话事件</span>
      </div>
      <button
        class="toggle"
        :class="{ active: hooksEnabled }"
        role="switch"
        :aria-checked="hooksEnabled"
        @click="hooksEnabled = !hooksEnabled"
      >
        <span class="toggle-thumb" />
      </button>
    </div>

    <!-- 安装状态 -->
    <div class="hook-status-block">
      <div class="status-header">
        <span class="status-label">Hook 脚本状态</span>
        <button class="btn-refresh" title="刷新状态" @click="checkHookStatus">
          ↻
        </button>
      </div>

      <div v-if="hookStatus === 'checking'" class="status-row">
        <span class="dot dot-yellow" />
        <span class="status-text">检测中…</span>
      </div>

      <div v-else-if="hookStatus === 'installed'" class="status-row">
        <span class="dot dot-green" />
        <span class="status-text">已安装</span>
        <span class="hook-path">{{ hookPath }}</span>
      </div>

      <div v-else-if="hookStatus === 'not_installed'" class="status-row">
        <span class="dot dot-red" />
        <span class="status-text">未安装</span>
        <span class="hint">启动应用将自动安装</span>
      </div>

      <div v-else class="status-row">
        <span class="dot dot-red" />
        <span class="status-text">检测失败</span>
      </div>
    </div>
  </section>
</template>

<style scoped>
.settings-section {
  padding: 12px 0;
}

.section-title {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.6px;
  margin-bottom: 12px;
}

.setting-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 0;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  margin-bottom: 12px;
}

.setting-label-group {
  display: flex;
  flex-direction: column;
  gap: 2px;
  flex: 1;
}

.setting-label {
  font-size: 13px;
  color: var(--text-primary);
}

.setting-desc {
  font-size: 11px;
  color: var(--text-secondary);
}

.toggle {
  position: relative;
  width: 36px;
  height: 20px;
  background: var(--text-tertiary);
  border: none;
  border-radius: 10px;
  cursor: pointer;
  transition: background var(--dur-fast);
  padding: 0;
  flex-shrink: 0;
}

.toggle.active {
  background: var(--accent-green);
}

.toggle-thumb {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 16px;
  height: 16px;
  background: #fff;
  border-radius: 50%;
  transition: transform var(--dur-fast);
}

.toggle.active .toggle-thumb {
  transform: translateX(16px);
}

.hook-status-block {
  background: var(--bg-secondary);
  border-radius: var(--radius-sm);
  padding: 10px 12px;
}

.status-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
}

.status-label {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.4px;
}

.btn-refresh {
  background: none;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 14px;
  padding: 2px 4px;
  border-radius: 4px;
  transition: color var(--dur-fast);
}

.btn-refresh:hover {
  color: var(--text-primary);
}

.status-row {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.dot-green { background: var(--accent-green); }
.dot-yellow { background: var(--accent-yellow); }
.dot-red { background: var(--accent-red); }

.status-text {
  font-size: 12px;
  color: var(--text-primary);
}

.hook-path {
  font-size: 10px;
  font-family: var(--font-mono);
  color: var(--text-secondary);
  word-break: break-all;
}

.hint {
  font-size: 11px;
  color: var(--text-secondary);
}
</style>
