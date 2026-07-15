#!/bin/bash -p
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
if (( $# == 0 )); then
    set -- rv
fi

exec python3 -B -E -s "$SCRIPT_DIR/run_suite.py" --profile official --arch "$@"
