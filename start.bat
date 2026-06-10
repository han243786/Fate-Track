@echo off
setlocal enabledelayedexpansion
chcp 65001 >nul 2>&1

set PROJECT_ROOT=%~dp0

echo ============================================
echo   MingGui Fate Track Launcher
echo ============================================
echo.
echo   [1] Desktop App ^(recommended^)
echo   [2] Dev Mode ^(backend + frontend^)
echo.
set /p MODE="Select mode [1/2]: "

if "%MODE%"=="2" goto dev
if "%MODE%"=="1" goto desktop
goto desktop

:desktop
echo.
echo Starting desktop shell...
cd /d "%PROJECT_ROOT%"
cargo run -p minggui-desktop
goto end

:dev
echo.
echo Starting backend...
start "MingGui Backend" cmd /k "cd /d "%PROJECT_ROOT%" && cargo run -p minggui-backend"

echo Starting frontend...
start "MingGui Frontend" cmd /k "cd /d "%PROJECT_ROOT%frontend" && node server.mjs"

echo Waiting for services...
timeout /t 3 /nobreak >nul
start "" "http://127.0.0.1:5173"

echo.
echo Backend : http://127.0.0.1:8787
echo Frontend: http://127.0.0.1:5173
echo.
echo Close the service windows to stop.
pause >nul

:end
endlocal
