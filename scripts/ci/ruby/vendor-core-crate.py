#!/usr/bin/env python3
"""
Vendor html-to-markdown-rs core crate into Ruby package.
Used by: ci-ruby.yaml, publish.yaml - Vendor core crate step

This script:
1. Reads workspace.dependencies from root Cargo.toml
2. Copies core crate to packages/ruby/vendor/html-to-markdown-rs/
3. Replaces workspace = true with explicit versions
4. Generates vendor/Cargo.toml with proper workspace setup
5. Updates native extension Cargo.toml to use vendored crate
"""

import os
import re
import shutil
import sys
from pathlib import Path

try:
    import tomllib
except ImportError:
    import tomli as tomllib  # type: ignore


def get_repo_root() -> Path:
    """Get repository root directory."""
    repo_root_env = os.environ.get("REPO_ROOT")
    if repo_root_env:
        return Path(repo_root_env)

    script_dir = Path(__file__).parent.absolute()
    return (script_dir / ".." / ".." / "..").resolve()


def read_toml(path: Path) -> dict[str, object]:
    """Read TOML file."""
    with open(path, "rb") as f:
        return tomllib.load(f)


def get_workspace_deps(repo_root: Path) -> dict[str, object]:
    """Extract workspace.dependencies from root Cargo.toml."""
    cargo_toml_path = repo_root / "Cargo.toml"
    data = read_toml(cargo_toml_path)
    return data.get("workspace", {}).get("dependencies", {})


def get_workspace_version(repo_root: Path) -> str:
    """Extract version from workspace.package."""
    cargo_toml_path = repo_root / "Cargo.toml"
    data = read_toml(cargo_toml_path)
    return data.get("workspace", {}).get("package", {}).get("version", "0.0.0")


def get_workspace_metadata(repo_root: Path) -> dict[str, str]:
    """Extract metadata from workspace.package."""
    cargo_toml_path = repo_root / "Cargo.toml"
    data = read_toml(cargo_toml_path)
    pkg = data.get("workspace", {}).get("package", {})
    return {
        "version": pkg.get("version", "0.0.0"),
        "edition": pkg.get("edition", "2024"),
        "rust-version": pkg.get("rust-version", "1.85"),
        "authors": pkg.get("authors", ["Na'aman Hirschfeld <naaman@kreuzberg.dev>"]),
        "license": pkg.get("license", "MIT"),
        "repository": pkg.get("repository", "https://github.com/kreuzberg-dev/html-to-markdown"),
        "homepage": pkg.get("homepage", "https://kreuzberg.dev"),
    }


def format_dependency(name: str, dep_spec: object) -> str:
    """Format a dependency spec for Cargo.toml."""
    if isinstance(dep_spec, str):
        return f'{name} = "{dep_spec}"'
    if isinstance(dep_spec, dict):
        version: str = dep_spec.get("version", "")
        package: str | None = dep_spec.get("package")
        features: list[str] = dep_spec.get("features", [])
        default_features: bool | None = dep_spec.get("default-features")
        optional: bool | None = dep_spec.get("optional")
        path: str | None = dep_spec.get("path")

        parts: list[str] = []

        if package:
            parts.append(f'package = "{package}"')
        if path:
            parts.append(f'path = "{path}"')
        if version:
            parts.append(f'version = "{version}"')
        if features:
            features_str = ", ".join(f'"{f}"' for f in features)
            parts.append(f"features = [{features_str}]")
        if default_features is False:
            parts.append("default-features = false")
        elif default_features is True:
            parts.append("default-features = true")
        if optional is True:
            parts.append("optional = true")
        elif optional is False:
            parts.append("optional = false")

        spec_str = ", ".join(parts)
        return f"{name} = {{ {spec_str} }}"

    return f'{name} = "{dep_spec}"'


