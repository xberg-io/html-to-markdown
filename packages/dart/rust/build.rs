use std::path::Path;

fn main() {
    // Re-run whenever any Rust source changes.
    println!("cargo:rerun-if-changed=src");

    // Optional FRB codegen: regenerate flutter_rust_bridge artifacts when the
    // tool is on PATH. Missing tool is not fatal — committed generated sources
    // are checked in, and CI environments without FRB still build cleanly.
    match std::process::Command::new("flutter_rust_bridge_codegen")
        .args(["generate", "--config-file", "flutter_rust_bridge.yaml"])
        .status()
    {
        Ok(status) if status.success() => {
            // FRB v2.12+ emits `use` lists in an order rustfmt 2024 edition rewrites
            // (e.g. `{transform_result_dco, Lifetimeable, Lockable}` →
            // `{Lifetimeable, Lockable, transform_result_dco}`). Run rustfmt against
            // the generated file so committed output is fmt-clean and `cargo fmt --check`
            // stays green in CI.
            match std::process::Command::new("rustfmt")
                .args(["--edition", "2024", "src/frb_generated.rs"])
                .status()
            {
                Ok(s) if s.success() => {}
                Ok(s) => println!("cargo:warning=rustfmt on src/frb_generated.rs exited {s}"),
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                    println!(
                        "cargo:warning=rustfmt not on PATH — skipping post-FRB format. Install rustfmt via rustup to keep generated bridge sources fmt-clean."
                    );
                }
                Err(err) => println!("cargo:warning=failed to spawn rustfmt: {err}"),
            }

            // Patch the generated Dart entrypoint so the published package resolves
            // its native library from its own installed location.
            patch_published_loader();

            // Rewrite FRB-generated handler.executeSync/handler.executeNormal calls
            // into direct handler invocations. FRB 2.x emits these calls assuming
            // `handler` is a BaseHandler field, but in service-API methods `handler`
            // is a user-supplied function parameter (FutureOr<R> Function(T)) which
            // does not expose those methods, so the generated Dart fails to compile.
            // The rewrite is idempotent (marker-gated) and runs after every FRB
            // invocation — including the rebuild that fires during `dart pub get`
            // in e2e flows, which is when this otherwise reverts.
            fix_handler_executor_calls();
        }
        Ok(status) => panic!("flutter_rust_bridge_codegen generate failed (exit code: {status})"),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            println!(
                "cargo:warning=flutter_rust_bridge_codegen not on PATH — skipping codegen. Install via `cargo install flutter_rust_bridge_codegen --locked` to regenerate FRB artifacts at build time."
            );
        }
        Err(err) => panic!("failed to spawn flutter_rust_bridge_codegen: {err}"),
    }
}

