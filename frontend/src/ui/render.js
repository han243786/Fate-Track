const ELEMENT_TONES = { "木":"wood", "火":"fire", "土":"earth", "金":"metal", "水":"water" };
const STEM_ELEMENT = { "甲":"木","乙":"木","丙":"火","丁":"火","戊":"土","己":"土","庚":"金","辛":"金","壬":"水","癸":"水" };
const STEM_YINYANG = { "甲":"阳木","乙":"阴木","丙":"阳火","丁":"阴火","戊":"阳土","己":"阴土","庚":"阳金","辛":"阴金","壬":"阳水","癸":"阴水" };
const BRANCH_ELEMENT = { "子":"水","丑":"土","寅":"木","卯":"木","辰":"土","巳":"火","午":"火","未":"土","申":"金","酉":"金","戌":"土","亥":"水" };
const METRIC_LABELS = {
  wood:"木", fire:"火", earth:"土", metal:"金", water:"水",
  peer:"比肩", rob_wealth:"劫财", eating_god:"食神", hurting_officer:"伤官",
  direct_wealth:"正财", indirect_wealth:"偏财", direct_officer:"正官", seven_killings:"七杀",
  direct_resource:"正印", indirect_resource:"偏印"
};
const SEVERITY_LABELS = {
  info: "提示",
  warn: "关注",
  warning: "关注",
  danger: "重要",
  critical: "重要"
};
const INSIGHT_TAG_HIDDEN_TITLES = ["日主", "格局", "用神"];

export function renderChart(dom, result) {
  dom.chart.pillars.innerHTML = "";
  const pillars = result.pillars ?? {};
  const labels = ["年柱","月柱","日柱","时柱"];
  const keys = ["year","month","day","hour"];
  const dayStem = pillars.day?.stem ?? "";

  keys.forEach((key, i) => {
    const p = pillars[key];
    const ganzhi = p?.ganzhi ?? "未知";
    const stem = p?.stem ?? "?";
    const branch = p?.branch ?? "?";
    const el = STEM_ELEMENT[stem] || "土";
    const tone = ELEMENT_TONES[el] || "earth";
    const isDay = key === "day";

    const card = document.createElement("article");
    card.className = `pillar-card ${tone}${isDay ? " active" : ""}`;

    card.innerHTML = `
      <div class="pillar-label">${labels[i]}</div>
      <div class="element-orb">${el}</div>
      <div class="stem-branch"><span>${stem}</span><span>${branch}</span></div>
      <div class="pillar-meta"><span>${STEM_YINYANG[stem]||stem}</span><span>${BRANCH_ELEMENT[branch]||""} ${branch}</span></div>
      ${isDay ? '<div class="day-master">◆ 日主 ◆</div>' : ''}
    `;
    dom.chart.pillars.append(card);
  });

  // Summary strip
  dom.chart.summary.innerHTML = "";
  const summaryItems = [
    { icon:"✺", label:"日主", value:`${dayStem}${STEM_ELEMENT[dayStem]||""}` },
    { icon:"⬡", label:"年柱", value:pillars.year?.ganzhi||"--" },
    { icon:"◈", label:"月柱", value:pillars.month?.ganzhi||"--" },
    { icon:"✵", label:"时柱", value:pillars.hour?.ganzhi||"未知" },
  ];
  summaryItems.forEach(s => {
    const div = document.createElement("div");
    div.innerHTML = `<span>${s.icon}</span>${s.label}：<strong>${s.value}</strong>`;
    dom.chart.summary.append(div);
  });
}

export function renderChartError(dom, error) {
  dom.chart.pillars.innerHTML = `<p class="empty-state">排盘暂时没有完成，请检查出生日期与时间后再试。</p>`;
  dom.chart.summary.innerHTML = "";
}

