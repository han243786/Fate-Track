import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

const html = readFileSync(new URL("../index.html", import.meta.url), "utf8");

describe("workspace markup", () => {
  it("contains the M7 workspace regions and labeled controls", () => {
    for (const id of [
      "control-title",
      "chart-form-title",
      "chart-title",
      "analysis-title",
      "case-title-heading",
      "share-title",
      "calendar-title",
      "data-title",
      "capability-title"
    ]) {
      assert.match(html, new RegExp(`aria-labelledby="${id}"`));
    }

    for (const id of [
      "chart-date",
      "chart-time",
      "chart-timezone",
      "chart-time-precision",
      "case-title",
      "case-tags",
      "case-note"
    ]) {
      assert.match(html, new RegExp(`id="${id}"`));
    }
  });

  it("keeps restricted and planned surfaces out of supported copy", () => {
    assert.match(html, /分享预览/);
    assert.match(html, /surface-badge restricted/);
    assert.doesNotMatch(html, /Luck Cycles<\/h2>/);
    assert.doesNotMatch(html, /Cloud Sync/);
    assert.doesNotMatch(html, /True Solar Time/);
    assert.doesNotMatch(html, /Astronomy Engine/);
  });
});
