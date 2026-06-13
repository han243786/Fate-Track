# M58 Closeout - Main Report Long-Section Condensation

## 1. Scope

M58 closes LOOP-126 as a quality-only main-report condensation loop. It starts from regenerated real samples after M57, keeps all M41-M57 report-copy gates active, and adds the M58 main long-section condensation gate.

## 2. Implementation

- Main chart report:
  - Condensed `十神关系` by removing one-paragraph-per-ten-god mini glossary output and replacing it with grouped signal reading.
  - Condensed `大运走势` by summarizing public plain readings instead of rendering professional + plain pairs.
  - Condensed `年度引动` by keeping the annual evidence story but summarizing public plain readings.
  - Replaced generic `这条十神线索` wording with concrete signal names such as `七杀信号`.
- Public body gates:
  - Added M58 guards for `读这一章时`, `这条线已经进入命盘视野`, `这条十神线索`, and professional-label rewrites such as `命理结构上，当前阶段大运`.
  - Preserved M57 staged-prose, M56 conclusion, M55 current-luck, M51 main-report tone, and M43/M45/M46 public-copy gates.

## 3. Capability Boundary

No capability status changed.

- no new route
- no DTO top-level shape change
- no `/api/capabilities` change
- no supported promotion
- no public `score_internal` or 0-100 score
- no mutation of raw `GET /api/luck/cycles`
- no flow-month, flow-day, event schedule, daily fortune, financial outcome, family event, career result, partner identity, or deterministic real-world claim
- restricted trace/evidence remains available; only assembled visible report prose was condensed

Post-preview runtime remains 10 supported, 14 restricted, 0 planned.

## 4. Real Output Evidence

Regenerated samples under `target/report-polish-samples/` with one consistent sample birth profile:

| Sample | Audit | Blocks | Last Block |
| --- | --- | ---: | --- |
| `main.json` | passed | 10 | `年度引动` |
| `relationship.json` | passed | 6 | `结论` |
| `wealth.json` | passed | 8 | `结论` |
| `family.json` | passed | 8 | `结论` |
| `career.json` | passed | 8 | `结论` |

Main visible section lengths after M58:

| Section | M58 length |
| --- | ---: |
| `十神关系` | 416 |
| `大运走势` | 633 |
| `年度引动` | 809 |

M57 baseline for the same sample was approximately `十神关系` 937, `大运走势` 1002, and `年度引动` 1257, so M58 reduces the largest main-report burden without deleting evidence trace.

`main.txt`, `relationship.txt`, `wealth.txt`, `family.txt`, and `career.txt` scanned as 0 M58/M57/M56 hits for `读这一章时`, `这条线已经进入命盘视野`, `这条十神线索`, `命理结构上，当前阶段大运`, `五行流向上，`, `藏干里，`, `地支关系上，`, `年度本身先露出的`, `流年天干把十神主题推到台前`, `天干把十神主题推到台前`, `主题怎样被推到台前`, `拿到台前观察`, `五行说明这股力量怎样靠近日主`, `五行关系继续说明力量怎样靠近`, `不是罗列符号`, `推到台前`, `走到台前`, `拿到台前`, `score_internal`, and `0-100`.

Representative visible output now includes:

- `若收成几组力量，可以这样读`
- `当前阶段重点看大运怎样给一段时间定调`
- `把这些线索收回到实际读法，重点是`
- `2026年的天干让七杀信号更醒目`

## 5. Validation

Gates passed:

- `cargo fmt`
- `cargo test report -- --nocapture`
- real sample regeneration through local backend on `127.0.0.1:8791`
- five-sample visible body scan

Governance gates:

- governance scaffold check
- `git diff --check`

## 6. Next Cursor

Next loop: LOOP-127. Future report polish should preserve M47 relationship golden sample, M48 topic count-field narrative baseline, M49 annual/timeline narrative baseline, M50 wealth/family/career advice-cohesion baseline, M51 main-report tone-cohesion baseline, M52 closeout-continuity baseline, M53 report density/topic-specificity baseline, M54 timeline-detail warmth baseline, M55 current-luck consistency baseline, M56 conclusion de-duplication baseline, M57 timeline prose de-staging baseline, and M58 main-report long-section condensation baseline. Next likely slice is wealth/family/career middle-chapter personality polishing or further main-report annual wording refinement without changing capability boundaries.
