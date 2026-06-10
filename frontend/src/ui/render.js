export function renderChart(dom, result) {
  dom.chart.pillars.innerHTML = "";
  const pillars = result.pillars ?? {};

  for (const [label, pillar] of [
    ["年", pillars.year],
    ["月", pillars.month],
    ["日", pillars.day],
    ["时", pillars.hour]
  ]) {
    dom.chart.pillars.append(pillarCard(label, pillar));
  }
}

export function renderChartError(dom, error) {
  dom.chart.pillars.innerHTML = "";
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
      list.append(metricItem(metric));
    }

    if (!list.children.length) {
      const empty = document.createElement("li");
      empty.className = "metric-empty";
      empty.textContent = "等待排盘结果";
      list.append(empty);
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

export function renderLuckCycles(dom, data) {
  dom.luck.container.innerHTML = "";
  if (!data || !data.cycles) {
    dom.luck.container.append(emptyState("排盘后自动显示大运"));
    return;
  }
  const list = document.createElement("ol");
  list.className = "luck-list";
  for (const c of data.cycles) {
    const item = document.createElement("li");
    item.textContent = `${c.label}: ${c.ganzhi}（${c.start_age}-${c.end_age}岁）`;
    list.append(item);
  }
  dom.luck.container.append(list);
}

function pillarCard(label, pillar) {
  const item = document.createElement("div");
  item.className = "pillar-card";
  item.dataset.pillar = label;

  const dd = document.createElement("dd");
  dd.className = "pillar-value";

  const ganzhi = pillar?.ganzhi ?? "未知";
  const chars = [...String(ganzhi)];
  if (ganzhi === "未知") {
    const unknown = document.createElement("span");
    unknown.className = "pillar-unknown";
    unknown.textContent = ganzhi;
    dd.append(unknown);
  } else {
    const stem = document.createElement("span");
    stem.className = "pillar-stem";
    stem.textContent = chars[0] ?? "--";

    const branch = document.createElement("span");
    branch.className = "pillar-branch";
    branch.textContent = chars.slice(1).join("") || "--";

    dd.append(stem, branch);
  }

  item.append(dd);
  return item;
}

const metricLabels = {
  wood: "木", fire: "火", earth: "土", metal: "金", water: "水",
  peer: "比肩", rob_wealth: "劫财",
  eating_god: "食神", hurting_officer: "伤官",
  direct_wealth: "正财", indirect_wealth: "偏财",
  direct_officer: "正官", seven_killings: "七杀",
  direct_resource: "正印", indirect_resource: "偏印",
  year_branch_hidden: "年支藏干", month_branch_hidden: "月支藏干",
  day_branch_hidden: "日支藏干", hour_branch_hidden: "时支藏干"
};

function metricItem(metric) {
  const item = document.createElement("li");
  item.className = "metric-item";

  const name = document.createElement("span");
  name.className = "metric-name";
  name.textContent = metricLabels[metric.id] ?? metric.id;

  const value = document.createElement("span");
  value.className = "metric-value";
  value.textContent = String(metric.weight_x2);

  item.append(name, value);
  return item;
}

function emptyState(message) {
  const item = document.createElement("p");
  item.className = "empty-state";
  item.textContent = message;
  return item;
}

