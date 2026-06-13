# M47 Relationship Golden Sample Baseline

Status: closed by LOOP-115. Closeout: `133-milestone-47-closeout.md`.

## 1. 目标

M47 承接“先以情感报告作为黄金样例”的轮回打磨目标。上一轮 M46 已经把 `出现几处` 这类计数台账改成较自然的中文，但真实情感样本中仍可见 `不作主线`、`有一处落点` 等规则计数字段。M47 将情感报告再提升一层：用户可见正文不展示计数表述，而是把配偶星、夫妻宫、表达、边界、安全感和年度引动翻译成命理师式解释。

本轮只处理 `relationship-report` 的可见正文、样本和回归门禁，不改变 API route、DTO 顶层结构、capability 状态或 raw `GET /api/luck/cycles` 语义。

## 2. 黄金样例标准

情感报告必须保持：

- M41 六块正文顺序：`总断`、`伴侣议题`、`夫妻宫`、`表达、边界与安全感`、`年度情感引动`、`结论`。
- 开头只保留一次现实边界提醒，正文继续围绕相处节奏、沟通方式、边界意识和稳定回应展开。
- 伴侣议题不再显示 `正财不作主线`、`偏财有一处落点`、`食伤有一处落点` 等计数字段。
- 十神信号必须转成关系质感：吸引、承诺、压力、边界、安全感、现实承接。
- `冲`、`合`、`刑害`、`六冲`、`自刑` 等关系术语在解释句和证据句中保持引号化。
- 年份必须使用显式年份，例如 `2026年`，不得回退为 `指定年份`、`这一年` 或 `2026 年`。
- 不得公开 `score_internal`、0-100 分、内部英文、后端变量、算法/系统口吻、确定性婚恋事件或伴侣身份断语。

## 3. 验收

- 重新生成 `target/report-polish-samples/relationship.txt` 和 `relationship.json`。
- `relationship.txt` 对 M47 禁用词扫描为 0：`不作主线`、`有一处落点`、`有两处落点`、`有三处落点`、旧固定开头、裸露 `被冲牵动`、`score_internal`。
- 五份真实报告样本继续通过 M45/M46 公共禁用词扫描。
- 五份 JSON 样本 top-level `forbidden_output_audit.status` 均为 `passed`。
- `cargo test relationship -- --nocapture`、`cargo test topic_report -- --nocapture`、`cargo test report -- --nocapture` 通过。
- 治理脚手架通过。
