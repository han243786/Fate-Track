# M48 Topic Count-Field Narrative Baseline

Status: closed by LOOP-116. Closeout: `135-milestone-48-closeout.md`.

## 1. 目标

M48 承接 M47 的情感黄金样例，把同类质量要求扩展到金钱、家庭、事业三个专项。上一轮 M47 已经禁止 `relationship-report` 将伴侣星、表达和安全感写成 `不作主线` / `有一处落点` 等计数字段；M48 要把剩余三专项里的同类字段也转成面向用户的命理解释。

本轮只处理 `wealth-report`、`family-report`、`career-report` 的可见正文、样本和回归门禁，同时顺手移除情感报告残留的 `这组结构说明` 承接。它不改变 API route、DTO 顶层结构、capability 状态、score 边界或 raw `GET /api/luck/cycles` 语义。

## 2. 叙事基线

金钱、家庭、事业报告必须保持：

- 不再向用户显示 `正财不作主线`、`偏财有一处落点`、`七杀有两处落点` 等计数字段。
- 十神线索要翻译成可读气质，例如 `偏财带出机会资源、外部流动和交换意识`，`七杀的分量更重，压力、挑战、边界考验和行动驱动会更容易被看见`。
- 允许保留专业词，但每段要说明该专业词在当前专题里的观察意义。
- 禁止 `参与这组结构`、`这组结构说明`、`落到这张盘上` 等中间层或模板承接回到可见正文。
- 金钱报告仍只讲资源模式，不输出收益、亏耗、金额、投资或借贷建议。
- 家庭报告仍只讲互动结构，不预测亲属健康、婚育、离合或家庭事件。
- 事业报告仍只讲责任、技能、资源和协作方式，不承诺岗位、收入、考试或职业结果。

## 3. 验收

- 重新生成 `target/report-polish-samples/main.txt`、`relationship.txt`、`wealth.txt`、`family.txt`、`career.txt` 及对应 JSON 样本。
- `relationship.txt`、`wealth.txt`、`family.txt`、`career.txt` 对 M48 禁用词扫描为 0：`不作主线`、`有一处落点`、`有两处落点`、`有三处落点`、`落到这张盘上`、`参与这组结构`、`这组结构说明`、`score_internal`。
- 五份 JSON 样本 top-level `forbidden_output_audit.status` 均为 `passed`。
- `cargo test topic_report -- --nocapture`、`cargo test report -- --nocapture`、`cargo test relationship -- --nocapture` 通过。
- 治理脚手架通过。
