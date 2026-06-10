# 命轨 · Fate Track

命轨是一套基于传统命理学的排盘与结构化分析工具，采用 Rust 后端 + 原生 JavaScript 前端的单体架构。项目的设计目标是：**只读排盘工作台，边界锁定，不做生成式推断。**

---

## 快速开始

**方式一：桌面应用（推荐）**

```powershell
cargo run -p minggui-desktop
```

启动后自动打开原生窗口，无需 Node、无需手动启动后端。

**方式二：开发模式**

```powershell
# 终端 1：启动后端
cargo run -p minggui-backend

# 终端 2：启动前端
cd frontend
node server.mjs
```

浏览器打开 `http://127.0.0.1:5173`。

---

## 能力边界

| 状态 | 数量 | 能力 |
|---|---|---|
| 已支持 | 10 | 健康检查、农历数据元信息、公历日期查询、日期层元数据、四柱排盘、命盘快照、结构化分析、大运排盘、术语查询、天文引擎 |
| 受限 | 7 | 排盘基础预览、案例管理、分享预览、用户设置、案例导出、数据衍生、排盘报告 |

边界已锁定，不再新增功能性需求。详见[能力晋级台账](markdown/20-roadmap/93-capability-promotion-ledger.md)。

---

## 接口总览

```
GET /api/health                                    已支持
GET /api/capabilities                              已支持
GET /api/lunar-data/meta                           已支持
GET /api/calendar/query?date=                      已支持    （Android 日期层，1901-2100）
GET /api/charts/basis/preview?date=&timezone=      受限
GET /api/charts?date=&timezone=&time_precision=    已支持    （四柱排盘）
GET /api/charts/detail?date=&timezone=             已支持    （不可变快照）
GET /api/analysis/snapshot?date=&timezone=         已支持    （结构化指标 + 深层分析）
GET /api/luck/cycles?date=&timezone=&sex=          已支持    （大运排盘）
GET /api/glossary?term=&category=                  已支持    （55 条术语）
GET /api/cases?action=                             受限      （本地易失存储）
GET /api/cases/export?id=                          受限
GET /api/settings?action=                          受限      （本地易失偏好）
GET /api/share/preview?action=                     受限      （脱敏分享）
GET /api/data/derive?type=                         受限      （≥5 条阈值）
GET /api/charts/report?date=&timezone=             受限      （口语化中文报告，9 章）
```

日期层基于 Android 万年历端口，验证范围为 1901-2100。天文引擎作为独立已支持能力存在，不替代 Android 日期层为运行时默认。

---

## 验证

```powershell
cargo test --lib                  # 后端 86 项
cd frontend && npm run check       # 前端 10 项
powershell -File tools/check-project.ps1   # 全项目门禁
```

---

## 项目结构

```
FT/
  backend/           Rust API 服务、领域模型、天文引擎、日历计算
  frontend/          JS 浏览器 UI、API 客户端、状态管理、渲染模块
  data/raw/          农历基础数据
  data/generated/    天文引擎生成数据
  docs/decisions/    架构决策记录
  docs/release/      发布候选与交付边界
  markdown/          治理文档、产品树、里程碑路线图
  tools/             项目检查脚本
```

---

## 导航

**入门**
- [里程碑总索引](markdown/20-roadmap/00-roadmap-index.md)
- [能力晋级台账](markdown/20-roadmap/93-capability-promotion-ledger.md)
- [V1 收口记录](docs/release/v1-closeout.md)

**治理**
- [防回退与治理锁](markdown/20-roadmap/91-anti-regression-and-governance-lock.md)
- [决策门](markdown/20-roadmap/90-decision-gates.md)
- [递归开发协议](markdown/20-roadmap/95-recursive-development-protocol.md)
- [模块树与契约](markdown/00-matrix-governance/module-tree.md)

**架构**
- [架构决策记录](docs/decisions/)
- [工程目录地图](markdown/10-overview/overview-full-feature-tree.md)