const FRB_GENERATED_DART: &str = "../lib/src/html_to_markdown_rs_bridge_generated/frb_generated.dart";
const FRB_HANDLER_EXECUTOR_MARKER: &str = "handler.executeSync(";
const LOADER_MARKER: &str = "_alefResolveExternalLibrary";
const FRB_INIT_PROLOGUE: &str = "  /// Initialize flutter_rust_bridge\n  static Future<void> init({\n    RustLibApi? api,\n    BaseHandler? handler,\n    ExternalLibrary? externalLibrary,\n    bool forceSameCodegenVersion = true,\n  }) async {\n";
const FRB_INIT_REPLACEMENT: &str = r#"  /// Resolve the prebuilt native library from this package's own installed
  /// location so the load works from any working directory and under hardened
  /// runtimes. Returns `null` to defer to flutter_rust_bridge's default loader.
  ///
  /// Published pub.dev packages stage natives under `lib/src/native/<rid>/`
  /// (e.g. `macos-arm64`, `linux-x64`). For local FRB-dev builds the dylib is
  /// emitted into `lib/src/html_to_markdown_rs_bridge_generated/`; that
  /// path is searched as a fallback.
  static Future<ExternalLibrary?> _alefResolveExternalLibrary() async {
    try {
      final packageRoot =
          await Isolate.resolvePackageUri(Uri.parse('package:h2m/h2m.dart'));
      if (packageRoot == null) return null;
      final libNames = _alefHostLibNames();
      final searchDirs = <Uri>[
        if (_alefHostRid() != null) packageRoot.resolve('src/native/${_alefHostRid()}/'),
        packageRoot.resolve('src/html_to_markdown_rs_bridge_generated/'),
      ];
      for (final dir in searchDirs) {
        for (final name in libNames) {
          final libPath = dir.resolve(name).toFilePath();
          if (File(libPath).existsSync()) {
            return ExternalLibrary.open(libPath);
          }
        }
      }
    } catch (_) {
      // Fall through to the default loader on any resolution failure.
    }
    return null;
  }

  /// Map the host platform to the pub.dev native staging RID. Returns `null`
  /// for unrecognized host triples so the FRB-dev fallback path runs instead.
  static String? _alefHostRid() {
    final abi = Abi.current();
    if (abi == Abi.macosArm64) return 'macos-arm64';
    if (abi == Abi.macosX64) return 'macos-x64';
    if (abi == Abi.linuxArm64) return 'linux-arm64';
    if (abi == Abi.linuxX64) return 'linux-x64';
    if (abi == Abi.windowsArm64) return 'windows-arm64';
    if (abi == Abi.windowsX64) return 'windows-x64';
    return null;
  }

  static List<String> _alefHostLibNames() {
    // The Dart-binding Rust crate is `{stem}-dart` (per the cargo manifest
    // template), which produces a cdylib named `lib{stem}_dart.{ext}` on Unix
    // and `{stem}_dart.dll` on Windows. On macOS, pub.dev-published packages
    // may ship the binary as a Framework bundle (preferred modern packaging)
    // — list that first so the loader finds it before the bare dylib.
    if (Platform.isMacOS)
      return const [
        'html_to_markdown_rs_dart.framework/html_to_markdown_rs_dart',
        'libhtml_to_markdown_rs_dart.dylib',
      ];
    if (Platform.isWindows) return const ['html_to_markdown_rs_dart.dll'];
    return const ['libhtml_to_markdown_rs_dart.so'];
  }

  /// Initialize flutter_rust_bridge
  static Future<void> init({
    RustLibApi? api,
    BaseHandler? handler,
    ExternalLibrary? externalLibrary,
    bool forceSameCodegenVersion = true,
  }) async {
    externalLibrary ??= await _alefResolveExternalLibrary();
"#;

/// Inject the published-package native-library loader into `frb_generated.dart`.
/// Idempotent: a no-op when the marker is already present or the FRB entrypoint
/// signature is absent.
fn patch_published_loader() {
    let path = Path::new(FRB_GENERATED_DART);
    let Ok(source) = std::fs::read_to_string(path) else {
        println!(
            "cargo:warning=published-loader patch skipped: {} not found",
            FRB_GENERATED_DART
        );
        return;
    };
    if source.contains(LOADER_MARKER) {
        return;
    }
    if !source.contains(FRB_INIT_PROLOGUE) {
        println!("cargo:warning=published-loader patch skipped: FRB init prologue not found");
        return;
    }

    let mut patched = source.replacen(FRB_INIT_PROLOGUE, FRB_INIT_REPLACEMENT, 1);

    // Ensure the helper's `File`/`Isolate`/`Abi` dependencies are imported.
    for (probe, line) in [
        ("import 'dart:io';", "import 'dart:io';\n"),
        ("import 'dart:isolate';", "import 'dart:isolate';\n"),
        ("import 'dart:ffi';", "import 'dart:ffi';\n"),
    ] {
        if patched.contains(probe) {
            continue;
        }
        if let Some(pos) = patched.find("\nimport ") {
            patched.insert_str(pos + 1, line);
        } else {
            patched.insert_str(0, line);
        }
    }

    if patched != source {
        if let Err(err) = std::fs::write(path, &patched) {
            println!("cargo:warning=failed to write published-loader patch: {err}");
            return;
        }
        match std::process::Command::new("dart")
            .args(["format", FRB_GENERATED_DART])
            .status()
        {
            Ok(s) if s.success() => {}
            Ok(s) => println!("cargo:warning=dart format on {} exited {}", FRB_GENERATED_DART, s),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                println!(
                    "cargo:warning=dart not on PATH — skipping post-patch format. Install Dart SDK to keep generated FRB Dart sources fmt-clean."
                );
            }
            Err(err) => println!("cargo:warning=failed to spawn dart format: {err}"),
        }
    }
}

