#!/usr/bin/env sh
set -eu

ROOT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
APP_BIN="$ROOT_DIR/src-tauri/target/debug/btclient"

if [ ! -x "$APP_BIN" ]; then
  printf '%s\n' "未找到调试版应用，请先运行：npm run tauri -- build --debug" >&2
  exit 1
fi

# WebKitGTK on WSLg can show a black window when GPU compositing/DMABUF is enabled.
export WEBKIT_DISABLE_COMPOSITING_MODE="${WEBKIT_DISABLE_COMPOSITING_MODE:-1}"
export WEBKIT_DISABLE_DMABUF_RENDERER="${WEBKIT_DISABLE_DMABUF_RENDERER:-1}"
export LIBGL_ALWAYS_SOFTWARE="${LIBGL_ALWAYS_SOFTWARE:-1}"

exec "$APP_BIN"
