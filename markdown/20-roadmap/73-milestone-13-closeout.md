# Milestone Closeout: M13 Luck Cycles

## 1. Scope
Closed DG-005 via ADR 0020. Implemented `GET /api/luck/cycles` with 大运 computation: direction by year-gan × sex, starting age by birth-to-jie days ÷ 3, 8 cycles of 10 years each.

## 2. Capability Status
| Capability | Before | After |
|---|---|---|
| `luck-cycles` | planned | supported |

## 3. Evidence
- `docs/decisions/0020-dg-005-luck-cycle-rules.md` — ADR closing DG-005
- `markdown/20-roadmap/90-decision-gates.md` — DG-005 marked closed
- `backend/src/domain/luck.rs` — compute_luck_cycles() with tests (5 tests)
- `backend/src/api/luck.rs` — cycles() API handler
- `backend/src/api/capabilities.rs` — luck-cycles status updated

## 4. Validation
- `cargo test` includes luck domain tests: yang male forward, yang female reverse, yin male reverse, yin female forward, 8-cycle continuity
