export function createAppState({ apiBase }) {
  return {
    apiBase,
    chartForm: {
      date: "2025-01-01",
      time: "10:30",
      sex: "unspecified"
    },
    selectedTopic: "relationship",
    wuxingTheme: "water"
  };
}
