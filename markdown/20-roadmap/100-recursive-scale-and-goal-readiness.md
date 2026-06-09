# Recursive Scale and Goal Readiness

> This file records the post-LOOP-005..007 recursion optimization. It is a governance control, not permission to start `goal_run`.

## 1. Evidence From Recent Loops

| Loop | Code scale | Result | Lesson |
| --- | --- | --- | --- |
| `LOOP-005` | 49 Android edge cases + one narrow month-pillar fix | Full gate passed | Large loops are safe when one strong golden manifest defines the invariant. |
| `LOOP-006` | 5 API envelope/boundary tests + `out_of_range` semantic split | Full gate passed | A loop can cover a whole contract family when the public surface is single-purpose. |
| `LOOP-007` | Frontend API client + state + DOM + render + CSS + tests | Full gate passed + local HTTP check | A frontend loop is safe when it consumes an already-supported API and explicitly forbids new business scope. |
| `LOOP-015` | M5 cases/settings domain + API + tests + governance closeout | Full gate passed | A near-milestone loop is safe when the invariant is local volatile storage and every public surface remains restricted. |
| `LOOP-016` | M6 share domain + API + privacy tests + governance closeout | Full gate passed | Privacy-heavy loops can still be large when token, redaction, revocation, and unavailable-response rules are one invariant. |
| `LOOP-017` | M7 frontend workspace + tests + browser verification + governance closeout | Full gate passed | A large frontend loop is safe when it consumes existing API capabilities and keeps all overclaim risks visible. |
| `LOOP-018` | M8 release candidate checker + release notes + governance closeout | Full gate passed | A release-validation loop is safe when it adds gates and evidence without adding business scope. |
| `LOOP-019` | M9 parallel astronomy ADR + manifest schema + preflight checker | Full gate passed | A future-engine loop is safe only when preflight artifacts are explicitly not treated as runtime evidence. |
| `LOOP-020` | M9 source-stack ADR + source policy + preflight checker expansion | Full gate passed | A source-decision loop is safe when official sources are selected but still separated from generated runtime evidence. |
| `LOOP-021` | M9 optional source availability probe + evidence file | Full gate passed; optional probe summary warning | Network probes are useful evidence but must not become local full-gate blockers. |
| `LOOP-022` | M9 not-accepted generated manifest draft + checker enforcement | Full gate passed | Draft manifests are safe only when blockers and not-accepted status are machine-checked. |
| `LOOP-023` | M9 generation plan with artifact shapes and command draft | Full gate passed | Generation plans are safe before code when they are explicitly not runnable and every planned artifact is not generated. |
| `LOOP-024` | M9 dry-run-only generator skeleton + checker execution | Full gate passed | Generator skeletons are safe when the gate executes dry-run and verifies no writes, no acceptance change, and no artifacts. |
| `LOOP-025` | M9 comparison schema + golden-case plan + replay policy draft | Full gate passed | Acceptance evidence can be broadened safely when every new file is planning-only and machine-checked. |
| `LOOP-026` | M9 comparison dry-run scaffold + checker execution | Full gate passed | Comparison tooling can enter safely when it emits zero rows and no accepted evidence. |
| `LOOP-027` | M9 golden-case dry-run scaffold + checker execution | Full gate passed | Golden-case tooling can enter safely when it reports required categories only and keeps generated rows at zero. |
| `LOOP-028` | M9 replay-policy dry-run scaffold + checker execution | Full gate passed | Replay controls can become executable preflight evidence only when they execute zero replay tests and keep replacement disallowed. |
| `LOOP-029` | M9 pre-closeout audit + checker enforcement | Full gate passed | A milestone can be audited without closing it when the audit is machine-checked as full-closeout blocked. |
| `LOOP-030` | M9 generated-data implementation path ADR + planning JSON | Full gate passed | A blocked milestone can keep moving when the next implementation path is explicit and still forbids artifact acceptance. |
| `LOOP-031` | M9 generator contract + dry-run integration | Full gate passed | Generation can approach code only after the contract is machine-checked and still produces no rows. |
| `LOOP-032` | M9 source adapter contract + checker enforcement | Full gate passed | Source integration can be planned safely when every adapter forbids runtime dependency and output claims. |
| `LOOP-033` | M9 artifact writer dry-run + checker execution | Full gate passed | Artifact output planning is safe when it previews paths and hash policy while writing nothing and computing zero hashes. |
| `LOOP-034` | M9 comparison runner dry-run + checker execution | Full gate passed | Android comparison planning is safe when baseline bindings are explicit and comparison rows stay at zero. |
| `LOOP-035` | M9 golden-row readiness + checker enforcement | Full gate passed | Golden categories can move toward materialization only when readiness gates keep every row blocked and not generated. |
| `LOOP-036` | M9 replay-test readiness + checker enforcement | Full gate passed | Replay readiness can be prepared safely when replay controls remain unexecuted and replacement stays disallowed. |
| `LOOP-037` | M9 preflight-only closeout decision + M10 route | Full gate passed | A blocked high-risk milestone can close only as preflight when the decision explicitly routes real implementation forward and keeps capability promotion forbidden. |
| `LOOP-038` | M10 guarded generator implementation entry + checker execution | Full gate passed | Non-dry-run entrypoints are safe before generated artifacts only when they are blocked by missing source snapshots and machine-checked for no writes, hashes, acceptance changes, or runtime changes. |
| `LOOP-039` | M10 source snapshot manifest schema/plan + dry-run checker | Full gate passed | Source materialization can be approached safely by defining the manifest boundary while machine-checking that the actual manifest, snapshots, generated artifacts, and hashes remain absent. |
| `LOOP-040` | M10 metadata-only source snapshot manifest + checker execution | Full gate passed | Source manifest materialization is safe when it records provenance metadata only and keeps every source payload `not_materialized`, with generated artifacts and hashes still at zero. |
| `LOOP-041` | M10 source payload materialization policy + dry-run checker | Full gate passed | Source payload work can be staged safely when payload paths and formats are defined while payload directory, files, hashes, generated artifacts, and runtime changes stay absent. |
| `LOOP-042` | M10 per-source payload schemas + checker execution | Full gate passed | Source payload schemas can be added safely before source capture only when schema files are machine-matched to policy and payload files, hashes, generated artifacts, acceptance changes, and runtime changes stay absent. |
| `LOOP-043` | M10 source capture procedure + dry-run checker | Full gate passed | Source capture can be prepared safely when procedure steps and first-payload prerequisites are machine-checked while capture status stays not_started and payload files, hashes, external full-gate calls, generated artifacts, acceptance changes, and runtime changes stay absent. |
| `LOOP-044` | M10 first source payload decision + dry-run checker | Full gate passed | First-payload materialization can be narrowed safely when one selected source is machine-checked while payload directory, selected payload, hashes, external full-gate calls, generated artifacts, acceptance changes, and runtime changes stay absent. |
| `LOOP-045` | M10 selected source payload preflight + dry-run checker | Full gate passed | Selected-source materialization can be staged safely when the next-loop scope is single-source-only and this loop keeps payload directory, selected payload, hashes, external full-gate calls, generated artifacts, acceptance changes, and runtime changes absent. |
| `LOOP-046` | M10 selected `naif-cspice` source payload materialization + checker update | Full gate passed | Source payload materialization can begin safely when exactly one selected payload/hash is machine-checked while all unselected payloads, generated artifacts, generated artifact hashes, acceptance changes, runtime changes, and capability promotion remain forbidden. |
| `LOOP-047` | M10 remaining source payload strategy decision + dry-run checker | Full gate passed | Remaining-source strategy can reduce downstream loop cost when it chooses one next candidate and keeps all new payload writes, new hashes, external calls, generated artifacts, acceptance changes, and runtime changes at zero. |
| `LOOP-048` | M10 selected IAU SOFA payload preflight + dry-run checker | Full gate passed | Remaining-source preflight can stay low-drift when it selects one source, keeps the selected payload absent, and scopes the next loop to selected-source-only materialization. |
| `LOOP-049` | M10 selected IAU SOFA payload materialization + checker update | Full gate passed | A second source payload can land safely when it is routine/version boundary evidence only and all runtime, generated artifact, and acceptance claims remain blocked. |
| `LOOP-050` | M10 post-IAU remaining source payload strategy + dry-run checker | Full gate passed | Post-materialization strategy can keep recursion efficient when it chooses the next source preflight and machine-checks that no JPL/GB payload, new hash, generated artifact, acceptance change, or runtime change happened. |
| `LOOP-051` | M10 selected JPL Horizons payload preflight + dry-run checker | Full gate passed | Online-source preflight can stay deterministic when query execution is explicitly outside the full gate and the selected payload remains absent until a later materialization loop. |
| `LOOP-052` | M10 selected JPL Horizons payload materialization + checker update | Full gate passed | Online-source payloads can be materialized as offline query-boundary evidence when response bodies, online full-gate calls, generated artifacts, runtime changes, and promotion remain blocked. |
| `LOOP-053` | M10 selected GB/T payload preflight + dry-run checker | Full gate passed | Rule-reference preflight is safe when it keeps capture and payload materialization false in the current loop while scoping the next loop to selected-source-only materialization. |
| `LOOP-054` | M10 selected GB/T payload materialization + checker update | Full gate passed | Standards-source payloads can be materialized safely when the payload is boundary-only, copies no standard text, implements no calendar rules, and checker enforcement keeps generated artifacts, acceptance, runtime behavior, Android replacement, and promotion blocked. |

