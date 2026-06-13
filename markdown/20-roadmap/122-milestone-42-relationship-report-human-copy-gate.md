# M42 Relationship Report Human Copy Gate

`closed` by LOOP-110. M42 completed as a relationship-report copy-quality loop only; no route, DTO, capability, public score, or raw `luck-cycles` behavior changed.

## 1. 目标

在 M41 六块情感专项报告已经成形的基础上，继续只打磨 `relationship-report` 单一切面。M42 不新增计算规则、不新增 API、不改变 capability 状态；它只把真实生成样本中的机器口吻继续压下去，让情感报告更像命理师对用户的完整解读。

本轮必须先读取真实输出样本，再改模板，再用门禁锁住结果。

## 2. 输入样本

本轮以 `target/report-polish-samples/relationship.txt` 的真实输出为第一审读对象。当前可见问题：

- “关系主题会被标记为已引动”偏系统判定口吻。
- “共享时间线共筛出 5 条情感相关重点”偏内部筛选口吻。
- “当前提取结果”“当前关系”偏数据报告口吻。
- “本次资料足以支撑情感专项的基础阅读”偏审计说明口吻。

这些问题不属于计算错误，但会削弱用户阅读体验，必须进入质量门禁。

## 3. 范围

### 允许

- 继续保留 M41 的六块正文顺序：`总断`、`伴侣议题`、`夫妻宫`、`表达、边界与安全感`、`年度情感引动`、`结论`。
- 只改 `relationship-report` 的正文模板、条件说明和 app/domain 测试。
- 用更自然的命理师表达替换“筛选、标记、提取、当前关系”等系统口吻。
- 继续允许低风险关系节奏建议，例如稳定回应、沟通边界、相处节奏、现实承接。
- 将真实样本审读结果写入 closeout。

### 禁止

- 不新增 public route。
- 不改变 `GET /api/charts/topic-report` 合约。
- 不改变 `GET /api/luck/cycles` raw supported 语义。
- 不新增或晋级 capability。
- 不公开 `score_internal`、0-100 分、排序分。
- 不输出确定性婚恋事件、伴侣身份、结婚离婚断语或现实承诺。
- 不把本轮扩展到金钱、家庭、事业报告正文。

## 4. 文案门禁

`relationship-report` 正文不得出现以下机器口吻：

- `标记为已引动`
- `共享时间线`
- `筛出`
- `当前提取结果`
- `当前关系：`
- `基础阅读`
- `后端`
- `前端`
- `score_internal`
- `指定年份`
- `这一年`
- `读盘时`
- `主题里`

正文必须继续保留：

- 具体年份，如 `2026年`
- 年柱，如 `丙午`
- 当前大运
- 夫妻宫
- 配偶星
- 食伤、比劫、印星
- 稳定回应、清楚边界、现实承接等低风险关系建议

## 5. 验收

- `relationship-report` 仍为 restricted。
- API 响应仍有六块正文，且顺序不变。
- `topic-timeline-overlay` 仍保留在 `signals` / `trace` 证据层，但不成为情感报告独立正文块。
- 真实样本重新生成后，情感正文不含 M42 禁用口吻。
- 全量治理门禁通过：`cargo test -- --nocapture`、`npm.cmd run check --prefix frontend`、`tools/check-project.ps1`、`cargo check -p minggui-desktop`。

## 6. Closeout 要求

关闭 M42 时至少同步：

- `README.md`
- `docs/release/v1-closeout.md`
- `markdown/00-matrix-governance/module-tree.md`
- `markdown/10-overview/overview-full-feature-tree.md`
- `markdown/命轨全量树.md`
- `markdown/20-roadmap/00-roadmap-index.md`
- `markdown/20-roadmap/89-post-preview-documentation-freeze.md`
- `markdown/20-roadmap/92-risk-register.md`
- `markdown/20-roadmap/93-capability-promotion-ledger.md`
- `markdown/20-roadmap/96-recursive-cursor.md`
- `markdown/20-roadmap/97-loop-closeout-log.md`
- `markdown/20-roadmap/README.md`
