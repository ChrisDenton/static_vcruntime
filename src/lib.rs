//! Statically link the VCRuntime when using the MSVC toolchain
//!
//! By default, Rust requires programs to deploy `vcruntime140.dll`
//! (or equivalent) when redistributing binaries. This crate statically links
//! the library instead.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [build-dependencies]
//! static_vcruntime = "2.0"
//! ```
//!
//! And in your [build script]:
//!
//! ```rust,ignore
//! fn main() {
//!     static_vcruntime::metabuild();
//! }
//! ```
//!
//! That is all. Then when you build a release binary, the runtime will be
//! statically linked:
//! 
//! ```text
//! cargo build --release
//! ```
//!
//! # Issues
//!
//! If you have problems then you may need to clean the build directory before rebuilding:
//!
//! ```text
//! cargo clean
//! ```
//!
//! If all else fails then, in the same directory as your Cargo.toml, create a folder called `.cargo`. In that folder create the file `config.toml` and add the following:
//!
//! ```ini
//! [target.'cfg(all(windows, target_env = "msvc"))']
//! rustflags = ["-C", "target-feature=+crt-static"]
//! ```
//!
//! This makes it easier to override the defaults.
//! 
//! [build script]: https://doc.rust-lang.org/cargo/reference/build-scripts.html

use std::{env, fs, io::Write, path::Path};

/// Use dynamically linked ucrt with a statically linked vcruntime.
/// 
/// This must be called from a [build script], like so:
/// 
/// ```rust,ignore
/// // build.rs
/// fn main() {
///     static_vcruntime::metabuild();
/// }
/// ```
/// 
/// [build script]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
pub fn metabuild() {
	// Early exit if not msvc
	let target = env::var("CARGO_CFG_TARGET_ENV").expect("`CARGO_CFG_TARGET_ENV` environment variable is missing. Ensure you're using `static_vcruntime` in a build script");
	if target != "msvc" {
		return;
	}

	// Disable conflicting libraries
	println!("cargo:rustc-link-arg=/NODEFAULTLIB:libvcruntimed.lib");
	println!("cargo:rustc-link-arg=/NODEFAULTLIB:vcruntime.lib");
	println!("cargo:rustc-link-arg=/NODEFAULTLIB:vcruntimed.lib");
	println!("cargo:rustc-link-arg=/NODEFAULTLIB:libcmtd.lib");
	println!("cargo:rustc-link-arg=/NODEFAULTLIB:msvcrt.lib");
	println!("cargo:rustc-link-arg=/NODEFAULTLIB:msvcrtd.lib");
	println!("cargo:rustc-link-arg=/NODEFAULTLIB:libucrt.lib");
	println!("cargo:rustc-link-arg=/NODEFAULTLIB:libucrtd.lib");
	// Set the libraries we want.
	println!("cargo:rustc-link-arg=/DEFAULTLIB:libcmt.lib");
	println!("cargo:rustc-link-arg=/DEFAULTLIB:libvcruntime.lib");
	println!("cargo:rustc-link-arg=/DEFAULTLIB:ucrt.lib");
}
