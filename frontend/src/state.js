export function createAppState({ apiBase, calendarDate = "2025-01-01" }) {
  return {
    apiBase,
    calendarDate,
    chartForm: {
      date: "2025-01-01",
      time: "10:30",
      timezone: "Asia/Shanghai",
      timePrecision: "exact",
      title: "工作台案例",
      tags: "工作台,复核",
      note: ""
    },
    chart: null,
    analysis: null,
    currentCase: null,
    cases: [],
    share: null,
    capabilities: []
  };
}
