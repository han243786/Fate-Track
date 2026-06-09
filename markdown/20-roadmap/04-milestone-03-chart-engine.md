# M3: Chart Engine

## 1. 目标

实现 V1 四柱排盘核心：年柱、月柱、日柱、时柱、未知时辰处理、边界元数据和可复现命盘结果。

## 2. 依赖

- M2 规则档与 ChartBasis 完成。
- DG-001、DG-003 必须关闭。
- DG-004 已关闭：M3 不开放农历输入。
- DG-007 已关闭：M3 保留当前 HTTP 骨架，不迁移 Axum。

## 3. 范围

| Work Package | 内容 |
| --- | --- |
| M3-WP1 | 年柱按 `ft-v1-default` 年界计算 |
| M3-WP2 | 月柱按节令/月令计算 |
| M3-WP3 | 日柱按默认日界计算，保留 Android/JDN 锚点差异登记 |
| M3-WP4 | 时柱按双小时段与五鼠遁计算 |
| M3-WP5 | 未知时辰返回 hour null 和候选/稳定性摘要 |
| M3-WP6 | Chart API 返回完整 metadata、warnings、ambiguity flags |
| M3-WP7 | 黄金样例覆盖立春、清明、春节、闰月、甲子日、未知时辰 |

## 4. 非目标

- 不实现解释性分析。
- 不实现大运。
- 不实现案例持久化。
- 不实现分享。

## 5. 能力状态

| Capability | Before | After |
| --- | --- | --- |
| `chart-create` | planned | supported |
| `chart-detail` | planned | supported 或 restricted |
| `analysis-snapshot` | planned | planned |

## 6. 防回退

- 四柱结果必须携带 `ruleset_id` 和 `algo_version`。
- 不能用默认中午处理未知时辰。
- 边界日不能只用日期近似，必须有规则说明；若尚无精确节气即时刻，必须标注限制。
- 任何与 Android 日期层三柱不一致的变化必须登记差异和原因。

## 7. 治理同步

- `93-capability-promotion-ledger.md` 晋级 `chart-create`。
- `module-tree.md` 登记新 engine public surface。
- `overview-full-feature-tree.md` 登记新增文件。
- `README.md` 更新 supported API。
- 如命盘规则 accepted，更新 ADR。

## 8. 验收

- Rust tests 覆盖四柱样例。
- API contract tests 覆盖正常、未知时辰、非法时间、边界警告。
- 未知时辰必须返回 `hour: null` 和候选时柱，不得默认中午。
- `/api/capabilities` 状态与台账一致。
- 前端若尚未实现，只能显示 planned/restricted。
- `tools/check-project.ps1` 通过。

## 9. 进入 M4 条件

- Day Master 可稳定取得。
- Pillar/StemBranch/HiddenStem 所需结构稳定。
- 安全解释政策已读并纳入分析输出设计。
