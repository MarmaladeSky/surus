#!/usr/bin/env bash
# Usage: ./check-protected.sh <surus-agent-run.tar.gz> <commit>
# Lists changes the agent made to the specification or the tests; exits 1 if any.
# The run is extracted into an "extracted" directory next to the archive.
set -euo pipefail

archive=$(realpath "$1")
commit=$2
work="$(dirname "$archive")/extracted"

cd "$(dirname "$0")"

mkdir "$work"
tar -xzf "$archive" -C "$work"

protected=(README.md docs tests)
editable='^M\ttests/dsl/([a-z_]+/dsl|operators/dsl/[a-z0-9_]+)[.]rs$'

run_git() {
    GIT_INDEX_FILE="$work/index" git --work-tree="$work/surus" "$@"
}

run_git read-tree "$commit"
run_git update-index -q --refresh > /dev/null || true

violations=$(
    {
        run_git diff-files --name-status -- "${protected[@]}"
        run_git ls-files --others -- "${protected[@]}" | sed 's/^/?\t/'
    } | awk -v editable="$editable" '$0 !~ editable || /[/]operators[/]dsl[/]mod[.]rs$/'
)

if [ -n "$violations" ]; then
    echo "Protected files changed:"
    echo "$violations"
    exit 1
fi
echo "Protected files intact"
