# ADR 0012: Structured Analysis Only

## Status

Accepted.

## Decision

M4 analysis output is structured and fixed-template only.

This closes DG-010:

```text
No generative expansion for M4 analysis.
```

The supported analysis surface may return:

- deterministic metrics;
- fixed analysis cards;
- sensitivity flags;
- disclaimer id;
- forbidden-output audit result.

It must not return free-form generated essays or deterministic medical, legal, financial, fertility, death, criminality, coercive, or relationship-event claims.

## Boundary

M4 does not include luck cycles. DG-005 remains open, and `/api/luck/cycles` remains planned.

## Evidence Required

- Unit tests for element and ten-god metrics.
- API contract test showing metrics, cards, and disclaimer.
- Safety test rejecting forbidden phrase classes.

## Consequences

- `analysis-snapshot` can become supported only as a bounded structured surface.
- Future generated text, if ever allowed, needs a new decision gate and safety tests.

