@echo off
echo Building Nebula Installer...
if not exist "dist" mkdir dist

:: Check for Inno Setup compiler
where iscc >nul 2>nul
if %errorlevel% neq 0 (
    echo Error: Inno Setup 6 Compiler ^(iscc^) not found in PATH.
    echo Please install Inno Setup 6 and add it to your PATH.
    exit /b 1
)

iscc setup.iss
if %errorlevel% neq 0 (
    echo Build Failed!
    exit /b 1
)

echo Build Success! Output is in the current directory.
pause
