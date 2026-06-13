# M28: Native Desktop Shell

## 1. 目标

为项目创建 Rust 原生桌面壳，将前端和后端打包为单一可执行文件。用户双击启动后自动打开原生窗口，无需安装 Node、无需手动启动后端。

## 2. 依赖

- M0-M27 全部能力已实现。
- 前端为纯静态 HTML/CSS/JS，无构建流程。
- 后端为 Rust library，可直接嵌入。

## 3. 范围

| Work Package | 内容 |
| --- | --- |
| M28-WP1 | 新增 `desktop/` workspace 成员 |
| M28-WP2 | Tao 创建原生窗口（最大化启动，最小 1180×720） |
| M28-WP3 | Wry 加载系统 WebView（Windows: WebView2，macOS: WKWebView） |
| M28-WP4 | `include_dir!` 编译时嵌入 `frontend/` 全部静态文件 |
| M28-WP5 | `include_bytes!` 嵌入 `lunar_data.yaml`，运行时释放到临时目录 |
| M28-WP6 | 内嵌 HTTP 服务：随机本机端口，`/api/*` 走 backend App，其余走静态 |
| M28-WP7 | 拦截 `config.js` 动态注入桌面端口，避免 API 请求失败 |
| M28-WP8 | 更新 `start.bat` 支持桌面/开发双模式选择 |

## 4. 非目标

- 不引入 Electron、Node、Tauri CLI
- 不新增能力
- 不修改后端或前端业务逻辑
- 不提供安装包（dmg/msi/deb）

## 5. 能力状态

无变化。M28 只交付桌面封装；V1 preview 能力矩阵保持 10 supported、7 restricted、0 target、0 planned。M0-M28 关闭后锁定的是 `v1.0.0-preview` 发布边界；post-preview 新能力从 M29 另行登记。
