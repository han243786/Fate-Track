@echo off
chcp 65001 >nul
setlocal

set PROJECT_ROOT=%~dp0
set BROWSER_URL=http://127.0.0.1:5173

echo ============================================
echo   MingGui (Fate Track) - One-click Launch
echo ============================================
echo.
echo Backend  : http://127.0.0.1:8787
echo Frontend : %BROWSER_URL%
echo.

echo [1/3] Starting Rust backend (minggui-backend)...
start "MingGui Backend" cmd /k "cd /d "%PROJECT_ROOT%" && cargo run -p minggui-backend"

echo [2/3] Starting JavaScript frontend...
start "MingGui Frontend" cmd /k "cd /d "%PROJECT_ROOT%frontend" && node server.mjs"

echo [3/3] Waiting for services, then opening browser...
timeout /t 3 /nobreak >nul
start "" "%BROWSER_URL%"

echo.
echo Both services started in separate windows.
echo Close those windows to stop the services, or press any key here to exit this launcher.
pause >nul
