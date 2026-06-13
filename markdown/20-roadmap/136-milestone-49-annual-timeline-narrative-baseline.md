# M49 Annual Timeline Narrative Baseline

Status: closed by LOOP-117. Closeout: `137-milestone-49-closeout.md`.

## 1. 目标

M49 承接 M47/M48 的报告文案基线，继续打磨五份真实报告里最影响读感的大运流年段落。M48 后，金钱、家庭、事业已经不再露出十神计数字段，但主盘年度引动和三专项 `本专题的大运流年` 仍有清单味，例如 `盘面上先看这几股牵动`、`2026年的主要牵动如下` 和项目符号证据。

本轮把主盘年度引动、金钱/家庭/事业专题大运流年从清单式证据改成连贯读盘顺序。情感报告仍保持 M47 黄金样例，不新增独立 topic timeline block。

## 2. 叙事基线

年度/大运流年段落必须保持：

- 用显式年份，例如 `2026年`，不得回退为 `指定年份`、`这一年` 或泛化年份。
- 证据以读盘顺序展开：先看天干，再看五行关系，再看藏干底色，再看地支关系，最后看当前大运是否参与。
- 不再输出项目符号证据、`主要牵动如下`、`盘面上先看这几股牵动` 或 `这些牵动只说明`。
- 四专项要把年度线索落回专题观察点：金钱看预算、产出、分配与规则；家庭看支持、边界、表达与责任；事业看责任、技能、资源与协作；情感继续保持关系节奏建议。
- 不得把年度引动包装成现实事件、收益、亲属命运、岗位成败、婚恋结果、流月流日或每日运势。

## 3. 验收

- 重新生成 `target/report-polish-samples/main.txt`、`relationship.txt`、`wealth.txt`、`family.txt`、`career.txt` 及对应 JSON 样本。
- 五份 `.txt` 样本对 M49 禁用词扫描为 0：`主要牵动如下`、`盘面上先看这几股牵动`、`这些牵动只说明`、`· 先看天干`、`· 流年`、`score_internal`。
- 五份 JSON 样本 top-level `forbidden_output_audit.status` 均为 `passed`。
- `cargo test topic_report -- --nocapture`、`cargo test report -- --nocapture`、`cargo test relationship -- --nocapture` 通过。
- 治理脚手架通过。
