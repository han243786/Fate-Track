# M10 Selected GB/T Payload Materialization

> LOOP-054 materializes the selected GB/T 33661-2017 rule-reference boundary payload. This is source-boundary evidence only; it is not copied standard text, not a generated astronomy artifact, not a runtime calendar algorithm, and not an `astronomy-engine` promotion.

## 1. Scope

- Selected source: `gb-t-33661-2017`.
- Payload: `data/generated/astronomy/source-snapshots/payloads/gb-t-33661-2017-rule-reference.json`.
- Evidence: `data/generated/astronomy/selected-gb-t-payload-materialization.json`.
- Hash: `7145ecb921d55580eac71d266b31f961b1b9e497cda805c942647737aa764f31`.

## 2. Delivered Evidence

| Artifact | Status |
| --- | --- |
| `gb-t-33661-2017-rule-reference.json` | materialized |
| `selected-gb-t-payload-materialization.json` | materialized evidence |
| `source-snapshots/source-snapshot-manifest.json` | records GB/T `rule_reference_payload_materialized` |
| `source-payload-materialization-policy.json` | records GB/T `materialized` and hash `computed` |
| `source-capture-procedure.json` | records GB/T `completed_for_rule_reference_boundary` |
| `tools/selected-gb-t-payload-materialization-preflight-dry-run.ps1` | closed dry-run observes four payloads |
| `tools/check-astronomy-preflight.ps1` | verifies four source-boundary payloads and no generated artifacts |

## 3. Guardrails

- No GB/T standard text is copied into the repository.
- No rule-reference boundary is treated as an implemented Chinese-calendar algorithm.
- No generated astronomy artifact is written.
- No generated artifact hash is computed.
- No external call is performed by the full project gate.
- No draft manifest acceptance changes.
- No `calendar-date-query` or `chart-create` runtime behavior changes.
- No Android baseline replacement.
- No `astronomy-engine` supported claim.

## 4. Acceptance

- The selected GB/T payload file exists and its sha256 equals `7145ecb921d55580eac71d266b31f961b1b9e497cda805c942647737aa764f31`.
- NAIF, IAU SOFA, JPL Horizons, and GB/T are the only materialized source-boundary payloads.
- `tools/check-astronomy-preflight.ps1` passes.
- Full project gate passes before LOOP-054 closeout.

## 5. Next Work

The next safe work package is generated astronomy artifact materialization preflight. It may define artifact-write entry conditions only; it must still forbid generated artifact writes, generated artifact hashes, manifest acceptance, runtime replacement, Android baseline replacement, and `astronomy-engine` promotion until a later governed loop explicitly opens that scope.
