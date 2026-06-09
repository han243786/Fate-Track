import { formatBytes, formatRange } from "../utils/format.js";

export function renderHealth(dom, state, label) {
  dom.healthPill.dataset.state = state;
  dom.healthLabel.textContent = label;
}

export function renderMeta(dom, meta) {
  dom.fields.yearCount.textContent = String(meta.year_count ?? "--");
  dom.fields.termCount.textContent = String(meta.term_count ?? "--");
  dom.fields.version.textContent = meta.version ?? "--";
  dom.fields.bytes.textContent = formatBytes(meta.bytes);
  dom.fields.range.textContent = formatRange(meta.min_year, meta.max_year);
  dom.fields.sourcePath.textContent = meta.path ?? "--";
}

export function renderCapabilities(dom, capabilities) {
  dom.capabilityList.innerHTML = "";

  for (const capability of capabilities) {
    const item = document.createElement("li");
    item.className = "capability-item";
    item.dataset.status = capability.status;

    const name = document.createElement("span");
    name.textContent = capability.id;

    const status = document.createElement("span");
    status.className = "capability-status";
    status.textContent = capability.status;

    item.append(name, status);
    dom.capabilityList.append(item);
  }
}

export function renderChart(dom, result) {
  dom.chart.status.dataset.state = "ok";
  dom.chart.status.textContent = result.metadata?.algo_version ?? "已支持";
  dom.chart.pillars.innerHTML = "";
  const pillars = result.pillars ?? {};
  for (const [label, pillar] of [
    ["年", pillars.year],
    ["月", pillars.month],
    ["日", pillars.day],
    ["时", pillars.hour]
  ]) {
    const item = document.createElement("div");
    const dt = document.createElement("dt");
    const dd = document.createElement("dd");
    dt.textContent = label;
    dd.textContent = pillar?.ganzhi ?? "未知";
    item.append(dt, dd);
    dom.chart.pillars.append(item);
  }
  renderStatusList(dom.chart.warnings, [
    ...(result.warnings ?? []),
    ...(result.ambiguity_flags ?? []),
    ...(result.unsupported_outputs ?? []).map((item) => `暂不支持：${item}`)
  ]);
}

export function renderChartError(dom, error) {
  dom.chart.status.dataset.state = "error";
  dom.chart.status.textContent = "错误";
  dom.chart.pillars.innerHTML = "";
  renderStatusList(dom.chart.warnings, [error.message]);
}

export function renderAnalysis(dom, snapshot) {
  dom.analysis.metrics.innerHTML = "";
  const metrics = snapshot.metrics ?? {};
  for (const [label, values] of [
    ["五行", metrics.elements],
    ["十神", metrics.ten_gods],
    ["藏干", metrics.hidden_stems]
  ]) {
    const column = document.createElement("section");
    column.className = "metric-column";
    const heading = document.createElement("h3");
    heading.textContent = label;
    const list = document.createElement("ul");
    for (const metric of values ?? []) {
      const item = document.createElement("li");
      item.textContent = `${metric.id}: ${metric.weight_x2}`;
      list.append(item);
    }
    column.append(heading, list);
    dom.analysis.metrics.append(column);
  }

  dom.analysis.cards.innerHTML = "";
  for (const card of snapshot.cards ?? []) {
    const item = document.createElement("article");
    item.className = "analysis-card";
    const title = document.createElement("h3");
    title.textContent = card.title;
    const body = document.createElement("p");
    body.textContent = card.body;
    item.append(title, body);
    dom.analysis.cards.append(item);
  }
}

export function renderAnalysisError(dom, error) {
  dom.analysis.metrics.innerHTML = "";
  dom.analysis.cards.innerHTML = "";
  const item = document.createElement("p");
  item.className = "empty-state";
  item.textContent = error.message;
  dom.analysis.cards.append(item);
}

export function renderCases(dom, cases) {
  dom.cases.list.innerHTML = "";
  if (!cases.length) {
    dom.cases.list.append(emptyState("暂无本地案例"));
    return;
  }
  for (const record of cases) {
    const item = document.createElement("article");
    item.className = "case-item";
    const title = document.createElement("h3");
    title.textContent = record.title;
    const meta = document.createElement("p");
    meta.textContent = `${record.status} / ${record.chart_snapshot_id}`;
    const tags = document.createElement("p");
    tags.textContent = (record.tags ?? []).join(", ") || "未分类";
    item.append(title, meta, tags);
    dom.cases.list.append(item);
  }
}

export function renderShare(dom, share) {
  dom.share.preview.innerHTML = "";
  if (!share) {
    dom.share.preview.append(emptyState("先创建案例，再创建脱敏分享预览"));
    return;
  }
  const token = document.createElement("p");
  token.className = "token-line";
  token.textContent = `令牌：${share.token ?? "--"}`;
  const details = document.createElement("dl");
  details.className = "share-details";
  for (const [label, value] of [
    ["DTO", share.public_dto?.dto_version],
    ["禁止索引", String(share.public_dto?.noindex ?? true)],
    ["可编辑", String(share.public_dto?.editable ?? false)],
    ["排盘算法", share.public_dto?.chart_snapshot?.chart_algo_version],
    ["分析算法", share.public_dto?.analysis_snapshot?.analysis_algo_version]
  ]) {
    const wrap = document.createElement("div");
    const dt = document.createElement("dt");
    const dd = document.createElement("dd");
    dt.textContent = label;
    dd.textContent = value ?? "--";
    wrap.append(dt, dd);
    details.append(wrap);
  }
  dom.share.preview.append(token, details);
}

export function renderCalendarResult(dom, result) {
  dom.calendar.status.dataset.state = "ok";
  dom.calendar.status.textContent = result.meta?.algorithm_version ?? "已支持";
  dom.calendar.gregorian.textContent = [
    result.gregorian?.year,
    pad2(result.gregorian?.month),
    pad2(result.gregorian?.day)
  ].join("-");
  dom.calendar.lunar.textContent = [
    result.lunar?.year,
    result.lunar?.month_name,
    result.lunar?.day_name
  ]
    .filter(Boolean)
    .join(" ");
  dom.calendar.ganzhi.textContent = [
    result.ganzhi?.year,
    result.ganzhi?.month,
    result.ganzhi?.day
  ]
    .filter(Boolean)
    .join(" / ");
  dom.calendar.ruleset.textContent = result.meta?.ruleset_id ?? "--";
}

export function renderCalendarError(dom, error) {
  dom.calendar.status.dataset.state = "error";
  dom.calendar.status.textContent = "错误";
  dom.calendar.gregorian.textContent = "--";
  dom.calendar.lunar.textContent = "--";
  dom.calendar.ganzhi.textContent = error.message;
  dom.calendar.ruleset.textContent = "--";
}

export function renderError(dom, error) {
  dom.fields.yearCount.textContent = "--";
  dom.fields.termCount.textContent = "--";
  dom.fields.version.textContent = "--";
  dom.fields.bytes.textContent = "--";
  dom.fields.range.textContent = "未知";
  dom.fields.sourcePath.textContent = error.message;
  dom.capabilityList.innerHTML = "";
  renderCalendarError(dom, error);
}

function renderStatusList(container, values) {
  container.innerHTML = "";
  for (const value of values) {
    const item = document.createElement("span");
    item.textContent = value;
    container.append(item);
  }
}

function emptyState(message) {
  const item = document.createElement("p");
  item.className = "empty-state";
  item.textContent = message;
  return item;
}

function pad2(value) {
  return String(value ?? "").padStart(2, "0");
}
