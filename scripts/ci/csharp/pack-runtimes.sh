#!/usr/bin/env bash
set -euo pipefail

version="${1:?release version is required}"
output="${2:?package output directory is required}"

# Use the same concrete RID graph consumers restore; #import entries are aliases.
rids="$(python3 - <<'PY'
import json
import xml.etree.ElementTree as ET
from pathlib import Path

project = Path('packages/csharp/HtmlToMarkdown')
package_id = ET.parse(project / 'HtmlToMarkdown.csproj').findtext('./PropertyGroup/PackageId')
graph = json.loads((project / 'runtime.json.template').read_text())['runtimes']
rids = sorted(rid for rid, dependencies in graph.items() if package_id in dependencies)
if not rids:
    raise SystemExit('No concrete NuGet runtime packages found in the release graph')
print('\n'.join(rids))
PY
)"

while IFS= read -r rid; do
  dotnet pack packages/csharp/HtmlToMarkdown.Runtime/HtmlToMarkdown.Runtime.csproj \
    -c Release "-p:Version=${version}" "-p:PublishedRID=${rid}" -o "$output"
done <<< "$rids"
