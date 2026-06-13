# M44 Relationship Copy Second-Pass Gate

Status: closed by LOOP-112. Closeout: `127-milestone-44-closeout.md`.

## 1. 目标

M44 承接用户对情感报告真实输出的二次反馈：

- 情感报告开头不能每次都落在同一句“您的情感线并不是没有缘分，而是关系一旦真正靠近，往往会带着比较强的牵动感”。
- 解释合冲刑害时，术语必须显性标注；例如 `如果被冲牵动` 必须改为 `如果被"冲"牵动`。

本轮仍是质量门禁，不新增计算规则、不新增 API、不改变 capability 状态。核心目标是把 M41/M42 的情感六块正文继续打磨成人读起来不模板、不生硬、不暴露系统口径的报告，并把同类术语裸露问题同步纳入主盘和专题可见正文门禁。

## 2. 输入样本

本轮以 `target/report-polish-samples/` 下重新生成的真实输出为审读对象：

- `relationship.txt`
- `main.txt`
- `wealth.txt`
- `family.txt`
- `career.txt`

第一优先级是 `relationship.txt`。其他四份样本只做同类回归扫描，防止合冲刑害等术语在可见证据里裸奔或退回机器口吻。

## 3. 范围

### 允许

- 修改 `relationship-report` 总断开头，使开场根据夫妻宫关系、配偶星明显程度和结构强弱出现不同表达。
- 修改夫妻宫、年度引动、证据文本中的合冲刑害术语展示，使 `冲`、`合`、`刑害`、`六冲`、`六合`、`三刑`、`六害`、`自刑` 等专业词在解释句里带引号。
- 新增 app/domain 门禁，禁止固定开头、裸露 `被冲牵动` / `被合牵动` / `被刑害牵动`、`形成六冲` / `形成自刑` 等回归。
- 顺手清理仍明显僵硬的口吻，如 `本次输入`、`本报告只做`、`结构敏感性：`、`目前可追溯证据如下`、`运运`。

### 禁止

- 不改变 `relationship-report` 的 M41 六块正文结构。
- 不新增 route、DTO 顶层字段或 public surface。
- 不改变 `/api/capabilities`。
- 不改变 raw `GET /api/luck/cycles` supported 语义。
- 不公开 `score_internal`、0-100 分或排序分。
- 不输出确定性婚恋事件、伴侣身份、分合结论或高风险现实决策建议。
- 不把 M44 扩展成新词典工程、流月/流日工程或 supported promotion。

## 4. 文案门禁

五份 assembled report 不得出现：

- `您的情感线并不是没有缘分`
- `被冲牵动`
- `被合牵动`
- `被刑害牵动`
- `形成六冲`
- `形成六合`
- `形成三刑`
- `形成六害`
- `形成自刑`
- `运运`
- `本次输入`
- `本报告只做`
- `结构敏感性：`
- `目前可追溯证据如下`
- `结构依据`
- `timeline-core-v1`
- `annual-trigger`
- `score_internal`
- `\d{4}\s+年` 形式的年份空格

情感报告仍必须保留：

- M41 六块正文顺序：`总断`、`伴侣议题`、`夫妻宫`、`表达、边界与安全感`、`年度情感引动`、`结论`。
- 开头一次提醒，不重复制造风险提示。
- 专业术语与通俗解释并存。
- 低风险关系节奏建议可保留，但不得进入确定性事件判断。

## 5. 验收

- `relationship.txt` 的固定开头消失，开场能随结构变化。
- `relationship.txt` 出现 `被"冲"牵动`、`形成"六冲"`、`形成"自刑"` 等引号化术语表达。
- 五份样本不得出现 `2026 年` 这类年份空格，必须使用 `2026年` 一类自然中文排版。
- 五份真实样本重新生成后，M44 禁用口吻命中为 0。
- 五份样本 ASCII word 命中为 0，assembled report 不暴露内部英文。
- 全量治理门禁通过：`cargo test -- --nocapture`、`npm.cmd run check --prefix frontend`、`tools/check-project.ps1`、`cargo check -p minggui-desktop`、`tools/check-governance-scaffold.ps1 -ProjectRoot .`。

## 6. Closeout 要求

关闭 M44 时至少同步：

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
