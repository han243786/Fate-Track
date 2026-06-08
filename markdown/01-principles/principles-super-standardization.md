# 命轨超级规范化

> Scope: proposal, development, validation, audit, release, and closeout.

## 1. Change Pipeline

```text
request
  -> proposal
  -> module impact
  -> implementation
  -> local gates
  -> CI gates
  -> audit
  -> release validation
  -> closeout
```

## 2. Change Levels

| Level | Trigger | Required Evidence |
| --- | --- | --- |
| light | no behavior or boundary change | basic gates |
| standard | feature, API, UI, test, data, or module change | proposal, tests, docs, module impact |
| heavy | architecture, security, migration, release, or multi-module change | full proposal, regression matrix, audit, closeout |

## 3. Gate Matrix

| # | Gate | Command or Check | Stage | Blocking |
| --- | --- | --- | --- | --- |
| 1 | Governance scaffold | `tools/check-governance-scaffold.ps1` | pre-commit/CI | yes |
| 2 | Rust format | `cargo fmt --check` | pre-commit/CI | yes |
| 3 | Rust compile | `cargo check` | CI | yes |
| 4 | JS syntax | `node --check frontend/server.mjs`; `node --check frontend/src/main.js` | CI | yes |
| 5 | Full tree drift | review changed files against full tree | closeout | yes |
| 6 | Module tree drift | review boundary changes against module tree | closeout | yes |
| 7 | Data provenance | raw/derived lunar data source review | closeout | yes for data changes |
| 8 | Release dry-run | project release dry-run | release | yes |

## 4. Audit Severity

| Severity | Meaning | Handling |
| --- | --- | --- |
| S0 | cannot continue safely | fix now |
| P1 | must fix next milestone | register |
| P2 | fix within two milestones | track |
| P3 | long-term improvement | backlog |

## 5. Closeout Requirements

- Scope summary.
- Module impact.
- Policy compliance.
- Gate results.
- Regression evidence.
- S0/P1/P2/P3 status.
- Residual risk.
- Follow-up owner and milestone.
- Whether governance docs or gates must evolve.

