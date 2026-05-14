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

Rscript -e 'for (pkg in c("devtools", "testthat", "rextendr", "lintr", "styler", "covr", "remotes")) { if (!requireNamespace(pkg, quietly = TRUE)) install.packages(pkg, repos = "https://cran.r-project.org") }'
