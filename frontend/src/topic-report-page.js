import { ApiClient } from "./api/client.js";
import { defaultApiBase, loadApiBase } from "./config.js";

const TOPICS = {
  relationship: { label: "情感", capability: "情感专项" },
  wealth: { label: "金钱", capability: "金钱专项" },
  family: { label: "家庭", capability: "家庭专项" },
  career: { label: "事业", capability: "事业专项" }
};

const apiBase = loadApiBase() ?? defaultApiBase;
const client = new ApiClient(apiBase);
const container = document.getElementById("topic-report-page");
const brandTitle = document.getElementById("topic-report-brand-title");
const params = new URLSearchParams(window.location.search);
const topic = normalizeTopic(params.get("topic"));
const topicMeta = TOPICS[topic];
const request = {
  topic,
  date: params.get("date") || "2025-01-01",
  time: params.get("time") || "",
  timezone: params.get("timezone") || "Asia/Shanghai",
  timePrecision: params.get("time_precision") || "unknown",
  sex: params.get("sex") || "unspecified",
  year: readYear(params.get("year"))
};

document.title = `${topicMeta.label}专项报告 - 命轨`;
if (brandTitle) brandTitle.textContent = `${topicMeta.label}专项报告`;

void boot();

async function boot() {
  try {
    const data = await client.topicReport(request);
    renderReport(data);
  } catch (error) {
    renderError("专项报告暂时没有整理完成，请回到工作台重新起盘后再试。");
  }
}

function renderReport(data) {
  if (!container) return;
  container.innerHTML = "";
  if (!data || !Array.isArray(data.blocks)) {
    renderError("暂时没有可读专项报告");
    return;
  }

  const blocks = buildTopicBlocks(data);
  const fragment = document.createDocumentFragment();
  fragment.append(createHero(data, blocks));
  fragment.append(createReportLayout(data, blocks));
  container.append(fragment);
  mountReportEffects();
}

function buildTopicBlocks(data) {
  const blocks = [];
  const narrativeOnly = topic === "relationship";

  if (narrativeOnly) {
    return Array.isArray(data.blocks) ? data.blocks : [];
  }

  if (Array.isArray(data.signals) && data.signals.length > 0) {
    blocks.push({
      id: "topic-signals",
      title: "结构线索总览",
      body: data.signals
        .map((signal) => {
          const label = safeText(signal.label, signal.id || "结构线索");
          const level = safeText(signal.qualitative_level, "观察");
          const summary = safeText(signal.summary, "暂无摘要。");
          return `${label}：${level}\n${summary}`;
        })
        .join("\n\n")
    });
  }

  blocks.push(...data.blocks);

  if (Array.isArray(data.warnings) && data.warnings.length > 0) {
    blocks.push({
      id: "topic-warnings",
      title: "阅读边界提醒",
      body: data.warnings.join("\n\n")
    });
  }

  return blocks;
}

function createHero(data, blocks) {
  const hero = createEl("section", "report-hero report-reveal is-visible");
  hero.id = "report-cover";
  hero.dataset.sectionIndex = "0";
  hero.setAttribute("aria-label", `${topicMeta.label}专项报告封面`);

  const luopan = createEl("div", "report-hero-luopan");
  luopan.setAttribute("aria-hidden", "true");
  for (let i = 1; i <= 5; i += 1) luopan.append(createEl("i", `report-hero-ring ring-${i}`));
  luopan.append(createEl("span", "report-hero-axis axis-v"));
  luopan.append(createEl("span", "report-hero-axis axis-h"));
  luopan.append(createEl("span", "report-hero-star"));

  const inner = createEl("div", "report-hero-inner");
  const kicker = createEl("div", "report-kicker", "命轨专项卷宗");
  const title = createEl("h1", "report-hero-title", `${topicMeta.label}专项报告`);
  const rule = createEl("i", "report-hero-rule");
  rule.setAttribute("aria-hidden", "true");
  const intro = createEl("p", "report-hero-lead", "只读排盘工作台 · 四专题结构推演 · 边界锁定");

  const meta = createEl("div", "report-meta");
  [
    ["专题", data.topic_label || topicMeta.label],
    ["年度", String(data.year || request.year)],
    ["日期", request.date],
    ["时间", request.time || "未知"],
    ["性别", sexLabel(request.sex)],
    ["章节", `${blocks.length} 节`]
  ].forEach(([label, value], index) => {
    const chip = createEl("span", "report-meta-chip", `${label}：${value}`);
    chip.style.setProperty("--chip-delay", `${index * 90}ms`);
    meta.append(chip);
  });

  const enter = createEl("a", "report-enter", blocks.length > 0 ? "进入专项卷宗" : "等待归档");
  enter.href = blocks.length > 0 ? "#report-section-1" : "#report-cover";
  enter.setAttribute("aria-label", `开始阅读${topicMeta.label}专项报告`);

  inner.append(kicker, title, rule, intro, meta, enter);
  hero.append(luopan, inner);
  return hero;
}

