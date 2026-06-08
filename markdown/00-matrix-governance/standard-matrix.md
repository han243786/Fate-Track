# 命轨 Standard Matrix

## 1. Hard Rules

| Rule ID | Source | Rule | Enforcement |
| --- | --- | --- | --- |
| STD-001 | GP-ARCH-001 | 能力声明必须有 Rust/API/data 真源。 | gate+audit |
| STD-002 | GP-ARCH-002 | sibling 模块直连必须登记例外。 | audit |
| STD-003 | GP-CODE-001 | Rust 源码必须通过格式化检查。 | gate |
| STD-004 | GP-CODE-002 | Rust workspace 必须通过编译检查。 | gate |
| STD-005 | GP-CODE-003 | JS 文件必须通过语法检查。 | gate |
| STD-006 | GP-DATA-001 | raw 农历数据不得被应用运行时改写。 | audit |
| STD-007 | GP-FE-001 | 前端不得宣称后端未支持能力。 | gate+audit |
| STD-008 | GP-CHG-003 | 文件职责变化必须同步工程树和模块树。 | audit |

## 2. Current Gate Commands

| Gate | Command | Blocking |
| --- | --- | --- |
| Governance scaffold | `powershell -NoProfile -ExecutionPolicy Bypass -File tools/check-governance-scaffold.ps1 -ProjectRoot .` | yes |
| Rust format | `cargo fmt --check` | yes |
| Rust compile | `cargo check` | yes |
| JS frontend server syntax | `node --check frontend/server.mjs` | yes |
| JS frontend app syntax | `node --check frontend/src/main.js` | yes |

## 3. Drift Rules

- 工程全量树必须在文件新增、删除、移动或职责变化时同步。
- 模块树必须在所有权、public surface、输入、输出或通信路径变化时同步。
- General Policy 必须在重复失败模式出现时更新。
- 门禁矩阵必须在新增阻断检查时更新。

