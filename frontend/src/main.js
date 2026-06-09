import { ApiClient } from "./api/client.js";
import { defaultApiBase, loadApiBase, saveApiBase } from "./config.js";
import { createAppState } from "./state.js";
import { getDom } from "./ui/dom.js";
import {
  renderAnalysis,
  renderAnalysisError,
  renderCalendarError,
  renderCalendarResult,
  renderCapabilities,
  renderCases,
  renderChart,
  renderChartError,
  renderError,
  renderHealth,
  renderMeta,
  renderShare
} from "./ui/render.js";

const dom = getDom();
const state = createAppState({ apiBase: loadApiBase() ?? defaultApiBase });

hydrateForm();

dom.refreshButton.addEventListener("click", () => {
  state.apiBase = normalizeApiBase(dom.apiInput.value);
  saveApiBase(state.apiBase);
  refresh();
});

dom.apiInput.addEventListener("change", () => {
  state.apiBase = normalizeApiBase(dom.apiInput.value);
  saveApiBase(state.apiBase);
});

dom.calendar.queryButton.addEventListener("click", () => {
  state.calendarDate = dom.calendar.input.value.trim() || state.calendarDate;
  queryCalendarDate();
});

dom.chartForm.runButton.addEventListener("click", () => {
  readChartForm();
  runChartWorkspace();
});

dom.chartForm.saveButton.addEventListener("click", () => {
  readChartForm();
  saveCase();
});

dom.chartForm.shareButton.addEventListener("click", () => {
  createSharePreview();
});

async function refresh() {
  const client = new ApiClient(state.apiBase);
  renderHealth(dom, "checking", "检测中");

  try {
    const health = await client.health();
    renderHealth(dom, health.status === "ok" ? "ok" : "error", health.status ?? "未知");

    const [meta, capabilityPayload, casePayload] = await Promise.all([
      client.lunarDataMeta(),
      client.capabilities(),
      client.listCases()
    ]);

    state.capabilities = capabilityPayload.capabilities ?? [];
    state.cases = casePayload.cases ?? [];
    renderMeta(dom, meta);
    renderCapabilities(dom, state.capabilities);
    renderCases(dom, state.cases);
    renderShare(dom, state.share);
    await Promise.all([queryCalendarDate(client), runChartWorkspace(client)]);
  } catch (error) {
    renderHealth(dom, "error", "离线");
    renderError(dom, error);
  }
}

async function runChartWorkspace(client = new ApiClient(state.apiBase)) {
  try {
    const request = chartRequest();
    const [chart, analysis] = await Promise.all([
      client.chartCreate(request),
      client.analysisSnapshot(request)
    ]);
    state.chart = chart;
    state.analysis = analysis;
    renderChart(dom, chart);
    renderAnalysis(dom, analysis);
  } catch (error) {
    renderChartError(dom, error);
    renderAnalysisError(dom, error);
  }
}

async function saveCase(client = new ApiClient(state.apiBase)) {
  try {
    const payload = await client.createCase({
      ...chartRequest(),
      id: caseIdFromForm(),
      title: state.chartForm.title,
      tags: state.chartForm.tags,
      note: state.chartForm.note
    });
    state.currentCase = payload;
    const casePayload = await client.listCases();
    state.cases = casePayload.cases ?? [];
    renderCases(dom, state.cases);
  } catch (error) {
    renderChartError(dom, error);
  }
}

async function createSharePreview(client = new ApiClient(state.apiBase)) {
  try {
    if (!state.currentCase?.id) {
      await saveCase(client);
    }
    if (!state.currentCase?.id) {
      throw new Error("case is required before share preview");
    }
    state.share = await client.createShare(state.currentCase.id);
    renderShare(dom, state.share);
  } catch (error) {
    state.share = null;
    renderShare(dom, { token: error.message, public_dto: null });
  }
}

async function queryCalendarDate(client = new ApiClient(state.apiBase)) {
  state.calendarDate = dom.calendar.input.value.trim() || state.calendarDate;
  dom.calendar.input.value = state.calendarDate;
  try {
    const result = await client.calendarDate(state.calendarDate);
    renderCalendarResult(dom, result);
  } catch (error) {
    renderCalendarError(dom, error);
  }
}

function hydrateForm() {
  dom.apiInput.value = state.apiBase;
  dom.calendar.input.value = state.calendarDate;
  dom.chartForm.date.value = state.chartForm.date;
  dom.chartForm.time.value = state.chartForm.time;
  dom.chartForm.timezone.value = state.chartForm.timezone;
  dom.chartForm.timePrecision.value = state.chartForm.timePrecision;
  dom.chartForm.title.value = state.chartForm.title;
  dom.chartForm.tags.value = state.chartForm.tags;
  dom.chartForm.note.value = state.chartForm.note;
}

function readChartForm() {
  state.chartForm.date = dom.chartForm.date.value || state.chartForm.date;
  state.chartForm.time = dom.chartForm.time.value || state.chartForm.time;
  state.chartForm.timezone = dom.chartForm.timezone.value.trim() || state.chartForm.timezone;
  state.chartForm.timePrecision = dom.chartForm.timePrecision.value;
  state.chartForm.title = dom.chartForm.title.value.trim() || state.chartForm.title;
  state.chartForm.tags = dom.chartForm.tags.value.trim();
  state.chartForm.note = dom.chartForm.note.value.trim();
}

function chartRequest() {
  return {
    date: state.chartForm.date,
    time: state.chartForm.time,
    timezone: state.chartForm.timezone,
    timePrecision: state.chartForm.timePrecision
  };
}

function caseIdFromForm() {
  return `case-${state.chartForm.date}-${Date.now()}`;
}

function normalizeApiBase(value) {
  return (value.trim() || defaultApiBase).replace(/\/$/, "");
}

refresh();
