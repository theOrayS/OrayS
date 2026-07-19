#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || true)"
if [[ -z "$ROOT" ]]; then
  echo "ERROR: must run inside the OrayS desktop worktree" >&2
  exit 2
fi

cd "$ROOT"

# Explicitly keep the Codex session and all child validation commands headless.
unset DISPLAY WAYLAND_DISPLAY MIR_SOCKET
export ORAYS_DESKTOP_HEADLESS=1

if [[ ! -f "$ROOT/.codex/config.toml" ]]; then
  echo "ERROR: missing worktree-local .codex/config.toml" >&2
  exit 2
fi

echo "Worktree: $ROOT"
echo "Branch:   $(git branch --show-current)"
echo "Config:   $ROOT/.codex/config.toml"
echo "Mode:     headless (DISPLAY/Wayland unset)"
echo
echo "Paste this after Codex starts:"
cat "$ROOT/.codex/prompts/GOAL_PROMPT.txt"
echo

exec codex "$@"
