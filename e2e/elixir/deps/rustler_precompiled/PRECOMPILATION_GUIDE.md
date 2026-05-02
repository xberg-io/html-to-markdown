# Precompilation guide

Rustler provides an easy way write safer NIFs for our OTP applications.
Rustler Precompiled makes the usage of NIFs created with Rustler easier,
so people don't need to have the Rust toolchain installed in order to use their projects.

When users install your package that is using this library, they will see Rustler Precompiled
downloading the precompiled artifact alongside with its SHA256 representing the fingerprint
of that file.

The precompilation happens in a CI server, always in a transparent way, and
the Hex package published should always include a checksum file to ensure
the NIFs stays the same, therefore mitigating supply chain attacks.

In this guide I will show you how to prepare your project to use Rustler Precompiled.

## Prepare for the build

Most of the work is done in the CI server. In this example we are going to use GitHub Actions.

The GH Actions service has the benefit of hosting artifacts for releases and make them
public available. This is important, because it's where the library is going to download
the artifacts from.

### Configure Github Actions

In order for the workflow to succeed, some "write permissions" will need to be enabled for the
repository.
The best way to do that is by using the `permissions:` key in your workflow file. This is effectively
changing the permissions of the `GITHUB_TOKEN` used in the workflow (or for an individual job).

In your job section, add the following:

```yaml
permissions:
  # For creating a new release.
  contents: write
  # The following are needed for the "actions/attest" GH Action.
  id-token: write
  attestations: write
  artifact-metadata: write
```

