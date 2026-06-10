# M19: Astronomy Comparison

## 1. 目标

执行 Android 日期层 vs 天文引擎的结构化对照，分类差异，不替换运行时基线。

## 2. 依赖

- M11 天文引擎已生成 4800 节气 + 2474 朔 + 2474 农历月。
- M1 Android 日期层已有 49 黄金样例。

## 3. 范围

| Work Package | 内容 |
| --- | --- |
| M19-WP1 | 对照框架：逐日/逐月比较 Android 与天文引擎输出 |
| M19-WP2 | 差异分类：android_table_difference / astronomy_source_difference / ruleset_difference / unresolved |
| M19-WP3 | 填充 `out/android-comparison-1901-2100.json` |
| M19-WP4 | 对照报告摘要 |

## 4. 非目标

- 不替换 Android 日期层
- 不扩大日期范围（1901-2100）
- 不引入外部 API 调用

## 5. 能力状态

`astronomy-engine` 保持 target。