const BRANCH_HIDDEN = {
  "子":["癸"], "丑":["己","癸","辛"], "寅":["甲","丙","戊"], "卯":["乙"],
  "辰":["戊","乙","癸"], "巳":["丙","戊","庚"], "午":["丁","己"], "未":["己","丁","乙"],
  "申":["庚","壬","戊"], "酉":["辛"], "戌":["戊","辛","丁"], "亥":["壬","甲"]
};

export function renderAnalysis(dom, snapshot, chartResult) {
  const metrics = snapshot.metrics ?? {};

  // Element bars
  dom.analysis.elementBars.innerHTML = "";
  const elements = metrics.elements ?? [];
  const maxVal = Math.max(...elements.map(m => m.weight_x2), 1);
  elements.forEach(m => {
    const cn = METRIC_LABELS[m.id] || m.id;
    const tone = ELEMENT_TONES[cn] || "earth";
    const row = document.createElement("div");
    row.className = `element-row ${tone}`;
    row.innerHTML = `
      <span class="element-name">${cn}</span>
      <span class="bar-track"><i style="width:${Math.round(m.weight_x2 / maxVal * 100)}%"></i></span>
      <strong>${m.weight_x2}</strong>
    `;
    dom.analysis.elementBars.append(row);
  });

  // God chips
  dom.analysis.godChips.innerHTML = "";
  const gods = metrics.ten_gods ?? [];
  let total = 0;
  gods.forEach(m => {
    if (m.weight_x2 > 0) {
      const chip = document.createElement("span");
      chip.className = "god-chip";
      chip.innerHTML = `<em>${METRIC_LABELS[m.id]||m.id}</em><strong>${m.weight_x2}</strong>`;
      dom.analysis.godChips.append(chip);
      total += m.weight_x2;
    }
  });
  dom.analysis.godTotal.textContent = String(total);

  // Hidden stem table — derive actual stems from chart pillar branches
  dom.analysis.hiddenStemTable.innerHTML = "";
  const pillars = chartResult?.pillars ?? {};
  const branchMap = {
    year_branch_hidden: pillars.year?.branch || null,
    month_branch_hidden: pillars.month?.branch || null,
    day_branch_hidden: pillars.day?.branch || null,
    hour_branch_hidden: pillars.hour?.branch || null,
  };
  const hiddenLabels = { year_branch_hidden:"年支", month_branch_hidden:"月支", day_branch_hidden:"日支", hour_branch_hidden:"时支" };
  const hidden = metrics.hidden_stems ?? [];

  if (hidden.length > 0) {
    const table = document.createElement("table");
    table.innerHTML = `
      <thead><tr><th>柱位</th><th>地支</th><th>藏干</th><th>数量</th></tr></thead>
      <tbody></tbody>
    `;
    const tbody = table.querySelector("tbody");

    const orderedKeys = ["year_branch_hidden","month_branch_hidden","day_branch_hidden","hour_branch_hidden"];
    orderedKeys.forEach(key => {
      const metric = hidden.find(m => m.id === key);
      const branch = branchMap[key];
      if (!branch) return;
      const stems = BRANCH_HIDDEN[branch] || [];
      const tr = document.createElement("tr");
      tr.innerHTML = `
        <td>${hiddenLabels[key]}</td>
        <td>${branch}</td>
        <td>${stems.join("、")}</td>
        <td>${stems.length}</td>
      `;
      tbody.append(tr);
    });
    dom.analysis.hiddenStemTable.append(table);
  }

  // Insight cards
  dom.analysis.cards.innerHTML = "";
  (snapshot.cards ?? []).forEach(card => {
    const article = document.createElement("article");
    article.className = "insight-card ornament-card";
    const severityTag = shouldRenderInsightTag(card.title)
      ? `<div class="insight-tag">${SEVERITY_LABELS[card.severity] || "提示"}</div>`
      : "";
    article.innerHTML = `
      <div class="insight-title"><span>◉</span><h2>${card.title}</h2></div>
      <p>${card.body}</p>
      ${severityTag}
    `;
    dom.analysis.cards.append(article);
  });
}

