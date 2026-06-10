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
  dom.chart.pillars.innerHTML = `<p class="empty-state">排盘失败: ${error.message}</p>`;
  dom.chart.summary.innerHTML = "";
}

export function renderAnalysis(dom, snapshot) {
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

  // Hidden stem table
  dom.analysis.hiddenStemTable.innerHTML = "";
  const hidden = metrics.hidden_stems ?? [];
  if (hidden.length > 0) {
    const table = document.createElement("table");
    table.innerHTML = `
      <thead><tr><th>柱位</th><th>藏干</th><th>权重</th></tr></thead>
      <tbody></tbody>
    `;
    const tbody = table.querySelector("tbody");
    const hiddenLabels = { year_branch_hidden:"年支", month_branch_hidden:"月支", day_branch_hidden:"日支", hour_branch_hidden:"时支" };
    hidden.forEach(m => {
      const tr = document.createElement("tr");
      tr.innerHTML = `<td>${hiddenLabels[m.id]||m.id}</td><td>—</td><td>${m.weight_x2}</td>`;
      tbody.append(tr);
    });
    dom.analysis.hiddenStemTable.append(table);
  }

  // Insight cards
  dom.analysis.cards.innerHTML = "";
  (snapshot.cards ?? []).forEach(card => {
    const article = document.createElement("article");
    article.className = "insight-card ornament-card";
    article.innerHTML = `
      <div class="insight-title"><span>◉</span><h2>${card.title}</h2></div>
      <p>${card.body}</p>
      <div class="insight-tag">${card.severity}</div>
    `;
    dom.analysis.cards.append(article);
  });
}

export function renderAnalysisError(dom, error) {
  dom.analysis.elementBars.innerHTML = `<p class="empty-state">${error.message}</p>`;
  dom.analysis.godChips.innerHTML = "";
  dom.analysis.hiddenStemTable.innerHTML = "";
  dom.analysis.cards.innerHTML = "";
}

export function renderLuckCycles(dom, data) {
  dom.luck.container.innerHTML = "";
  dom.luck.current.innerHTML = "";
  dom.luck.direction.textContent = "";

  if (!data || !data.cycles) {
    dom.luck.container.innerHTML = `<li class="empty-state">排盘后自动显示大运</li>`;
    return;
  }

  dom.luck.direction.textContent = data.direction === "forward" ? "顺行（阳年男命 / 阴年女命）" : "逆行（阳年女命 / 阴年男命）";

  data.cycles.forEach((c, i) => {
    const li = document.createElement("li");
    const isFirst = i === 0;
    li.className = `luck-item${isFirst ? " glow" : ""}`;
    li.innerHTML = `
      <span class="timeline-node" aria-hidden="true"></span>
      <article class="luck-card">
        <div class="luck-age"><strong>${c.start_age}-${c.end_age}岁</strong></div>
        <div class="luck-gz">${c.ganzhi}</div>
        <div class="luck-tags"><span>${c.label}</span></div>
      </article>
    `;
    dom.luck.container.append(li);
  });

  // Current luck card
  const current = data.cycles.length > 0 ? data.cycles[0] : null;
  if (current) {
    dom.luck.current.innerHTML = `
      <p>当前大运：<strong>${current.start_age}-${current.end_age}岁 · ${current.ganzhi}</strong></p>
      <p>起运年龄：${current.start_age}岁 <span>ⓘ</span></p>
    `;
  }
}
