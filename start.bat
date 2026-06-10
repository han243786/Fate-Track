@echo off
chcp 65001 >nul
setlocal

set PROJECT_ROOT=%~dp0

echo ============================================
echo   命轨 Fate Track — 一键启动
echo ============================================
echo.
echo   1. 桌面应用（推荐）
echo   2. 开发模式（后端 + 前端分别启动）
echo.
choice /c 12 /n /m "请选择启动方式 [1/2]: "

if errorlevel 2 goto dev
if errorlevel 1 goto desktop

:desktop
echo.
echo [桌面应用] 启动命轨桌面壳...
cd /d "%PROJECT_ROOT%"
cargo run -p minggui-desktop
goto end

:dev
echo.
echo [开发模式] 启动后端...
start "MingGui Backend" cmd /k "cd /d "%PROJECT_ROOT%" && cargo run -p minggui-backend"

echo [开发模式] 启动前端...
start "MingGui Frontend" cmd /k "cd /d "%PROJECT_ROOT%frontend" && node server.mjs"

echo [开发模式] 等待服务就绪后打开浏览器...
timeout /t 3 /nobreak >nul
start "" "http://127.0.0.1:5173"

echo.
echo 后端 http://127.0.0.1:8787
echo 前端 http://127.0.0.1:5173
echo.
echo 关闭对应窗口即可停止服务。
pause >nul

:end
endlocal
