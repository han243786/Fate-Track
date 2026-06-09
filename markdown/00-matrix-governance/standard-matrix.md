# 命轨 Standard Matrix

## 1. Hard Rules

| Rule ID | Source | Rule | Enforcement |
| --- | --- | --- | --- |
| STD-001 | GP-ARCH-001 | 能力声明必须有 Rust/API/data 真源。 | gate+audit |
| STD-002 | GP-ARCH-002 | sibling 模块直连必须登记例外。 | audit |
| STD-003 | GP-CODE-001 | Rust 源码必须通过格式化检查。 | gate |
| STD-004 | GP-CODE-002 | Rust workspace 必须通过测试检查。 | gate |
| STD-005 | GP-CODE-003 | JS 文件必须通过语法和测试检查。 | gate |
| STD-006 | GP-DATA-001 | raw 农历数据不得被应用运行时改写。 | audit |
| STD-007 | GP-FE-001 | 前端不得宣称后端未支持能力。 | gate+audit |
| STD-008 | GP-CHG-003 | 文件职责变化必须同步工程树和模块树。 | audit |
| STD-009 | GP-DATA-003 | 日期层改动必须保留 Android 边界样例或登记替代黄金样例。 | gate+audit |
| STD-010 | GP-ARCH-005 | 研究目标必须经 Rust/API/test/capability 支撑后才能标 supported。 | gate+audit |
| STD-011 | GP-DATA-005 | 命盘或日期层规则响应必须回显规则档与算法版本元数据。 | gate+audit |
| STD-012 | GP-DATA-006 | 黄金数据和衍生历法表必须登记引擎、版本、hash、范围和证据。 | gate+audit |
| STD-013 | GP-SEC-004 | 日志不得原样记录出生请求、完整命盘、私有备注或 token。 | gate+audit |
| STD-014 | GP-FE-005 | 分析文案不得输出确定性高风险人生断言。 | gate+audit |
| STD-015 | GP-CHG-004 | 外部研究报告进入实现前必须完成中文译文和治理落点。 | audit |
| STD-016 | GP-CHG-005 | 代码实现不得跳过对应里程碑、决策门和防回退锁。 | gate+audit |
| STD-017 | GP-CHG-006 | 能力晋级 supported 必须同步台账、API、README 或 UI 状态。 | gate+audit |
| STD-018 | GP-CHG-007 | 里程碑关闭必须有 closeout 证据，且 S0 风险清零。 | gate+audit |
| STD-019 | GP-CHG-008 | 递归开发必须维护活游标，并记录当前允许/禁止范围。 | gate+audit |
| STD-020 | GP-CHG-009 | 每轮递归必须写入 closeout，下一轮必须读取上一轮结果。 | gate+audit |
| STD-021 | GP-CHG-010 | `design_only` 状态下不得推进业务代码或 capability 晋级。 | gate+audit |
| STD-022 | M9 pre-closeout audit | `precloseout-audit.json` 不得允许 full M9 closeout 或 astronomy replacement，除非 generated artifacts、hashes、comparison、golden rows、replay tests 和 runtime integration 已存在。 | gate+audit |
| STD-023 | M9 implementation plan | `implementation-plan.json` 必须保持 `planning_only`，且下一步先定义 generator contract，再允许任何 generated artifact acceptance。 | gate+audit |
| STD-024 | M9 generator contract | `generator-contract.json` 必须保持 `contract_only`，planned outputs 必须为 `not_generated`，hash algorithm 必须为 `sha256`。 | gate+audit |
| STD-025 | M9 source adapter contract | `source-adapter-contract.json` 必须保持 `contract_only`，覆盖 GB/T/Horizons/SOFA/SPICE，且不得启用 runtime dependency 或 output claim。 | gate+audit |
| STD-026 | M9 artifact writer dry-run | `artifact-writer-plan.json` 必须保持 `dry_run_only` 和 `no_write_preview`；dry-run 不得写文件、计算 hash 或宣称 accepted evidence。 | gate+audit |
| STD-027 | M9 comparison runner dry-run | `comparison-runner-plan.json` 必须保持 `dry_run_only`；comparison dry-run 必须保持 rows/differences 为 0 且不得宣称 accepted evidence。 | gate+audit |
| STD-028 | M9 golden-row readiness | `golden-row-readiness-plan.json` 必须保持 `readiness_only`；所有 category 必须保持 `not_generated` 和 `blocked_until_generated_rows`。 | gate+audit |
| STD-029 | M9 replay-test readiness | `replay-test-readiness-plan.json` 必须保持 `readiness_only`；replay dry-run 必须保持 replay_tests_executed 为 0，且不得允许 replacement。 | gate+audit |
| STD-030 | M9 preflight-only closeout | `preflight-closeout-decision.json` 必须保持 `close_m9_as_preflight_only`、full engine closeout=false、`astronomy-engine` target、next milestone=M10，且不得允许 generated-data acceptance、runtime change 或 Android replacement。 | gate+audit |
| STD-031 | M10 guarded generator entry | `generator-implementation-entry.json` 和 `generate-astronomy-tables.ps1 -PrepareImplementation` 必须保持 no writes、hashes=0、acceptance unchanged、runtime unchanged、metadata-only source manifest blocked、`astronomy-engine` target。 | gate+audit |
| STD-032 | M10 source snapshot manifest boundary | `source-snapshot-manifest.schema.json` 必须保持 `schema_only`；`source-snapshot-manifest-plan.json` 必须保持 `manifest_materialized_metadata_only`；dry-run 必须保持 metadata manifest present、writes=false、snapshots=0、artifacts=0、hashes=0。 | gate+audit |
| STD-033 | M10 source snapshot manifest metadata | `source-snapshots/source-snapshot-manifest.json` 必须只记录 NAIF、IAU SOFA、JPL Horizons、GB/T 四个 source-boundary payload path/hash，且 runtime dependency/output claim 均为 false。 | gate+audit |
| STD-034 | M10 source payload materialization policy | `source-payload-materialization-policy.json` 必须只允许四个 selected source-boundary payload/hash；dry-run 必须保持 generated artifacts 0、acceptance unchanged、runtime unchanged。 | gate+audit |
| STD-035 | M10 source payload schemas | `source-payload-schemas/*.schema.json` 必须保持 `schema_only`；每个 planned payload 必须有匹配 schema，dry-run 必须保持 schema files 4、payload directory absent、payload files 0、source payloads 0、payload hashes 0、generated artifacts 0。 | gate+audit |
| STD-036 | M10 source capture procedure | `source-capture-procedure.json` 必须记录四个 selected source-boundary payload 的 capture/materialization/hash 状态；dry-run 必须保持 external calls false、generated artifacts 0、acceptance unchanged、runtime unchanged。 | gate+audit |
| STD-037 | M10 first source payload decision | `source-payload-materialization-decision.json` 必须保持 `decision_only` 和 `single_source_only`，选中源必须是 `naif-cspice`；dry-run 必须保持 payload directory absent、selected payload absent、payload files 0、source payloads 0、payload hashes 0、external calls false、generated artifacts 0。 | gate+audit |
| STD-038 | M10 selected source payload preflight | `selected-source-payload-materialization-preflight.json` 必须保持 `preflight_only`，next-loop scope 必须是 selected-source-only；dry-run 必须保持 payload directory absent、selected payload absent、payload files 0、source payloads 0、payload hashes 0、external calls false、generated artifacts 0。 | gate+audit |