## 2. Optimized Loop Size Model

The old minimum-slice rule reduced risk but added too much overhead. The optimized rule is:

```text
Maximize code per loop inside one invariant.
Do not maximize unrelated surfaces per loop.
```

| Size | Allowed contents | Required guard | Use when |
| --- | --- | --- | --- |
| `S` | One file or one tiny contract | Local unit test | Unknown domain or uncertain evidence |
| `M` | One work-package family across 2-5 files | Full project gate + governance sync | Normal recursive development |
| `L` | One invariant across backend/frontend/docs/tests | Strong manifest or existing supported API | Evidence is clear and rollback blast radius is bounded |
| `XL` | Whole milestone | Milestone closeout template + no open blocking gate | Only in `milestone_loop` |
| `GOAL` | Multiple milestones | Goal readiness audit + explicit user command | Only after this file's criteria pass |

## 3. Anti-Drift Rules For Larger Loops

- One loop may touch many files only if all touched files serve one named invariant.
- Each loop must identify its primary invariant before implementation.
- Capability promotion must still be explicit in `93-capability-promotion-ledger.md`.
- Every public surface change must update module tree, engineering tree, and README or UI docs.
- A larger loop must run the full gate, not only targeted tests.
- If a targeted test exposes an algorithm mismatch, the fix may remain inside the same loop only when the mismatch is inside the current invariant.
- If a loop adds frontend UI, it must state whether the UI is supported, restricted, or probe-only.
- If the loop starts to require a second invariant, split it.

