import { afterEach, describe, it } from "node:test";
import assert from "node:assert/strict";
import { ApiClient } from "../src/api/client.js";

const originalFetch = globalThis.fetch;

afterEach(() => {
  globalThis.fetch = originalFetch;
});

describe("ApiClient.calendarDate", () => {
  it("queries the supported calendar date route", async () => {
    let requestedUrl = "";
    globalThis.fetch = async (url) => {
      requestedUrl = url;
      return {
        ok: true,
        json: async () => ({ meta: { ruleset_id: "ft-date-layer-android-v1" } })
      };
    };

    const payload = await new ApiClient("http://127.0.0.1:8787/").calendarDate("2025-01-01");

    assert.equal(
      requestedUrl,
      "http://127.0.0.1:8787/api/calendar/query?date=2025-01-01"
    );
    assert.equal(payload.meta.ruleset_id, "ft-date-layer-android-v1");
  });

  it("surfaces non-ok responses as errors", async () => {
    globalThis.fetch = async () => ({
      ok: false,
      status: 404,
      statusText: "Not Found"
    });

    await assert.rejects(
      () => new ApiClient("http://127.0.0.1:8787").calendarDate("2101-01-01"),
      /404 Not Found/
    );
  });
});

describe("ApiClient workspace routes", () => {
  it("queries chart and analysis with encoded chart input", async () => {
    const urls = [];
    globalThis.fetch = async (url) => {
      urls.push(url);
      return {
        ok: true,
        json: async () => ({ ok: true })
      };
    };

    const client = new ApiClient("http://127.0.0.1:8787");
    const request = {
      date: "2025-01-01",
      time: "10:30",
      timezone: "Asia/Shanghai",
      timePrecision: "exact"
    };
    await client.chartCreate(request);
    await client.analysisSnapshot(request);

    assert.equal(
      urls[0],
      "http://127.0.0.1:8787/api/charts?date=2025-01-01&timezone=Asia%2FShanghai&time_precision=exact&time=10%3A30"
    );
    assert.equal(
      urls[1],
      "http://127.0.0.1:8787/api/analysis/snapshot?date=2025-01-01&timezone=Asia%2FShanghai&time_precision=exact&time=10%3A30"
    );
  });

  it("creates cases and share previews through restricted routes", async () => {
    const urls = [];
    globalThis.fetch = async (url) => {
      urls.push(url);
      return {
        ok: true,
        json: async () => ({ ok: true })
      };
    };

    const client = new ApiClient("http://127.0.0.1:8787/");
    await client.createCase({
      id: "case-a",
      title: "Case A",
      tags: "alpha,beta",
      note: "private note",
      date: "2025-01-01",
      time: "10:30",
      timezone: "Asia/Shanghai",
      timePrecision: "exact"
    });
    await client.listCases();
    await client.createShare("case-a");

    assert.match(urls[0], /\/api\/cases\?action=create/);
    assert.match(urls[0], /title=Case\+A/);
    assert.match(urls[0], /note=private\+note/);
    assert.equal(urls[1], "http://127.0.0.1:8787/api/cases?action=list");
    assert.equal(
      urls[2],
      "http://127.0.0.1:8787/api/share/preview?action=create&case_id=case-a&ttl_seconds=3600"
    );
  });
});
