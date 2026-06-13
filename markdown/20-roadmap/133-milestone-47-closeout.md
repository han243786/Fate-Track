# M47 Closeout - Relationship Golden Sample Baseline

## 1. Scope

M47 closes LOOP-115 as a relationship-report quality-only loop. It starts from the regenerated real `relationship.txt` sample and turns the relationship report into the current golden sample for later report polishing.

## 2. Implementation

- Replaced the relationship report's generic star-count summaries with relationship-specific summaries for spouse-star, expression/boundary, and support/safety signals.
- Removed user-visible count-table phrasing from the relationship body, including `不作主线` and `有一处落点`.
- Kept the M41 six-block relationship structure and M44 quoted relation-term rule.
- Preserved internal trace and qualitative levels while translating public prose into relationship qualities: attraction, commitment, pressure, boundary, safety, stable response, and real-world support.
- Removed now-unused generic relationship count-summary helpers so the old `落点` phrasing cannot re-enter the relationship path through dead code.
- Extended the relationship domain test into a golden-sample guard that requires key reader-facing terms and rejects M47 regression phrases.

## 3. Capability Boundary

No capability status changed.

- no new route
- no DTO top-level shape change
- no `/api/capabilities` change
- no supported promotion
- no public `score_internal` or 0-100 score
- no mutation of raw `GET /api/luck/cycles`
- no flow-month, flow-day, event schedule, daily fortune, partner identity, deterministic marriage/separation claim, or high-risk relationship decision

Post-preview runtime remains 10 supported, 14 restricted, 0 planned.

## 4. Real Output Evidence

Regenerated samples under `target/report-polish-samples/`:

| Sample | Audit | Blocks | Warnings |
| --- | --- | ---: | ---: |
| `main.json` | passed | 10 | 0 |
| `relationship.json` | passed | 6 | 1 |
| `wealth.json` | passed | 8 | 0 |
| `family.json` | passed | 8 | 0 |
| `career.json` | passed | 8 | 0 |

`relationship.txt` M47 scan returned 0 hits for `不作主线`, `有一处落点`, `有两处落点`, `有三处落点`, the old fixed opener, bare `被冲牵动`, and `score_internal`.

The five-report M45/M46 public-copy scan returned 0 hits for system-tone, score, list/debug wording, flow-month/day overclaim, and internal public-score terms.

## 5. Validation

Targeted gates passed:

- `cargo test relationship -- --nocapture`
- `cargo test topic_report -- --nocapture`
- `cargo test report -- --nocapture`
- `powershell -ExecutionPolicy Bypass -File tools/check-governance-scaffold.ps1 -ProjectRoot .`

## 6. Next Cursor

Next loop: LOOP-116. Future report polish should use M47 relationship output as the current style baseline: direct reader address, professional terms with natural explanation, no count-table leakage, no internal variables, no deterministic claims, and no capability promotion without a new milestone and decision gate.
