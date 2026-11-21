source("tools/msrv.R")

env_debug <- Sys.getenv("DEBUG")
env_not_cran <- Sys.getenv("NOT_CRAN")

vendor_exists <- file.exists("src/rust/vendor.tar.xz")

is_not_cran <- nzchar(env_not_cran)
is_debug <- nzchar(env_debug)

if (is_debug) {
  is_not_cran <- TRUE
  message("Creating DEBUG build.")
}

if (!is_not_cran) {
  message("Building for CRAN.")
}

.cran_flags <- ifelse(
  !is_not_cran && vendor_exists,
  "-j 2 --offline",
  ""
)

.vendoring <- ifelse(!is_not_cran && vendor_exists, "yes", "no")

.profile <- ifelse(is_debug, "", "--release")
.clean_targets <- ifelse(is_debug, "", "$(TARGET_DIR)")

webr_target <- "wasm32-unknown-emscripten"
is_wasm <- identical(R.version$platform, webr_target)

target_libpath <- if (is_wasm) "wasm32-unknown-emscripten" else NULL
cfg <- if (is_debug) "debug" else "release"
.libdir <- paste(c(target_libpath, cfg), collapse = "/")
.target <- ifelse(is_wasm, paste0("--target=", webr_target), "")

is_windows <- .Platform[["OS.type"]] == "windows"
mv_fp <- ifelse(is_windows, "src/Makevars.win.in", "src/Makevars.in")
mv_ofp <- ifelse(is_windows, "src/Makevars.win", "src/Makevars")

if (file.exists(mv_ofp)) {
  invisible(file.remove(mv_ofp))
}

mv_txt <- readLines(mv_fp)

new_txt <- gsub("@CRAN_FLAGS@", .cran_flags, mv_txt) |>
  gsub("@VENDORING@", .vendoring, x = _) |>
  gsub("@PROFILE@", .profile, x = _) |>
  gsub("@CLEAN_TARGET@", .clean_targets, x = _) |>
  gsub("@LIBDIR@", .libdir, x = _) |>
  gsub("@TARGET@", .target, x = _)

con <- file(mv_ofp, open = "wb")
writeLines(new_txt, con, sep = "\n")
close(con)

message(sprintf("Generated %s (vendoring=%s)", mv_ofp, .vendoring))
