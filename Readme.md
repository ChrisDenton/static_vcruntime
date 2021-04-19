Statically link the VCRuntime when using the MSVC toolchain.

By default, Rust requires programs to deploy `vcruntime140.dll`
(or equivalent) when redistributing binaries. This crate statically links
the library instead.

# Usage

Add this to your `Cargo.toml`:

```ini
[dependencies]
static_vcruntime = "1.4"
```

And put the follwing in your `main.rs`:

```rust
extern crate static_vcruntime;
```

Then when you build a release binary, the runtime will be statically linked:
```text
cargo build --release
```

# Issues

If this doesn't work for you then you may need to clean the build directory before rebuilding:

```text
cargo clean
```

If you are still having problems then, in the same directory as your Cargo.toml, create a folder called `.cargo`. In that folder create the file `config.toml` and add the following:

```ini
[target.'cfg(all(windows, target_env = "msvc"))']
rustflags = ["-C", "target-feature=+crt-static"]
```