function createReportLayout(data, blocks) {
  const layout = createEl("section", "report-layout");
  const index = createEl("nav", "report-index");
  index.setAttribute("aria-label", "专项报告目录");

  const indexHead = createEl("div", "report-index-head");
  indexHead.append(
    createEl("div", "report-index-title", "报告目录"),
    createEl("div", "report-index-subtitle", topicMeta.capability)
  );

  const indexList = createEl("div", "report-index-list");
  indexList.append(createIndexLink("00", "封面", "#report-cover"));
  indexList.append(createIndexLink("T", "时间解释导航", "#topic-timeline-guide"));

  const chapters = createEl("div", "report-chapters");
  const timelineGuide = createTopicTimelineGuide(data, blocks);
  if (timelineGuide) chapters.append(timelineGuide);
  if (data.disclaimer) chapters.append(createDisclaimer(data.disclaimer));

  blocks.forEach((block, indexNumber) => {
    const num = String(indexNumber + 1).padStart(2, "0");
    const id = `report-section-${indexNumber + 1}`;
    indexList.append(createIndexLink(num, safeText(block.title, "未命名章节"), `#${id}`));
    chapters.append(createReportBlock(block, indexNumber));
  });

  chapters.append(createArchiveFooter(blocks.length));
  index.append(indexHead, indexList);
  layout.append(index, chapters);
  return layout;
}

function createIndexLink(number, title, href) {
  const link = document.createElement("a");
  link.href = href;
  link.className = href === "#report-cover" ? "is-active" : "";
  link.append(createEl("span", "report-index-no", number));
  link.append(createEl("span", "report-index-text", title));
  return link;
}

function createDisclaimer(text) {
  const disclaimer = createEl("section", "report-disclaimer report-reveal");
  disclaimer.dataset.sectionIndex = "disclaimer";
  disclaimer.append(createEl("div", "report-disclaimer-title", "边界声明"));
  disclaimer.append(createEl("p", "", safeText(text, "本报告仅作结构化排盘展示，不构成现实决策建议。")));
  return disclaimer;
}

