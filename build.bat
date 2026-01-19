@echo off
chcp 65001 >nul
echo ==========================================
echo   Antigravity Manager Build Script
echo ==========================================
echo.

REM Check if npm is available
where npm >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] npm is not installed or not in PATH.
    echo Please install Node.js first: https://nodejs.org/
    pause
    exit /b 1
)

REM Check if cargo is available
where cargo >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Rust/Cargo is not installed or not in PATH.
    echo Please install Rust first: https://rustup.rs/
    pause
    exit /b 1
)

echo [INFO] Installing npm dependencies...
call npm install
if %errorlevel% neq 0 (
    echo [ERROR] Failed to install npm dependencies.
    pause
    exit /b 1
)

echo.
echo [INFO] Building Tauri application (Release mode)...
echo This may take several minutes...
echo.

call npm run tauri -- build

if %errorlevel% equ 0 (
    echo.
    echo ==========================================
    echo   Build completed successfully!
    echo ==========================================
    echo.
    echo Output location:
    echo   src-tauri\target\release\antigravity-tools.exe
    echo.
    echo Installer location:
    echo   src-tauri\target\release\bundle\
    echo.
) else (
    echo.
    echo [ERROR] Build failed. Please check the error messages above.
    echo.
)

pause
