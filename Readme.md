Statically link the VCRuntime when using the MSVC toolchain.

By default, Rust requires programs to deploy `vcruntime140.dll`
(or equivalent) when redistributing binaries. This crate statically links
the library instead.

# Usage

Add this to your `Cargo.toml`:

```toml
[build-dependencies]
static_vcruntime = "2.0"
```

And in your [build script]:

```rust,ignore
fn main() {
    static_vcruntime::metabuild();
}
```

That is all. Then when you build a release binary, the runtime will be
statically linked:
 
```text
cargo build --release
```

# Issues

If you have problems then you may need to clean the build directory before rebuilding:

```text
cargo clean
```

If all else fails then, in the same directory as your Cargo.toml, create a folder called `.cargo`. In that folder create the file `config.toml` and add the following:

```ini
[target.'cfg(all(windows, target_env = "msvc"))']
rustflags = ["-C", "target-feature=+crt-static"]
```

This makes it easier to override the defaults.

[build script]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
