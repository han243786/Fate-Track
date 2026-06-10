# 命轨 · Fate Track

命轨是一套基于传统命理学的排盘与结构化分析工具，采用 Rust 后端 + 原生 JavaScript 前端的单体架构。项目的设计目标是：**只读排盘工作台，边界锁定，不做生成式推断。**

对外，它是一个可运行的八字排盘 Web 应用；对内，它附带一套完整的项目治理体系（递归开发协议、能力晋级台账、里程碑路线图、防回退锁、决策门）。

---

## 快速开始

```powershell
# 终端 1：启动 Rust 后端
cargo run -p minggui-backend

# 终端 2：启动前端开发服务器
cd frontend
node server.mjs
```

浏览器打开 `http://127.0.0.1:5173`。

---

## 能力边界

| 状态 | 数量 | 能力 |
|---|---|---|
| supported | 10 | health, lunar-data-meta, calendar-date-query, calendar-date-query-v1-meta, chart-create, chart-detail, analysis-snapshot, luck-cycles, glossary, astronomy-engine |
| restricted | 7 | chart-basis-preview, case-management, share-preview, settings, case-export, data-derivation, chart-report |

边界已锁定。详见 [`93-capability-promotion-ledger.md`](markdown/20-roadmap/93-capability-promotion-ledger.md)。

---

## API 总览

```
GET /api/health                                    supported
GET /api/capabilities                              supported
GET /api/lunar-data/meta                           supported
GET /api/calendar/query?date=                      supported   (Android date-layer, 1901-2100)
GET /api/charts/basis/preview?date=&timezone=      restricted
GET /api/charts?date=&timezone=&time_precision=    supported   (4 pillars)
GET /api/charts/detail?date=&timezone=             supported   (immutable snapshot)
GET /api/analysis/snapshot?date=&timezone=         supported   (structured metrics + deep analysis)
GET /api/luck/cycles?date=&timezone=&sex=          supported   (ADR 0020)
GET /api/glossary?term=&category=                  supported   (55 entries)
GET /api/cases?action=                             restricted  (local volatile, no DB)
GET /api/cases/export?id=                          restricted
GET /api/settings?action=                          restricted  (local volatile)
GET /api/share/preview?action=                     restricted  (hashed token, redacted)
GET /api/data/derive?type=                         restricted  (>=5 threshold)
GET /api/charts/report?date=&timezone=             restricted  (colloquial CN report, 9 blocks)
```

日期层基于 Android 万年历端口，验证范围为 `1901-2100`（ADR 0008）。天文引擎作为独立 supported 计算能力存在（ADR 0019、ADR 0021），不替代 Android 日期层为运行时默认。

---

## 验证

```powershell
cargo test --lib                  # Rust 后端（86 项）
cd frontend && npm run check       # JS 前端（10 项）
powershell -File tools/check-project.ps1   # 全项目门禁
```

---

## 项目结构

```
FT/
  backend/           Rust API 服务、领域模型、天文引擎、日历计算
  frontend/          JS 浏览器 UI、API client、状态管理、渲染模块
  data/raw/          农历基础数据
  data/generated/    天文引擎生成数据
  docs/decisions/    架构决策记录（ADR）
  docs/release/      发布候选与交付边界
  markdown/          治理文档、产品树、里程碑路线图
  tools/             项目检查脚本
```

---

## 导航

**入门**
- [`markdown/20-roadmap/00-roadmap-index.md`](markdown/20-roadmap/00-roadmap-index.md) — 里程碑总索引
- [`markdown/20-roadmap/93-capability-promotion-ledger.md`](markdown/20-roadmap/93-capability-promotion-ledger.md) — 能力矩阵与晋级条件
- [`docs/release/v1-closeout.md`](docs/release/v1-closeout.md) — V1 收口记录

**治理**
- [`markdown/20-roadmap/91-anti-regression-and-governance-lock.md`](markdown/20-roadmap/91-anti-regression-and-governance-lock.md) — 防回退规则
- [`markdown/20-roadmap/90-decision-gates.md`](markdown/20-roadmap/90-decision-gates.md) — 决策门
- [`markdown/20-roadmap/95-recursive-development-protocol.md`](markdown/20-roadmap/95-recursive-development-protocol.md) — 递归开发协议
- [`markdown/00-matrix-governance/module-tree.md`](markdown/00-matrix-governance/module-tree.md) — 模块契约

**架构**
- [`docs/decisions/`](docs/decisions/) — 全部 ADR
- [`markdown/10-overview/overview-full-feature-tree.md`](markdown/10-overview/overview-full-feature-tree.md) — 工程目录地图