def replace_workspace_deps_in_toml(toml_path: Path, workspace_deps: dict[str, object]) -> None:
    """Replace workspace = true with explicit versions in a Cargo.toml file."""
    with open(toml_path) as f:
        content = f.read()

    for name, dep_spec in workspace_deps.items():
        # Dotted key: name.workspace = true
        pattern0 = rf"^{re.escape(name)}\.workspace = true$"
        content = re.sub(pattern0, format_dependency(name, dep_spec), content, flags=re.MULTILINE)

        # Simple: name = { workspace = true }
        pattern1 = rf"^{re.escape(name)} = \{{ workspace = true \}}$"
        content = re.sub(pattern1, format_dependency(name, dep_spec), content, flags=re.MULTILINE)

        # Complex: name = { workspace = true, ... }
        def replace_with_fields(match: re.Match[str]) -> str:
            other_fields_str = match.group(1).strip()
            base_spec = format_dependency(name, dep_spec)
            if " = { " not in base_spec:
                version_val = base_spec.split(" = ", 1)[1].strip('"')
                spec_part = f'version = "{version_val}"'
            else:
                spec_part = base_spec.split(" = { ", 1)[1].rstrip("} ").rstrip("}")

            # Extract existing keys from workspace spec using bracket-aware parsing
            workspace_fields: dict[str, str] = {}
            bracket_depth = 0
            current_field = ""
            for char in spec_part:
                if char == "[":
                    bracket_depth += 1
                    current_field += char
                elif char == "]":
                    bracket_depth -= 1
                    current_field += char
                elif char == "," and bracket_depth == 0:
                    field = current_field.strip()
                    if field and "=" in field:
                        key, val = field.split("=", 1)
                        workspace_fields[key.strip()] = val.strip()
                    current_field = ""
                else:
                    current_field += char

            if current_field.strip():
                field = current_field.strip()
                if field and "=" in field:
                    key, val = field.split("=", 1)
                    workspace_fields[key.strip()] = val.strip()

            # Extract crate-specific keys
            crate_fields: dict[str, str] = {}
            bracket_depth = 0
            current_field = ""
            for char in other_fields_str:
                if char == "[":
                    bracket_depth += 1
                    current_field += char
                elif char == "]":
                    bracket_depth -= 1
                    current_field += char
                elif char == "," and bracket_depth == 0:
                    field = current_field.strip()
                    if field and "=" in field:
                        key, val = field.split("=", 1)
                        crate_fields[key.strip()] = val.strip()
                    current_field = ""
                else:
                    current_field += char

            if current_field.strip():
                field = current_field.strip()
                if field and "=" in field:
                    key, val = field.split("=", 1)
                    crate_fields[key.strip()] = val.strip()

            # Merge: crate-specific fields override workspace fields
            merged_fields = {**workspace_fields, **crate_fields}
            merged_parts = [f"{k} = {v}" for k, v in merged_fields.items()]
            merged_spec = ", ".join(merged_parts)

            return f"{name} = {{ {merged_spec} }}"

        pattern2 = rf"^{re.escape(name)} = \{{ workspace = true, (.+?) \}}$"
        content = re.sub(pattern2, replace_with_fields, content, flags=re.MULTILINE | re.DOTALL)

    with open(toml_path, "w") as f:
        f.write(content)


def generate_vendor_cargo_toml(
    repo_root: Path,
    workspace_deps: dict[str, object],
    metadata: dict[str, str],
    copied_crates: list[str],
) -> None:
    """Generate vendor/Cargo.toml with workspace setup."""
    deps_lines: list[str] = []
    for name, dep_spec in sorted(workspace_deps.items()):
        # Skip deps with paths (they're workspace-internal)
        if isinstance(dep_spec, dict) and "path" in dep_spec:
            continue
        deps_lines.append(format_dependency(name, dep_spec))

    deps_str = "\n".join(deps_lines)
    members_str = ", ".join(f'"{m}"' for m in copied_crates)
    authors_str = ", ".join(f'"{a}"' for a in metadata["authors"])

    vendor_toml = f"""[workspace]
members = [{members_str}]
resolver = "2"

[workspace.package]
version = "{metadata["version"]}"
edition = "{metadata["edition"]}"
rust-version = "{metadata["rust-version"]}"
authors = [{authors_str}]
license = "{metadata["license"]}"
repository = "{metadata["repository"]}"
homepage = "{metadata["homepage"]}"

[workspace.lints.rust]
unexpected_cfgs = {{ level = "warn", check-cfg = ['cfg(alef)'] }}

[workspace.dependencies]
{deps_str}
"""

    vendor_dir = repo_root / "packages" / "ruby" / "vendor"
    vendor_dir.mkdir(parents=True, exist_ok=True)

    toml_path = vendor_dir / "Cargo.toml"
    with open(toml_path, "w") as f:
        f.write(vendor_toml)


