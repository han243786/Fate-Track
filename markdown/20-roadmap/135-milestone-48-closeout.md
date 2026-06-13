# M48 Closeout - Topic Count-Field Narrative Baseline

## 1. Scope

M48 closes LOOP-116 as a wealth/family/career report quality-only loop. It starts from regenerated real report samples and extends the M47 relationship golden-sample rule to the remaining topic reports: visible text must explain ten-god signals as topic qualities, not expose count-field output.

## 2. Implementation

- Rewrote the shared topic `ten_god_group_summary` output so active ten-god signals render as reader-facing qualities such as `偏财带出机会资源、外部流动和交换意识` instead of `偏财有一处落点`.
- Removed remaining public count-field phrasing from wealth, family, and career blocks, including `不作主线`, `有一处落点`, `有两处落点`, and `有三处落点`.
- Replaced old template bridges such as `落到这张盘上` with more specific report bridges, including `放回这张命盘看` and `放回家庭结构里`.
- Removed the relationship block's residual `这组结构说明` bridge so the M47 golden sample remains clean.
- Extended app/domain public-body guards to reject old count fields and middle-layer phrases such as `参与这组结构` and `这组结构说明`.

## 3. Capability Boundary

No capability status changed.

- no new route
- no DTO top-level shape change
- no `/api/capabilities` change
- no supported promotion
- no public `score_internal` or 0-100 score
- no mutation of raw `GET /api/luck/cycles`
- no flow-month, flow-day, event schedule, daily fortune, financial outcome, family event, career result, partner identity, or deterministic real-world claim

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

`relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt` M48 scan returned 0 hits for `不作主线`, `有一处落点`, `有两处落点`, `有三处落点`, `落到这张盘上`, `参与这组结构`, `这组结构说明`, `这条信号`, generic selected-year wording, `白话`, and `score_internal`.

The visible wealth/family/career text now renders topic qualities such as:

- `偏财带出机会资源、外部流动和交换意识`
- `伤官带出想法出口、技术表达和解决问题的锋芒`
- `七杀的分量更重，压力、挑战、边界考验和行动驱动会更容易被看见`

## 5. Validation

Targeted gates passed:

- `cargo test topic_report -- --nocapture`
- `cargo test report -- --nocapture`
- `cargo test relationship -- --nocapture`
- `powershell -ExecutionPolicy Bypass -File tools/check-governance-scaffold.ps1 -ProjectRoot .`

## 6. Next Cursor

Next loop: LOOP-117. Future report polish should keep M47 as the relationship style baseline and M48 as the remaining-topic count-field baseline. Further work can now move to deeper yearly-reading narrative quality, but must start from regenerated real samples and must not expand capability status without a new milestone and decision gate.
