# M10: Generated Astronomy Implementation

## 1. Goal

Implement the first real generated-data path for the astronomy upgrade while preserving the Android date layer as the accepted-current runtime baseline.

M10 converts the M9 preflight contracts into generated artifacts and validation evidence. It must not silently replace `calendar-date-query` or `chart-create`.

## 2. Entry Conditions

- M9 is closed only as preflight by `44-milestone-09-preflight-closeout.md`.
- `data/generated/astronomy/preflight-closeout-decision.json` records `m9_full_astronomy_engine_closed=false`.
- `astronomy-engine` remains `target`.
- No generated artifact is accepted before hash, manifest, comparison, golden-row, and replay controls exist.

## 3. Work Packages

| Work Package | Content | Capability ceiling |
| --- | --- | --- |
| M10-WP1 | Implement a non-dry-run generator entrypoint behind explicit acceptance flags. | target |
| M10-WP2 | Materialize local source snapshots or pinned routine versions required by `source-adapter-contract.json`. | target |
| M10-WP3 | Write generated artifacts under `data/generated/astronomy/out/` with deterministic canonical JSON. | target |
| M10-WP4 | Compute and record `sha256` hashes for every generated artifact. | target |
| M10-WP5 | Update the draft manifest into an accepted generated-data manifest only after artifact and hash checks pass. | restricted at most |
| M10-WP6 | Produce Android-vs-astronomy comparison rows without changing runtime behavior. | restricted at most |
| M10-WP7 | Materialize golden rows and replay tests, still without replacement. | restricted at most |

## 4. Non-Goals

- No default runtime replacement.
- No wider date-range support claim.
- No true solar time support claim.
- No IANA timezone-history support claim.
- No `astronomy-engine` supported claim before M10 closeout evidence and a later replacement policy allow it.

## 5. Acceptance

- Real generated artifacts exist and are reproducible.
- Every artifact has a recorded `sha256` hash.
- The manifest records generation command, source references, range, hash evidence, and acceptance status.
- Android-vs-astronomy comparison output exists and every difference is classified.
- Golden rows exist for the required M9 categories.
- Replay tests execute and prove old Android snapshots remain reproducible.
- `tools/check-project.ps1` passes.

## 6. Governance Sync

M10 closeout must update:

- `data/generated/astronomy/README.md`
- `markdown/00-matrix-governance/module-tree.md`
- `markdown/00-matrix-governance/standard-matrix.md`
- `markdown/10-overview/overview-full-feature-tree.md`
- `markdown/20-roadmap/93-capability-promotion-ledger.md`
- `markdown/20-roadmap/96-recursive-cursor.md`
- `markdown/20-roadmap/97-loop-closeout-log.md`

## 7. First Recursive Slice

LOOP-038 should start with M10 generator implementation entry.

The first slice should read the M9 preflight closeout and decide the smallest safe non-dry-run boundary. If source materialization is not ready, it must continue with contract tightening rather than writing partial accepted artifacts.

## 8. Generator Implementation Entry

LOOP-038 adds `46-milestone-10-generator-entry.md` and `data/generated/astronomy/generator-implementation-entry.json`.

The generator now exposes `tools/generate-astronomy-tables.ps1 -PrepareImplementation` as a guarded non-dry-run entry shape. This entrypoint remains blocked until a source snapshot manifest, local generation adapter, generated artifacts, hashes, comparison evidence, golden rows, replay tests, and replacement ADR exist.

## 9. Source Snapshot Manifest Boundary

LOOP-039 adds `47-milestone-10-source-snapshot-boundary.md`, `source-snapshot-manifest.schema.json`, `source-snapshot-manifest-plan.json`, and `tools/source-snapshot-manifest-dry-run.ps1`.

The boundary defines the future source snapshot manifest shape and selected-source coverage while keeping `data/generated/astronomy/source-snapshots/source-snapshot-manifest.json` absent. It still forbids source snapshot file writes, generated astronomy artifacts, hashes, manifest acceptance, runtime replacement, and `astronomy-engine` promotion.

## 10. Source Snapshot Manifest Metadata

LOOP-040 adds `48-milestone-10-source-snapshot-manifest.md` and materializes `data/generated/astronomy/source-snapshots/source-snapshot-manifest.json` as metadata only.

The manifest records selected-source provenance for GB/T 33661, JPL Horizons, IAU SOFA, and NAIF CSPICE/SPICE, but every source remains `not_materialized`. Source payload files, generated astronomy artifacts, hashes, manifest acceptance, runtime replacement, and `astronomy-engine` promotion remain forbidden.

