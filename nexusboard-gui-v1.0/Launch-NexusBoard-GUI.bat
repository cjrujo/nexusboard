@echo off
setlocal
cd /d "%~dp0"

echo ==================================================
echo   NexusBoard GUI v1.0 (Tauri v2 Desktop App)
echo ==================================================

where node >nul 2>nul
if %errorlevel% neq 0 (
  echo [ERROR] Node.js 18+ is required. Install from https://nodejs.org/
  pause
  exit /b 1
)

where cargo >nul 2>nul
if %errorlevel% neq 0 (
  echo [ERROR] Rust/Cargo is required for Tauri v2. Install from https://rustup.rs/
  pause
  exit /b 1
)

if not exist "node_modules" (
  echo [1/2] Installing Tauri v2 CLI dependencies...
  call npm install
  if %errorlevel% neq 0 (
    echo [ERROR] npm install failed.
    pause
    exit /b 1
  )
)

echo [2/2] Launching NexusBoard Desktop App...
call npm run dev
pause