## 4. Goal Readiness Audit

Current status: `not_ready_for_goal`, `ready_for_milestone_loop`.

| Criterion | Status | Evidence |
| --- | --- | --- |
| User explicitly asked to start goal | not met | User asked to optimize and assess goal capability, not to start it |
| Three consecutive successful loops | met | `LOOP-005`, `LOOP-006`, `LOOP-007` |
| Full gate green after latest loop | met | Rust 17 tests, frontend 6 tests, governance scaffold OK |
| Cursor can resume accurately | met | `96-recursive-cursor.md` points to `LOOP-011` |
| No S0 risk | provisionally met | No active S0 was raised in LOOP-005..007 closeouts |
| Blocking decision gates resolved | met for M1 | `DG-002` is closed by ADR 0008 as 1901-2100 |
| Milestone closeout path prepared | met for M1 | `11-milestone-01-closeout-readiness.md` maps M1 WP1-WP5 evidence |
| Milestone loop proven | met | `LOOP-010` closed M1 via `12-milestone-01-closeout.md` |
| Goal segmentation defined | not met | Goal would need milestone-level stop points and per-milestone closeouts |

## 5. Decision

The project can use larger `single_loop` and has now proven one `milestone_loop` closeout.

LOOP-015 adds evidence that a milestone-sized implementation loop can land more code without drift when it is bounded by one invariant, explicit decision-gate closure, restricted capability status, and full governance sync.

LOOP-016 adds evidence that the same model works for privacy-sensitive share surfaces when public DTO redaction and token lifecycle rules are verified by tests before closeout.

LOOP-017 adds evidence that a frontend milestone can land as one large loop when the UI is an API consumer, not a new business-capability source, and browser verification is included before closeout.

LOOP-018 adds evidence that release validation can close as one milestone loop when the invariant is status freeze and reproducible gates, not feature expansion.

LOOP-019 adds evidence that high-risk engine upgrades can start safely when the first loop closes the decision gate, adds manifest/checker controls, and keeps the target capability unpromoted.

LOOP-020 adds evidence that source/tooling selection can advance M9 without drifting into unsupported engine claims when a checker guards the source policy and first generated range.

LOOP-021 adds evidence that optional external availability checks can be recorded while keeping deterministic local gates stable.