## 11. Source Payload Materialization Policy

LOOP-041 adds `49-milestone-10-source-payload-policy.md`, `data/generated/astronomy/source-payload-materialization-policy.json`, and `tools/source-payload-materialization-dry-run.ps1`.

The policy defines future per-source payload paths and formats while keeping payload directory creation, payload files, source payload hashes, generated astronomy artifacts, generated artifact hashes, manifest acceptance, runtime replacement, and `astronomy-engine` promotion forbidden.

## 12. Source Payload Schemas

LOOP-042 adds `50-milestone-10-source-payload-schemas.md` and four schema-only files under `data/generated/astronomy/source-payload-schemas/`.

The schemas define future payload shapes for GB/T, JPL Horizons, IAU SOFA, and NAIF CSPICE while keeping source payload files, source payload hashes, generated astronomy artifacts, manifest acceptance, runtime replacement, and `astronomy-engine` promotion forbidden.

## 13. Source Capture Procedure

LOOP-043 adds `51-milestone-10-source-capture-procedure.md`, `data/generated/astronomy/source-capture-procedure.json`, and `tools/source-capture-procedure-dry-run.ps1`.

The procedure defines per-source capture steps and first-payload preconditions while keeping payload directory creation, payload files, source payload hashes, external full-gate calls, generated astronomy artifacts, manifest acceptance, runtime replacement, and `astronomy-engine` promotion forbidden.

## 14. First Source Payload Decision

LOOP-044 adds `52-milestone-10-first-source-payload-decision.md`, `data/generated/astronomy/source-payload-materialization-decision.json`, and `tools/source-payload-materialization-decision-dry-run.ps1`.

The decision selects `naif-cspice` as the first single-source payload candidate while keeping payload directory creation, payload files, source payload hashes, external full-gate calls, generated astronomy artifacts, manifest acceptance, runtime replacement, and `astronomy-engine` promotion forbidden.

## 15. Selected Source Payload Preflight

LOOP-045 adds `53-milestone-10-selected-source-payload-preflight.md`, `data/generated/astronomy/selected-source-payload-materialization-preflight.json`, and `tools/selected-source-payload-materialization-preflight-dry-run.ps1`.

The preflight narrows the next loop to selected-source-only `naif-cspice` payload materialization while keeping payload directory creation, selected payload file writes, source payload hashes, external full-gate calls, generated astronomy artifacts, manifest acceptance, runtime replacement, and `astronomy-engine` promotion forbidden in LOOP-045.

## 16. Selected Source Payload Materialization

LOOP-046 adds `54-milestone-10-selected-source-payload-materialization.md`, `data/generated/astronomy/selected-source-payload-materialization.json`, and `data/generated/astronomy/source-snapshots/payloads/naif-cspice-kernel-boundary.json`.

The materialization records exactly one selected `naif-cspice` source-boundary payload and sha256 `4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2`. It keeps GB/T, JPL Horizons, and IAU SOFA payload files absent; writes no generated astronomy artifacts; computes no generated artifact hashes; leaves draft manifest acceptance and runtime behavior unchanged; and keeps `astronomy-engine` as target.

## 17. Remaining Source Payload Strategy

LOOP-047 adds `55-milestone-10-remaining-source-payload-strategy.md`, `data/generated/astronomy/remaining-source-payload-strategy.json`, and `tools/remaining-source-payload-strategy-dry-run.ps1`.

The strategy selects `iau-sofa-ansi-c` as the next preflight-only source candidate, then JPL Horizons, then GB/T. It keeps existing payload files at one, writes no new payloads, computes no new source hashes, performs no external full-gate calls, writes no generated astronomy artifacts, leaves draft manifest acceptance and runtime behavior unchanged, and keeps `astronomy-engine` as target.

## 18. Selected IAU SOFA Payload Preflight

LOOP-048 adds `56-milestone-10-selected-iau-sofa-payload-preflight.md`, `data/generated/astronomy/selected-iau-sofa-payload-materialization-preflight.json`, and `tools/selected-iau-sofa-payload-materialization-preflight-dry-run.ps1`.

The preflight scopes the next loop to selected-source-only `iau-sofa-ansi-c` local routine/version boundary materialization. It keeps the IAU SOFA payload absent in LOOP-048, keeps existing payload files at one, writes no new payloads, computes no new source hashes, performs no external full-gate calls, writes no generated astronomy artifacts, leaves draft manifest acceptance and runtime behavior unchanged, and keeps `astronomy-engine` as target.

## 19. Selected IAU SOFA Payload Materialization

