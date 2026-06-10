# M20: Golden Rows + Replay Tests

## 1. 目标

物化天文黄金样例并执行重放测试，验证天文引擎可复现性。

## 2. 依赖

- M19 对照报告已完成。
- M11 天文引擎已就绪。

## 3. 范围

| Work Package | 内容 |
| --- | --- |
| M20-WP1 | 黄金样例：1901-2100-boundary, 2033-anomaly, lichun-boundary, qingming-boundary, jiazi-day-anchor, near-midnight 六类 |
| M20-WP2 | 重放测试：使用旧 Android 快照验证新引擎不改变历史结果 |
| M20-WP3 | 黄金样例 JSON + sha256 |

## 4. 非目标

- 不替换 Android 基线
- 不扩大日期范围

## 5. 能力状态

`astronomy-engine` 保持 target。
