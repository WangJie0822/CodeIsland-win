<script setup lang="ts">
/**
 * MarkdownRenderer — Stage 1 placeholder 实现
 *
 * 依赖约束：markdown-it 和 shiki 未安装（package.json 禁改）。
 * 本组件使用纯 Vue + 正则处理基本 Markdown 语法。
 *
 * Stage 2 待办：
 *   npm i markdown-it shiki @tauri-apps/plugin-shell
 *   替换 renderMarkdown() 为 markdown-it + shiki 完整渲染。
 *   shell.open 替换 fallback window.open。
 */
import { computed, ref } from "vue";

const props = defineProps<{
  source: string;
  inline?: boolean;
  maxHeight?: number;
}>();

// ---------- HTML escape ----------
function escapeHtml(s: string): string {
  return s
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#39;");
}

// ---------- Inline 处理 ----------
function renderInline(raw: string): string {
  let s = escapeHtml(raw);
  // 行内代码 `code`（先处理，避免被其他规则替换）
  s = s.replace(/`([^`]+)`/g, (_, code) => `<code class="md-inline-code">${code}</code>`);
  // 加粗 **text**
  s = s.replace(/\*\*([^*]+)\*\*/g, "<strong>$1</strong>");
  // 斜体 *text*（单星号）
  s = s.replace(/\*([^*]+)\*/g, "<em>$1</em>");
  // 链接 [text](url)
  s = s.replace(
    /\[([^\]]+)\]\((https?:\/\/[^)]+)\)/g,
    (_, text, href) =>
      `<a class="md-link" href="${escapeHtml(href)}" data-href="${escapeHtml(href)}">${text}</a>`,
  );
  return s;
}

// ---------- Block 处理 ----------
function renderMarkdown(source: string): string {
  const lines = source.split("\n");
  const out: string[] = [];
  let inCode = false;
  let codeLang = "";
  let codeLines: string[] = [];

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    // 代码块开始/结束
    if (/^```/.test(line)) {
      if (!inCode) {
        inCode = true;
        codeLang = line.slice(3).trim();
        codeLines = [];
      } else {
        const escaped = codeLines.map(escapeHtml).join("\n");
        const dataLang = codeLang ? ` data-lang="${escapeHtml(codeLang)}"` : "";
        out.push(`<div class="md-code-block"${dataLang}><button class="md-copy-btn" data-code="${escapeHtml(codeLines.join("\n"))}">复制</button><pre class="md-code-pre"><code>${escaped}</code></pre></div>`);
        inCode = false;
        codeLang = "";
        codeLines = [];
      }
      continue;
    }

    if (inCode) {
      codeLines.push(line);
      continue;
    }

    // 标题 # / ## / ###
    const headMatch = line.match(/^(#{1,3})\s+(.+)/);
    if (headMatch) {
      const level = headMatch[1].length;
      out.push(`<h${level} class="md-h${level}">${renderInline(headMatch[2])}</h${level}>`);
      continue;
    }

    // 无序列表 - item
    const liMatch = line.match(/^[-*]\s+(.+)/);
    if (liMatch) {
      out.push(`<li class="md-li">${renderInline(liMatch[1])}</li>`);
      continue;
    }

    // 空行
    if (line.trim() === "") {
      out.push("<br/>");
      continue;
    }

    // 普通段落
    out.push(`<p class="md-p">${renderInline(line)}</p>`);
  }

  // 未关闭代码块
  if (inCode && codeLines.length > 0) {
    const escaped = codeLines.map(escapeHtml).join("\n");
    out.push(`<div class="md-code-block"><pre class="md-code-pre"><code>${escaped}</code></pre></div>`);
  }

  return out.join("");
}

const rendered = computed(() => {
  if (!props.source) return "";
  if (props.inline) {
    // inline 模式：取第一行，渲染 inline 语法
    const firstLine = props.source.split("\n")[0];
    return renderInline(firstLine);
  }
  return renderMarkdown(props.source);
});

