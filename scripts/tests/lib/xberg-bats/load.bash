# shellcheck shell=bash
# Shared Bats helpers for xberg-io script suites.
#
# Load with `bats_load_library xberg-bats`, which resolves against $BATS_LIB_PATH.
#
# Consumer repositories carry a COMMITTED copy of this file at tests/lib/xberg-bats/load.bash
# rather than fetching it at test time. Two reasons, both learned the hard way:
#
#   * install-bats refuses to run outside GitHub Actions (it writes $GITHUB_PATH), so a library
#     hosted inside the action would make `bats scripts/tests` impossible on a laptop.
#   * CI and the developer must load the same bytes. A helper resolved differently in the two
#     places reintroduces exactly the local-green/CI-red class that shadow_system_path_without
#     exists to kill.
#
# Every symbol is xberg_-prefixed so a suite that still defines its own `stub`/`make_stub` during
# migration keeps working unchanged.

# Bumped on any breaking change to the API below, so a stale vendored copy fails loudly
# instead of silently missing a helper. ~keep
export XBERG_BATS_LIB_VERSION=1

# Minimum number of commands a shadowed system mirror must contain. A mirror of nothing would
# make every test in the file pass for the wrong reason -- the script under test would find none
# of the utilities it needs and fail for an unrelated cause. Same idea as the enterprise
# reachability gate's MINIMUM_COMMAND_PATHS. ~keep
: "${XBERG_SYS_BIN_MINIMUM:=40}"

# --------------------------------------------------------------------------------------------
# Fixtures
# --------------------------------------------------------------------------------------------

# Create the four GitHub Actions workflow-command files and export their paths.
#
# They are created AND truncated: a script that appends to $GITHUB_OUTPUT must be asserted
# against its own writes, not against whatever a previous test in the same file left behind.
xberg_setup_github_env() {
	local github_dir="$BATS_TEST_TMPDIR/github"
	mkdir -p "$github_dir"

	export GITHUB_OUTPUT="$github_dir/output"
	export GITHUB_ENV="$github_dir/env"
	export GITHUB_PATH="$github_dir/path"
	export GITHUB_STEP_SUMMARY="$github_dir/step-summary"

	: >"$GITHUB_OUTPUT"
	: >"$GITHUB_ENV"
	: >"$GITHUB_PATH"
	: >"$GITHUB_STEP_SUMMARY"
}

# Export a workspace directory, mirroring what a checkout gives an action.
xberg_setup_workspace() {
	local name="${1:-workspace}"
	export XBERG_WORKSPACE="$BATS_TEST_TMPDIR/$name"
	export GITHUB_WORKSPACE="$XBERG_WORKSPACE"
	mkdir -p "$XBERG_WORKSPACE"
}

# The common case: a stub directory PREPENDED to the host PATH, plus the workflow files.
#
# Use this when the test is about what the script DOES and the host's real utilities are fine.
# When the script instead PROBES for a command (`command -v gh`) and the branch it takes is the
# thing under test, use xberg_setup_isolated + xberg_shadow_system_path_without: prepending
# cannot express "the host does not supply gh", and the probe will find the runner's copy.
xberg_setup() {
	xberg_setup_isolated
	export PATH="$XBERG_STUB_BIN:$PATH"
}

# Same fixtures, but PATH is left alone so the caller can build an isolated one.
xberg_setup_isolated() {
	export XBERG_STUB_BIN="$BATS_TEST_TMPDIR/stub-bin"
	export XBERG_WORK="$BATS_TEST_TMPDIR/work"
	# Path only -- deliberately not created. A trace file that exists before the run cannot
	# distinguish "the command was never called" from "the command wrote nothing". ~keep
	export XBERG_TRACE="$BATS_TEST_TMPDIR/trace"

	mkdir -p "$XBERG_STUB_BIN" "$XBERG_WORK"
	xberg_setup_github_env
}

# --------------------------------------------------------------------------------------------
# Stubs
# --------------------------------------------------------------------------------------------

# Write an executable stub into $XBERG_STUB_BIN. Body lines are passed as arguments.
#
# The shebang is supplied here; `set -eu` deliberately is NOT. A stub's job is to emit canned
# output and an exit code, and errexit turns an intentionally-false test (`[ "$1" = run ]`) into
# a surprise exit that looks like the script under test misbehaved. A stub that genuinely wants
# errexit can open its body with it.
xberg_stub() {
	local name="$1"
	shift

	printf '%s\n' '#!/usr/bin/env bash' "$@" >"$XBERG_STUB_BIN/$name"
	chmod +x "$XBERG_STUB_BIN/$name"
}

# A stub that does nothing but exit with the given status.
xberg_stub_exit() {
	xberg_stub "$1" "exit ${2:-0}"
}