function shouldRenderInsightTag(title) {
  return !INSIGHT_TAG_HIDDEN_TITLES.some(keyword => String(title || "").includes(keyword));
}

export function renderAnalysisError(dom, error) {
  dom.analysis.elementBars.innerHTML = `<p class="empty-state">结构分析暂时没有完成，请重新起盘后再试。</p>`;
  dom.analysis.godChips.innerHTML = "";
  dom.analysis.hiddenStemTable.innerHTML = "";
  dom.analysis.cards.innerHTML = "";
}

const TOPIC_LABELS = {
  relationship: "\u60c5\u611f",
  wealth: "\u91d1\u94b1",
  family: "\u5bb6\u5ead",
  career: "\u4e8b\u4e1a"
};

export function renderTopicReportLoading(dom, topic) {
  const label = topicLabel(topic);
  dom.topicReport.title.textContent = `${label}\u7ed3\u6784\u7ebf\u7d22`;
  dom.topicReport.status.textContent = "\u6574\u7406\u4e2d";
  dom.topicReport.content.innerHTML = "";
  const state = document.createElement("p");
  state.className = "empty-state";
  state.textContent = "\u7ed3\u6784\u7ebf\u7d22\u6b63\u5728\u6574\u7406";
  dom.topicReport.content.append(state);
}

export function renderTopicReportIdle(dom, message = "\u4e13\u9879\u63a8\u6f14\u5f85\u6574\u7406") {
  dom.topicReport.title.textContent = "\u4e13\u9879\u63a8\u6f14";
  dom.topicReport.status.textContent = "\u5f85\u6574\u7406";
  dom.topicReport.content.innerHTML = "";
  const state = document.createElement("p");
  state.className = "empty-state";
  state.textContent = message;
  dom.topicReport.content.append(state);
}

export function renderTopicReport(dom, report) {
  const label = report.topic_label || topicLabel(report.topic);
  dom.topicReport.title.textContent = `${label}\u7ed3\u6784\u7ebf\u7d22`;
  dom.topicReport.status.textContent = `${report.year} \u00b7 ${yearSourceLabel(report.year_source)}`;
  dom.topicReport.content.innerHTML = "";

  const signals = document.createElement("section");
  signals.className = "topic-signal-section topic-signal-only";
  const title = document.createElement("h3");
  title.textContent = "\u7ed3\u6784\u7ebf\u7d22";
  signals.append(title);

  if (Array.isArray(report.signals) && report.signals.length > 0) {
    const grid = document.createElement("div");
    grid.className = "topic-signal-grid";
    report.signals.forEach(signal => {
      const item = document.createElement("div");
      item.className = "topic-signal";
      const head = document.createElement("div");
      head.className = "topic-signal-head";
      const labelNode = document.createElement("span");
      labelNode.textContent = signal.label || signal.id;
      const level = document.createElement("strong");
      level.textContent = signal.qualitative_level || "";
      head.append(labelNode, level);
      const summary = document.createElement("p");
      summary.textContent = localizeVisibleText(signal.summary || "");
      item.append(head, summary);
      grid.append(item);
    });
    signals.append(grid);
  } else {
    const empty = document.createElement("p");
    empty.className = "empty-state";
    empty.textContent = "\u6682\u65e0\u7ed3\u6784\u7ebf\u7d22";
    signals.append(empty);
  }

  dom.topicReport.content.append(signals);
}

export function renderTopicReportError(dom, error) {
  dom.topicReport.title.textContent = "\u4e13\u9879\u63a8\u6f14";
  dom.topicReport.status.textContent = "\u6682\u672a\u6574\u7406";
  dom.topicReport.content.innerHTML = "";
  const state = document.createElement("p");
  state.className = "empty-state";
  state.textContent = "\u7ed3\u6784\u7ebf\u7d22\u6682\u65f6\u6ca1\u6709\u6574\u7406\u5b8c\u6210\uff0c\u8bf7\u91cd\u65b0\u8d77\u76d8\u540e\u518d\u8bd5\u3002";
  dom.topicReport.content.append(state);
}

