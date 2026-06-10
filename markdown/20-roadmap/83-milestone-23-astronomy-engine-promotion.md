# M23: Astronomy Engine Promotion

## 1. 目标

将 `astronomy-engine` 从 target 晋级为 supported。天文计算引擎已全部实现，生成数据已产出，Android 对照已完成且零差异。剩余工作仅为治理收口：replacement ADR、运行时集成决策、能力晋级台账更新。

## 2. 依赖

- M11 天文引擎已实现：`backend/src/astronomy/` 含太阳（Meeus 标准）、月球（简化理论 ~60 项）、节气（视黄经二分法）、农历推导（GB/T 置闰规则）、对照引擎。
- M19 对照引擎已执行：Android vs 天文引擎日柱对照 1598 样本，0 差异。
- M20 黄金样例 + 重放测试已完成。
- 生成数据已产出：`data/generated/astronomy/out/solar-terms-1901-2100.json`（4800 节气）、`data/generated/astronomy/out/new-moons-1901-2100.json`（2474 新月）、`data/generated/astronomy/out/lunar-calendar-1901-2100.json`（2474 农历月）。

## 3. 范围

| Work Package | 内容 |
| --- | --- |
| M23-WP1 | 撰写 replacement ADR：明确天文引擎替代 Android 日期层的条件、范围和回退策略 |
| M23-WP2 | 运行时集成决策：是否将 `backend.astronomy` 接入 `/api/calendar/query` 或保持并行只读对照 |
| M23-WP3 | 能力晋级：`astronomy-engine` 从 target → supported，同步 `capabilities.rs`、capability ledger、模块树、v1-closeout、README |
| M23-WP4 | 回归保护：`cargo test astronomy` 17 项测试保持全绿；对照报告保持 0 差异 |

## 4. 非目标

- 不扩大验证范围（保持 1901-2100）
- 不替换 Android 日期层为运行时默认（除非 ADR 明确授权）
- 不引入真太阳时（仍标 unsupported）
- 不引入 IANA 时区历史（仍标 not resolved）
- 不引入星历范围外计算

## 5. 能力状态

| Capability | Before | After |
| --- | --- | --- |
| `astronomy-engine` | target | supported |

晋级条件：
1. ADR 登记 replacement 策略（含回退规则）。
2. `cargo test astronomy` 全部通过。
3. `tools/check-astronomy-preflight.ps1` 通过。
4. Android 对照 1598 样本保持 0 差异。
5. `/api/capabilities` 新增 `astronomy-engine` 声明。
6. capability ledger §1 新增条目。

## 6. 验证

```powershell
cargo test astronomy              # 17 passed
cargo test                        # 全部 Rust 测试
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-astronomy-preflight.ps1 -ProjectRoot .
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-project.ps1
```

## 7. 约束

- LOCK-001：不得删除或弱化 Android 日期层三柱黄金样例。
- LOCK-002：必须满足 capability ledger 全部条件才可标 supported。
- DG-008 已关闭，replacement 必须另有 ADR。
- 不得在 ADR 未登记前静默替换 Android 基线。