# A stub that appends its whole invocation to $XBERG_TRACE, then succeeds.
#
# Pair with xberg_assert_trace to assert the complete ordered sequence of calls as one string.
# Asserting the trace catches an argument that silently stopped being passed, which asserting
# only the final exit status never will.
xberg_stub_trace() {
	local name="$1" label="${2:-$1}"
	xberg_stub "$name" \
		"printf '%s %s\\n' '$label' \"\$*\" >>\"\$XBERG_TRACE\"" \
		'exit 0'
}

# A curl that fails loudly if anything invokes it.
#
# This is how a test proves a code path is offline -- a cache hit, an early return, an
# already-installed short circuit. Asserting "the output looks right" cannot distinguish a cache
# hit from a silent re-download; a curl that cannot run can.
xberg_stub_curl_offline() {
	xberg_stub curl \
		'printf "%s\n" "curl must not run on this code path" >&2' \
		'exit 99'
}

# A curl that writes the given text to its --output/-o target, or to stdout when there is none.
# shellcheck disable=SC2016  # the stub body must expand when the STUB runs, not now
xberg_stub_curl_body() {
	local body="$1"
	export XBERG_CURL_BODY="$body"
	xberg_stub curl \
		'target=""' \
		'for ((i = 1; i <= $#; i++)); do' \
		'  case "${!i}" in' \
		'    --output | -o) next=$((i + 1)); target="${!next}" ;;' \
		'  esac' \
		'done' \
		'if [ -n "$target" ]; then' \
		'  printf "%s" "$XBERG_CURL_BODY" >"$target"' \
		'else' \
		'  printf "%s" "$XBERG_CURL_BODY"' \
		'fi' \
		'exit 0'
}

# A curl that copies a real file to its --output/-o target.
#
# Generalises the trick in ensure-gh's suite: the fixture is a genuine archive built in setup,
# so the script's `tar` handling is exercised for real rather than mocked away. That is how a
# GNU-tar-shells-out-to-gzip class of bug gets caught.
# shellcheck disable=SC2016  # the stub body must expand when the STUB runs, not now
xberg_stub_curl_file() {
	local source_path="$1"
	export XBERG_CURL_FILE="$source_path"
	xberg_stub curl \
		'for ((i = 1; i <= $#; i++)); do' \
		'  case "${!i}" in' \
		'    --output | -o)' \
		'      next=$((i + 1))' \
		'      /bin/cp "$XBERG_CURL_FILE" "${!next}"' \
		'      exit 0' \
		'      ;;' \
		'  esac' \
		'done' \
		'exit 1'
}

# --------------------------------------------------------------------------------------------
# Host isolation
# --------------------------------------------------------------------------------------------

# Record the absolute path of real commands BEFORE the PATH is shadowed.
#
# A stub that must pass through to the genuine tool for out-of-scope calls needs the real path;
# resolving it after shadowing finds the stub and recurses forever.
xberg_capture_real() {
	local name resolved variable
	for name in "$@"; do
		resolved="$(command -v "$name" || true)"
		[ -n "$resolved" ] || {
			printf 'xberg_capture_real: %s is not on PATH\n' "$name" >&2
			return 1
		}
		variable="XBERG_REAL_$(printf '%s' "$name" | tr '[:lower:]-' '[:upper:]_')"
		export "$variable=$resolved"
	done
}

