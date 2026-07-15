#!/bin/bash -p
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"

usage() {
    printf 'Usage: %s [rv|la] [--output-dir PATH] [--fail-fast]\n' "$0" >&2
}

architecture="${1:-rv}"
if (( $# > 0 )); then
    shift
fi
case "$architecture" in
    rv|la) ;;
    *)
        printf 'infrastructure error: invalid official architecture: %s\n' "$architecture" >&2
        usage
        exit 2
        ;;
esac

runner_args=()
output_dir_seen=0
fail_fast_seen=0
while (( $# > 0 )); do
    case "$1" in
        --output-dir)
            if (( output_dir_seen || $# < 2 )); then
                printf 'infrastructure error: --output-dir requires exactly one value\n' >&2
                usage
                exit 2
            fi
            output_dir_seen=1
            runner_args+=("$1" "$2")
            shift 2
            ;;
        --fail-fast)
            if (( fail_fast_seen )); then
                printf 'infrastructure error: duplicate --fail-fast\n' >&2
                usage
                exit 2
            fi
            fail_fast_seen=1
            runner_args+=("$1")
            shift
            ;;
        *)
            printf 'infrastructure error: unsupported official entry argument: %s\n' "$1" >&2
            usage
            exit 2
            ;;
    esac
done

exec python3 -I -S -B -X pycache_prefix=/dev/null "$SCRIPT_DIR/run_suite.py" \
    --profile official --arch "$architecture" "${runner_args[@]}"
