# 命轨模块树

> 逻辑白箱所有权图，不是文件树。文件树见 `markdown/10-overview/overview-full-feature-tree.md`。

## 1. Root Domains

| Domain | Owns |
| --- | --- |
| `system` | workspace、运行入口、构建命令、交付说明 |
| `backend` | Rust API、服务边界、农历数据读取、未来排盘能力 |
| `frontend` | JS UI、浏览器状态、API 结果展示 |
| `data` | 原始数据、衍生产物、数据生命周期 |
| `governance` | 全量树、模块树、策略矩阵、流程矩阵、ADR |
| `tools` | 脚手架检查、清单生成、未来门禁脚本 |

## 2. Active Modules

### Module ID: `system.workspace`

**Real files**:
- `.gitignore`
- `Cargo.lock`
- `Cargo.toml`
- `README.md`

**Responsibility**: 定义 Rust workspace、项目入口、运行命令和基础检查命令。

**Public surface**:
| Surface | Input | Output | Caller | Forbidden |
| --- | --- | --- | --- | --- |
| `cargo run -p minggui-backend` | workspace | backend process | developer | 不得依赖未登记数据路径 |

**Regression protection**: `cargo check`。

### Module ID: `backend.api`

**Real files**:
- `backend/Cargo.toml`
- `backend/src/main.rs`

**Responsibility**: 提供当前最小 Rust 后端 API，读取项目内农历数据元信息。

**Inputs**:
| Input | Source | Type | Constraints |
| --- | --- | --- | --- |
| `FT_BACKEND_ADDR` | environment | string | 可选，默认 `127.0.0.1:8787` |
| `FT_LUNAR_DATA_PATH` | environment | path | 可选，默认 `data/raw/lunar_data.yaml` |
| `data/raw/lunar_data.yaml` | `data.lunar.raw` | YAML | 只读 |

**Outputs**:
| Output | Destination | Type | Constraints |
| --- | --- | --- | --- |
| `/api/health` | frontend/developer | JSON | 必须返回服务状态 |
| `/api/lunar-data/meta` | frontend/developer | JSON | 必须只暴露元信息，不改写数据 |

**Public surface**:
| Surface | Input | Output | Caller | Forbidden |
| --- | --- | --- | --- | --- |
| `GET /api/health` | HTTP | status JSON | `frontend.console` | 不得读取敏感信息 |
| `GET /api/lunar-data/meta` | HTTP | data meta JSON | `frontend.console` | 不得把缺失数据伪装为成功 |

**Parent communication rule**: 前端只能通过 HTTP API 访问后端能力。

**Forbidden lateral links**: 不得直接读取 `frontend/`；不得写入 `data/raw/`。

**Regression protection**: `cargo fmt --check`、`cargo check`。

### Module ID: `frontend.console`

**Real files**:
- `frontend/index.html`
- `frontend/server.mjs`
- `frontend/package.json`
- `frontend/src/main.js`
- `frontend/src/styles.css`

**Responsibility**: 提供浏览器端控制台，连接 Rust 后端并展示农历数据状态。

**Inputs**:
| Input | Source | Type | Constraints |
| --- | --- | --- | --- |
| API base URL | user/localStorage | URL | 默认 `http://127.0.0.1:8787` |
| backend API JSON | `backend.api` | JSON | 必须处理失败状态 |

**Outputs**:
| Output | Destination | Type | Constraints |
| --- | --- | --- | --- |
| UI state | browser | DOM | 不得宣称未实现排盘能力 |

**Public surface**:
| Surface | Input | Output | Caller | Forbidden |
| --- | --- | --- | --- | --- |
| `node server.mjs` | local files | static web server | developer | 不得代理或泄露本机任意文件 |
| `refresh()` | API base | DOM update | UI button | 不得吞掉 API 错误 |

**Parent communication rule**: 只经 `backend.api` 获取后端能力。

**Forbidden lateral links**: 不得直接读取 `data/raw/`；不得在 UI 中把 planned 能力写成 supported。

**Regression protection**: `node --check frontend/server.mjs`、`node --check frontend/src/main.js`。

### Module ID: `data.lunar.raw`

**Real files**:
- `data/README.md`
- `data/raw/lunar_data.yaml`

**Responsibility**: 保存农历基础数据原始副本，作为第一阶段农历能力真源。

**Inputs**:
| Input | Source | Type | Constraints |
| --- | --- | --- | --- |
| source copy | `D:\myproject\Perpetual calendar\data\yaml\lunar_data.yaml` | YAML | 必须登记来源 |

**Outputs**:
| Output | Destination | Type | Constraints |
| --- | --- | --- | --- |
| lunar skip table | `backend.api` | YAML text | 后端只读 |

**Public surface**:
| Surface | Input | Output | Caller | Forbidden |
| --- | --- | --- | --- | --- |
| raw YAML file | filesystem | lunar data | backend/data tools | 应用运行时不得修改 |

**Parent communication rule**: 所有衍生数据必须新增 `data.lunar.derived` 节点。

**Forbidden lateral links**: 前端不得直接消费 raw YAML。

**Regression protection**: 后端元信息 API 必须能读出版本、年份范围和节气数量。

### Module ID: `governance.matrix`

**Real files**:
- `markdown/00-matrix-governance/README.md`
- `markdown/00-matrix-governance/process-matrix.md`
- `markdown/00-matrix-governance/standard-matrix.md`
- `markdown/00-matrix-governance/guidance-matrix.md`
- `markdown/00-matrix-governance/module-tree.md`
- `markdown/General_Policy.md`
- `markdown/01-principles/principles-super-standardization.md`
- `markdown/10-overview/overview-full-feature-tree.md`
- `markdown/命轨全量树.md`
- `docs/decisions/0001-stack-and-data-source.md`

**Responsibility**: 维护项目真实结构、产品范围、实现规则、流程规则和决策记录。

**Public surface**:
| Surface | Input | Output | Caller | Forbidden |
| --- | --- | --- | --- | --- |
| governance docs | change scope | required evidence | developer/reviewer | 不得与真实代码和数据冲突 |

**Regression protection**: `tools/check-governance-scaffold.ps1`。

### Module ID: `tools.governance`

**Real files**:
- `tools/check-governance-scaffold.ps1`
- `tools/inventory-project.ps1`

**Responsibility**: 提供治理脚手架检查和工程目录盘点。

**Public surface**:
| Surface | Input | Output | Caller | Forbidden |
| --- | --- | --- | --- | --- |
| `check-governance-scaffold.ps1` | project root | pass/fail | developer/CI | 不得跳过必需治理文件 |
| `inventory-project.ps1` | project root | full tree draft | developer | 不得扫描生成物、缓存和依赖目录 |

**Regression protection**: `powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-governance-scaffold.ps1 -ProjectRoot .`。
