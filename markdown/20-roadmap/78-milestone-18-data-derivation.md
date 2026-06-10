# M18: Data Derivation

## 1. 目标

实现从本地案例库聚合衍生统计数据。全部本地计算，最小聚合阈值 5 条记录，不暴露个体数据。

## 2. 依赖

- M5 case-management 本地案例存取。
- M17 case-export 已实现案例数据导出。

## 3. 范围

| Work Package | 内容 |
| --- | --- |
| M18-WP1 | 五行分布统计：所有案例的五行 weight 聚合 |
| M18-WP2 | 十神分布统计：所有案例的十神 weight 聚合 |
| M18-WP3 | 日主频率：日干出现次数分布 |
| M18-WP4 | 隐私保护：聚合最小阈值 5，低于阈值不输出 |
| M18-WP5 | `GET /api/data/derive?type=elements|ten_gods|day_masters` |

## 4. 非目标

- 不导出个体案例数据
- 不提供自定义查询语言
- 不引入第三方分析库

## 5. 能力状态

| Capability | Before | After |
| --- | --- | --- |
| `data-derivation` | restricted (stub) | restricted (real) |
