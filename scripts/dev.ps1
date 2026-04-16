# Windows 开发启动脚本
Set-Location (Split-Path $PSScriptRoot -Parent)
Write-Host "=== Code Island (dev) ==="
Write-Host "Named Pipe: \\.\pipe\codeisland"
npm run tauri dev
