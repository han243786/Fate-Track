# 命轨

命轨当前采用 Rust 后端、JavaScript 前端，并接入 `heavy-scale-exploitation-governance-1` 作为项目治理体系。

## Project Layout

| Path | Responsibility |
| --- | --- |
| `backend/` | Rust API service and lunar-data capability source |
| `frontend/` | JavaScript browser UI |
| `data/raw/` | Raw lunar-calendar source data |
| `markdown/` | Governance, product tree, policies, and closeout templates |
| `tools/` | Governance inventory and scaffold checks |
| `docs/decisions/` | Architecture decision records |

## Run

```powershell
cargo run -p minggui-backend
```

```powershell
cd frontend
node server.mjs
```

Open `http://127.0.0.1:5173`.

## Checks

```powershell
cargo fmt --check
cargo check
cd frontend
node --check server.mjs
node --check src/main.js
powershell -NoProfile -ExecutionPolicy Bypass -File ..\tools\check-governance-scaffold.ps1 -ProjectRoot ..
```

