const defaultApiBase = "http://127.0.0.1:8787";
const apiInput = document.querySelector("#api-base");
const refreshButton = document.querySelector("#refresh-button");
const healthPill = document.querySelector("#health-pill");
const healthLabel = document.querySelector("#health-label");

const fields = {
  yearCount: document.querySelector("#year-count"),
  termCount: document.querySelector("#term-count"),
  version: document.querySelector("#data-version"),
  bytes: document.querySelector("#data-bytes"),
  range: document.querySelector("#data-range"),
  sourcePath: document.querySelector("#source-path")
};

apiInput.value = localStorage.getItem("minggui.apiBase") ?? defaultApiBase;

refreshButton.addEventListener("click", () => {
  localStorage.setItem("minggui.apiBase", apiInput.value.trim() || defaultApiBase);
  refresh();
});

apiInput.addEventListener("change", () => {
  localStorage.setItem("minggui.apiBase", apiInput.value.trim() || defaultApiBase);
});

async function refresh() {
  const apiBase = (apiInput.value.trim() || defaultApiBase).replace(/\/$/, "");
  setHealth("checking", "连接中");

  try {
    const health = await getJson(`${apiBase}/api/health`);
    setHealth(health.status === "ok" ? "ok" : "error", health.status ?? "未知");

    const meta = await getJson(`${apiBase}/api/lunar-data/meta`);
    renderMeta(meta);
  } catch (error) {
    setHealth("error", "连接失败");
    renderError(error);
  }
}

async function getJson(url) {
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`${response.status} ${response.statusText}`);
  }
  return response.json();
}

function setHealth(state, label) {
  healthPill.dataset.state = state;
  healthLabel.textContent = label;
}

function renderMeta(meta) {
  fields.yearCount.textContent = String(meta.year_count ?? "--");
  fields.termCount.textContent = String(meta.term_count ?? "--");
  fields.version.textContent = meta.version ?? "--";
  fields.bytes.textContent = formatBytes(meta.bytes);
  fields.range.textContent =
    meta.min_year && meta.max_year ? `${meta.min_year}-${meta.max_year}` : "范围未知";
  fields.sourcePath.textContent = meta.path ?? "--";
}

function renderError(error) {
  fields.yearCount.textContent = "--";
  fields.termCount.textContent = "--";
  fields.version.textContent = "--";
  fields.bytes.textContent = "--";
  fields.range.textContent = "等待数据";
  fields.sourcePath.textContent = error.message;
}

function formatBytes(value) {
  if (!Number.isFinite(value)) {
    return "--";
  }
  if (value < 1024) {
    return `${value} B`;
  }
  return `${(value / 1024).toFixed(1)} KB`;
}

refresh();

