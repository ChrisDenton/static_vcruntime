Statically link the VCRuntime when using the MSVC toolchain.

By default, Rust requires programs to deploy `vcruntime140.dll`
(or equivalent) when redistributing binaries. This crate statically links
the library instead.

# Usage

Add this to your `Cargo.toml`:

```ini
[dependencies]
static_vcruntime = "1.1"
```

And put the follwing in your `main.rs`:

```rust
extern crate static_vcruntime;
```

That is all. Then when you build a release binary, the runtime will be statically linked:
```text
cargo build --release
```
