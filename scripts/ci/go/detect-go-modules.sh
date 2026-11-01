#!/usr/bin/env bash
set -euo pipefail

# A read loop rather than mapfile: macOS runners ship bash 3.2, which has no mapfile.
all_modules=()
while IFS= read -r mod; do
  all_modules+=("$mod")
done < <(git ls-files -- '*/go.mod' | sort)
if [[ "${#all_modules[@]}" -eq 0 ]]; then
  echo "modules=[]" >>"$GITHUB_OUTPUT"
  exit 0
fi

# non-test .go files). golangci-lint cannot analyse these and would fail with
modules=()
for mod in "${all_modules[@]}"; do
  dir=$(dirname "$mod")
  if git ls-files -- "${dir}/"'*.go' | grep -qv '_test\.go$'; then
    modules+=("$mod")
  fi
done

if [[ "${#modules[@]}" -eq 0 ]]; then
  echo "modules=[]" >>"$GITHUB_OUTPUT"
  exit 0
fi

json=$(printf '%s\n' "${modules[@]}" | sed 's|/go\.mod$||' | jq -R -s -c 'split("\n") | map(select(length > 0))')
echo "modules=$json" >>"$GITHUB_OUTPUT"