function createTopicTimelineGuide(data, blocks) {
  const overlayBlock = blocks.find((block) => block.id === "topic-timeline-overlay" || block.title === "本专题的大运流年");
  const relationshipTriggerBlock = topic === "relationship"
    ? blocks.find((block) => block.id === "relationship-trigger" || block.title === "年度情感引动")
    : null;
  const trace = Array.isArray(data.trace) ? data.trace.filter((item) => item.id === "topic-timeline-overlay" || item.source === "timeline-core-v1") : [];
  if (!overlayBlock && !relationshipTriggerBlock && trace.length === 0) return null;

  const section = createEl("section", "timeline-report-guide topic-timeline-guide report-reveal");
  section.id = "topic-timeline-guide";
  section.dataset.sectionIndex = "timeline";
  section.setAttribute("aria-label", `${topicMeta.label}专项时间解释导航`);

  const header = createEl("div", "timeline-guide-head");
  header.append(
    createEl("div", "report-block-kicker", "专题时间解释"),
    createEl("h2", "timeline-guide-title", "本专题的时间解释"),
    createEl("p", "timeline-guide-lead", "这里把大运与流年的结构线索落到当前专题。页面只呈现专题叠加、阅读依据和阅读边界，不额外加入评分或事件断语。")
  );

  const controls = createTopicYearControls();
  const cards = createEl("div", "timeline-guide-cards");
  cards.append(
    createTimelineGuideCard({
      title: "当前专题",
      value: data.topic_label || topicMeta.label,
      note: topicMeta.capability,
      href: "#topic-timeline-guide",
      tone: "topic"
    }),
    createTimelineGuideCard({
      title: "显式年度",
      value: `${data.year || request.year} 年`,
      note: data.year_source === "explicit" ? "显式年份" : safeText(data.year_source, "年度"),
      href: "#topic-timeline-guide",
      tone: "annual"
    }),
    createTimelineGuideCard({
      title: "专题叠加章节",
      value: relationshipTriggerBlock ? relationshipTriggerBlock.title : (overlayBlock ? overlayBlock.title : "章节待整理"),
      note: trace.length > 0 ? "已整理阅读依据" : "以章节正文为主",
      href: relationshipTriggerBlock
        ? findReportBlockHref(blocks, relationshipTriggerBlock.title)
        : findReportBlockHref(blocks, "本专题的大运流年"),
      tone: "evidence"
    })
  );

  const evidence = createEl("div", "timeline-evidence-grid");
  evidence.id = "topic-timeline-evidence";
  evidence.append(createTopicTraceDetails(trace, topic === "relationship" ? [] : data.warnings));
  section.append(header, controls, cards, evidence);
  return section;
}

function createTopicYearControls() {
  const form = createEl("form", "timeline-year-control");
  form.setAttribute("aria-label", "专项报告显式年份选择");
  form.append(createYearField("专题观察年", "year", request.year), createEl("button", "timeline-year-submit", "应用年份"));
  form.addEventListener("submit", (event) => {
    event.preventDefault();
    const formData = new FormData(form);
    const next = new URLSearchParams(window.location.search);
    next.set("topic", topic);
    next.set("date", request.date);
    if (request.time) next.set("time", request.time);
    next.set("timezone", request.timezone);
    next.set("time_precision", request.timePrecision);
    next.set("sex", request.sex || "unspecified");
    next.set("year", sanitizeYear(formData.get("year"), request.year));
    window.location.href = `/topic-report.html?${next.toString()}`;
  });
  return form;
}

function createYearField(label, name, value) {
  const field = createEl("label", "timeline-year-field");
  field.append(createEl("span", "", label));
  const input = document.createElement("input");
  input.name = name;
  input.type = "number";
  input.inputMode = "numeric";
  input.min = "1901";
  input.max = "2100";
  input.step = "1";
  input.value = String(value || new Date().getFullYear());
  field.append(input);
  return field;
}

function createTimelineGuideCard({ title, value, note, href, tone }) {
  const card = createEl("a", `timeline-guide-card ${tone || ""}`);
  card.href = href || "#topic-timeline-guide";
  card.append(
    createEl("span", "timeline-guide-card-title", title),
    createEl("strong", "", value),
    createEl("em", "", note)
  );
  return card;
}

function createTopicTraceDetails(trace, warnings) {
  const details = createEl("details", "timeline-evidence-detail");
  details.append(createEl("summary", "", "阅读依据与边界"));
  const content = createEl("div", "timeline-evidence-content");
  if (trace.length > 0) {
    trace.forEach((item) => {
      const label = traceSourceLabel(item.source || item.id);
      const interpretation = safeText(item.interpretation, "本段依据当前命盘结构与显式年份合参。");
      content.append(createTimelineEvidenceItem(label, interpretation));
    });
  } else {
    content.append(createTimelineEvidenceItem("说明", "当前专题以完整章节正文作为阅读主体。"));
  }
  if (Array.isArray(warnings) && warnings.length > 0) {
    const list = createEl("ul", "timeline-evidence-list");
    warnings.slice(0, 4).forEach((warning) => list.append(createEl("li", "", safeText(warning, "边界提醒"))));
    content.append(createTimelineEvidenceItem(
      "边界提醒",
      "以下提醒来自阅读边界规则；本段是命理结构阅读，不是现实事件预告。"
    ));
    content.append(list);
  }
  details.append(content);
  return details;
}