LOOP-022 adds evidence that generated-data planning can move forward without false promotion when the manifest itself carries acceptance blockers and the checker enforces them.

LOOP-023 adds evidence that future generator scope can be fixed without runtime drift by machine-checking planned artifact status and forbidden runtime claims.

LOOP-024 adds evidence that generator tooling can enter the repo before real generation as long as dry-run behavior is the only executable path and is tested by the gate.

LOOP-025 adds evidence that comparison, golden-case, and replay requirements can be fixed before data generation without drifting into accepted evidence.

LOOP-026 adds evidence that comparison output shape can be exercised before generated data exists, while keeping rows and accepted evidence at zero.

LOOP-027 adds evidence that golden-case coverage requirements can be exercised before rows exist, while keeping generated evidence at zero and target status unchanged.

LOOP-028 adds evidence that replacement safety controls can be machine-checked before replay tests exist, while still forbidding Android baseline replacement.

LOOP-029 adds evidence that pre-closeout audits are useful only when they can block overclaiming as strongly as they summarize readiness.

LOOP-030 adds evidence that generated-data work can resume after a blocked audit when the next stage is only a generator contract, not premature data generation.

LOOP-031 adds evidence that generation scripts can be tightened before generation by making dry-run depend on a contract-only input/output and hash policy.

LOOP-032 adds evidence that external-source planning stays stable when source adapters are contract-only and full gates remain local.

LOOP-033 adds evidence that generated-artifact handling can be prepared safely when path/hash preview remains separate from artifact creation and acceptance.

LOOP-034 adds evidence that Android-vs-astronomy comparison can be prepared safely when Android baseline metadata is bound before generated rows exist.

LOOP-035 adds evidence that golden-case readiness can be made explicit without creating golden rows or boundary-test claims.

LOOP-036 adds evidence that old-snapshot replay readiness can be made explicit without executing replay tests or changing runtime behavior.

LOOP-037 adds evidence that preflight closeout can prevent milestone drift when full implementation evidence is missing. It closes M9 only as governance preflight, keeps `astronomy-engine` as `target`, and moves generated-data implementation into M10.

LOOP-038 adds evidence that M10 can begin implementation without drifting into false data acceptance. The generator has a guarded non-dry-run entrypoint, but source snapshots are still missing and the checker enforces generation blocked, no writes, hashes 0, acceptance unchanged, runtime unchanged, and target capability status.

LOOP-039 adds evidence that source snapshot work can be staged without materializing sources prematurely. The source snapshot manifest schema and plan define provenance requirements while dry-run enforcement keeps the actual manifest absent, source snapshots 0, generated artifacts 0, hashes 0, and runtime unchanged.

LOOP-040 adds evidence that the source snapshot manifest itself can be materialized without accepting source payloads. The manifest records selected-source provenance metadata while checker enforcement keeps source payloads 0, generated artifacts 0, hashes 0, acceptance unchanged, runtime unchanged, and `astronomy-engine` target.

LOOP-041 adds evidence that source payload materialization can be planned without creating payloads. The policy defines per-source payload paths while checker enforcement keeps the payload directory absent, payload files 0, source payload hashes 0, generated artifacts 0, acceptance unchanged, runtime unchanged, and `astronomy-engine` target.

LOOP-042 adds evidence that per-source payload shapes can be defined without materializing sources. Schema-only files now bind GB/T, JPL Horizons, IAU SOFA, and NAIF CSPICE payload kinds to the source payload policy while checker enforcement keeps payload files 0, source hashes 0, generated artifacts 0, acceptance unchanged, runtime unchanged, and `astronomy-engine` target.

LOOP-043 adds evidence that source capture can be proceduralized before materialization. The procedure defines source-specific capture steps and first-payload preconditions while checker enforcement keeps capture `not_started`, payload files 0, source hashes 0, external full-gate calls false, generated artifacts 0, acceptance unchanged, runtime unchanged, and `astronomy-engine` target.

LOOP-044 adds evidence that first payload materialization can be narrowed before writing source payloads. The decision selects `naif-cspice` as the first single-source candidate while checker enforcement keeps the payload directory absent, selected payload absent, source hashes 0, external full-gate calls false, generated artifacts 0, acceptance unchanged, runtime unchanged, and `astronomy-engine` target.

