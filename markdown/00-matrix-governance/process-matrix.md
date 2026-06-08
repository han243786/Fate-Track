# 命轨 Process Matrix

## 1. State Machine

| State | Entry | Exit |
| --- | --- | --- |
| proposed | 需求进入 | 影响范围和档位已声明 |
| designed | 提案足够清晰 | 风险、测试、文档、模块影响已知 |
| implementing | 代码/文档/测试修改中 | 本地门禁通过 |
| validating | 实现完成 | Rust/JS/治理检查通过 |
| auditing | 验证完成 | S0 清零，P1 已登记 |
| closing | 里程碑或交付候选 | closeout 完成 |

## 2. Level Decision

| Trigger | Required Level |
| --- | --- |
| docs-only, no behavior | light |
| Rust API、JS UI、测试、数据、模块所有权变化 | standard |
| 架构、安全、迁移、发布、跨模块、不可逆数据变化 | heavy |

## 3. Proposal Checklist

- Problem.
- Non-goals.
- Impacted users.
- Impacted modules.
- Impacted files.
- Policy clauses.
- Validation commands.
- Rollback or refusal behavior.

## 4. Current Validation Set

```powershell
cargo fmt --check
cargo check
node --check frontend/server.mjs
node --check frontend/src/main.js
powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-governance-scaffold.ps1 -ProjectRoot .
```