function createTimelineEvidenceItem(label, text) {
  const item = createEl("p", "timeline-evidence-item");
  item.append(createEl("strong", "", label));
  item.append(document.createTextNode(`：${safeText(text, "暂无说明。")}`));
  return item;
}

function createReportBlock(block, indexNumber) {
  const num = String(indexNumber + 1).padStart(2, "0");
  const timelineKind = topicTimelineBlockKind(block);
  const section = createEl("section", `report-block report-reveal${timelineKind ? " is-timeline-block" : ""}`);
  section.id = `report-section-${indexNumber + 1}`;
  section.dataset.sectionIndex = String(indexNumber + 1);
  section.dataset.sectionNumber = num;
  if (timelineKind) section.dataset.timelineKind = timelineKind;
  section.style.setProperty("--block-delay", `${Math.min(indexNumber * 80, 480)}ms`);

  const frame = createEl("span", "report-block-frame");
  frame.setAttribute("aria-hidden", "true");
  const aura = createEl("span", "report-block-aura");
  aura.setAttribute("aria-hidden", "true");
  const scan = createEl("span", "report-block-scan");
  scan.setAttribute("aria-hidden", "true");

  const kicker = createEl("div", "report-block-kicker", `专项章节 ${num}`);
  const title = createEl("h2", "report-block-title", safeText(block.title, "未命名章节"));
  const rule = createEl("i", "report-block-rule");
  rule.setAttribute("aria-hidden", "true");
  const body = createReportBody(block, timelineKind);

  const footer = createEl("footer", "report-block-footer");
  footer.append(
    createEl("span", "", topicMeta.capability),
    createEl("span", "", "显式年份"),
    createEl("span", "", timelineKind ? "专题时间解释" : "命轨")
  );

  section.append(frame, aura, scan, kicker, title, rule, body, footer);
  return section;
}

function createReportBody(block, timelineKind) {
  const body = createEl("div", "report-block-body");
  const text = safeText(block.body, "暂无正文内容。");
  text.split(/\n{2,}/).filter(Boolean).forEach((paragraph) => {
    body.append(createEl("p", "", paragraph));
  });
  if (timelineKind) {
    const details = createEl("details", "timeline-inline-detail");
    details.append(
      createEl("summary", "", "专题边界"),
      createEl("p", "", "本章节只展示共享时间解释引擎投射到当前专题后的解释，不是现实事件预告，也不构成确定性事件断语或现实决策承诺。")
    );
    body.append(details);
  }
  return body;
}

function createArchiveFooter(blockCount) {
  const footer = createEl("section", "report-archive-footer report-reveal");
  footer.id = "report-archive-end";
  footer.dataset.sectionIndex = "end";
  footer.append(
    createEl("div", "archive-seal", "专"),
    createEl("h2", "", "专项报告已归档"),
    createEl("p", "", `本次${topicMeta.label}专项报告共 ${blockCount} 个章节。页面仅展示只读结构化内容，不追加现实断言。`)
  );
  return footer;
}

