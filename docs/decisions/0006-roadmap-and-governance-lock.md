# ADR 0006: Roadmap and Governance Lock Before Code Expansion

## Status

Accepted.

## Decision

Before adding more feature code beyond the current foundation and Android date-layer baseline, Fate-Track will use `markdown/20-roadmap/` as the authoritative development milestone layer.

Implementation must proceed milestone by milestone. Each milestone must preserve the governance lock:

- no capability is promoted to `supported` without Rust/API/test/capability evidence;
- no date-layer or chart-rule regression is accepted without ADR and replacement tests;
- no privacy, logging, sharing, or safe-interpretation rule is weakened for implementation speed;
- no new public surface closes without module-tree, engineering-tree, policy, and closeout evidence.

## Roadmap Files

| File | Role |
| --- | --- |
| `00-roadmap-index.md` | sequence, dependencies, invariants, execution protocol |
| `01-milestone-00-foundation-lock.md` through `10-milestone-09-astronomy-upgrade.md` | implementation phases |
| `90-decision-gates.md` | open decisions that block implementation shortcuts |
| `91-anti-regression-and-governance-lock.md` | anti-regression and anti-decoupling rules |
| `92-risk-register.md` | S0/P1/P2 risk tracking |
| `93-capability-promotion-ledger.md` | supported capability evidence requirements |
| `94-closeout-evidence-template.md` | closeout evidence format |

## Rationale

The project now has enough inputs to plan end to end: code skeleton, governance system, Android date layer, research translations, ADRs, target ruleset, privacy policy, and validation gates.

Without a milestone layer, future implementation can drift in two dangerous ways:

- feature code may outrun governance and claim unsupported abilities;
- governance docs may remain abstract and fail to block regressions.

The roadmap layer ties implementation order to evidence, tests, and capability state.

## Consequences

- Code work should begin only after the relevant milestone is selected and its decision gates are reviewed.
- Each milestone closeout must use the closeout evidence template or equivalent PR evidence.
- Any change to milestone order, capability promotion, or anti-regression locks must update this ADR or the roadmap files.

