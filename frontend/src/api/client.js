export class ApiClient {
  constructor(apiBase) {
    this.apiBase = apiBase.replace(/\/$/, "");
  }

  health() {
    return this.#getJson("/api/health");
  }

  lunarDataMeta() {
    return this.#getJson("/api/lunar-data/meta");
  }

  capabilities() {
    return this.#getJson("/api/capabilities");
  }

  calendarDate(date) {
    return this.#getJson(`/api/calendar/query?date=${encodeURIComponent(date)}`);
  }

  chartCreate({ date, time, timezone, timePrecision }) {
    const params = new URLSearchParams({
      date,
      timezone,
      time_precision: timePrecision
    });
    if (timePrecision === "exact" && time) {
      params.set("time", time);
    }
    return this.#getJson(`/api/charts?${params.toString()}`);
  }

  analysisSnapshot({ date, time, timezone, timePrecision }) {
    const params = new URLSearchParams({
      date,
      timezone,
      time_precision: timePrecision
    });
    if (timePrecision === "exact" && time) {
      params.set("time", time);
    }
    return this.#getJson(`/api/analysis/snapshot?${params.toString()}`);
  }

  createCase({ id, title, tags, note, date, time, timezone, timePrecision }) {
    const params = new URLSearchParams({
      action: "create",
      id,
      title,
      date,
      timezone,
      time_precision: timePrecision
    });
    if (tags) params.set("tags", tags);
    if (note) params.set("note", note);
    if (timePrecision === "exact" && time) params.set("time", time);
    return this.#getJson(`/api/cases?${params.toString()}`);
  }

  listCases() {
    return this.#getJson("/api/cases?action=list");
  }

  createShare(caseId) {
    const params = new URLSearchParams({
      action: "create",
      case_id: caseId,
      ttl_seconds: "3600"
    });
    return this.#getJson(`/api/share/preview?${params.toString()}`);
  }

  async #getJson(path) {
    const response = await fetch(`${this.apiBase}${path}`);
    if (!response.ok) {
      throw new Error(`${response.status} ${response.statusText}`);
    }
    return response.json();
  }
}
