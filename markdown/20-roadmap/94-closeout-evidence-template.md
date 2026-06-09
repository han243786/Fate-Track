# 里程碑 Closeout 证据模板

> 每个里程碑关闭时复制本模板，填入对应 closeout 文件或 PR 描述。没有 closeout 证据，不得进入下一阶段实现。

```markdown
# Milestone Closeout: M?

## 1. Scope

**Milestone**:
**Implemented scope**:
**Explicit non-goals**:

## 2. Capability Status

| Capability | Before | After | Evidence |
| --- | --- | --- | --- |

## 3. Recursive Cursor

| Field | Value |
| --- | --- |
| `loop_id` | |
| `mode` | |
| `cursor_before` | |
| `cursor_after` | |
| `next_resume_instruction` | |

## 4. Governance Sync

| Document | Updated? | Notes |
| --- | --- | --- |
| `markdown/20-roadmap/*` | yes/no | |
| `markdown/00-matrix-governance/module-tree.md` | yes/no | |
| `markdown/10-overview/overview-full-feature-tree.md` | yes/no | |
| `markdown/General_Policy.md` | yes/no | |
| `markdown/00-matrix-governance/standard-matrix.md` | yes/no | |
| `README.md` | yes/no | |
| ADR / research intake | yes/no | |
| recursive cursor / loop log | yes/no | |

## 5. Validation

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1
```

**Result**:

## 6. Regression Protection

| Risk | Protection |
| --- | --- |

## 7. Remaining Risks

| Risk | Severity | Next action |
| --- | --- | --- |

## 8. Next Milestone Entry Check

| Required condition | Met? |
| --- | --- |
```