const containerStyle = computed(() => {
  const style: Record<string, string> = {};
  if (props.maxHeight) {
    style.maxHeight = `${props.maxHeight}px`;
    style.overflowY = "auto";
  }
  return style;
});

// ---------- 事件处理 ----------
function handleClick(e: MouseEvent) {
  const target = e.target as HTMLElement;

  // 复制按钮
  if (target.classList.contains("md-copy-btn")) {
    const code = target.getAttribute("data-code") ?? "";
    // 解码 HTML entities
    const decoded = code
      .replace(/&amp;/g, "&")
      .replace(/&lt;/g, "<")
      .replace(/&gt;/g, ">")
      .replace(/&quot;/g, '"')
      .replace(/&#39;/g, "'");
    navigator.clipboard?.writeText(decoded).catch(() => {});
    const prev = target.textContent;
    target.textContent = "已复制";
    setTimeout(() => { target.textContent = prev; }, 1500);
    return;
  }

  // 链接
  const link = target.closest("a.md-link") as HTMLAnchorElement | null;
  if (link) {
    e.preventDefault();
    const href = link.getAttribute("data-href") ?? "";
    if (!href) return;
    // Stage 2: 替换为 @tauri-apps/plugin-shell shell.open(href)
    window.open(href, "_blank");
  }
}

const copyState = ref<Record<string, boolean>>({});
void copyState; // 避免 unused 警告，Stage 2 可用于按钮状态管理
</script>

<template>
  <span v-if="inline" class="md-inline" v-html="rendered" @click="handleClick" />
  <div
    v-else
    class="md-block"
    :style="containerStyle"
    v-html="rendered"
    @click="handleClick"
  />
</template>

<style scoped>
.md-inline {
  font-size: inherit;
  color: inherit;
}

.md-block {
  font-size: 12px;
  color: var(--text-secondary);
  line-height: 1.5;
}

/* 以下样式使用 :deep() 因为内容由 v-html 注入 */
.md-inline :deep(.md-inline-code),
.md-block :deep(.md-inline-code) {
  font-family: var(--font-mono);
  font-size: 0.9em;
  background: rgba(255, 255, 255, 0.1);
  padding: 1px 4px;
  border-radius: 3px;
  color: var(--text-primary);
}

.md-block :deep(.md-code-block) {
  position: relative;
  background: rgba(0, 0, 0, 0.3);
  border-radius: 4px;
  margin: 6px 0;
  overflow: hidden;
}

.md-block :deep(.md-code-pre) {
  font-family: var(--font-mono);
  font-size: 11px;
  padding: 8px 10px;
  overflow-x: auto;
  white-space: pre;
  color: var(--text-primary);
  margin: 0;
}

.md-block :deep(.md-copy-btn) {
  position: absolute;
  top: 4px;
  right: 4px;
  background: rgba(255, 255, 255, 0.1);
  border: none;
  color: var(--text-tertiary);
  font-size: 10px;
  cursor: pointer;
  padding: 2px 6px;
  border-radius: 3px;
  opacity: 0;
  transition: opacity 150ms;
}

.md-block :deep(.md-code-block:hover .md-copy-btn) {
  opacity: 1;
}

.md-block :deep(.md-copy-btn:hover) {
  color: var(--text-primary);
  background: rgba(255, 255, 255, 0.18);
}

.md-block :deep(.md-link) {
  color: var(--accent-blue);
  text-decoration: none;
  cursor: pointer;
}
.md-block :deep(.md-link:hover) {
  text-decoration: underline;
}

.md-block :deep(.md-h1) {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 6px 0 4px;
}
.md-block :deep(.md-h2) {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 5px 0 3px;
}
.md-block :deep(.md-h3) {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 4px 0 2px;
}

.md-block :deep(.md-p) {
  margin: 2px 0;
}

.md-block :deep(.md-li) {
  margin-left: 16px;
  padding: 1px 0;
  list-style-type: disc;
}
</style>
