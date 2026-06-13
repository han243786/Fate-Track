# ADR 0022: DG-012 Timeline Reading Boundary

## Status

Accepted. Closes DG-012 for M34 and permits M35 internal timeline foundation work.

## Decision

The 大运/流年 interpretation layer enters post-preview as a separate restricted-upper-bound reading capability. It must not alter the supported raw `luck-cycles` calculation surface.

M35 may implement an internal compositional timeline foundation that produces traceable structures such as `TimelineSignal`, `TimelineEvidence`, `PlainReading`, and `RuleVersion`. Public route, UI, report chapter, and `/api/capabilities` promotion remain out of scope until later milestones close their own evidence.

## Context

M13 already supports raw luck-cycle calculation through `GET /api/luck/cycles`. M29-M33 already support restricted four-topic reports with explicit `year`, qualitative signals, no public `score_internal`, and forbidden-claim audit.

The new timeline-reading work needs to explain major luck and annual triggers without creating three regressions:

- Polluting the raw `luck-cycles` DTO with interpretation text, topic overlays, or scoring fields.
- Turning four-topic annual triggers into an overclaimed complete flow-year, flow-month, or daily fortune system.
- Replacing a rule engine with thousands of static success/failure sentences.

## Options Considered

1. Add interpretation fields directly to `GET /api/luck/cycles`.
   - Rejected. This would change the meaning of the supported M13 raw capability and make rollback hard.
2. Extend the existing topic-report route as the only timeline carrier.
   - Rejected for M35. Topic reports can later consume shared timeline signals, but they should not own the core timeline engine.
3. Build a separate internal timeline foundation first, then expose restricted reading surfaces through M36-M40.
   - Accepted. This preserves raw calculation, keeps evidence traceable, and lets each later surface pass its own gate.

## Chosen Option

Create a separate internal timeline foundation under backend domain code, starting in M35. The foundation may compute structural signals and plain-language reading fragments, but it does not create a public API or user-visible capability by itself.

## Policy Clauses

- `GET /api/luck/cycles` remains raw supported calculation only.
- Any future public annual-trigger API must require explicit `year`; frontend defaults may prefill UI state, but request handling must not infer a silent year.
- `score_internal`, weighted scores, 0-100 fortune scores, ranking scores, and deterministic success/failure numbers must not enter public API or UI.
- M34-M40 only cover major luck reading and specified annual trigger reading. Flow-month, flow-day, daily fortune, event prediction, and date selection require separate milestones and gates.
- All reading text must be structural, non-deterministic, and include professional wording, plain-language explanation, and boundary reminder.
- Timeline outputs must pass forbidden-claim audit for deterministic money, romance, family, career, disease, death, legal, and investment claims.

## Impacted Modules

| Module | Impact |
| --- | --- |
| `backend.domain.timeline` | New internal M35 foundation may be implemented after this ADR. |
| `backend.api.luck` | No semantic change; no reading fields, scores, or topic overlays. |
| `backend.api.topic_report` | No M35 change; M38 may later consume shared timeline signals after its own gate. |
| `frontend.ui` | No M35 change; timeline UI waits for M39. |
| `governance.roadmap` | M34 closeout and M35 entry must reference this ADR. |

## Required Tests

M35 must include backend domain tests proving:

- The engine returns traceable signals, evidence, readings, rule version, warnings, and audit result.
- Reading text includes professional and plain-language forms plus a safety boundary.
- Deterministic future claims are rejected by audit.
- `score_internal`, public fortune scores, and complete flow-month/day claims are absent.
- Unknown hour downgrades evidence instead of fabricating hour-pillar certainty.

Later route/UI milestones must add route, frontend, and no-overclaim tests before any public exposure.

## Rollback Rule

If timeline reading contaminates `GET /api/luck/cycles`, exposes public scores, silently infers API year, or claims complete flow-month/day/event prediction, the exposure must be removed and the affected capability returned to planned/internal status until the gate is repaired.

## Docs To Update

- `markdown/20-roadmap/90-decision-gates.md`
- `markdown/20-roadmap/106-milestone-34-timeline-reading-governance.md`
- `markdown/20-roadmap/107-milestone-35-timeline-lexicon-rule-engine.md`
- `markdown/20-roadmap/93-capability-promotion-ledger.md`
- `markdown/20-roadmap/92-risk-register.md`
- `markdown/20-roadmap/96-recursive-cursor.md`
- `markdown/20-roadmap/97-loop-closeout-log.md`
