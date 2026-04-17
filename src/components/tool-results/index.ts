import { defineAsyncComponent, type Component } from "vue";

/** 按 tool_name 路由到对应视图组件 */
const toolViewMap: Record<string, () => Promise<{ default: Component }>> = {
  Read:           () => import("./ReadResult.vue"),
  Edit:           () => import("./EditResult.vue"),
  MultiEdit:      () => import("./EditResult.vue"),
  Write:          () => import("./WriteResult.vue"),
  Bash:           () => import("./BashResult.vue"),
  Grep:           () => import("./GrepResult.vue"),
  Glob:           () => import("./GlobResult.vue"),
  Task:           () => import("./TaskResult.vue"),
  TodoWrite:      () => import("./TodoWriteResult.vue"),
  WebSearch:      () => import("./WebSearchResult.vue"),
  WebFetch:       () => import("./WebFetchResult.vue"),
};

const GenericResult = defineAsyncComponent(() => import("./GenericResult.vue"));

/**
 * 根据 tool_name 返回对应的异步组件。
 * 未知 tool 降级到 GenericResult。
 */
export function resolveToolView(toolName: string): Component {
  const loader = toolViewMap[toolName];
  if (!loader) return GenericResult;
  return defineAsyncComponent(loader);
}

export { GenericResult };
