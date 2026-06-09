# ADR 0015: M9 Astronomy Parallel Strategy

## Status

Accepted for M9 preflight.

## Decision

M9 will introduce the astronomy upgrade as a versioned parallel track first. It will not silently replace the accepted Android date-layer baseline.

Chosen strategy:

```text
android-date-layer-v1 remains accepted-current for V1.
astronomy-engine-v0 starts as target/restricted until generated tables, manifests, hashes, and comparison reports pass.
replacement requires a later ADR after dual-engine evidence exists.
```

This closes DG-008 for M9 preflight by choosing when and how the astronomy engine may enter the project: parallel first, replacement later only with evidence.

## Evidence Basis

| Source | M9 Use |
| --- | --- |
| V1 research intake | Records long-term source of truth as astronomy-derived solar terms, new moons, and golden tables. |
| Chinese V1 research report | Recommends 1901-2100 as first official verification range and separates algorithmic reach from official validation. |
| Design report | Recommends modern astronomical Chinese calendar rules, UTC+8 baseline, solar longitude terms, SOFA/JPL validation tier, and 2033 anomaly regressions. |
| ADR 0008 | Keeps Android 1901-2100 as current official validated range until a generated-table replacement is proven. |

## Required M9 Artifacts Before Capability Promotion

- Generated data manifest with engine id, version, source references, range, command, and hashes.
- Android-vs-astronomy comparison report.
- Golden cases for 1901-2100 boundaries, 2033 anomaly, Lichun/Qingming, Jiazi anchors, and near-midnight cases.
- Explicit difference taxonomy: Android table difference, astronomy source difference, ruleset difference, timezone/history difference, or unresolved.
- Replay policy that preserves old `algo_version` behavior for existing chart snapshots.

## Rejected Options

| Option | Reason |
| --- | --- |
| Silent replacement of Android date layer | Violates ADR 0008 and would break reproducibility. |
| Claiming wider range before generated evidence | Violates capability promotion rules and release candidate boundary. |
| Implementing true solar time inside M9 preflight | Requires longitude/time-equation policy and additional validation beyond DG-008 closure. |
| Treating research citations as runtime evidence | Research is target guidance; supported capability still needs code, data, tests, and hashes. |

## Impacted Modules

- `backend.calendar.astronomy`
- `data.generated.astronomy`
- `tools.governance`
- `governance.release`
- `governance.capability-ledger`

## Required Tests

- Manifest schema validation.
- Golden table hash validation.
- Android-vs-astronomy diff classification validation.
- Regression that V1 Android API status does not change unless a replacement ADR is accepted.

## Rollback Rule

If generated astronomy data cannot be reproduced or diff classification is incomplete, keep `astronomy-engine` as target/restricted and retain Android date-layer behavior unchanged.
