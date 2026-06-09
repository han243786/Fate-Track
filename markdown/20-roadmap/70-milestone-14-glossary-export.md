# M14: Glossary and Case Export

## 1. 目标

实现术语表查询和案例导出。术语表为前端提供结构化解释数据；案例导出允许用户将本地案例导出为标准格式。

## 2. 依赖

- M3 chart-create 提供稳定实体命名。
- M4 analysis-snapshot 提供分析术语。
- M5 case-management 提供案例数据源。
- M12 chart-detail 提供可导出快照。

## 3. 范围

### Glossary (术语表)

| Work Package | 内容 |
| --- | --- |
| M14-WP1 | `GlossaryEntry` 领域模型：id、中文术语、拼音、英文翻译、简短解释、分类 |
| M14-WP2 | `GET /api/glossary` 路由：支持 `?term=` 搜索和 `?category=` 过滤 |
| M14-WP3 | 术语数据源（硬编码 JSON，覆盖天干、地支、五行、十神、藏干、节气、纳音） |
| M14-WP4 | API 测试 + 完整性校验 |

### Case Export (案例导出)

| Work Package | 内容 |
| --- | --- |
| M14-WP5 | `GET /api/cases/export` 路由：导出指定案例为 JSON |
| M14-WP6 | 导出格式：包含 chart snapshot、analysis snapshot、case metadata |
| M14-WP7 | 导出脱敏：可选是否包含私有备注 |
| M14-WP8 | API 测试 + 格式校验 |

## 4. 非目标

- 不实现在线术语编辑器。
- 不支持批量导出全库。
- 不导出为 PDF 或其他富格式。
- 不添加导出分享/导入功能。

## 5. 能力状态

| Capability | Before | After |
| --- | --- | --- |
| `glossary` | planned | supported |
| `case-export` | planned | restricted |
| `case-management` | restricted | restricted（不变化） |

## 6. 防回退

- 术语定义必须与代码中使用的实体名称一致（stem/branch/element/ten_god 等）。
- 导出不得泄露其他案例的私有数据。
- 导出 JSON 格式必须稳定，字段不可随意重命名。

## 7. 治理同步

- `backend/src/api/mod.rs` 新增 glossary 和 case-export 路由。
- `backend/src/api/capabilities.rs` 更新 glossary 和 case-export 状态。
- `93-capability-promotion-ledger.md` 更新。
- module tree、engineering tree、README 同步。

## 8. 验收

- `GET /api/glossary` 返回完整术语列表。
- `GET /api/glossary?term=十神` 返回匹配条目。
- `GET /api/cases/export?id=xxx` 返回 JSON 导出。
- 导出含算法版本和快照引用。
- `tools/check-project.ps1` 通过。
