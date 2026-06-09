import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { formatBytes, formatRange } from "../src/utils/format.js";

describe("formatBytes", () => {
  it("formats byte values", () => {
    assert.equal(formatBytes(42), "42 B");
    assert.equal(formatBytes(42413), "41.4 KB");
  });

  it("rejects invalid values", () => {
    assert.equal(formatBytes(Number.NaN), "--");
  });
});

describe("formatRange", () => {
  it("formats complete year ranges", () => {
    assert.equal(formatRange(1901, 2100), "1901-2100");
  });

  it("uses fallback for incomplete ranges", () => {
    assert.equal(formatRange(null, 2100), "未知");
  });
});
