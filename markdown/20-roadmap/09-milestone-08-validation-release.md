# M8: Validation and Release Candidate

## 1. 目标

把前面所有 supported/restricted 能力收束为 V1 release candidate。M8 不追求新增功能，重点是验证、冻结、可复现和交付说明。

## 2. 依赖

- M1-M7 所有进入 V1 的能力已关闭或明确降级。
- S0 风险清零。
- capability ledger 与 `/api/capabilities` 一致。

## 3. 范围

| Work Package | 内容 |
| --- | --- |
| M8-WP1 | 全量后端测试、前端测试、治理检查 |
| M8-WP2 | API contract 冻结 |
| M8-WP3 | 黄金样例冻结和 hash/版本登记 |
| M8-WP4 | 前端关键路径 E2E |
| M8-WP5 | 移动端和可访问性检查 |
| M8-WP6 | 隐私、日志、分享脱敏审查 |
| M8-WP7 | README、release notes、closeout |
| M8-WP8 | 回滚/降级说明 |

## 4. 非目标

- 不在 release candidate 阶段临时加入大功能。
- 不临时放宽门禁。
- 不用文案遮盖未完成能力。

## 5. 能力状态

| Capability | Before | After |
| --- | --- | --- |
| all V1 capabilities | mixed | supported/restricted/planned 明确冻结 |
| `release-candidate` | planned | supported after M8 full gate and closeout |

## 6. 防回退

- Release 前不得删除失败样例来通过测试。
- 不得把 restricted 功能写进“已完整支持”清单。
- 不得把 S0 风险作为“已知问题”带入发布。
- 不得隐藏验证范围或算法版本。

## 7. 治理同步

- 关闭所有 V1 相关里程碑 closeout。
- `92-risk-register.md` 清零或登记剩余 P1/P2。
- `93-capability-promotion-ledger.md` 冻结状态。
- `README.md` 和产品树同步实际能力。

## 8. 验收

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1
```

此外需要：

- API contract test pass。
- E2E/manual browser evidence。
- share redaction evidence。
- accessibility checklist。
- release closeout。

## 9. 进入 M9 条件

- V1 release candidate 不再依赖未关闭 S0/P1 决策。
- Android 日期层作为 current baseline 的边界记录完整。
- 星历升级被明确为后续版本，不影响 V1 支持声明。

## 10. Closeout Update

M8 closeout is recorded in `26-milestone-08-closeout.md`. The release candidate document is `docs/release/v1-release-candidate.md`, and the reproducible release check is `tools/check-release-candidate.ps1` through the full project gate.

`release-candidate` is promoted as a governance/release capability only. It does not add a new backend business API and does not expand the supported business capability set.
