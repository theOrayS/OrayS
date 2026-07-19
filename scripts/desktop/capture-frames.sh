#!/usr/bin/env bash
set -euo pipefail

repo_root=$(git -C "$(dirname "${BASH_SOURCE[0]}")" rev-parse --show-toplevel)

if [[ $# -ne 3 ]]; then
    printf '%s\n' 'usage: capture-frames.sh QMP_SOCKET OUTPUT.ppm TRANSCRIPT.jsonl' >&2
    exit 2
fi

python3 "$repo_root/scripts/desktop/qmp_screendump.py" \
    --socket "$1" \
    --output "$2" \
    --transcript "$3"
