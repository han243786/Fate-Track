import { ApiClient } from "./api/client.js";
import { defaultApiBase, loadApiBase } from "./config.js";
import { createAppState } from "./state.js";
import { getDom } from "./ui/dom.js";
import {
  renderAnalysis,
  renderAnalysisError,
  renderChart,
  renderChartError,
  renderLuckCycles,
  renderTopicReport,
  renderTopicReportError,
  renderTopicReportIdle,
  renderTopicReportLoading
} from "./ui/render.js";

const STORAGE_KEY = "ft-chart-form";
const THEME_STORAGE_KEY = "ft-wuxing-theme";
const TOPIC_LABELS = {
  relationship: "情感",
  wealth: "金钱",
  family: "家庭",
  career: "事业"
};
const WUXING_THEMES = [
  { id: "metal", mark: "金", label: "金" },
  { id: "wood", mark: "木", label: "木" },
  { id: "water", mark: "水", label: "水" },
  { id: "fire", mark: "火", label: "火" },
  { id: "earth", mark: "土", label: "土" }
];

const dom = getDom();
const state = createAppState({ apiBase: loadApiBase() ?? defaultApiBase });
let topicRequestVersion = 0;
let chartWorkspaceRuns = 0;

restoreForm();
restoreTheme();
hydrateForm();
bindSexButtons();
bindTopicButtons();
bindThemeButton();
bindBoundaryPanel();
syncSelectedTopic();
syncWuxingTheme();

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
  const year = String(currentReadingYear());
  params.set("reading_year", year);
  params.set("year", year);
  window.location.href = `/report.html?${params.toString()}`;
});

async function runChartWorkspace(client = new ApiClient(state.apiBase)) {
  topicRequestVersion += 1;
  chartWorkspaceRuns += 1;
  renderTopicReportIdle(
    dom,
    chartWorkspaceRuns > 1 ? "排盘资料已更新，请重新选择专项推演。" : "专项报告待生成"
  );
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
      const [luck, report] = await Promise.all([
        client.luckCycles(request),
        client.chartReport({
          ...request,
          readingYear: currentReadingYear(),
          year: currentReadingYear()
        })
      ]);
      renderLuckCycles(dom, luck, report.luck_reading, report.annual_trigger_reading);
    } catch (e) {
      try {
        const luck = await client.luckCycles(request);
        renderLuckCycles(dom, luck);
      } catch {
        dom.luck.container.innerHTML = `<li class="empty-state">大运: ${e.message}</li>`;
      }
    }
  } catch (error) {
    renderChartError(dom, error);
    renderAnalysisError(dom, error);
  }
}

async function runTopicReport(topic, client = new ApiClient(state.apiBase)) {
  setSelectedTopic(topic);
  const request = chartRequest();
  const requestVersion = topicRequestVersion + 1;
  topicRequestVersion = requestVersion;
  const requestKey = chartRequestKey(request);
  renderTopicReportLoading(dom, topic);
  try {
    const report = await client.topicReport({
      ...request,
      topic,
      year: new Date().getFullYear()
    });
    if (requestVersion !== topicRequestVersion || requestKey !== chartRequestKey(chartRequest())) {
      return;
    }
    renderTopicReport(dom, report);
  } catch (error) {
    if (requestVersion !== topicRequestVersion) return;
    renderTopicReportError(dom, error);
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

function bindTopicButtons() {
  dom.topics.buttons.forEach(button => {
    if (button.disabled) return;
    button.addEventListener("click", () => {
      readChartForm();
      persistForm();
      runTopicReport(button.dataset.topic || "relationship");
    });
  });

  dom.topics.fullReportButton.addEventListener("click", () => {
    readChartForm();
    persistForm();
    window.location.href = `/topic-report.html?${topicReportParams(state.selectedTopic).toString()}`;
  });
}

function bindThemeButton() {
  dom.theme.wuxingButton.addEventListener("click", () => {
    const index = WUXING_THEMES.findIndex(theme => theme.id === state.wuxingTheme);
    const next = WUXING_THEMES[(index + 1) % WUXING_THEMES.length] || WUXING_THEMES[0];
    state.wuxingTheme = next.id;
    persistTheme();
    syncWuxingTheme();
  });
}

function bindBoundaryPanel() {
  dom.boundary.button.addEventListener("click", () => {
    const willOpen = dom.boundary.panel.hasAttribute("hidden");
    dom.boundary.panel.toggleAttribute("hidden", !willOpen);
    dom.boundary.button.setAttribute("aria-expanded", String(willOpen));
    dom.boundary.button.textContent = willOpen ? "收起边界" : "查看边界";
  });
}

function showLunar(lunar) {
  dom.lunarDisplay.textContent = lunar
    ? `${lunar.lunar?.year || ""}${lunar.lunar?.month_name || ""}${lunar.lunar?.day_name || ""}`
    : "--";
}

function currentReadingYear() {
  return new Date().getFullYear();
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

function restoreTheme() {
  try {
    const saved = localStorage.getItem(THEME_STORAGE_KEY);
    if (WUXING_THEMES.some(theme => theme.id === saved)) {
      state.wuxingTheme = saved;
    }
  } catch { /* storage unavailable */ }
}

function persistTheme() {
  try {
    localStorage.setItem(THEME_STORAGE_KEY, state.wuxingTheme);
  } catch { /* storage unavailable */ }
}

function hydrateForm() {
  dom.chartForm.date.value = state.chartForm.date;
  dom.chartForm.time.value = state.chartForm.time;
  dom.chartForm.sex.value = state.chartForm.sex;
  dom.chartForm.sexButtons.forEach(btn => {
    btn.classList.toggle("active", btn.dataset.sex === state.chartForm.sex);
  });
}

function setSelectedTopic(topic) {
  state.selectedTopic = TOPIC_LABELS[topic] ? topic : "relationship";
  syncSelectedTopic();
}

function syncSelectedTopic() {
  const label = TOPIC_LABELS[state.selectedTopic] || TOPIC_LABELS.relationship;
  dom.topics.buttons.forEach(button => {
    button.classList.toggle("is-selected", button.dataset.topic === state.selectedTopic);
  });
  dom.topics.selectedLabel.textContent = `当前：${label}`;
  dom.topics.fullReportButton.setAttribute("aria-label", `查看${label}专项报告`);
}

function syncWuxingTheme() {
  const theme = WUXING_THEMES.find(item => item.id === state.wuxingTheme) || WUXING_THEMES[0];
  document.documentElement.dataset.wuxingTheme = theme.id;
  dom.theme.wuxingMark.textContent = theme.mark;
  dom.theme.wuxingLabel.textContent = `当前：${theme.label}`;
  dom.theme.wuxingButton.dataset.theme = theme.id;
  dom.theme.wuxingButton.setAttribute("aria-label", `切换五行颜色风格，当前为${theme.label}`);
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

function chartRequestKey(request) {
  return [
    request.date,
    request.time || "",
    request.timezone,
    request.timePrecision,
    request.sex || "unspecified"
  ].join("|");
}

function topicReportParams(topic) {
  const params = new URLSearchParams();
  params.set("topic", topic);
  params.set("date", state.chartForm.date);
  if (state.chartForm.time) params.set("time", state.chartForm.time);
  params.set("timezone", "Asia/Shanghai");
  params.set("time_precision", state.chartForm.time ? "exact" : "unknown");
  params.set("sex", state.chartForm.sex || "unspecified");
  params.set("year", String(new Date().getFullYear()));
  return params;
}

runChartWorkspace();
