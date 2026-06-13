# Milestone 60 Closeout: Topic Timeline Reader-Facing Polish

## Result

M60 is closed as a quality-only report-copy loop. Wealth, family, and career `本专题的大运流年` chapters now read as direct topic interpretation instead of visible engine-layer explanation.

## Completed Changes

| Area | Evidence |
| --- | --- |
| Wealth topic timeline | `本专题的大运流年` now opens around 2026 resource rhythm, opportunity approach, output continuity, budget boundary, and whether resources can stay. |
| Family topic timeline | `本专题的大运流年` now opens around 2026 support, household speech, responsibility placement, boundaries, and emotional settling. |
| Career topic timeline | `本专题的大运流年` now opens around 2026 task pressure, skill delivery, resource support, collaboration boundary, and sustainable action. |
| Domain gate | `backend/src/domain/topic_report.rs` requires M60 reader-facing anchors and rejects stale timeline scaffolds in assembled wealth/family/career reports. |
| API gate | `backend/src/app.rs` requires the same anchors and rejects the same forbidden phrases in real `/api/charts/topic-report` responses. |
| Samples | `target/report-polish-samples/main.json`, `relationship.json`, `wealth.json`, `family.json`, and `career.json` were regenerated from one consistent profile and extracted to `.txt` samples. |

## Validation

- `cargo fmt` passed.
- `cargo test topic_report -- --nocapture` passed 9 tests.
- `cargo test report -- --nocapture` passed 23 tests.
- Regenerated sample audit: `main`, `relationship`, `wealth`, `family`, and `career` all returned `forbidden_output_audit.status = passed`.
- Sample scan confirmed:
  - `wealth`: 4 M60 required anchors present, 8 M60 forbidden phrases absent.
  - `family`: 4 M60 required anchors present, 8 M60 forbidden phrases absent.
  - `career`: 4 M60 required anchors present, 8 M60 forbidden phrases absent.

- Final gates passed: governance scaffold check and `git diff --check`.

## Capability Status

No capability changes.

- V1 preview remains 10 supported, 7 restricted.
- Post-preview current runtime remains 10 supported, 14 restricted, 0 planned.
- `relationship-report`, `wealth-report`, `family-report`, `career-report`, `luck-reading`, `annual-trigger-reading`, and `topic-timeline-reading` remain restricted.

## Follow-Up Cursor

Next loop should preserve M47-M60 report-copy baselines. Future report polish should continue to start from regenerated real samples, keep the relationship report as the current golden sample, and avoid any capability expansion without a new milestone and decision gate.
