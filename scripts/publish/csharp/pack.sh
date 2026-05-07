#!/usr/bin/env bash
set -euo pipefail

if [[ -d "dist/csharp-ffi" ]]; then
  scripts/publish/csharp/stage-ffi.sh "dist/csharp-ffi" "packages/csharp/HtmlToMarkdown"
fi

# Pack the inner csproj — it owns the source compilation and now owns the
# `KreuzbergDev.HtmlToMarkdown` PackageId. The outer wrapper csproj is dead
# weight (left in place for IDE compatibility but no longer used here).
dotnet pack packages/csharp/HtmlToMarkdown/HtmlToMarkdown.csproj --configuration Release --output artifacts/csharp
