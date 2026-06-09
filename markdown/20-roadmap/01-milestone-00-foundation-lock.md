# M0: Foundation Lock

## 1. 目标

锁定当前项目基础：Rust 后端、JS 前端、治理体系、研究报告纳入、Android 日期层基线和完整检查命令。M0 的作用是把“可以继续开发”的地基冻结成可审计状态。

## 2. 当前状态

| 项 | 状态 |
| --- | --- |
| 代码骨架 | 已建立 |
| Android 日期层 | 已移植为当前 date-layer baseline |
| 研究报告 | 已中文化并纳入治理 |
| ADR | 0001-0005 已存在 |
| 完整检查 | `tools/check-project.ps1` |

## 3. 交付件

- `README.md` 能说明运行、API、检查和研究入口。
- `docs/decisions/0001-0005` 能解释技术栈、数据源、Android 日期层、研究基线、目标规则与隐私安全。
- `markdown/reserch/` 保留源报告、中文译文和纳入台账。
- `markdown/20-roadmap/` 提供从 M0 到 M9 的开发路线。
- `tools/check-project.ps1` 是当前唯一完整本地门禁入口。

## 4. 禁止事项

- 不得在 M0 后把研究报告目标直接标为 supported。
- 不得删除 Android edge cases。
- 不得绕过 `tools/check-project.ps1` 作为 closeout 证据。
- 不得把当前 HTTP 骨架描述成 Axum 已落地。

## 5. 治理同步

| 文件 | 要求 |
| --- | --- |
| `markdown/00-matrix-governance/module-tree.md` | 登记 `governance.roadmap` 和 `governance.research` |
| `markdown/10-overview/overview-full-feature-tree.md` | 登记 roadmap 目录 |
| `markdown/General_Policy.md` | 登记防回退和里程碑规则 |
| `markdown/00-matrix-governance/standard-matrix.md` | 登记 roadmap/closeout 标准 |

## 6. 验收

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1
```

必须通过 Rust tests、frontend checks 和治理脚手架检查。

## 7. 进入 M1 条件

- M0 文件全部存在。
- 能力晋级台账中现有 supported 能力已列出。
- 决策门 DG-002 的影响范围已明确：如果未关闭，M1 只能按当前 Android 范围强化。

