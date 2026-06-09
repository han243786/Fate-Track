# M10 Remaining Source Payload Strategy

## 1. Scope

LOOP-047 decides the remaining source payload sequence after the selected `naif-cspice` boundary payload has been materialized.

The strategy selects `iau-sofa-ansi-c` as the next source payload candidate for a future preflight-only loop.

## 2. Current State

| Source | Payload State |
| --- | --- |
| `naif-cspice` | materialized source-boundary payload with sha256 `4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2` |
| `iau-sofa-ansi-c` | not materialized; selected as next preflight candidate |
| `jpl-horizons-api` | not materialized; sequenced after SOFA |
| `gb-t-33661-2017` | not materialized; sequenced after JPL |

## 3. Decision

Remaining payload order:

1. `iau-sofa-ansi-c`
2. `jpl-horizons-api`
3. `gb-t-33661-2017`

The next loop may define selected-source-only preflight for `iau-sofa-ansi-c` routine/version evidence. It must not write the SOFA payload file in LOOP-047.

## 4. Rationale

`iau-sofa-ansi-c` is the lowest remaining payload risk because it can start as a local routine/version boundary before any generated astronomy artifact, external full-gate call, runtime dependency, or copied standard text.

`jpl-horizons-api` is sequenced after SOFA because it needs offline query parameter/sample governance before any external request or full-gate dependency.

`gb-t-33661-2017` is sequenced last because rule-reference scope needs extra care before text capture or implementation claims.

## 5. Evidence

| Evidence | Status |
| --- | --- |
| `data/generated/astronomy/remaining-source-payload-strategy.json` | strategy decision only |
| `tools/remaining-source-payload-strategy-dry-run.ps1` | dry-run inspection only |
| `tools/check-astronomy-preflight.ps1` | enforces strategy, current materialized count, next source, and no writes |

## 6. Forbidden Work

- Do not write `iau-sofa-routine-version.json`.
- Do not write JPL Horizons or GB/T payload files.
- Do not compute new source payload hashes.
- Do not perform external API calls in the full project gate.
- Do not write generated astronomy artifacts.
- Do not compute generated artifact hashes.
- Do not mark the draft manifest accepted.
- Do not change runtime behavior.
- Do not replace `android-date-layer-v1`.
- Do not claim `astronomy-engine` supported.

## 7. Closeout Criteria

- `remaining-source-payload-strategy-dry-run.ps1` reports `remaining_source_payload_strategy_dry_run`.
- Existing payload files count remains 1.
- New source payload writes remain 0.
- New source payload hashes remain 0.
- Generated artifacts and generated artifact hashes remain 0.
- Draft manifest remains `not_accepted`.
- Runtime behavior remains unchanged.
- `astronomy-engine` remains `target`.
- `tools/check-project.ps1` passes.
