# ADR 0009: M2 Chart Basis Contract

## Status

Accepted.

## Decision

M2 establishes a restricted chart-basis contract under:

```text
GET /api/charts/basis/preview
```

This route returns ruleset metadata, birth-profile contract fields, chart-request contract fields, validated range, supported contract outputs, and explicit unsupported outputs.

This closes:

| Gate | Decision |
| --- | --- |
| DG-001 | The V1 default ruleset id is `ft-v1-default`. |
| DG-003 | The default day boundary is local civil midnight `00:00`; Zi-start remains an unsupported advanced option until later tests and UI copy exist. |
| DG-004 | V1 does not directly open lunar input in M2; lunar input remains planned/unsupported while chart-basis and full chart contracts mature. |

## Boundary

The M2 route is `restricted`, not full chart support.

It does not calculate:

- full four pillars;
- hour pillar;
- IANA timezone history;
- true solar time;
- lunar input conversion;
- persisted charts.

## Evidence

| Evidence | Scope |
| --- | --- |
| `backend/src/domain/bazi.rs` | Defines `RulesetId`, `CalculationMetadata`, `BirthProfile`, `ChartRequest`, and `ChartBasis` contract structures. |
| `backend/src/api/chart_basis.rs` | Exposes restricted preview route and validates query inputs. |
| App-layer tests | Cover successful restricted contract, lunar input rejection, true solar time rejection, and invalid exact time rejection. |
| `/api/capabilities` | Declares `chart-basis-preview` as `restricted`, while `chart-create` remains `planned`. |

## Consequences

- M3 may build chart calculation on top of `ChartBasis`.
- `chart-create` must remain `planned` until full four-pillar behavior and hour handling are implemented and tested.
- Lunar input cannot silently appear in API or UI while this ADR is active.