# Mirror the host's standard command directories into a private bin, MINUS the named commands.
#
# Call from setup_file: the mirror is read-only and identical for every test in the file, and
# rebuilding a few thousand symlinks per test is pure wall-clock.
#
# An allow-list was tried first and was the wrong shape. What these tests need is not "the script
# may use exactly these ten utilities" -- that guesses at an implementation detail and breaks the
# moment the script reaches for one more. What they need is "the host does not supply gh", with
# an otherwise realistic system underneath. Naming the excluded command states that directly.
#
# The alternative, PATH="$STUB_BIN:/usr/bin:/bin", states nothing: a script that decides whether
# to download by probing `command -v gh` finds the runner's copy on ubuntu and not on macOS, so
# the probe takes the wrong branch on one of them -- green locally, red in CI. ~keep
xberg_shadow_system_path_without() {
	[ "$#" -gt 0 ] || {
		printf 'xberg_shadow_system_path_without: name at least one command to exclude\n' >&2
		return 1
	}

	export XBERG_SYS_BIN="${BATS_FILE_TMPDIR:-$BATS_TEST_TMPDIR}/sys-bin"
	mkdir -p "$XBERG_SYS_BIN"

	local excluded=" $* " directory source name
	for directory in /usr/local/bin /usr/bin /bin /usr/sbin /sbin; do
		[ -d "$directory" ] || continue
		for source in "$directory"/*; do
			[ -x "$source" ] || continue
			name="${source##*/}"
			case "$excluded" in *" $name "*) continue ;; esac
			[ -e "$XBERG_SYS_BIN/$name" ] || ln -s "$source" "$XBERG_SYS_BIN/$name"
		done
	done

	# The whole point of the mirror is that these commands are missing from it, and a silent
	# leak would put every test in the file back on the host's copy without failing. Assert the
	# precondition rather than assuming it. ~keep
	for name in "$@"; do
		[ ! -e "$XBERG_SYS_BIN/$name" ] || {
			printf 'xberg_shadow_system_path_without: mirror leaked %s\n' "$name" >&2
			return 1
		}
	done

	# A mirror that came out empty would also satisfy the loop above, and every test would then
	# pass because the script found none of the utilities it needs -- the right answer for the
	# wrong reason. ~keep
	local mirrored
	mirrored="$(find "$XBERG_SYS_BIN" -maxdepth 1 -type l | wc -l | tr -d '[:space:]')"
	[ "$mirrored" -ge "$XBERG_SYS_BIN_MINIMUM" ] || {
		printf 'xberg_shadow_system_path_without: mirrored only %s commands, expected >= %s\n' \
			"$mirrored" "$XBERG_SYS_BIN_MINIMUM" >&2
		return 1
	}
}

# The PATH to hand a script under test: stubs first, then the shadowed system mirror.
xberg_isolated_path() {
	printf '%s:%s' "$XBERG_STUB_BIN" "$XBERG_SYS_BIN"
}

# --------------------------------------------------------------------------------------------
# Assertions
#
# Each prints a labelled expected/actual block to stderr and returns 1, so a failure reads as a
# diff instead of `[ "$output" = ... ] returned 1`.
# --------------------------------------------------------------------------------------------

xberg__fail_compare() {
	local label="$1" expected="$2" actual="$3"
	printf '%s mismatch\n--- expected ---\n%s\n--- actual ---\n%s\n' \
		"$label" "$expected" "$actual" >&2
	return 1
}

# $status and $output are set by bats' `run`, not by this file. ~keep
# shellcheck disable=SC2154
xberg_assert_status() {
	[ "$status" -eq "$1" ] || xberg__fail_compare "status" "$1" "$status"
}

# shellcheck disable=SC2154
xberg_assert_output() {
	[ "$output" = "$1" ] || xberg__fail_compare "output" "$1" "$output"
}

# Assert stdout equals the given lines joined by newlines.
#
# Replaces the $'first\nsecond\nthird' idiom: identical byte-exactness, but the expectation is
# readable as arguments instead of an ANSI-C quoting puzzle.
# shellcheck disable=SC2154
xberg_assert_lines() {
	local expected
	expected="$(printf '%s\n' "$@")"
	[ "$output" = "$expected" ] || xberg__fail_compare "output" "$expected" "$output"
}

# shellcheck disable=SC2154
xberg_assert_output_contains() {
	case "$output" in
	*"$1"*) ;;
	*) xberg__fail_compare "output (substring)" "$1" "$output" ;;
	esac
}

# shellcheck disable=SC2154
xberg_assert_no_output() {
	[ -z "$output" ] || xberg__fail_compare "output" "(empty)" "$output"
}

xberg_assert_file() {
	local path="$1" expected="$2" actual
	[ -f "$path" ] || {
		printf 'expected file to exist: %s\n' "$path" >&2
		return 1
	}
	actual="$(cat "$path")"
	[ "$actual" = "$expected" ] || xberg__fail_compare "contents of $path" "$expected" "$actual"
}

xberg_assert_file_absent() {
	[ ! -e "$1" ] || {
		printf 'expected no file at: %s\n--- contents ---\n%s\n' "$1" "$(cat "$1" 2>/dev/null)" >&2
		return 1
	}
}

xberg_assert_github_output() { xberg_assert_file "$GITHUB_OUTPUT" "$1"; }
xberg_assert_github_env() { xberg_assert_file "$GITHUB_ENV" "$1"; }
xberg_assert_github_path() { xberg_assert_file "$GITHUB_PATH" "$1"; }
xberg_assert_github_summary() { xberg_assert_file "$GITHUB_STEP_SUMMARY" "$1"; }

# Assert the complete ordered call trace recorded by xberg_stub_trace.
xberg_assert_trace() {
	local expected
	expected="$(printf '%s\n' "$@")"
	xberg_assert_file "$XBERG_TRACE" "$expected"
}

# Assert no traced command ran at all.
xberg_assert_trace_empty() {
	xberg_assert_file_absent "$XBERG_TRACE"
}