function topicLabel(topic) {
  return TOPIC_LABELS[topic] || "\u4e13\u9879";
}

function yearSourceLabel(source) {
  return source === "explicit" ? "\u663e\u5f0f\u5e74\u5ea6" : source || "\u5e74\u5ea6";
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

export function renderLuckCycles(dom, data, luckReading, annualTriggerReading) {
  dom.luck.container.innerHTML = "";
  dom.luck.current.innerHTML = "";
  dom.luck.direction.textContent = "";

  if (!data || !data.cycles) {
    dom.luck.container.innerHTML = `<li class="empty-state">排盘后自动显示大运</li>`;
    return;
  }

  dom.luck.direction.textContent = data.direction === "forward" ? "顺行（阳年男命 / 阴年女命）" : "逆行（阳年女命 / 阴年男命）";
  const currentIndex = Number.isInteger(luckReading?.current_index)
    ? luckReading.current_index - 1
    : 0;
  const currentCycle = luckReading?.current_cycle ?? data.cycles[currentIndex] ?? data.cycles[0] ?? null;
  const firstReading = localizeVisibleText(luckReading?.readings?.[0]?.plain || "");
  const annualSummary = localizeVisibleText(annualTriggerReading?.signals?.find(signal => signal.source === "annual-trigger")?.summary
    || annualTriggerReading?.signals?.[0]?.summary
    || "");

  data.cycles.forEach((c, i) => {
    const li = document.createElement("li");
    const isCurrent = currentCycle
      ? c.start_age === currentCycle.start_age && c.end_age === currentCycle.end_age && c.ganzhi === currentCycle.ganzhi
      : i === currentIndex;
    li.className = `luck-item${isCurrent ? " current glow" : ""}`;
    if (isCurrent) li.setAttribute("aria-current", "step");
    li.innerHTML = `
      <span class="timeline-node" aria-hidden="true"></span>
      <article class="luck-card">
        <div class="luck-age"><strong>${c.start_age}-${c.end_age}岁</strong></div>
        <div class="luck-gz">${c.ganzhi}</div>
        <div class="luck-tags">${isCurrent ? '<span class="luck-current-badge">当前</span>' : ""}<span>${c.label}</span></div>
      </article>
    `;
    dom.luck.container.append(li);
  });

  // Current luck card
  if (currentCycle) {
    const yearText = luckReading?.reference_year
      ? `${luckReading.reference_year}年 · ${luckReading.reference_age}岁`
      : "未指定观察年份";
    const annualText = annualTriggerReading?.year
      ? `${annualTriggerReading.year}年 · ${annualTriggerReading.annual_pillar?.ganzhi || "年度干支"}`
      : "未指定年度引动";
    const readingText = firstReading || "大运解释层未返回日常摘要，当前仅展示阶段坐标。";
    const hasReadingBasis = (
      (luckReading?.signals?.length || 0)
      + (annualTriggerReading?.signals?.length || 0)
      + (luckReading?.evidence?.length || 0)
      + (annualTriggerReading?.evidence?.length || 0)
    ) > 0;
    dom.luck.current.innerHTML = `
      <div class="current-luck-meta">
        <span>观察年份 <strong>${yearText}</strong></span>
        <span>引动年份 <strong>${annualText}</strong></span>
        <span>依据 <strong>${hasReadingBasis ? "已整理" : "待整理"}</strong></span>
      </div>
      <p>当前大运：<strong>${currentCycle.start_age}-${currentCycle.end_age}岁 · ${currentCycle.ganzhi}</strong></p>
      <p class="luck-reading-summary">${readingText}</p>
      ${annualSummary ? `<p class="annual-trigger-summary">年度引动：${annualSummary}</p>` : ""}
      <p class="timeline-boundary-copy">工作台只保留短摘要；完整大运、年度引动和阅读依据请进入命盘报告页阅读。</p>
    `;
  }
}
