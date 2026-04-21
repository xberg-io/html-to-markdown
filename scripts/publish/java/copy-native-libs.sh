#!/usr/bin/env bash
set -euo pipefail

# This script copies downloaded native FFI libraries into the Maven resources directory
# so they can be bundled in the published JAR

artifacts_dir="${1:?ARTIFACTS_DIR is required (e.g. java-ffi-artifacts)}"
resources_dir="packages/java/src/main/resources/natives"

echo "Copying native libraries from ${artifacts_dir} to ${resources_dir}"

mkdir -p "${resources_dir}"

# Remove any existing native libraries
if [[ -d "${resources_dir:?}" ]]; then
  rm -rf "${resources_dir:?}"/*
fi

# Copy all platform-specific native libraries
for platform_dir in "${artifacts_dir}"/*; do
  if [[ -d "${platform_dir}" ]]; then
    platform_name="$(basename "${platform_dir}")"
    echo "  Copying ${platform_name}..."
    mkdir -p "${resources_dir}/${platform_name}"
    cp -r "${platform_dir}/native/." "${resources_dir}/${platform_name}/"
  fi
done

echo "Native libraries copied successfully:"
find "${resources_dir}" -type f -name "*.so" -o -name "*.dylib" -o -name "*.dll" | while read -r lib; do
  echo "  - ${lib}"
  ls -lh "${lib}"
done
