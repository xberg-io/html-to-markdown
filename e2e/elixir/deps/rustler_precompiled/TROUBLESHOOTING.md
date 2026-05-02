# Troubleshooting

Some problems are related to specific targets that need additional configuration or dependencies.
In this document you can find the way to fix those problems.

## Supporting "musl" ABI targets

This type of targets are common for people running on systems like the Alpine Linux.
The main difference is that you need to disable "static linking". This can be done
by changing the configuration of your target (or targets) in the `.cargo/cargo.toml` file.

For example, if the idea is to support `x86_64-unknown-linux-musl`, you need to add
the following:

```toml
# This is needed for "musl". See https://github.com/rust-lang/rust/issues/59302
[target.x86_64-unknown-linux-musl]
rustflags = [
  "-C", "target-feature=-crt-static"
]
```

It's common to run containers of Linux in machines that are using the Arch64 architecture
(like the M family from Apple). So you will probably want to repeat the same configuration
for the `aarch64-unknown-linux-musl` target.

## Using different NIF versions

By default Rustler builds for the NIF version 2.15, which aims Erlang/OTP 22 and above.
If your project needs something that was released in more recent NIF versions, you can configure
the enabled cargo features for Rustler by adding features to your project - that enable the desired
NIF versions.

Your "Cargo.toml" would look like this:

```toml
[dependencies]
rustler = { version = "0.37", default-features = false }

# And then, your features.
[features]
default = ["nif_version_2_15"]
nif_version_2_15 = ["rustler/nif_version_2_15"]
nif_version_2_16 = ["rustler/nif_version_2_16"]
nif_version_2_17 = ["rustler/nif_version_2_17"]
```

In your code, you would use these features - like `nif_version_2_16` - to control how your
code is going to be compiled. You can hide some features behind these "cargo features".

If the ideia is to depend on one version, let's say "2.16", you can declare your rustler dependency like this:

```toml
[dependencies]
rustler = { version = "0.37", default-features = false, features = ["nif_version_2_16"] }
```

The available NIF versions are the following:

* `2.14` - for OTP 21 and above.
* `2.15` - for OTP 22 and above.
* `2.16` - for OTP 24 and above.
* `2.17` - for OTP 26 and above.

And the default NIF version activated by Rustler versions is the following:

* Rustler `~> v0.29` - NIF `2.15`

The [`rustler-precompiled-action`](https://github.com/philss/rustler-precompiled-action) would
only require that you specify the NIF version, so it would active the respective feature.

Note that it's important to follow the format of `nif_version_MAJOR_MINOR` in order to make that
GitHub Action work automatically.
You can also specify the `cargo-args` in that GitHub Action usage, as described in the docs.

## Link to libatomic for 32-bit builds

If you are targeting 32 bits platforms like ARM, you may need to add a configuration to "link libatomic".
More details can be found in this issue: [#53](https://github.com/philss/rustler_precompiled/issues/53).

Modify your `.cargo/config.yml` file for the targets affected with something like the following:

```toml
# Libatomic is needed for 32 bits ARM.
# See: https://github.com/philss/rustler_precompiled/issues/53
[target.arm-unknown-linux-gnueabihf]
rustflags = [
  "-l", "dylib=atomic"
]
```
