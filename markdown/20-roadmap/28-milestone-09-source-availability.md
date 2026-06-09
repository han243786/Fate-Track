# M9 Source Availability Evidence

## 1. Scope

This evidence belongs to LOOP-021. It checks whether the selected ADR 0016 source stack is reachable enough to continue generated astronomy evidence planning.

It is not generated astronomy data and does not promote `astronomy-engine`.

## 2. Probe Command

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\probe-astronomy-sources.ps1 -ProjectRoot .
```

## 3. Latest Probe Result

Run time: 2026-06-08 15:26 local.

| Source | Required | Result | Evidence |
| --- | --- | --- | --- |
| GB/T 33661-2017 SAMR page | no | warning | PowerShell TLS request failed; keep as reference-page warning and manually review when needed. |
| NASA/JPL Horizons documentation | yes | ok | HTTP 200, 112553 bytes. |
| NASA/JPL Horizons API smoke | yes | ok | HTTP 200, 306 bytes. |
| IAU SOFA official site | yes | ok | HTTP 200, 461173 bytes. |
| NAIF SPICE toolkit page | yes | ok | HTTP 200, 15496 bytes. |

## 4. Interpretation

- Source availability is sufficient to continue M9 generated manifest planning.
- The GB/T standard remains selected as the calendar-rule reference, but automatic URL probing may need browser/manual review due to connection handling.
- Full project gates stay independent of network availability.
- `astronomy-engine` remains target.

## 5. Next Work

LOOP-022 should plan or create a real generated manifest instance only after choosing the concrete generation method and artifact shape. The manifest must not be accepted until hashes, comparison report, and golden cases exist.
