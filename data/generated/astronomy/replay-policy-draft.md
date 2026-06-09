# M9 Replay Policy Draft

## Status

Draft only. This policy is not yet accepted.

## Rule

Existing V1 chart snapshots using `android-date-layer-v1` must remain reproducible after any astronomy engine is introduced.

## Required Before Replacement

- Store or preserve `algo_version` and `ruleset_id` on every chart snapshot.
- Keep Android date-layer replay available for existing snapshots.
- Add a replacement ADR before changing default runtime behavior.
- Compare Android and astronomy outputs before promotion.
- Classify every difference through the comparison taxonomy.

## Forbidden

- Silent replacement of `android-date-layer-v1`.
- Recomputing old snapshots with a new engine without a replay marker.
- Marking `astronomy-engine` supported before generated artifacts, hashes, comparison report, golden cases, and replay tests exist.
