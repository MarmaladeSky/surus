#!/usr/bin/env bash
# Usage: CLAUDE_CODE_OAUTH_TOKEN=... ./run-experiment.sh <model> <effort>
set -euo pipefail

command -v sshpass > /dev/null || { echo "sshpass not found" >&2; exit 1; }

model=$1
effort=$2
out="runs/$(date +%Y%m%d-%H%M%S)-$model-$effort"

echo "Running $model with effort $effort; results go to $out"

cd "$(dirname "$0")"
mkdir -p "$out"
git rev-parse HEAD > "$out/commit"

vm() {
    sshpass -p agent ssh -p 2222 -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null agent@localhost "$@"
}

rm -f workspace.img
nix run .#nixosConfigurations.agent.config.microvm.declaredRunner > "$out/vm.log" 2>&1 < /dev/null &

until vm test -f /workspace/surus/Cargo.toml 2>/dev/null; do sleep 5; done

printf '%s\n' "$CLAUDE_CODE_OAUTH_TOKEN" | vm '
read -r CLAUDE_CODE_OAUTH_TOKEN
export CLAUDE_CODE_OAUTH_TOKEN
export SURUS_TEST_DATABASE_URL="postgres:///agent?host=/run/postgresql&user=agent"
cd /workspace/surus
claude --version > /workspace/claude.version
claude -p "$(cat docs/initial_prompt.md)" \
  --model '"$model"' \
  --permission-mode bypassPermissions \
  --max-turns 300 \
  --output-format stream-json --verbose \
  --effort '"$effort"' \
  2> /workspace/run.err < /dev/null | tee /workspace/run.jsonl
' || true

vm 'tar -C /workspace -czf - --exclude=surus/target surus' > "$out/surus-agent-run.tar.gz"
vm 'cat /workspace/run.jsonl' > "$out/run.jsonl"
vm 'cat /workspace/run.err' > "$out/run.err"
vm 'cat /workspace/claude.version' > "$out/claude.version"

vm 'sudo poweroff' || true

./check-protected.sh "$out/surus-agent-run.tar.gz" "$(cat "$out/commit")" | tee "$out/protected.txt"