def main() -> None:
    """Main vendoring function."""
    repo_root: Path = get_repo_root()

    print("=== Vendoring html-to-markdown-rs core crate ===")

    workspace_deps: dict[str, object] = get_workspace_deps(repo_root)
    metadata: dict[str, str] = get_workspace_metadata(repo_root)
    core_version: str = metadata["version"]

    print(f"Core version: {core_version}")
    print(f"Workspace dependencies: {len(workspace_deps)}")

    vendor_base: Path = repo_root / "packages" / "ruby" / "vendor"

    # Clean only crate directories, preserving vendor/bundle/ (Bundler gems)
    crate_names = ["html-to-markdown-rs"]
    for name in crate_names:
        crate_path = vendor_base / name
        if crate_path.exists():
            shutil.rmtree(crate_path)
    # Clean the vendor Cargo.toml (will be regenerated)
    vendor_cargo = vendor_base / "Cargo.toml"
    if vendor_cargo.exists():
        vendor_cargo.unlink()
    # Clean old vendor/html-to-markdown if it exists (legacy name)
    legacy = vendor_base / "html-to-markdown"
    if legacy.exists():
        shutil.rmtree(legacy)
    print("Cleaned vendor crate directories")

    vendor_base.mkdir(parents=True, exist_ok=True)

    # Copy core crate
    crates_to_copy: list[tuple[str, str]] = [
        ("crates/html-to-markdown", "html-to-markdown-rs"),
    ]

    copied_crates: list[str] = []
    for src_rel, dest_name in crates_to_copy:
        src: Path = repo_root / src_rel
        dest: Path = vendor_base / dest_name
        if src.exists():
            try:
                shutil.copytree(src, dest)
                copied_crates.append(dest_name)
                print(f"Copied {dest_name}")
            except Exception as e:
                print(f"Warning: Failed to copy {dest_name}: {e}", file=sys.stderr)
        else:
            print(f"Warning: Source directory not found: {src_rel}")

    # Clean build artifacts from vendored crates
    artifact_dirs: list[str] = ["target"]
    temp_patterns: list[str] = ["*.swp", "*.bak", "*.tmp", "*~"]

    for crate_dir in copied_crates:
        crate_path: Path = vendor_base / crate_dir
        if crate_path.exists():
            for artifact_dir in artifact_dirs:
                artifact: Path = crate_path / artifact_dir
                if artifact.exists():
                    shutil.rmtree(artifact)
            for pattern in temp_patterns:
                for f in crate_path.rglob(pattern):
                    f.unlink()

    print("Cleaned build artifacts")

    # Update workspace inheritance in vendored Cargo.toml files
    for crate_dir in copied_crates:
        crate_toml = vendor_base / crate_dir / "Cargo.toml"
        if crate_toml.exists():
            with open(crate_toml) as f:
                content = f.read()

            content = re.sub(
                r"^version\.workspace = true$",
                f'version = "{core_version}"',
                content,
                flags=re.MULTILINE,
            )
            content = re.sub(
                r"^edition\.workspace = true$",
                f'edition = "{metadata["edition"]}"',
                content,
                flags=re.MULTILINE,
            )
            content = re.sub(
                r"^rust-version\.workspace = true$",
                f'rust-version = "{metadata["rust-version"]}"',
                content,
                flags=re.MULTILINE,
            )
            authors_toml = ", ".join(f'"{a}"' for a in metadata["authors"])
            content = re.sub(
                r"^authors\.workspace = true$",
                f"authors = [{authors_toml}]",
                content,
                flags=re.MULTILINE,
            )
            content = re.sub(
                r"^license\.workspace = true$",
                f'license = "{metadata["license"]}"',
                content,
                flags=re.MULTILINE,
            )
            content = re.sub(
                r"^repository\.workspace = true$",
                f'repository = "{metadata["repository"]}"',
                content,
                flags=re.MULTILINE,
            )
            content = re.sub(
                r"^homepage\.workspace = true$",
                f'homepage = "{metadata["homepage"]}"',
                content,
                flags=re.MULTILINE,
            )
            content = re.sub(
                r"^documentation\.workspace = true$",
                'documentation = "https://docs.rs/html-to-markdown-rs"',
                content,
                flags=re.MULTILINE,
            )
            content = re.sub(
                r"^readme\.workspace = true$",
                'readme = "README.md"',
                content,
                flags=re.MULTILINE,
            )

            # Keep [lints] workspace = true so the crate inherits the vendor
            # workspace's [workspace.lints.rust] (notably check-cfg=['cfg(alef)']).

            with open(crate_toml, "w") as f:
                f.write(content)

            replace_workspace_deps_in_toml(crate_toml, workspace_deps)
            print(f"Updated {crate_dir}/Cargo.toml")

    # Generate vendor/Cargo.toml with workspace setup
    generate_vendor_cargo_toml(repo_root, workspace_deps, metadata, copied_crates)
    print("Generated vendor/Cargo.toml")

    ext_dir = repo_root / "packages" / "ruby" / "ext" / "html_to_markdown_rb"

    # Outer Cargo.toml (used by rb_sys/mkmf at gem install time)
    # From: path = "../../../../crates/html-to-markdown"
    # To:   path = "../../vendor/html-to-markdown-rs"
    outer_toml = ext_dir / "Cargo.toml"
    if outer_toml.exists():
        with open(outer_toml) as f:
            content = f.read()
        content = re.sub(
            r'path = "\.\./\.\./\.\./\.\./crates/html-to-markdown"',
            'path = "../../vendor/html-to-markdown-rs"',
            content,
        )
        with open(outer_toml, "w") as f:
            f.write(content)
        print("Updated ext/html_to_markdown_rb/Cargo.toml to use vendored crate")

    # Inner native/Cargo.toml (used by rake-compiler-dock cross-compile)
    # From: path = "../../../../../crates/html-to-markdown"
    # To:   path = "../../../vendor/html-to-markdown-rs"
    native_toml = ext_dir / "native" / "Cargo.toml"
    if native_toml.exists():
        with open(native_toml) as f:
            content = f.read()
        content = re.sub(
            r'path = "\.\./\.\./\.\./\.\./\.\./crates/html-to-markdown"',
            'path = "../../../vendor/html-to-markdown-rs"',
            content,
        )
        with open(native_toml, "w") as f:
            f.write(content)
        print("Updated ext/html_to_markdown_rb/native/Cargo.toml to use vendored crate")

    print(f"\nVendoring complete (core version: {core_version})")
    print(f"Copied crates: {', '.join(sorted(copied_crates))}")

    if "html-to-markdown-rs" in copied_crates:
        print("Extension Cargo.toml files updated to use vendored crate")


if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)
