# M45 Report System-Tone Cleanup Gate

Status: closed by LOOP-113. Closeout: `129-milestone-45-closeout.md`.

## 1. 目标

M45 承接五份真实报告的轮回打磨 goal：主盘、情感、金钱、家庭、事业 assembled report 必须继续基于真实生成结果审读，避免“系统说明”“算法变量”“计数表格”进入用户正文。

本轮重点清理：

- 主盘报告中的 `当前算法`、`系统给出`、`综合评分`、`置信度是算法`、`第1运` 等系统/工程口吻。
- 四专项报告中的 `相关信号共`、`未见明显显性信号`、`共找到`、`今年最值得留意`、`降级参考` 等机器化表达。
- timeline 证据中的大运阶段标签，使 `第1运丙子` 一类连写转为 `第一运·丙子` 这类可读标签。

本轮只改报告正文与质量门禁，不新增计算规则、不新增 route、不改变 capability 状态。

## 2. 范围

### 允许

- 改写 `chart-report` 可见正文中的强弱、格局、用神、大运与年度引动描述。
- 改写 `topic-report` 可见正文中的十神分组摘要、敏感性说明和专题大运流年章节。
- 改写 `timeline` 输出标签，使报告证据更适合面向用户阅读。
- 扩展 app/domain 文案门禁，禁止系统口吻、机器计数、后端变量和阿拉伯序号式大运标签回归。
- 重新生成五份真实样本并以样本作为验收依据。

### 禁止

- 不改变 `/api/capabilities`。
- 不新增 DTO 顶层字段、路由或 public surface。
- 不改变 raw `GET /api/luck/cycles` supported 语义。
- 不公开 `score_internal`、0-100 分或排序分。
- 不宣称完整流月、流日、择日、每日推送或事件预测。
- 不把金钱、家庭、事业正文变成现实财务、亲属命运或职业结果建议。

## 3. 文案门禁

五份真实 assembled report 不得出现：

- `当前算法`
- `系统给出`
- `综合评分`
- `相关信号共`
- `未见明显显性信号`
- `这份盘面目前没有触发`
- `降级参考`
- `共找到`
- `今年最值得留意`
- `当前报告没有收到`
- `系统再解释`
- `当前可计算`
- `置信度是算法`
- `\([0-9]+分\)` 形式的十神分数
- `第[0-9]+运`、`第[0-9]+优先`、`第[0-9]+个`
- 内部英文 capability id、rule id、`score_internal`

## 4. 验收

- `target/report-polish-samples/main.txt`、`relationship.txt`、`wealth.txt`、`family.txt`、`career.txt` 重新生成。
- 五份样本禁用口吻命中为 0。
- 五份样本 ASCII word 命中为 0。
- 主盘报告强弱/格局/用神/大运不再使用系统或算法说明口吻。
- 四专项十神摘要使用自然中文计数，例如 `财星在盘中有一处线索，正财暂不明显，偏财一处可见`。
- 专题 timeline 使用 `2026年重点看的牵动`，不再使用 `今年最值得留意`。
- 大运证据标签使用 `第一运·丙子` 等可读形式。
- 全量门禁通过：`cargo test -- --nocapture`、`npm.cmd run check --prefix frontend`、`cargo check -p minggui-desktop`、`tools/check-governance-scaffold.ps1`、`tools/check-project.ps1`。

## 5. Closeout 要求

关闭 M45 时必须同步：

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