function mountReportEffects() {
  const root = document.documentElement;
  const body = document.body;
  const progress = document.getElementById("report-progress");
  const progressLabel = document.getElementById("report-progress-label");
  const hero = document.querySelector(".report-hero");
  const revealItems = document.querySelectorAll(".report-reveal");
  const indexLinks = document.querySelectorAll(".report-index a");
  const activeTargets = document.querySelectorAll(".report-hero, .timeline-report-guide, .report-block");
  const sections = document.querySelectorAll(".timeline-report-guide, .report-block");
  const reducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  if (reducedMotion) body.classList.add("is-reduced-motion");

  let ticking = false;
  function updateScrollState() {
    const scrollTop = window.scrollY || document.documentElement.scrollTop;
    const max = Math.max(1, document.documentElement.scrollHeight - window.innerHeight);
    const ratio = clamp(scrollTop / max, 0, 1);
    const percent = Math.round(ratio * 100);
    root.style.setProperty("--scroll-progress", ratio.toFixed(5));
    root.style.setProperty("--scroll-x", `${ratio * 46}px`);
    root.style.setProperty("--scroll-y", `${ratio * -128}px`);
    root.style.setProperty("--ambient-rotate", `${-8 + ratio * 6}deg`);
    if (progress) progress.style.transform = `scaleX(${ratio})`;
    if (progressLabel) progressLabel.textContent = `只读报告 · ${String(percent).padStart(2, "0")}%`;

    if (hero) {
      const rect = hero.getBoundingClientRect();
      const hh = Math.max(1, hero.offsetHeight);
      body.classList.toggle("report-index-live", scrollTop > hh * 0.72);
      const hp = clamp(-rect.top / hh, 0, 1);
      hero.style.setProperty("--hero-y", `${hp * -54}px`);
      hero.style.setProperty("--hero-scale", `${1 - hp * 0.045}`);
      hero.style.setProperty("--hero-opacity", `${1 - hp * 0.72}`);
      hero.style.setProperty("--hero-luopan-scale", `${1 + hp * 0.12}`);
      hero.style.setProperty("--hero-luopan-opacity", `${0.32 - hp * 0.22}`);
    }

    const current = document.querySelector(".report-block.is-current");
    if (current) {
      const r = current.getBoundingClientRect();
      const start = window.innerHeight * 0.78;
      const end = -r.height * 0.18;
      const sp = clamp((start - r.top) / Math.max(1, start - end), 0, 1);
      current.style.setProperty("--section-progress", sp.toFixed(4));
      current.style.setProperty("--section-glow", `${0.22 + sp * 0.32}`);
      current.style.setProperty("--section-scan-y", `${sp * 100}%`);
    }
    ticking = false;
  }

  function requestTick() {
    if (ticking) return;
    ticking = true;
    requestAnimationFrame(updateScrollState);
  }

  window.addEventListener("scroll", requestTick, { passive: true });
  window.addEventListener("resize", requestTick);
  updateScrollState();

  if (!("IntersectionObserver" in window)) {
    revealItems.forEach((item) => item.classList.add("is-visible"));
    return;
  }

  const revealObserver = new IntersectionObserver((entries) => {
    entries.forEach((entry) => {
      entry.target.classList.toggle("is-in-view", entry.isIntersecting);
      if (entry.isIntersecting || entry.boundingClientRect.top < 0) {
        entry.target.classList.add("is-visible");
      }
    });
  }, { threshold: [0, 0.12, 0.24, 0.38], rootMargin: "0px 0px -10% 0px" });
  revealItems.forEach((item) => revealObserver.observe(item));

  const activeRatios = new Map();
  const activeObserver = new IntersectionObserver((entries) => {
    entries.forEach((entry) => {
      if (entry.isIntersecting) activeRatios.set(entry.target.id, entry.intersectionRatio);
      else activeRatios.delete(entry.target.id);
    });
    let bestId = "report-cover";
    let bestRatio = 0;
    activeRatios.forEach((ratio, id) => {
      if (ratio > bestRatio) {
        bestRatio = ratio;
        bestId = id;
      }
    });
    setActiveTarget(bestId);
    requestTick();
  }, { threshold: buildThresholdList(10), rootMargin: "-42% 0px -42% 0px" });
  activeTargets.forEach((target) => activeObserver.observe(target));

  const jumpLinks = document.querySelectorAll(".report-index a, .report-enter");
  jumpLinks.forEach((link) => {
    link.addEventListener("click", (event) => {
      const href = link.getAttribute("href");
      if (!href || !href.startsWith("#")) return;
      const target = document.querySelector(href);
      if (!target) return;
      event.preventDefault();
      target.scrollIntoView({
        behavior: reducedMotion ? "auto" : "smooth",
        block: target.classList.contains("report-hero") ? "start" : "center"
      });
    });
  });

  function setActiveTarget(activeId) {
    const activeTarget = document.getElementById(activeId);
    indexLinks.forEach((link) => {
      link.classList.toggle("is-active", link.getAttribute("href") === `#${activeId}`);
    });
    sections.forEach((section) => {
      section.classList.toggle("is-current", section.id === activeId);
    });
    if (activeTarget?.dataset.sectionIndex) {
      root.style.setProperty("--active-section", activeTarget.dataset.sectionIndex);
    }
  }
}

