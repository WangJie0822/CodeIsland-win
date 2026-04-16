#!/bin/bash
# macOS 开发启动脚本
set -e
cd "$(dirname "$0")/.."
echo "=== Code Island (dev) ==="
echo "前端: http://localhost:5173"
echo "Unix Socket: /tmp/codeisland.sock"
npm run tauri dev
