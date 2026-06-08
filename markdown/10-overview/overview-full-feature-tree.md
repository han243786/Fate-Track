# 命轨全量工程树

> 来源：`heavy-scale-exploitation-governance-1` 接入后按当前真实目录整理。本文是工程目录地图，产品功能地图保留在 `markdown/命轨全量树.md`。

## Project Root

```text
FT/
  backend/
    src/
      main.rs
    Cargo.toml
  data/
    raw/
      lunar_data.yaml
    README.md
  docs/
    decisions/
      0001-stack-and-data-source.md
  frontend/
    src/
      main.js
      styles.css
    index.html
    package.json
    server.mjs
  markdown/
    00-matrix-governance/
      guidance-matrix.md
      module-tree.md
      process-matrix.md
      README.md
      standard-matrix.md
    01-principles/
      principles-super-standardization.md
    10-overview/
      overview-full-feature-tree.md
    templates/
      closeout-template.md
      proposal-template.md
    General_Policy.md
    命轨全量树.md
  tools/
    check-governance-scaffold.ps1
    inventory-project.ps1
  .gitignore
  Cargo.lock
  Cargo.toml
  README.md
```

## Top-Level Responsibilities

| Path | Kind | Responsibility | Module Tree |
| --- | --- | --- | --- |
| `backend/` | directory | Rust 后端服务；当前提供健康检查与农历数据元信息 API。 | `backend.api` |
| `frontend/` | directory | JavaScript 前端；当前提供连接后端与展示农历数据状态的浏览器界面。 | `frontend.console` |
| `data/` | directory | 项目内数据区；`raw/` 保存外部农历基础数据的原始副本。 | `data.lunar.raw` |
| `docs/` | directory | 架构决策记录。 | `governance.decisions` |
| `markdown/` | directory | 治理体系、产品全量树、策略矩阵、流程矩阵和模板。 | `governance.matrix` |
| `tools/` | directory | 治理脚本；当前包含脚手架检查和工程清单生成。 | `tools.governance` |
| `.gitignore` | file | 排除 Rust/Node 构建产物、缓存和日志。 | `system.workspace` |
| `Cargo.lock` | file | Rust 应用依赖锁文件。 | `system.workspace` |
| `Cargo.toml` | file | Rust workspace 根配置。 | `system.workspace` |
| `README.md` | file | 项目入口、运行命令和基础检查命令。 | `system.workspace` |

## Source Classification

| Path | Class | Rule |
| --- | --- | --- |
| `backend/src/main.rs` | source | 必须通过 `cargo fmt --check` 与 `cargo check`。 |
| `frontend/src/main.js` | source | 必须通过 `node --check frontend/src/main.js`。 |
| `frontend/server.mjs` | source/tool | 必须通过 `node --check frontend/server.mjs`。 |
| `data/raw/lunar_data.yaml` | raw data | 禁止应用代码直接改写；衍生产物必须登记生成命令。 |
| `markdown/命轨全量树.md` | product map | 功能范围变化时同步更新。 |
| `markdown/00-matrix-governance/*` | governance | 模块、规则、门禁或流程变化时同步更新。 |
| `target/`, `node_modules/`, `dist/`, `.cache/` | generated/cache | 不进入治理全量树，不作为源文件评审对象。 |

## Follow-Up

- Rust 后端新增真实排盘/农历查询能力时，必须先补 `backend.calendar` 模块节点。
- JavaScript 前端新增页面或路由时，必须在 `frontend.console` 下补 UI 节点和后端能力来源。
- 农历数据导入数据库、生成 Rust 常量或生成 JSON 时，必须新增 `data.lunar.derived` 节点。