The permissions related to the "actions/attest" GitHub Action are optional if you don't plan to use attestations,
but they enhance the security a little bit.
Read the [Artifact attestations](https://docs.github.com/en/actions/concepts/security/artifact-attestations)
docs for more details.

### Configure Targets

Usually we want to build for the most popular targets and the minimum NIF version supported.

NIF versions are more stable than OTP versions because they usually change only after two major
releases of Erlang/OTP. But older versions are compatible with newer versions if they have the same MAJOR
number. For example, the NIF `2.15` is compatible with `2.16` and `2.17`. So you only need to
compile for `2.15` if you want to support these versions.

In case a new feature from the newer versions is needed, then you can build for both versions as well.
See [the trobleshooting](TROUBLESHOOTING.md) document to find how to do that.

For this guide our targets will be the following:

- OS: Linux, Windows, macOS
- Architectures: `x86_64`, `aarch64` (ARM 64 bits)
- NIF version: `2.15` - this is the default for Rustler since `v0.29`.

In summary the build matrix looks like this:

```yaml
matrix:
  nif: ["2.15"]
  job:
    - { target: aarch64-unknown-linux-gnu   , os: ubuntu-22.04 , use-cross: true }
    - { target: aarch64-apple-darwin        , os: macos-15      }
    - { target: x86_64-apple-darwin         , os: macos-15-intel }
    - { target: x86_64-unknown-linux-gnu    , os: ubuntu-22.04  }
    - { target: x86_64-unknown-linux-musl   , os: ubuntu-22.04 , use-cross: true }
    - { target: x86_64-pc-windows-gnu       , os: windows-2022  }
    - { target: x86_64-pc-windows-msvc      , os: windows-2022  }
```

A complete workflow example can be found in the [`rustler_precompilation_example`](https://github.com/philss/rustler_precompilation_example/blob/main/.github/workflows/release.yml) project.
You can copy that file and modify it with your desired inputs.

That workflow is using a GitHub Action especially made for our goal: [philss/rustler-precompiled-action](https://github.com/philss/rustler-precompiled-action).
The GitHub Action will deal with the installation of `cross` and the build of the project, naming the files in the correct format.

Some targets are only supported by later versions of `cross`. For those, you might want to
install `cross` directly from GitHub. You can see an example in [this
pipeline](https://github.com/kloeckner-i/mail_parser/blob/f4af5083aec73a47f0e41a202ba46a91f60602cf/.github/workflows/release.yml#L101-L105).

## Additional configuration before build

In our build we are going to cross compile our crate project (the Rust code for our NIF) using
a variety of targets, as we saw in the previous section. For this to work we need to guide the Rust
compiler in some cases by providing additional configuration in the `.cargo/config.toml` file of our project.

Here is an example of that file:

```toml
# This is needed for "musl". See https://github.com/rust-lang/rust/issues/59302
[target.x86_64-unknown-linux-musl]
rustflags = [
  "-C", "target-feature=-crt-static"
]

# Provides a small build size for the "release" profile, but takes more time to build.
[profile.release]
lto = true
```

For more common configuration needed for other targets, see the [troubleshooting document](TROUBLESHOOTING.md).

In addition to that, we also need a tool called [`cross`](https://github.com/rust-embedded/cross) that
makes the build easier for some targets (the ones using `use-cross: true` in our example).
This tool will be installed automatically by the `rustler-precompiled-action`.

## The Rustler module

We need to tell `RustlerPrecompiled` where to find our NIF files, and we need to tell which version to use.

```elixir
defmodule RustlerPrecompilationExample.Native do
  version = Mix.Project.config()[:version]

  use RustlerPrecompiled,
    otp_app: :rustler_precompilation_example,
    crate: "example",
    base_url:
      "https://github.com/philss/rustler_precompilation_example/releases/download/v#{version}",
    force_build: System.get_env("RUSTLER_PRECOMPILATION_EXAMPLE_BUILD") in ["1", "true"],
    version: version

  # When your NIF is loaded, it will override this function.
  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
end
```

This example was also extracted from the [`rustler_precompilation_example`](https://github.com/philss/rustler_precompilation_example/blob/main/lib/rustler_precompilation_example/native.ex) project.
RustlerPrecompiled will try to figure out the target and download the correct file for us. This will happen in compile
time only.

Optionally it's possible to force the compilation by setting an env var, like the example suggests.

It's also possible to force the build by using a pre release version, like `0.1.0-dev`.
The only requirement to force the build is to have Rustler declared as a dependency as well:

`{:rustler, ">= 0.0.0", optional: true}`

## The release flow

### Generating a checksum file

When you need to release a Hex package using precompiled NIFs, you first need to
build the release in the CI, wait for all artifacts to be available and then generate
the **checksum file** that is **MANDATORY** for your package to work.

This checksum file is generated by running the following command after the build is complete:

    $ mix rustler_precompiled.download YourRustlerModule --all --print

With the module I used for this guide, the command would be:

    $ mix rustler_precompiled.download RustlerPrecompilationExample.Native --all --print

The file generated will be named `checksum-Elixir.RustlerPrecompilationExample.Native.exs` and
it's **extremely important that you include this file in your Hex package** (by updating the `files:`
field in your `mix.exs`). Otherwise your package **won't work**. Your `files:` key at your
package configuration will look like this:

```elixir
defp package do
  [
    files: [
      "lib",
      "native/my_nif/.cargo",
      "native/my_nif/src",
      "native/my_nif/Cargo*",
      "checksum-*.exs",
      "mix.exs"
    ],
    # ...
  ]
end
```

Note: you don't need to track the checksum file in your version control system (git or other).
Another thing is that you want to make sure that the "target" directory is not released with
the Hex package. To ensure that, you can remove the directory before releasing: `rm -rf native/my_nif/target`.

For an example, refer to the `mix.exs` file of the [rustler precompilation example](https://github.com/philss/rustler_precompilation_example/blob/main/mix.exs)
or elixir-nx's [explorer](https://github.com/elixir-nx/explorer/blob/723eea63204e43bc9238d2488fd355f17a1e13f2/mix.exs#L65-L72) library.

Tip: use the `mix hex.build --unpack` command to confirm which files are being included (and if the package looks good before publishing).

### Recommended flow

To recap, the suggested flow is the following:

1. release a new tag
2. push the code to your repository with the new tag: `git push origin main --tags`
3. wait for all NIFs to be built
4. run the `mix rustler_precompiled.download` task (with the flag `--all`)
5. release the package to Hex.pm (make sure your release includes the correct files).

## Conclusion

The ability to use precompiled NIFs written in Rust can increase the adoption of some packages,
because people won't need to have Rust installed. But this comes with some drawbacks and more
responsibilities to the maintainers, so use this feature carefully.
