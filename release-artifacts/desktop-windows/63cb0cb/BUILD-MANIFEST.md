# Fate Track Windows Desktop Artifact

## Source

- Repository: `han243786/Fate-Track`
- Source commit: `63cb0cb`
- Branch at build time: `main`
- Artifact kind: Windows x64 desktop preview zip

## Build Command

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\package-desktop-windows.ps1
```

## Quality Gates

- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `npm.cmd run check`
- `powershell -NoProfile -ExecutionPolicy Bypass -File tools\check-project.ps1`
- `cargo build -p minggui-desktop --release --locked`

## Artifact

- File: `Fate-Track-Windows-x64.zip`
- SHA256: `0027647628d3614a93f861ff1babc43ab9f3412f58cd86f8ba4d350e05a3a766`