function normalizeTopic(value) {
  return Object.prototype.hasOwnProperty.call(TOPICS, value || "") ? value : "relationship";
}

function traceSourceLabel(source) {
  const labels = {
    "timeline-core-v1": "共享时间解释规则",
    "analysis.ten_gods": "十神结构分析",
    "deep_analysis.pattern": "格局与用神分析",
    "luck/year ten-gods and branches": "大运流年十神与宫位关系",
    "topic-timeline-overlay": "专题时间叠加"
  };
  return labels[source] || "结构依据";
}

function readYear(value) {
  if (value && /^\d{4}$/.test(value)) return Number.parseInt(value, 10);
  return new Date().getFullYear();
}

function buildThresholdList(steps) {
  return Array.from({ length: steps + 1 }, (_, i) => i / steps);
}

function findReportBlockHref(blocks, title) {
  const index = blocks.findIndex((block) => block.title === title);
  return index >= 0 ? `#report-section-${index + 1}` : "#topic-timeline-guide";
}

function topicTimelineBlockKind(block) {
  if (block?.id === "topic-timeline-overlay" || block?.title === "本专题的大运流年") return "topic";
  if (block?.title === "大运与流年引动") return "annual";
  return "";
}

function sanitizeYear(value, fallback) {
  const text = String(value || "").trim();
  return /^\d{4}$/.test(text) ? text : String(fallback || new Date().getFullYear());
}

function createEl(tag, className = "", text = "") {
  const element = document.createElement(tag);
  if (className) element.className = className;
  if (text) element.textContent = localizeVisibleText(text);
  return element;
}

function safeText(value, fallback) {
  if (typeof value !== "string") return fallback;
  const trimmed = value.trim();
  return trimmed.length > 0 ? localizeVisibleText(trimmed) : fallback;
}

function localizeVisibleText(value) {
  return String(value || "")
    .replaceAll("shared timeline engine", "共享时间解释引擎")
    .replaceAll("timeline engine", "时间解释引擎")
    .replaceAll("timeline-core-v1", "共享时间解释规则")
    .replaceAll("topic-timeline-overlay", "专题时间叠加")
    .replaceAll("luck-annual-overlay", "大运年度叠加")
    .replaceAll("major-luck-current", "当前大运")
    .replaceAll("major-luck-previous", "上一阶段大运")
    .replaceAll("major-luck-next", "下一阶段大运")
    .replaceAll("annual-trigger", "年度引动")
    .replaceAll("direct_officer", "正官")
    .replaceAll("seven_killings", "七杀")
    .replaceAll("direct_resource", "正印")
    .replaceAll("indirect_resource", "偏印")
    .replaceAll("eating_god", "食神")
    .replaceAll("hurting_officer", "伤官")
    .replaceAll("direct_wealth", "正财")
    .replaceAll("indirect_wealth", "偏财")
    .replaceAll("rob_wealth", "劫财")
    .replaceAll("relationship-report", "情感专项")
    .replaceAll("wealth-report", "金钱专项")
    .replaceAll("family-report", "家庭专项")
    .replaceAll("career-report", "事业专项")
    .replaceAll("restricted", "边界锁定");
}

function sexLabel(value) {
  if (value === "male") return "男";
  if (value === "female") return "女";
  return "未指定";
}

function clamp(value, min, max) {
  return Math.min(max, Math.max(min, value));
}

function renderError(message) {
  if (!container) return;
  container.replaceChildren(createEl("p", "empty-state report-error-state", message));
}
