# 决策门

> 决策门用于阻止“边想边做”造成回退。未关闭的决策门不得被代码实现静默绕过。

## 1. 决策门状态

| Gate ID | 问题 | 当前状态 | 最晚关闭点 | 未关闭时的执行限制 |
| --- | --- | --- | --- | --- |
| DG-001 | V1 默认规则档是否正式命名为 `ft-v1-default` | closed: `ft-v1-default` | M2 开始前 | `chart-create` 仍不得标 supported |
| DG-002 | V1 官方验证范围采用 `1901-2100` 还是更宽 | closed: `1901-2100` | M1 关闭前 | 范围外保持 unsupported；更宽范围必须走 M9 星历/天文升级 |
| DG-003 | 日界默认是否固定为 `00:00`，子初是否仅作高级选项 | closed: `00:00`, Zi-start planned | M2 关闭前 | 不得实现多日界 UI 宣称 |
| DG-004 | V1 是否直接开放农历输入 | closed: no direct lunar input in M2 | M2 关闭前 | 农历输入能力保持 planned |
| DG-005 | 大运顺逆与起运年龄默认规则 | closed: ADR 0020 yang-year+male forward, birth-to-jie/3 | M4 开始前 | 大运实现完成 |
| DG-006 | 服务端持久化、匿名保存、账号和云同步范围 | closed for M5: local volatile only | M5 开始前 | 不得宣称账号、数据库持久化、云同步或跨设备同步 |
| DG-007 | 是否迁移 Axum + 多 crate 架构 | closed for M3: keep current HTTP skeleton | M3 或 M9 前按实际选择 | 不得假装 Axum 已落地；如迁移需独立 ADR |
| DG-008 | 星历/天文引擎何时替代或并行 Android 日期层 | closed for M9 preflight: parallel first by ADR 0015 | M9 开始前 | Android 日期层保持 accepted-current；替换必须另有 ADR 和双引擎证据 |
| DG-009 | share token storage, expiration, revocation, and noindex policy | closed for M6: local volatile hashed token + noindex public DTO | before M6 implementation | share must not claim permanent public links, accounts, database persistence, cloud sync, cross-device sync, or enumerable public directory |
| DG-010 | 分析文案是否允许生成式扩写 | closed: structured fixed-template only | M4 开始前 | 不得返回自由生成长文或高风险确定性断言 |

## 2. 决策记录格式

```markdown
## Gate ID: DG-XXX

**Decision**:
**Chosen option**:
**Rejected options**:
**Reason**:
**Impacted modules**:
**Policy clauses**:
**Required tests**:
**Docs to update**:
**Rollback rule**:
```

## 3. 关闭规则

- 决策门必须落入 ADR、研究纳入台账或里程碑 closeout。
- 影响 API、数据、隐私、安全、发布的决策门必须更新 General Policy 或 Standard Matrix。
- 关闭决策门后，相关里程碑文件必须同步状态，不得只在对话中口头确认。
