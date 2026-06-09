export function getDom() {
  return {
    apiInput: required("#api-base"),
    refreshButton: required("#refresh-button"),
    healthPill: required("#health-pill"),
    healthLabel: required("#health-label"),
    fields: {
      yearCount: required("#year-count"),
      termCount: required("#term-count"),
      version: required("#data-version"),
      bytes: required("#data-bytes"),
      range: required("#data-range"),
      sourcePath: required("#source-path")
    },
    chartForm: {
      date: required("#chart-date"),
      time: required("#chart-time"),
      timezone: required("#chart-timezone"),
      timePrecision: required("#chart-time-precision"),
      title: required("#case-title"),
      tags: required("#case-tags"),
      note: required("#case-note"),
      runButton: required("#chart-run-button"),
      saveButton: required("#case-save-button"),
      shareButton: required("#share-create-button")
    },
    chart: {
      status: required("#chart-status"),
      pillars: required("#pillar-grid"),
      warnings: required("#chart-warnings")
    },
    analysis: {
      metrics: required("#analysis-metrics"),
      cards: required("#analysis-cards")
    },
    cases: {
      list: required("#case-list")
    },
    share: {
      preview: required("#share-preview")
    },
    calendar: {
      input: required("#calendar-date"),
      queryButton: required("#calendar-query-button"),
      status: required("#calendar-status"),
      gregorian: required("#calendar-gregorian"),
      lunar: required("#calendar-lunar"),
      ganzhi: required("#calendar-ganzhi"),
      ruleset: required("#calendar-ruleset")
    },
    capabilityList: required("#capability-list")
  };
}

function required(selector) {
  const element = document.querySelector(selector);
  if (!element) {
    throw new Error(`Missing required UI node: ${selector}`);
  }
  return element;
}
