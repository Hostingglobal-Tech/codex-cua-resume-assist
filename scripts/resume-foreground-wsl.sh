#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MESSAGE="${1:-continue}"

MAIN_BIN="$ROOT/target/release/codex-cua-resume-assist"
WIN_HELPER="$ROOT/target/x86_64-pc-windows-gnu/release/codex-cua-win-capture.exe"

if [[ ! -x "$MAIN_BIN" ]]; then
  cargo build --release --manifest-path "$ROOT/Cargo.toml"
fi

if [[ ! -f "$WIN_HELPER" ]]; then
  cargo build --release --target x86_64-pc-windows-gnu --bin codex-cua-win-capture --manifest-path "$ROOT/Cargo.toml"
fi

export CODEX_CUA_WIN_CAPTURE="$WIN_HELPER"
exec "$MAIN_BIN" --api --execute --terminal-send "$MESSAGE"
