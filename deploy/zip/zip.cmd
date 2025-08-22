@echo off
setlocal enabledelayedexpansion

echo Creating zip archive...

if exist "output.zip" (
    echo Deleting existing output.zip...
    del "output.zip"
)

powershell -Command "Get-ChildItem -Path '.' -Exclude 'output.zip', 'target', '.idea', 'web\node_modules', 'web\dist' | Compress-Archive -DestinationPath 'output.zip' -Force"

if %errorlevel% == 0 (
    echo Successfully created output.zip
) else (
    echo Failed to create output.zip
    exit /b 1
)