/// Rewrite FRB-emitted `handler.executeSync(SyncTask(...))` calls into
/// `SyncTask(...).executeSync()` form, but ONLY inside service-API methods
/// whose signature contains a callback-function parameter (`Function(`).
///
/// Methods without a callback parameter invoke the inherited `BaseHandler`
/// class field — which IS callable as `.executeSync(task)` and must be left
/// untouched. The pre-rename build-script did an unconditional global string
/// replace and corrupted those class-field calls; this scope-aware version
/// only fires where the rewrite is needed.
///
/// Idempotent: if no `handler.executeSync(` marker is present, exits early.
#[allow(clippy::collapsible_if)]
fn fix_handler_executor_calls() {
    let path = Path::new(FRB_GENERATED_DART);
    let Ok(source) = std::fs::read_to_string(path) else {
        return;
    };
    if !source.contains(FRB_HANDLER_EXECUTOR_MARKER) {
        return;
    }

    let lines: Vec<&str> = source.lines().collect();
    let mut out = String::with_capacity(source.len());
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim_start();
        let is_func_start = !trimmed.is_empty()
            && !trimmed.starts_with("//")
            && !trimmed.starts_with("class ")
            && !trimmed.starts_with("abstract class ")
            && !trimmed.starts_with("mixin ")
            && !trimmed.starts_with('}')
            && !trimmed.starts_with(')')
            && !trimmed.starts_with(']')
            && line.contains('(');
        if !is_func_start {
            out.push_str(line);
            out.push('\n');
            i += 1;
            continue;
        }
        let start = i;
        let mut paren: i32 = 0;
        let mut brace: i32 = 0;
        let mut body_started = false;
        while i < lines.len() {
            let l = lines[i];
            for c in l.chars() {
                match c {
                    '(' => paren += 1,
                    ')' => paren -= 1,
                    '{' => {
                        brace += 1;
                        body_started = true;
                    }
                    '}' => brace -= 1,
                    _ => {}
                }
            }
            i += 1;
            if body_started && brace <= 0 {
                break;
            }
        }
        let block_text = lines[start..i].join("\n");
        let rewritten = if block_text.contains("Function(") {
            rewrite_executor_to_task(&block_text)
        } else {
            block_text
        };
        out.push_str(&rewritten);
        out.push('\n');
    }
    if !source.ends_with('\n') && out.ends_with('\n') {
        out.pop();
    }
    if out != source {
        if let Err(err) = std::fs::write(path, &out) {
            println!("cargo:warning=failed to write handler-executor rewrite: {err}");
        }
    }
}

fn rewrite_executor_to_task(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let mut cursor = 0;
    while let Some((rel, method)) = next_executor_call(&src[cursor..]) {
        let start = cursor + rel;
        let open = start + "handler.".len() + method.len();
        let Some(close) = matching_paren(src, open) else {
            break;
        };
        out.push_str(&src[cursor..start]);
        let inner = src[open + 1..close].trim();
        let inner = inner.strip_suffix(',').map(str::trim_end).unwrap_or(inner);
        out.push_str(inner);
        out.push('.');
        out.push_str(method);
        out.push_str("()");
        cursor = close + 1;
    }
    out.push_str(&src[cursor..]);
    out
}

fn next_executor_call(src: &str) -> Option<(usize, &'static str)> {
    let s = src.find("handler.executeSync(");
    let n = src.find("handler.executeNormal(");
    match (s, n) {
        (Some(a), Some(b)) if a <= b => Some((a, "executeSync")),
        (Some(_), Some(b)) => Some((b, "executeNormal")),
        (Some(a), None) => Some((a, "executeSync")),
        (None, Some(b)) => Some((b, "executeNormal")),
        (None, None) => None,
    }
}

fn matching_paren(src: &str, open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (offset, ch) in src[open..].char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(open + offset);
                }
            }
            _ => {}
        }
    }
    None
}
