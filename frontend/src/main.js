import { ApiClient } from "./api/client.js";
import { defaultApiBase, loadApiBase } from "./config.js";
import { createAppState } from "./state.js";
import { getDom } from "./ui/dom.js";
import {
  renderAnalysis,
  renderAnalysisError,
  renderChart,
  renderChartError,
  renderLuckCycles
} from "./ui/render.js";

const STORAGE_KEY = "ft-chart-form";

const dom = getDom();
const state = createAppState({ apiBase: loadApiBase() ?? defaultApiBase });

restoreForm();
hydrateForm();
bindSexButtons();

dom.chartForm.runButton.addEventListener("click", () => {
  readChartForm();
  persistForm();
  runChartWorkspace();
});

dom.chartForm.reportButton.addEventListener("click", () => {
  readChartForm();
  persistForm();
  const params = new URLSearchParams();
  params.set("date", state.chartForm.date);
  if (state.chartForm.time) params.set("time", state.chartForm.time);
  params.set("timezone", "Asia/Shanghai");
  params.set("time_precision", state.chartForm.time ? "exact" : "unknown");
  if (state.chartForm.sex) params.set("sex", state.chartForm.sex);
  window.location.href = `/report.html?${params.toString()}`;
});

async function runChartWorkspace(client = new ApiClient(state.apiBase)) {
  try {
    const request = chartRequest();
    const [chart, analysis, lunar] = await Promise.all([
      client.chartCreate(request),
      client.analysisSnapshot(request),
      client.calendarDate(request.date)
    ]);
    renderChart(dom, chart);
    renderAnalysis(dom, analysis, chart);
    showLunar(lunar);
    try {
      const luck = await client.luckCycles(chartRequest());
      renderLuckCycles(dom, luck);
    } catch (e) {
      dom.luck.container.innerHTML = `<li class="empty-state">大运: ${e.message}</li>`;
    }
  } catch (error) {
    renderChartError(dom, error);
    renderAnalysisError(dom, error);
  }
}

function bindSexButtons() {
  dom.chartForm.sexButtons.forEach(btn => {
    btn.addEventListener("click", () => {
      dom.chartForm.sexButtons.forEach(b => b.classList.remove("active"));
      btn.classList.add("active");
      dom.chartForm.sex.value = btn.dataset.sex || "unspecified";
    });
  });
}

function showLunar(lunar) {
  dom.lunarDisplay.textContent = lunar
    ? `${lunar.lunar?.year || ""}${lunar.lunar?.month_name || ""}${lunar.lunar?.day_name || ""}`
    : "--";
}

function persistForm() {
  try {
    sessionStorage.setItem(STORAGE_KEY, JSON.stringify(state.chartForm));
  } catch { /* storage unavailable */ }
}

function restoreForm() {
  try {
    const raw = sessionStorage.getItem(STORAGE_KEY);
    if (raw) {
      const saved = JSON.parse(raw);
      if (saved.date) state.chartForm.date = saved.date;
      if (saved.time !== undefined) state.chartForm.time = saved.time;
      if (saved.sex) state.chartForm.sex = saved.sex;
    }
  } catch { /* ignore corrupt data */ }
}

function hydrateForm() {
  dom.chartForm.date.value = state.chartForm.date;
  dom.chartForm.time.value = state.chartForm.time;
  dom.chartForm.sex.value = state.chartForm.sex;
  dom.chartForm.sexButtons.forEach(btn => {
    btn.classList.toggle("active", btn.dataset.sex === state.chartForm.sex);
  });
}

function readChartForm() {
  state.chartForm.date = dom.chartForm.date.value || state.chartForm.date;
  state.chartForm.time = dom.chartForm.time.value || state.chartForm.time;
  state.chartForm.sex = dom.chartForm.sex.value;
}

function chartRequest() {
  return {
    date: state.chartForm.date,
    time: state.chartForm.time,
    timezone: "Asia/Shanghai",
    timePrecision: state.chartForm.time ? "exact" : "unknown",
    sex: state.chartForm.sex
  };
}

runChartWorkspace();
