# 命轨 General Policy

> Scope: all code, tests, docs, scripts, generated artifacts, data changes, and release changes.
> Rule: a change that violates blocking policy cannot close out until fixed or explicitly exempted.

## 1. Architecture Rules

| ID | Rule | Check | Severity | Evidence |
| --- | --- | --- | --- | --- |
| GP-ARCH-001 | 用户可见能力必须有真实能力来源，不得只在 UI 或文档中静态声明。 | gate+audit | S0 | source/API/test |
| GP-ARCH-002 | 子模块默认不得横向直连 sibling 模块，必须经父模块、API、adapter、事件或契约层通信。 | audit | P1 | module tree |
| GP-ARCH-003 | 新 public API 必须登记输入、输出、错误、调用方和兼容性。 | gate+audit | P1 | module tree/API docs/tests |
| GP-ARCH-004 | Rust 后端是计算、校验、数据读取和未来持久化的能力真源；JS 前端不得复制核心算法。 | audit | P1 | module tree/code review |
| GP-ARCH-005 | 研究报告中的目标合约不得直接等同于 supported 能力；必须由 Rust/API/test/capability 四项同时支撑。 | gate+audit | S0 | ADR/capabilities/tests |

## 2. Code Rules

| ID | Rule | Check | Severity | Evidence |
| --- | --- | --- | --- | --- |
| GP-CODE-001 | Rust 源码必须通过 `cargo fmt --check`。 | gate | P1 | formatter |
| GP-CODE-002 | Rust workspace 必须通过 `cargo test`。 | gate | P1 | compiler/tests |
| GP-CODE-003 | JS 源码必须通过 `npm.cmd run check`。 | gate | P1 | syntax/tests |
| GP-CODE-004 | 错误、unsupported 状态和非法参数不得被静默忽略。 | gate+audit | S0 | tests/audit |
| GP-CODE-005 | Stub、fake 或 TODO 实现不得被文档或 UI 宣称为 supported。 | gate+audit | S0 | source/docs |

## 3. Data Rules

| ID | Rule | Check | Severity | Evidence |
| --- | --- | --- | --- | --- |
| GP-DATA-001 | `data/raw/lunar_data.yaml` 是当前农历基础数据真源，应用运行时不得改写。 | audit | S0 | data README/code review |
| GP-DATA-002 | 任何农历衍生产物必须登记来源文件、生成命令、生成时间和验证证据。 | gate+audit | P1 | ADR/script/closeout |
| GP-DATA-003 | 数据范围、闰月、节气、干支等能力声明必须能追溯到 raw data 或已登记算法。 | audit | P1 | data docs/tests |
| GP-DATA-004 | 日期层算法改动必须保留 Android 边界样例，或提供更强的替代黄金样例并登记原因。 | gate+audit | P1 | tests/ADR |
| GP-DATA-005 | 命盘或日期层响应一旦涉及规则选择，必须回显 `ruleset_id`、`algo_version` 或等价版本元数据。 | gate+audit | P1 | API contract/tests |
| GP-DATA-006 | 黄金样例、星历表、时区表或衍生历法表必须登记生成引擎、版本、hash、范围和回归证据。 | gate+audit | P1 | data manifest/tests |

## 4. Security and Privacy Rules

| ID | Rule | Check | Severity | Evidence |
| --- | --- | --- | --- | --- |
| GP-SEC-001 | 密钥、令牌、凭证不得进入日志、前端状态、截图、fixtures 或报告。 | gate+audit | S0 | scan/review |
| GP-SEC-002 | 出生时间、出生地、姓名/代号等命盘输入默认按高敏信息处理。 | audit | S0 | product tree/UI review |
| GP-SEC-003 | 本地 HTTP 服务不得暴露项目根以外文件。 | gate+audit | S0 | frontend server review |
| GP-SEC-004 | 日志不得原样记录出生请求体、出生地点、完整命盘 JSON、私有备注、access token、session ID 或分享 token。 | gate+audit | S0 | logging review/tests |
| GP-SEC-005 | 公开分享必须默认脱敏，并且不得把私有案例实时状态直接暴露为公开视图。 | gate+audit | S0 | share tests/API review |

## 5. Frontend Rules

| ID | Rule | Check | Severity | Evidence |
| --- | --- | --- | --- | --- |
| GP-FE-001 | UI 不得先于 Rust 后端能力真源宣称功能可用。 | gate+audit | S0 | API/source review |
| GP-FE-002 | 用户可见文案必须区分 supported、restricted、planned、unsupported。 | audit | P1 | screenshots/docs |
| GP-FE-003 | 前端不得直接读取 raw YAML；必须通过后端 API 或登记后的静态衍生产物。 | audit | P1 | code review |
| GP-FE-004 | 命盘、分析、运势和分享页面必须显示规则/版本/不确定性提示，不得隐藏边界规则。 | audit | P1 | UI review/screenshots |
| GP-FE-005 | 分析文案不得输出确定性疾病、死亡、法律、金融、关系、犯罪、道德或强制性人生断言。 | gate+audit | S0 | copy review/tests |

## 6. Change Management Rules

| ID | Rule | Check | Severity | Evidence |
| --- | --- | --- | --- | --- |
| GP-CHG-001 | 新能力必须写影响范围、非目标、失败/拒绝行为和回归保护。 | gate+audit | P1 | proposal/closeout |
| GP-CHG-002 | 削弱或删除测试必须登记 retired reason 和替代保护。 | audit | P1 | test plan |
| GP-CHG-003 | 文件新增、删除、迁移或职责变化必须同步工程全量树和模块树。 | audit | P1 | governance diff |
| GP-CHG-004 | 外部研究报告进入实现前必须先完成中文译文、研究纳入台账和 ADR/矩阵落点。 | audit | P1 | research intake/ADR |
| GP-CHG-005 | 里程碑实现前必须读取对应 roadmap、决策门、风险台账和防回退锁，不得跳阶段实现。 | gate+audit | P1 | roadmap/closeout |
| GP-CHG-006 | 能力晋级 supported 必须同步 capability ledger、capability API、README 或 UI 状态。 | gate+audit | S0 | capability ledger/API/docs |
| GP-CHG-007 | 任一里程碑关闭前必须提交 closeout 证据；S0 风险未清零不得进入下一阶段。 | gate+audit | S0 | closeout/risk register |
| GP-CHG-008 | 递归式开发必须维护 `96-recursive-cursor.md`；每轮实现前后必须更新游标或说明未变更原因。 | gate+audit | P1 | recursive cursor |
| GP-CHG-009 | 每轮递归必须写入 `97-loop-closeout-log.md` 或等价 closeout；下一轮必须读取上一轮结果。 | gate+audit | P1 | loop closeout |
| GP-CHG-010 | `design_only` 游标状态下不得推进业务代码、API 行为、前端功能或 capability 晋级。 | gate+audit | S0 | cursor/scope review |

## 7. Exception Format

```markdown
## Exception ID: GP-EX-001

**Policy hit**:
**Reason**:
**Scope**:
**Risk**:
**Exit condition**:
**Owner**:
**Closeout tracking**:
```
