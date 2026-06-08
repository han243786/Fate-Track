# 命轨矩阵治理入口

本目录接入 `D:\Skill_Shelf\heavy-scale-exploitation-governance-1`，作为命轨项目的治理入口。当前项目状态按 `mid_project/minimal` 接入：先记录现实，再逐步把低噪声规则升为阻断门禁。

## 1. Matrix Roles

| Matrix | File | Owns |
| --- | --- | --- |
| Process matrix | `process-matrix.md` | 需求、实现、验证、审计、closeout 的推进方式 |
| Standard matrix | `standard-matrix.md` | 硬规则、策略来源、禁止模式和对应门禁 |
| Guidance matrix | `guidance-matrix.md` | 变更先读哪些树、影响哪些模块、要交什么证据 |
| Module tree | `module-tree.md` | Rust 后端、JS 前端、农历数据、治理脚本的逻辑边界 |

## 2. Project Baseline

| Item | Current Decision |
| --- | --- |
| Backend | Rust workspace；入口 `backend/src/main.rs` |
| Frontend | 原生 JavaScript；入口 `frontend/index.html` 与 `frontend/src/main.js` |
| Lunar data | 项目副本 `data/raw/lunar_data.yaml`，来源 `D:\myproject\Perpetual calendar\data\yaml\lunar_data.yaml` |
| Product map | `markdown/命轨全量树.md` |
| Engineering tree | `markdown/10-overview/overview-full-feature-tree.md` |

## 3. Required Change Declaration

每个非平凡变更必须声明：

- change level: light, standard, heavy
- impacted full-tree nodes
- impacted module-tree nodes
- impacted policy clauses
- validation commands
- docs to update
- residual risks

## 4. Closeout Rule

如果代码、测试、文档、产品树、模块树或治理矩阵互相矛盾，本次变更不能 closeout。