LOOP-045 adds evidence that a selected source can be prepared for materialization without writing payloads. The preflight sets next-loop scope to selected-source-only for `naif-cspice` while checker enforcement keeps payload directory absent, selected payload absent, source hashes 0, external full-gate calls false, generated artifacts 0, acceptance unchanged, runtime unchanged, and `astronomy-engine` target.

LOOP-046 adds evidence that source payload materialization itself can stay bounded when it is selected-source-only. It materializes only `naif-cspice-kernel-boundary.json`, records one source sha256, rejects unselected payloads, and keeps generated artifacts, generated artifact hashes, manifest acceptance, runtime behavior, Android replacement, and `astronomy-engine` promotion absent.

LOOP-047 adds evidence that remaining-source sequencing should be its own control before the next payload preflight. The strategy chooses `iau-sofa-ansi-c` as the next preflight-only candidate while keeping new payload writes, new hashes, external calls, generated artifacts, manifest acceptance, runtime behavior, Android replacement, and `astronomy-engine` promotion absent.

LOOP-048 adds evidence that the next remaining source can enter preflight without materialization drift. The preflight chooses only `iau-sofa-ansi-c`, keeps the selected SOFA payload absent, keeps one existing `naif-cspice` payload, and keeps new payload writes, new source hashes, external calls, generated artifacts, manifest acceptance, runtime behavior, Android replacement, and `astronomy-engine` promotion absent.

LOOP-049 adds evidence that a second selected source payload can be materialized without turning into runtime integration. The payload records only the IAU SOFA routine/version boundary, keeps JPL/GB/T payloads absent, and keeps external calls, generated artifacts, generated artifact hashes, manifest acceptance, runtime behavior, Android replacement, and `astronomy-engine` promotion absent.

LOOP-050 adds evidence that strategy-only loops are still worthwhile after source payload materialization when they remove ambiguity for the next implementation slice. The post-IAU strategy selects JPL Horizons as the next selected-source-only preflight while keeping JPL/GB payloads absent, new source hashes 0, external calls false, generated artifacts 0, manifest acceptance unchanged, runtime unchanged, Android replacement absent, and `astronomy-engine` target.

LOOP-051 adds evidence that an online validation source can enter the recursive pipeline without making the full project gate depend on the network. The selected JPL Horizons preflight keeps the payload absent, records the required query/payload boundary, forbids online query execution in the full gate, and keeps new source hashes, generated artifacts, manifest acceptance, runtime behavior, Android replacement, and `astronomy-engine` promotion absent.

LOOP-052 adds evidence that an online validation source payload can be materialized without becoming a runtime integration. The selected JPL Horizons payload records only offline query-parameter snapshot boundaries and one source sha256, keeps response bodies absent, forbids online JPL query execution in the full gate, keeps GB/T absent, and keeps generated artifacts, generated artifact hashes, manifest acceptance, runtime behavior, Android replacement, and `astronomy-engine` promotion absent.

LOOP-053 adds evidence that standards rule-reference payload work can be prepared without accidentally materializing rules. The selected GB/T preflight keeps `gb-t-33661-2017-rule-reference.json` absent, source-reference capture false for this loop, payload materialization false for this loop, source hashes 0, external calls false, generated artifacts 0, manifest acceptance unchanged, runtime unchanged, Android replacement absent, and `astronomy-engine` target.

LOOP-054 adds evidence that a standards rule-reference payload can be materialized without copying protected text or implementing rules. The selected GB/T payload records only a boundary reference and sha256 `7145ecb921d55580eac71d266b31f961b1b9e497cda805c942647737aa764f31`, keeps standard_text_copied=false, generated artifacts 0, manifest acceptance unchanged, runtime unchanged, Android replacement absent, and `astronomy-engine` target.

The project should not start `goal_run` yet.

Reason: the current governance system is now strong enough for milestone loops through M8, but `goal_run` still needs explicit user authorization and a scoped list of milestone stop points.

## 6. Required Before Goal Run

Before invoking Codex goal mode, complete:

1. Define explicit milestone stop points for the goal scope.
2. Confirm which open decision gates can block each goal milestone.
3. Accept an explicit user command such as: `start goal_run for M2-M3 under recursive governance`.

## 7. Goal Run Guardrail

If goal mode is later started, it must still stop at every milestone boundary:

```text
goal_run:
  for milestone in planned_scope:
    read cursor + previous closeout
    implement optimized loops
    run full gate
    write milestone closeout
    update cursor
    stop if any decision gate blocks the next milestone
```

Goal mode is acceleration, not permission to bypass Read/Govern/Validate/Closeout.
