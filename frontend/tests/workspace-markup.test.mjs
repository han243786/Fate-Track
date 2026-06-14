import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

const html = readFileSync(new URL("../index.html", import.meta.url), "utf8");
const mainSource = readFileSync(new URL("../src/main.js", import.meta.url), "utf8");
const reportHtml = readFileSync(new URL("../report.html", import.meta.url), "utf8");
const topicReportHtml = readFileSync(new URL("../topic-report.html", import.meta.url), "utf8");
const topicReportSource = readFileSync(new URL("../src/topic-report-page.js", import.meta.url), "utf8");
const renderSource = readFileSync(new URL("../src/ui/render.js", import.meta.url), "utf8");
const stylesSource = readFileSync(new URL("../src/styles.css", import.meta.url), "utf8");
const reportThemesSource = readFileSync(new URL("../src/report-themes.css", import.meta.url), "utf8");

describe("workspace markup", () => {
  it("contains the product panels", () => {
    for (const id of [
      "chart-form-title",
      "chart-title",
      "luck-title",
      "analysis-title"
    ]) {
      assert.match(html, new RegExp(`aria-labelledby="${id}"`));
    }

    for (const id of [
      "chart-date",
      "chart-time",
      "chart-sex",
      "chart-run-button",
      "topic-relationship-button",
      "topic-wealth-button",
      "topic-family-button",
      "topic-career-button",
      "topic-full-report-button",
      "topic-selected-label",
      "wuxing-theme-button",
      "wuxing-theme-mark",
      "wuxing-theme-label",
      "product-boundary-button",
      "product-boundary-panel",
      "product-boundary-title",
      "topic-report-panel",
      "topic-report-content",
      "lunar-display"
    ]) {
      assert.match(html, new RegExp(`id="${id}"`));
    }
  });

  it("unlocks all four topic reports after M33", () => {
    for (const [id, topic] of [
      ["topic-relationship-button", "relationship"],
      ["topic-wealth-button", "wealth"],
      ["topic-family-button", "family"],
      ["topic-career-button", "career"]
    ]) {
      assert.match(html, new RegExp(`id="${id}"[^>]*data-topic="${topic}"`));
      assert.doesNotMatch(html, new RegExp(`id="${id}"[^>]*disabled`));
      assert.match(html, new RegExp(`id="${id}"[^>]*class="[^"]*is-ready`));
    }
  });

  it("keeps the workspace topic panel to structure signals", () => {
    assert.match(html, /查看专项报告/);
    assert.match(topicReportHtml, /src\/topic-report-page\.js/);
    assert.match(renderSource, /topic-signal-section/);
    assert.doesNotMatch(renderSource, /report\.blocks/);
    assert.doesNotMatch(renderSource, /topic-trace-row/);
  });

  it("adds a frontend-only wuxing color switch without expanding capabilities", () => {
    assert.match(html, /id="wuxing-theme-button"/);
    assert.match(html, /当前：水/);
    assert.match(mainSource, /WUXING_THEMES/);
    assert.match(mainSource, /id: "metal"[\s\S]*id: "wood"[\s\S]*id: "water"[\s\S]*id: "fire"[\s\S]*id: "earth"/);
    assert.doesNotMatch(mainSource, /mystic|mark: "玄"|label: "玄"/);
    assert.match(mainSource, /dataset\.wuxingTheme/);
    assert.match(mainSource, /ft-wuxing-theme/);
    for (const theme of ["wood", "fire", "earth", "metal", "water"]) {
      assert.match(stylesSource, new RegExp(`data-wuxing-theme="${theme}"`));
    }
    assert.doesNotMatch(stylesSource, /data-wuxing-theme="mystic"/);
    assert.doesNotMatch(mainSource, /wuxing.*ApiClient|ApiClient.*wuxing/i);
  });

  it("keeps wuxing palettes systematic instead of single-hue recolors", () => {
    assert.match(stylesSource, /HSL palettes keep hue, saturation, lightness, and contrast explicit/);
    for (const theme of ["metal", "wood", "water", "fire", "earth"]) {
      const block = stylesSource.match(new RegExp(`html\\[data-wuxing-theme="${theme}"\\] \\{[\\s\\S]*?\\n\\}`));
      assert.ok(block, `${theme} palette exists`);
      assert.match(block[0], /--bg: hsl\(/);
      assert.match(block[0], /--ink: hsl\(/);
      assert.match(block[0], /--gold: hsl\(/);
      assert.match(block[0], /--jade: hsl\(/);
      assert.match(block[0], /--bg-glow-a: hsl\(/);
    }
    assert.match(reportThemesSource, /data-wuxing-theme="metal"[\s\S]*--theme-accent: hsl\(/);
    assert.match(reportThemesSource, /data-wuxing-theme="earth"[\s\S]*--theme-accent-2: hsl\(/);
    assert.doesNotMatch(reportThemesSource, /data-wuxing-theme="metal"[\s\S]*#cfd8d3/);
  });

  it("clears stale topic output when the chart is recalculated", () => {
    assert.match(mainSource, /renderTopicReportIdle/);
    assert.match(mainSource, /排盘资料已更新，请重新选择专项推演/);
    assert.match(mainSource, /topicRequestVersion/);
    assert.match(mainSource, /chartRequestKey/);
  });

  it("passes explicit reading year into primary luck reading", () => {
    assert.match(mainSource, /reading_year/);
    assert.match(mainSource, /readingYear: currentReadingYear/);
    assert.match(mainSource, /year: currentReadingYear/);
    assert.match(mainSource, /annual_trigger_reading/);
    assert.match(reportHtml, /readingYear: params\.get\("reading_year"\)/);
    assert.match(reportHtml, /year: params\.get\("year"\)/);
    assert.match(reportHtml, /引动年/);
    assert.match(renderSource, /luck-reading-summary/);
    assert.match(renderSource, /annual-trigger-summary/);
    assert.doesNotMatch(renderSource, /score_internal/);
    assert.doesNotMatch(renderSource, /0-100/);
  });

  it("renders M39 timeline report UI without adding a new capability", () => {
    assert.match(reportHtml, /timeline-report-guide/);
    assert.match(reportHtml, /timeline-year-control/);
    assert.match(reportHtml, /createTimelineReadingDetails/);
    assert.match(reportHtml, /reading_year/);
    assert.match(reportHtml, /year/);
    assert.match(topicReportSource, /topic-timeline-guide/);
    assert.match(topicReportSource, /topic-timeline-overlay/);
    assert.match(topicReportSource, /narrativeOnly = topic === "relationship"/);
    assert.match(topicReportSource, /relationship-trigger/);
    assert.match(topicReportSource, /createTopicTraceDetails/);
    assert.match(renderSource, /current-luck-meta/);
    assert.match(renderSource, /timeline-boundary-copy/);
    assert.match(stylesSource, /timeline-evidence-detail/);
    assert.doesNotMatch(reportHtml, /score_internal/);
    assert.doesNotMatch(topicReportSource, /score_internal/);
    assert.doesNotMatch(stylesSource, /score_internal/);
  });

  it("uses the provided report visual theme layer without changing report data", () => {
    assert.match(reportHtml, /data-report-theme="main"/);
    assert.match(topicReportHtml, /data-report-theme="relationship"/);
    assert.match(reportHtml, /src\/report-themes\.css/);
    assert.match(topicReportHtml, /src\/report-themes\.css/);
    assert.match(reportHtml, /ft-wuxing-theme/);
    assert.match(topicReportHtml, /ft-wuxing-theme/);
    assert.match(reportThemesSource, /\.report-root\[data-wuxing-theme\]/);
    for (const theme of ["main", "relationship", "wealth", "family", "career"]) {
      assert.match(reportThemesSource, new RegExp(`data-report-theme="${theme}"`));
    }
    for (const theme of ["metal", "wood", "water", "fire", "earth"]) {
      assert.match(stylesSource, new RegExp(`data-wuxing-theme="${theme}"`));
      assert.match(reportThemesSource, new RegExp(`data-wuxing-theme="${theme}"`));
    }
    assert.match(reportThemesSource, /--theme-accent-2/);
    assert.match(reportThemesSource, /--theme-panel-2/);
    assert.doesNotMatch(reportThemesSource, /score_internal/);
  });

  it("keeps M40 timeline quality-gate boundaries in public UI sources", () => {
    const timelineUi = [
      html,
      reportHtml,
      topicReportHtml,
      topicReportSource,
      mainSource,
      renderSource,
      stylesSource
    ].join("\n");

    assert.match(reportHtml, /本章节只展示已经整理好的大运\/年度引动解释/);
    assert.match(reportHtml, /不提供流月、流日、择日、事件预测/);
    assert.match(topicReportSource, /不构成确定性事件断语或现实决策承诺/);
    assert.match(topicReportSource, /不额外加入评分或事件断语/);
    assert.match(reportHtml, /input\.min = "1901"/);
    assert.match(reportHtml, /input\.max = "2100"/);
    assert.match(topicReportSource, /input\.min = "1901"/);
    assert.match(topicReportSource, /input\.max = "2100"/);

    for (const forbidden of [
      "score_internal",
      "0-100",
      "后端返回",
      "前端追加",
      "专业解释",
      "白话解释",
      "流月运势",
      "流日运势",
      "每日运势",
      "结果保证",
      "确定发生",
      "保证发财",
      "必然发财",
      "必然结婚",
      "必升职"
    ]) {
      assert.doesNotMatch(timelineUi, new RegExp(forbidden));
    }
  });

  it("keeps newly added report chrome localized", () => {
    assert.match(html, /四柱结构工作台/);
    assert.match(html, /当前功能边界已锁定/);
    assert.match(html, /本地排盘与报告/);
    assert.match(html, /本地计算/);
    assert.match(html, /可查看/);
    assert.match(reportHtml, /命轨报告/);
    assert.match(reportHtml, /报告章节/);
    assert.match(topicReportHtml, /命轨专项报告/);
    assert.match(topicReportSource, /专项章节/);
    assert.match(topicReportSource, /共同时间解读脉络/);

    for (const visibleEnglish of [
      "Fate Track Archive",
      "Fate Track Topic Archive",
      "Archive Section",
      "Topic Section",
      "explicit year",
      "\u663e\u5f0f\u5e74\u4efd",
      "\u663e\u5f0f\u5e74\u5ea6",
      "只读卷宗",
      "专项卷宗",
      "结构化卷宗",
      "等待归档",
      "已归档",
      "时间解释引擎",
      "共享时间解释引擎",
      "投射到当前专题",
      "unknown_hour_timeline_evidence_downgraded",
      "annual_trigger_not_requested",
      "topic_timeline_",
      "major_luck_cycle_missing",
      "reference_age_outside_supported_cycle_range",
      "timeline evidence",
      "deterministic snapshot",
      "observation year",
      "annual year",
      "已接入",
      "微盘",
      "三统"
    ]) {
      assert.doesNotMatch([html, reportHtml, topicReportHtml, topicReportSource].join("\n"), new RegExp(visibleEnglish));
    }

    for (const internalMarker of [
      "shared timeline engine",
      "timeline engine",
      "annual-trigger",
      "major-luck-current",
      "luck-annual-overlay"
    ]) {
      assert.match([reportHtml, topicReportSource, renderSource].join("\n"), /localizeVisibleText/);
      assert.match([reportHtml, topicReportSource, renderSource].join("\n"), new RegExp(internalMarker));
    }
  });

  it("keeps visible report chrome away from mechanical wording and backend errors", () => {
    const visibleChrome = [
      html,
      reportHtml,
      topicReportHtml,
      topicReportSource,
      mainSource,
      readFileSync(new URL("../src/api/client.js", import.meta.url), "utf8"),
      renderSource
    ].join("\n");

    assert.match(reportHtml, /正在整理报告/);
    assert.match(topicReportHtml, /正在整理专项报告/);
    assert.match(reportHtml, /阅读依据/);
    assert.match(reportHtml, /结构线索/);
    assert.match(topicReportSource, /结构线索总览/);
    assert.match(html, /专项推演待整理/);
    assert.match(mainSource, /专项推演待整理/);
    assert.match(renderSource, /\\u7ed3\\u6784\\u7ebf\\u7d22/);

    for (const forbidden of [
      "正在生成报告",
      "正在生成专项报告",
      "报告生成失败",
      "专项报告生成失败",
      "生成失败",
      "暂无报告数据",
      "暂无专项报告数据",
      "待生成",
      "专项报告待生成",
      "404 Not Found",
      "Bad Request",
      "Method Not Allowed",
      "结构信号正在提取",
      "结构信号生成失败",
      "证据追踪",
      "确定性快照",
      "时间证据",
      "未生成章节",
      "大运:"
    ]) {
      assert.doesNotMatch(visibleChrome, new RegExp(forbidden));
    }

    assert.doesNotMatch(reportHtml, /error instanceof Error \? error\.message/);
    assert.doesNotMatch(topicReportSource, /error instanceof Error \? error\.message/);
    assert.doesNotMatch(mainSource, /error\.message|e\.message|String\(error\)/);
    assert.doesNotMatch(renderSource, /error\.message/);
  });

  it("shows a user-facing inline product boundary panel without expanding features", () => {
    assert.match(html, /id="product-boundary-button"/);
    assert.match(html, /查看边界/);
    assert.match(html, /命轨当前只做这些事/);
    assert.match(html, /可以查看/);
    assert.match(html, /当前不做/);
    assert.match(html, /在线智能解读/);
    assert.match(html, /五行主题只改变视觉风格/);
    assert.match(html, /id="product-boundary-panel"/);
    assert.match(mainSource, /bindBoundaryPanel/);
    assert.doesNotMatch(html, /<dialog|method="dialog"/);
    assert.doesNotMatch(mainSource, /showModal|\.close\(\)/);
    assert.doesNotMatch(stylesSource, /::backdrop|product-boundary-dialog/);
    assert.doesNotMatch(mainSource, /boundary.*ApiClient|ApiClient.*boundary/i);
  });

  it("keeps workspace runtime enum labels out of visible cards", () => {
    assert.match(renderSource, /SEVERITY_LABELS/);
    assert.match(renderSource, /INSIGHT_TAG_HIDDEN_TITLES = \["日主", "格局", "用神"\]/);
    assert.match(renderSource, /shouldRenderInsightTag\(card\.title\)/);
    assert.match(renderSource, /info: "留意"/);
    assert.match(renderSource, /function metricLabel\(id\)/);
    assert.doesNotMatch(renderSource, /METRIC_LABELS\[m\.id\]\s*\|\|\s*m\.id/);
    assert.doesNotMatch(renderSource, /大运解释层未返回/);
    assert.doesNotMatch(renderSource, /未返回日常摘要/);
    assert.doesNotMatch(reportHtml, /未收到观察年|未收到引动年/);
    assert.match(reportHtml, /publicTimelinePart/);
    assert.doesNotMatch(renderSource, /<div class="insight-tag">\$\{card\.severity\}<\/div>/);
    assert.doesNotMatch(renderSource, /证据 <strong>/);
    assert.doesNotMatch(renderSource, /证据追踪/);
  });

  it("keeps unsupported features out of the product", () => {
    assert.doesNotMatch(html, /Cloud Sync/);
    assert.doesNotMatch(html, /True Solar Time/);
    assert.doesNotMatch(html, /Astronomy Engine/);
  });
});
