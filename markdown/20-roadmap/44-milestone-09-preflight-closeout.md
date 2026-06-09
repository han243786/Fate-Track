# M9 Preflight Closeout

## 1. Scope

This closeout belongs to LOOP-037. It closes M9 only as a preflight milestone.

It does not close the full astronomy-engine implementation. It does not accept generated astronomy artifacts, does not replace the Android baseline, does not change runtime date or chart behavior, and does not promote `astronomy-engine` beyond `target`.

## 2. Decision

| Field | Value |
| --- | --- |
| decision | close M9 as preflight only |
| machine record | `data/generated/astronomy/preflight-closeout-decision.json` |
| M9 preflight closed | yes |
| full M9 astronomy-engine closeout | no |
| `astronomy-engine` status | target |
| next milestone | M10 Generated Astronomy Implementation |

## 3. Evidence Accepted For Preflight Closeout

| Evidence | Status |
| --- | --- |
| ADR 0015 parallel-first strategy | accepted for preflight |
| ADR 0016 source stack | accepted for preflight |
| ADR 0017 generated-data planning path | accepted for preflight |
| source policy | accepted for preflight |
| manifest schema | accepted for preflight |
| not-accepted draft manifest | accepted as planning evidence only |
| generation plan | accepted as planning evidence only |
| generator contract | accepted as contract evidence only |
| source adapter contract | accepted as contract evidence only |
| artifact writer dry-run | accepted as dry-run evidence only |
| comparison runner dry-run | accepted as dry-run evidence only |
| golden-row readiness | accepted as readiness evidence only |
| replay-test readiness | accepted as readiness evidence only |
| pre-closeout audit | accepted as evidence that full closeout remains blocked |

## 4. Full Closeout Blockers

M9 full astronomy-engine closeout remains blocked by the following missing evidence:

1. Real generated astronomy artifacts.
2. `sha256` hashes for every generated artifact.
3. Completed Android-vs-astronomy comparison report.
4. Generated golden rows for 1901-2100 boundaries, 2033 anomaly, Lichun, Qingming, Jiazi day anchor, and near-midnight event cases.
5. Executed replay tests proving old `android-date-layer-v1` snapshots remain reproducible.
6. Runtime astronomy integration.
7. A later replacement ADR before any default behavior changes.

## 5. Capability Result

| Capability | Before | After |
| --- | --- | --- |
| `astronomy-engine` | target | target |
| `calendar-date-query` | supported through Android date layer | unchanged |
| `chart-create` | supported for current V1 chart core | unchanged |

## 6. Next Work

The next milestone is M10 Generated Astronomy Implementation. It may begin with generator implementation and local source materialization only after reading this closeout, `preflight-closeout-decision.json`, the M9 contracts, and the active recursive cursor.

M10 must still keep `astronomy-engine` as `target` until generated artifacts, hashes, comparison report, golden rows, replay tests, and replacement policy are complete.

## 7. Validation

The full project gate must include `tools/check-astronomy-preflight.ps1`, and that checker must verify this closeout cannot be mistaken for generated-data acceptance.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1
```
