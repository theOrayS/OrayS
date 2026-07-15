#!/bin/bash -p
set -uo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/../.." && pwd)"
MANIFEST="$SCRIPT_DIR/semantic_evidence_manifest.json"
EVIDENCE_ROOT="$REPO_ROOT/build/pr3-evidence"
REQUIRED_DIR="$EVIDENCE_ROOT/required"
PYTHON=(python3 -I -S -B -X pycache_prefix=/dev/null)

status=0
"${PYTHON[@]}" "$SCRIPT_DIR/semantic_evidence.py" merge \
    --manifest "$MANIFEST" \
    --output "$REQUIRED_DIR" \
    --shard "$EVIDENCE_ROOT/host/semantic-evidence-v1.json" \
    --shard "$EVIDENCE_ROOT/rv64/semantic-evidence-v1.json" \
    --shard "$EVIDENCE_ROOT/la64/semantic-evidence-v1.json"
merge_status=$?
if (( merge_status > status )); then
    status=$merge_status
fi

if [[ -f "$REQUIRED_DIR/semantic-evidence-v1.json" ]]; then
    "${PYTHON[@]}" "$SCRIPT_DIR/render_semantic_evidence.py" \
        --manifest "$MANIFEST" \
        --input "$REQUIRED_DIR/semantic-evidence-v1.json" \
        --output "$REQUIRED_DIR/reports"
    render_status=$?
    if (( render_status > status )); then
        status=$render_status
    fi
else
    printf 'semantic evidence aggregate missing after merge: %s\n' \
        "$REQUIRED_DIR/semantic-evidence-v1.json" >&2
    if (( status < 2 )); then
        status=2
    fi
fi

exit "$status"