LOOP-049 adds `57-milestone-10-selected-iau-sofa-payload-materialization.md`, `data/generated/astronomy/selected-iau-sofa-payload-materialization.json`, and `data/generated/astronomy/source-snapshots/payloads/iau-sofa-routine-version.json`.

The materialization records the selected `iau-sofa-ansi-c` routine/version boundary payload and sha256 `436e197eb7e5aa24e22a493b6d7a79214ff4d7e5255b8f7763a4fbb3385d556f`. It keeps JPL Horizons and GB/T payload files absent, writes no generated astronomy artifacts, computes no generated artifact hashes, leaves draft manifest acceptance and runtime behavior unchanged, and keeps `astronomy-engine` as target.

## 20. Post-IAU Remaining Source Payload Strategy

LOOP-050 adds `58-milestone-10-post-iau-remaining-source-payload-strategy.md`, `data/generated/astronomy/post-iau-remaining-source-payload-strategy.json`, and `tools/post-iau-remaining-source-payload-strategy-dry-run.ps1`.

The strategy preserves the two existing source-boundary payloads for `naif-cspice` and `iau-sofa-ansi-c`, selects `jpl-horizons-api` as the next selected-source-only preflight candidate, and leaves `gb-t-33661-2017` for a later governed scope. It writes no JPL or GB/T payload files, computes no new source hashes, performs no external full-gate calls, writes no generated astronomy artifacts, leaves draft manifest acceptance and runtime behavior unchanged, and keeps `astronomy-engine` as target.

## 21. Selected JPL Horizons Payload Preflight

LOOP-051 adds `59-milestone-10-selected-jpl-horizons-payload-preflight.md`, `data/generated/astronomy/selected-jpl-horizons-payload-materialization-preflight.json`, and `tools/selected-jpl-horizons-payload-materialization-preflight-dry-run.ps1`.

The preflight scopes the next loop to selected-source-only `jpl-horizons-api` validation-query snapshot payload materialization. It keeps the JPL payload absent in LOOP-051, keeps GB/T absent, writes no payload files, computes no new source hashes, performs no external full-gate calls, executes no online JPL query in the full project gate, writes no generated astronomy artifacts, leaves draft manifest acceptance and runtime behavior unchanged, and keeps `astronomy-engine` as target.

## 22. Selected JPL Horizons Payload Materialization

LOOP-052 adds `60-milestone-10-selected-jpl-horizons-payload-materialization.md`, `data/generated/astronomy/selected-jpl-horizons-payload-materialization.json`, and `data/generated/astronomy/source-snapshots/payloads/jpl-horizons-validation-samples.json`.

The materialization records the selected `jpl-horizons-api` validation-query snapshot boundary payload and sha256 `acddbee906bd4540795993a828b9308af5ab964c002739929e44e28249b444f9`. It keeps GB/T payload files absent, includes no JPL response bodies, executes no online JPL query in the full project gate, writes no generated astronomy artifacts, computes no generated artifact hashes, leaves draft manifest acceptance and runtime behavior unchanged, replaces no Android baseline, and keeps `astronomy-engine` as target.

## 23. Selected GB/T Payload Preflight

LOOP-053 adds `61-milestone-10-selected-gb-t-payload-preflight.md`, `data/generated/astronomy/selected-gb-t-payload-materialization-preflight.json`, and `tools/selected-gb-t-payload-materialization-preflight-dry-run.ps1`.

The preflight scopes the next loop to selected-source-only `gb-t-33661-2017` calendar rule-reference payload materialization. It keeps `gb-t-33661-2017-rule-reference.json` absent in LOOP-053, computes no GB/T source hash, captures no rule text in the full project gate, writes no generated astronomy artifacts, computes no generated artifact hashes, leaves draft manifest acceptance and runtime behavior unchanged, replaces no Android baseline, and keeps `astronomy-engine` as target.

## 24. Selected GB/T Payload Materialization

LOOP-054 adds `62-milestone-10-selected-gb-t-payload-materialization.md`, `data/generated/astronomy/selected-gb-t-payload-materialization.json`, and `data/generated/astronomy/source-snapshots/payloads/gb-t-33661-2017-rule-reference.json`.

The materialization records the selected `gb-t-33661-2017` calendar rule-reference boundary payload and sha256 `7145ecb921d55580eac71d266b31f961b1b9e497cda805c942647737aa764f31`. It copies no GB/T standard text, treats no rule-reference boundary as an implemented calendar algorithm, writes no generated astronomy artifacts, computes no generated artifact hashes, leaves draft manifest acceptance and runtime behavior unchanged, replaces no Android baseline, and keeps `astronomy-engine` as target.
