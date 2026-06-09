# M9 Pre-Closeout Audit

## 1. Scope

This evidence belongs to LOOP-029. It audits whether M9 can close as a full astronomy-engine milestone.

Conclusion: M9 is ready only for preflight closeout review. Full M9 astronomy-engine closeout is blocked because generated artifacts, hashes, completed comparison report, generated golden rows, replay tests, and runtime integration do not exist yet.

## 2. Capability Status

| Capability | Current status | Audit result |
| --- | --- | --- |
| `astronomy-engine` | target | Must remain target. |
| `calendar-date-query` | supported through Android date layer | Must not switch to astronomy-backed output. |
| `chart-create` | supported for current V1 chart core | Must preserve existing `algo_version` semantics. |

## 3. M9 Acceptance Audit

| M9 acceptance item | Current evidence | Result |
| --- | --- | --- |
| 星历黄金表生成可复现 | `generation-plan.json` and `tools/generate-astronomy-tables.ps1 -DryRun` only | blocked |
| 生成数据有 manifest/hash/命令 | draft manifest and command shape exist; hashes are missing | blocked |
| Android 对照差异报告完整 | `comparison-report-template.md` and zero-row comparison dry-run only | blocked |
| 2033、立春、清明、甲子日、真太阳时边界测试通过 | `golden-cases-plan.json` and zero-row golden dry-run only | blocked |
| 旧版本命盘可复现 | `replay-policy-draft.md` and zero-test replay-policy dry-run only | blocked |
| `tools/check-project.ps1` 通过 | Full gate passes | met for preflight only |

## 4. Preflight Evidence That Is Ready

| Evidence | Status |
| --- | --- |
| ADR 0015 parallel-first strategy | ready |
| ADR 0016 source stack | ready |
| source policy | ready |
| manifest schema | ready |
| not-accepted draft manifest | ready |
| generation plan | ready, not runnable |
| generator dry-run | ready, zero writes |
| comparison schema and report template | ready, zero rows |
| golden-case plan and dry-run | ready, zero rows |
| replay policy and dry-run | ready, zero replay tests |

## 5. Machine Audit

`data/generated/astronomy/precloseout-audit.json` records this same result:

- status: `full_m9_closeout_blocked_preflight_ready`
- full closeout allowed: false
- preflight closeout allowed: true
- generated artifacts accepted: false
- Android baseline replacement allowed: false

## 6. Required Before Full M9 Closeout

1. Implement a real generation command.
2. Generate planned artifacts under `data/generated/astronomy/out/`.
3. Record sha256 hashes for every generated artifact.
4. Complete Android-vs-astronomy comparison report and classify every difference.
5. Generate golden rows for the required categories.
6. Add replay tests proving existing `android-date-layer-v1` snapshots remain reproducible.
7. Add a replacement ADR before any default runtime behavior changes.

## 7. Next Work

LOOP-030 must choose one of two paths:

- continue M9 into generated-data implementation planning without capability promotion; or
- close M9 as a preflight-only milestone and split actual generated astronomy implementation into a later milestone.

Either path must keep `astronomy-engine` as target until accepted generated evidence exists.
