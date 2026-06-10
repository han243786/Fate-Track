# M17: Case Export + Analysis Report

## 1. 目标

实现案例导出和分析报告生成。全部本地计算，不依赖网络。导出 JSON 格式含完整命盘快照、分析结果和可选私密备注。

## 2. 依赖

- M5 case-management 已提供本地案例存取。
- M4 analysis-snapshot 已提供结构化分析。
- M12 chart-detail 已提供命盘快照。

## 3. 范围

| Work Package | 内容 |
| --- | --- |
| M17-WP1 | `GET /api/cases/export?id=xxx` 完整实现，返回 JSON 含 chart snapshot + analysis snapshot + metadata |
| M17-WP2 | 分析报告生成器：聚合五行/十神/藏干指标为可读文本摘要 |
| M17-WP3 | `include_notes=true` 可选包含私密备注 |
| M17-WP4 | API 测试 + 隐私校验（不导出其他案例数据） |

## 4. 非目标

- 不导出 PDF/富格式（JSON 纯文本）
- 不批量导出
- 不导入
- 不涉及网络调用

## 5. 能力状态

| Capability | Before | After |
| --- | --- | --- |
| `case-export` | restricted (stub) | restricted (real) |
