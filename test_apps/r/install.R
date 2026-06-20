# alef-generated installer for registry-mode R test_app.
# Installs the configured R package from GitHub releases.
# Requires `R` on PATH.

# Version override: pass as commandArgs()[6] to test an
# arbitrary tag; defaults to the alef-pinned version from
# [crates.e2e.registry.packages.r].version.
args <- commandArgs(trailingOnly = TRUE)
VERSION <- if (length(args) > 0) args[1] else "3.6.18"

# Construct the GitHub release tarball URL.
url <- sprintf(
  "https://github.com/kreuzberg-dev/html-to-markdown/releases/download/v%s/htmltomarkdown_%s.tar.gz",
  VERSION,
  VERSION
)

# Install from the release tarball without requiring devtools or remotes.
tryCatch({
  install.packages(url, repos = NULL, type = "source", quiet = TRUE)
  message(paste("Successfully installed htmltomarkdown", VERSION))
}, error = function(e) {
  message(paste("Error installing htmltomarkdown from", url))
  message(conditionMessage(e))
  quit(status = 1)
})
