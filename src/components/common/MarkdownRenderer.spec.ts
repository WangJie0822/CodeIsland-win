import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import MarkdownRenderer from "./MarkdownRenderer.vue";

describe("MarkdownRenderer", () => {
  describe("HTML escape（XSS 防护）", () => {
    it("转义尖括号", () => {
      const w = mount(MarkdownRenderer, { props: { source: "<script>alert(1)</script>" } });
      expect(w.html()).not.toContain("<script>");
      expect(w.html()).toContain("&lt;script&gt;");
    });

    it("转义 & 符号", () => {
      const w = mount(MarkdownRenderer, { props: { source: "AT&T" } });
      expect(w.html()).toContain("AT&amp;T");
    });
  });

  describe("inline 模式", () => {
    it("仅渲染第一行", () => {
      const w = mount(MarkdownRenderer, {
        props: { source: "line one\nline two", inline: true },
      });
      expect(w.text()).toContain("line one");
      expect(w.text()).not.toContain("line two");
    });

    it("渲染行内代码", () => {
      const w = mount(MarkdownRenderer, {
        props: { source: "use `const` here", inline: true },
      });
      expect(w.find("code.md-inline-code").text()).toBe("const");
    });

    it("渲染加粗", () => {
      const w = mount(MarkdownRenderer, {
        props: { source: "**bold text**", inline: true },
      });
      expect(w.find("strong").text()).toBe("bold text");
    });

    it("渲染斜体", () => {
      const w = mount(MarkdownRenderer, {
        props: { source: "*italic text*", inline: true },
      });
      expect(w.find("em").text()).toBe("italic text");
    });
  });

  describe("block 模式（基本 Markdown）", () => {
    it("渲染代码块", () => {
      const src = "```ts\nconst x = 1;\n```";
      const w = mount(MarkdownRenderer, { props: { source: src } });
      expect(w.find(".md-code-block").exists()).toBe(true);
      expect(w.find(".md-code-pre").text()).toContain("const x = 1;");
    });

    it("代码块内容 HTML 转义", () => {
      const src = "```\n<div>hello</div>\n```";
      const w = mount(MarkdownRenderer, { props: { source: src } });
      // 原始 html() 不应包含未转义的 <div>
      const codeHtml = w.find(".md-code-pre").html();
      expect(codeHtml).toContain("&lt;div&gt;");
    });

    it("渲染链接", () => {
      const src = "[Tauri](https://tauri.app)";
      const w = mount(MarkdownRenderer, { props: { source: src } });
      const link = w.find("a.md-link");
      expect(link.text()).toBe("Tauri");
      expect(link.attributes("data-href")).toBe("https://tauri.app");
    });

    it("渲染标题", () => {
      const w = mount(MarkdownRenderer, { props: { source: "## Section" } });
      expect(w.find(".md-h2").text()).toBe("Section");
    });

    it("maxHeight 设置容器高度", () => {
      const w = mount(MarkdownRenderer, { props: { source: "text", maxHeight: 200 } });
      const style = (w.element as HTMLElement).style;
      expect(style.maxHeight).toBe("200px");
    });
  });
});
