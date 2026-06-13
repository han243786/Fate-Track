# M35 Closeout: Timeline Lexicon And Rule Engine

## 0. Status

`closed by LOOP-099 as internal foundation`.

M35 closes the backend-internal foundation for timeline reading. It does not expose a public route, frontend UI, report chapter, `/api/capabilities` declaration, public score, or supported/restricted user-facing timeline capability.

## 1. Scope Closed

| Work Package | Evidence |
| --- | --- |
| M35-WP1 `TimelineSignal` | `backend/src/domain/timeline.rs` defines signal id, label, category, qualitative level, source, topic applicability, risk tags, and summary. |
| M35-WP2 `TimelineEvidence` | Evidence records signal id, source, relation, chart anchor, trigger, and detail. |
| M35-WP3 `PlainReading` | Reading records professional sentence, plain-language sentence, and boundary reminder. |
| M35-WP4 `timeline-lexicon` | Lexicon covers ten gods, five elements, branch relations, hidden stems, pattern, and useful-god concepts. |
| M35-WP5 compositional rules | Engine composes major luck, explicit annual pillar, original chart anchors, hidden stems, branch relations, five-element flow, and luck/year overlay. |
| M35-WP6 trace and version | `TimelineRuleVersion` records `ft-v1-default`, `timeline-core-v1`, and disclaimer id. Every reading links to a signal and evidence. |
| M35-WP7 anti-template-explosion | Tests prove compositional behavior without 10 x 12 / 60-jiazi static fortune tables. |

## 2. Validation

```powershell
cargo test timeline -- --nocapture
```

Result: passed.

Covered tests:

- lexicon explains core primitives.
- foundation builds traceable signals without public score.
- annual trigger requires explicit pillar input.
- major luck and annual overlay is compositional.
- unknown hour downgrades evidence instead of fabricating hour certainty.
- deterministic claims are rejected by audit.

## 3. Boundary Evidence

No M35 change:

- no public HTTP route.
- no frontend UI.
- no report chapter.
- no `/api/capabilities` declaration.
- no mutation of `GET /api/luck/cycles`.
- no public `score_internal`, fortune score, or ranking score.
- no silent `year` inference.
- no flow-month, flow-day, daily fortune, date selection, event prediction, or advice claim.

## 4. Next Milestone

M36 may consume the internal foundation for primary chart luck reading only after preserving ADR 0022 constraints:

- raw `luck-cycles` remains unchanged.
- user-visible reading remains restricted upper bound.
- every reading keeps professional/plain/boundary structure.
- no deterministic future claim or public score.
