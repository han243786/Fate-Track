# V1 Closeout

## Capability Matrix

| # | Capability | Status | Route |
|---|-----------|--------|-------|
| 1 | health | supported | GET /api/health |
| 2 | lunar-data-meta | supported | GET /api/lunar-data/meta |
| 3 | calendar-date-query | supported | GET /api/calendar/query?date= |
| 4 | calendar-date-query-v1-meta | supported | GET /api/calendar/query (meta field) |
| 5 | chart-basis-preview | restricted | GET /api/charts/basis/preview |
| 6 | chart-create | supported | GET /api/charts?date=&timezone= |
| 7 | chart-detail | supported | GET /api/charts/detail |
| 8 | analysis-snapshot | supported | GET /api/analysis/snapshot |
| 9 | luck-cycles | supported | GET /api/luck/cycles |
| 10 | case-management | restricted | GET /api/cases |
| 11 | share-preview | restricted | GET /api/share/preview |
| 12 | settings | restricted | GET /api/settings |
| 13 | glossary | supported | GET /api/glossary |
| 14 | case-export | restricted | GET /api/cases/export |
| 15 | data-derivation | restricted | GET /api/data/derive |
| 16 | astronomy-engine | supported | data/generated/astronomy/out/* (ADR 0021) |

## Decision Gates

All 10 DG-001 through DG-010 closed.

## Milestones

M0 through M27 closed. Final capability matrix: 10 supported, 7 restricted, 0 target, 0 planned. Boundary locked.

## Known Limitations

1. 农历输入不直接支持 (DG-004 closed)
2. 无数据库持久化 — 案例本地易失存储 (DG-006 closed)
3. 无真太阳时 (metadata declares unsupported)
4. 无 IANA 时区历史 (metadata declares not resolved)
5. 天文引擎不替换 Android 日期层 (DG-008 closed, replacement ADR pending)
6. 流年/流月未实现 (luck API declares planned)
7. 时辰未知时大运仍可排 (仅使用年月日柱)

## Validation

```
cargo test:            81 passed, 0 failed
npm run check:         10 passed, 0 failed  
check-project.ps1:     Governance OK, Release OK, Astronomy OK
decision gates:        10/10 closed
```

## Next

- 天文引擎运行时集成 (replacement ADR)
- 流年/流月实现
- 真太阳时支持
- IANA 时区历史
- 持久化存储

---
Closed: 2026-06-09
