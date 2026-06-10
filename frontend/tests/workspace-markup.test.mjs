import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

const html = readFileSync(new URL("../index.html", import.meta.url), "utf8");

describe("workspace markup", () => {
  it("contains the product panels", () => {
    for (const id of [
      "chart-form-title",
      "chart-title",
      "luck-title",
      "analysis-title"
    ]) {
      assert.match(html, new RegExp(`aria-labelledby="${id}"`));
    }

    for (const id of [
      "chart-date",
      "chart-time",
      "chart-sex",
      "chart-run-button",
      "lunar-display"
    ]) {
      assert.match(html, new RegExp(`id="${id}"`));
    }
  });

  it("keeps unsupported features out of the product", () => {
    assert.doesNotMatch(html, /Cloud Sync/);
    assert.doesNotMatch(html, /True Solar Time/);
    assert.doesNotMatch(html, /Astronomy Engine/);
  });
});
