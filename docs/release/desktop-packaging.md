# Desktop Packaging — 命轨桌面壳封装边界

## 封装目标

将 `minggui-desktop` 编译为三平台原生可执行文件，无需外部依赖（Node、Python、数据库等）。

## 当前封装状态

- M28 已实现 Tao + Wry 桌面壳
- `cargo run -p minggui-desktop` 可在开发机启动
- 未做正式发布打包（无安装包、无签名、无公证）

## 三平台产物类型

| 平台 | 产物 | 构建 Runner |
|---|---|---|
| Windows | `.exe` + `.zip` | `windows-latest` |
| macOS | raw executable + `.zip` | `macos-latest` |
| Linux | `tar.gz` (或 AppImage) | `ubuntu-latest` |

## 构建前置条件

### Windows
- 系统自带 WebView2（Win10+ 已内置）
- 无需额外依赖

### macOS
- 系统自带 WebKit
- 无需额外依赖
- 公开分发需要签名 + 公证（Apple Notary）

### Linux
- 需要 `libwebkit2gtk-4.1-dev`（Ubuntu/Debian）
- CI 需在 `ubuntu-latest` runner 安装此包

## 发布边界

- **内部测试**：可直接分发 zip/tar.gz 产物
- **公开分发**：Windows 需代码签名（EV/OV），macOS 需签名+公证，Linux 建议 AppImage
- 当前阶段：**内部测试**（不签名）

## 本地 Windows 预封装

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\package-desktop-windows.ps1
```

脚本会执行严格 Rust lint、完整 Rust 测试、前端检查、治理门禁和 release 构建，然后生成：

- `dist/desktop-windows/Fate-Track-Windows-x64.zip`
- `dist/desktop-windows/SHA256SUMS.txt`

Windows zip 内包含 `minggui-desktop.exe`、`README.md`、`docs/release/desktop-packaging.md`、`docs/release/current-product-boundary.md`、`docs/release/v1-release-candidate.md` 和 `docs/release/v1-closeout.md`。

如果门禁已经在同一提交上完成，可临时使用 `-SkipQualityGate` 只重新生成本地 zip。

## 不做的事情

- 不生成 `.msi` / `.deb` / `.dmg` 安装包
- 不做代码签名
- 不做 macOS 公证
- 不做自动更新

## CI 样板

```yaml
jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - if: runner.os == 'Linux'
        run: sudo apt-get update && sudo apt-get install -y libwebkit2gtk-4.1-dev
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - run: cargo clippy --all-targets -- -D warnings
      - run: cargo test
      - run: cargo build -p minggui-desktop --release --locked
      - uses: actions/upload-artifact@v4
        with:
          name: minggui-desktop-${{ runner.os }}
          path: target/release/minggui-desktop*
```
