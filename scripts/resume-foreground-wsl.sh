#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MESSAGE="${1:-continue}"

MAIN_BIN="$ROOT/target/release/codex-cua-resume-assist"
WIN_HELPER="$ROOT/target/x86_64-pc-windows-gnu/release/codex-cua-win-capture.exe"

ensure_wsl_interop() {
  if [[ -n "${WSL_INTEROP:-}" && -S "${WSL_INTEROP:-}" ]]; then
    return 0
  fi

  local socket
  while IFS= read -r socket; do
    if [[ -S "$socket" ]] && (
      cd /mnt/c/Windows
      WSL_INTEROP="$socket" /usr/bin/timeout 5s /mnt/c/Windows/system32/cmd.exe /C exit 0
    ) >/dev/null 2>&1; then
      export WSL_INTEROP="$socket"
      return 0
    fi
  done < <(find /run/WSL -maxdepth 1 -type s -name '*_interop' 2>/dev/null | sort -Vr)

  echo "WSL interop is not available; Windows helper cannot run" >&2
  return 1
}

ensure_wsl_interop

if [[ ! -x "$MAIN_BIN" ]]; then
  cargo build --release --manifest-path "$ROOT/Cargo.toml"
fi

if [[ ! -f "$WIN_HELPER" ]]; then
  cargo build --release --target x86_64-pc-windows-gnu --bin codex-cua-win-capture --manifest-path "$ROOT/Cargo.toml"
fi

export CODEX_CUA_WIN_CAPTURE="$WIN_HELPER"
exec "$MAIN_BIN" --api --execute --terminal-send "$MESSAGE"
