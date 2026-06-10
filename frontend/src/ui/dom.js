export function getDom() {
  return {
    chartForm: {
      date: required("#chart-date"),
      time: required("#chart-time"),
      sex: required("#chart-sex"),
      runButton: required("#chart-run-button"),
      reportButton: required("#chart-report-button")
    },
    lunarDisplay: required("#lunar-display"),
    chart: {
      pillars: required("#pillar-grid")
    },
    analysis: {
      metrics: required("#analysis-metrics"),
      cards: required("#analysis-cards")
    },
    luck: {
      container: required("#luck-cycles")
    }
  };
}

function required(selector) {
  const element = document.querySelector(selector);
  if (!element) {
    throw new Error(`Missing required UI node: ${selector}`);
  }
  return element;
}