| STD-039 | M10 selected source payload materialization | `selected-source-payload-materialization.json` and `source-snapshots/payloads/naif-cspice-kernel-boundary.json` must record exactly one selected `naif-cspice` source-boundary payload with sha256 `4c946457eb38425feb7bf87fce47583cd75456447c33f5152f4890f786afe5a2`; all unselected payload files, generated astronomy artifacts, generated artifact hashes, acceptance changes, runtime changes, CSPICE toolkit/kernel integration claims, Android replacement, and `astronomy-engine` promotion must remain forbidden. | gate+audit |
| STD-040 | M10 remaining source payload strategy | `remaining-source-payload-strategy.json` must remain `strategy_decision_only`, select `iau-sofa-ansi-c` as the next preflight-only source candidate, keep existing payload files at 1, new payload writes at 0, new source hashes at 0, generated artifacts at 0, acceptance unchanged, runtime unchanged, and `astronomy-engine` target. | gate+audit |
| STD-041 | M10 selected IAU SOFA payload preflight | `selected-iau-sofa-payload-materialization-preflight.json` must remain `preflight_only`, keep selected payload exists=false, existing payload count=1, new payload writes=0, new source hashes=0, generated artifacts=0, generated artifact hashes=0, acceptance unchanged, runtime unchanged, and `astronomy-engine` target. | gate+audit |
| STD-042 | M10 selected IAU SOFA payload materialization | `selected-iau-sofa-payload-materialization.json` and `source-snapshots/payloads/iau-sofa-routine-version.json` must record exactly one selected `iau-sofa-ansi-c` routine/version boundary payload with sha256 `436e197eb7e5aa24e22a493b6d7a79214ff4d7e5255b8f7763a4fbb3385d556f`; JPL/GB/T payloads, generated astronomy artifacts, generated artifact hashes, acceptance changes, runtime changes, SOFA integration claims, Android replacement, and `astronomy-engine` promotion must remain forbidden. | gate+audit |
| STD-043 | M10 post-IAU remaining source payload strategy | `post-iau-remaining-source-payload-strategy.json` must remain `strategy_decision_only`, select `jpl-horizons-api` as the historical next selected-source-only preflight candidate, and after LOOP-052 report JPL materialized only through selected payload evidence while keeping GB/T payload absent, generated artifacts 0, generated artifact hashes 0, acceptance unchanged, runtime unchanged, and `astronomy-engine` target. | gate+audit |
| STD-044 | M10 selected JPL Horizons payload preflight | `selected-jpl-horizons-payload-materialization-preflight.json` must remain `preflight_only`, select `jpl-horizons-api`, and after LOOP-052 verify selected payload exists=true, existing payload count=3, full-gate online query execution=false, new source payload writes 1, new source payload hashes 1, generated artifacts 0, generated artifact hashes 0, acceptance unchanged, runtime unchanged, and `astronomy-engine` target. | gate+audit |
| STD-045 | M10 selected JPL Horizons payload materialization | `selected-jpl-horizons-payload-materialization.json` and `source-snapshots/payloads/jpl-horizons-validation-samples.json` must record exactly one selected `jpl-horizons-api` validation-query snapshot boundary payload with sha256 `acddbee906bd4540795993a828b9308af5ab964c002739929e44e28249b444f9`; GB/T payloads, online JPL full-gate queries, response-body claims, generated astronomy artifacts, generated artifact hashes, acceptance changes, runtime changes, Android replacement, and `astronomy-engine` promotion must remain forbidden. | gate+audit |
| STD-046 | M10 selected GB/T payload preflight | `selected-gb-t-payload-materialization-preflight.json` must remain `preflight_only`, select `gb-t-33661-2017`, preserve LOOP-053 no-capture/no-materialization policy, and after LOOP-054 closed dry-run must observe selected payload exists=true, existing payload count=4, generated artifacts 0, acceptance unchanged, runtime unchanged, Android replacement false, and `astronomy-engine` target. | gate+audit |
| STD-047 | M10 selected GB/T payload materialization | `selected-gb-t-payload-materialization.json` and `source-snapshots/payloads/gb-t-33661-2017-rule-reference.json` must record exactly one selected GB/T rule-reference boundary payload with sha256 `7145ecb921d55580eac71d266b31f961b1b9e497cda805c942647737aa764f31`; copied standard text, implemented calendar-rule claims, generated astronomy artifacts, generated artifact hashes, acceptance changes, runtime changes, Android replacement, and `astronomy-engine` promotion must remain forbidden. | gate+audit |

## 2. Current Gate Commands

| Gate | Command | Blocking |
| --- | --- | --- |
| Project full check | `powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1` | yes |
| Governance scaffold | `powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-governance-scaffold.ps1 -ProjectRoot .` | yes |
| Release candidate check | `powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-release-candidate.ps1 -ProjectRoot .` | yes |
| Astronomy preflight check | `powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-astronomy-preflight.ps1 -ProjectRoot .` | yes |
| Rust format | `cargo fmt --check` | yes |
| Rust tests | `cargo test` | yes |
| JS syntax and tests | `cd frontend; npm.cmd run check` | yes |

## 3. Drift Rules

- 工程全量树必须在文件新增、删除、移动或职责变化时同步。
- 模块树必须在所有权、public surface、输入、输出或通信路径变化时同步。
- General Policy 必须在重复失败模式出现时更新。
- 门禁矩阵必须在新增阻断检查时更新。
