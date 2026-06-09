# 命轨 Guidance Matrix

Use this matrix to locate the blast radius of a requested change.

| Request Type | First Read | Then Check | Required Evidence |
| --- | --- | --- | --- |
| Rust backend feature | `backend.api` | GP, data source, frontend consumers, active milestone | API contract, `cargo test`, regression evidence |
| JS frontend feature | `frontend.ui` | backend source, GP-FE rules, active milestone | screenshot/manual check, `npm.cmd run check` |
| lunar data change | `data.lunar.raw` | GP-DATA rules, backend parser | provenance, validation output |
| product scope change | `markdown/命轨全量树.md` | module tree, GP-FE, privacy rules | product tree diff, non-goals |
| bug fix | failing path | owning module, tests | reproduction or regression check |
| API change | `backend.api` | frontend consumer, module tree | compatibility note |
| governance change | `governance.matrix` | standard/process matrix | scaffold check |
| release change | `system.workspace` | README, gate commands | dry-run and rollback note |
| research report intake | `governance.research` | ADR, module tree, General Policy, standard matrix | Chinese translation, intake row, target/support boundary |
| roadmap or milestone work | `governance.roadmap` | decision gates, risk register, capability ledger | milestone file, closeout evidence, anti-regression check |
| recursive development loop | `governance.roadmap` | recursive cursor, previous loop closeout, active milestone | cursor update, loop result, validation evidence |

## Required Output for Standard/Heavy Changes

- impacted module IDs
- impacted policy IDs
- validation commands
- docs to update
- risks that remain
