#!/usr/bin/env bash
set -euo pipefail

if command -v apt-get >/dev/null 2>&1; then
  sudo apt-get update
  sudo apt-get install -y --no-install-recommends \
    libuv1-dev \
    libgit2-dev \
    libssl-dev \
    libcurl4-openssl-dev \
    libxml2-dev \
    libfontconfig1-dev \
    libfreetype6-dev \
    libharfbuzz-dev \
    libfribidi-dev
fi

# Pick a CRAN mirror. `type = "binary"` only works on macOS / Windows — CRAN
# serves no Linux binaries. Posit Package Manager (PPM) does, so on Linux we
# route to PPM keyed on the distro codename, which serves pre-built binaries
# without needing `type = "binary"`.
if [[ "$(uname -s)" == "Linux" ]] && [[ -r /etc/os-release ]]; then
  # shellcheck disable=SC1091
  . /etc/os-release
  REPO_URL="https://packagemanager.posit.co/cran/__linux__/${VERSION_CODENAME:-jammy}/latest"
  INSTALL_TYPE='"source"' # PPM serves binaries; type='source' avoids a Linux-binary lookup
else
  REPO_URL="https://cran.r-project.org"
  INSTALL_TYPE='"binary"'
fi

Rscript -e "options(repos = c(CRAN = '${REPO_URL}')); for (pkg in c('devtools', 'testthat', 'rextendr', 'lintr', 'styler', 'covr', 'remotes')) { if (!requireNamespace(pkg, quietly = TRUE)) install.packages(pkg, type = ${INSTALL_TYPE}) }"
