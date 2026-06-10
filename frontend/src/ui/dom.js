export function getDom() {
  return {
    chartForm: {
      date: required("#chart-date"),
      time: required("#chart-time"),
      sex: required("#chart-sex"),
      sexButtons: Array.from(document.querySelectorAll(".gender-option")),
      runButton: required("#chart-run-button"),
      reportButton: required("#chart-report-button")
    },
    lunarDisplay: required("#lunar-display"),
    chart: {
      pillars: required("#pillar-grid"),
      summary: required("#chart-summary")
    },
    analysis: {
      metrics: required("#analysis-metrics"),
      cards: required("#analysis-cards"),
      elementBars: required("#elementBars"),
      godChips: required("#godChips"),
      godTotal: required("#godTotal"),
      hiddenStemTable: required("#hiddenStemTable")
    },
    luck: {
      container: required("#luck-cycles"),
      direction: required("#luck-direction"),
      current: required("#currentLuck")
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
