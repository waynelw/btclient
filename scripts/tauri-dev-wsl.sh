#!/usr/bin/env sh
set -eu

ROOT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"

if [ ! -x "$ROOT_DIR/node_modules/.bin/tauri" ]; then
  printf '%s\n' "未找到 Tauri CLI，请先运行：npm install" >&2
  exit 1
fi

export CARGO_HOME="${CARGO_HOME:-$ROOT_DIR/.cargo}"
export RUSTUP_HOME="${RUSTUP_HOME:-$ROOT_DIR/.rustup}"
export PATH="$CARGO_HOME/bin:$PATH"

if ! command -v cargo >/dev/null 2>&1; then
  printf '%s\n' "未找到 cargo，请先在项目内安装 Rust 工具链" >&2
  exit 1
fi

# WebKitGTK on WSLg can show a black window when GPU compositing/DMABUF is enabled.
export WEBKIT_DISABLE_COMPOSITING_MODE="${WEBKIT_DISABLE_COMPOSITING_MODE:-1}"
export WEBKIT_DISABLE_DMABUF_RENDERER="${WEBKIT_DISABLE_DMABUF_RENDERER:-1}"
export LIBGL_ALWAYS_SOFTWARE="${LIBGL_ALWAYS_SOFTWARE:-1}"

exec "$ROOT_DIR/node_modules/.bin/tauri" dev
