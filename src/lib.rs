//! Statically link the VCRuntime while dynamically linking the UCRT.
//! This only applies when using the MSVC toolchain.
//!
//! By default, Rust requires programs to deploy `vcruntime140.dll`
//! (or equivalent) when redistributing binaries.
//! You can use the `-C target-feature=+crt-static` rustc flag to statically link it
//! but that also statically links the Universal CRT.
//! The Universal CRT is a component of Windows so can always be dynamically linked.
//!
//! See [Details] for more information.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [build-dependencies]
//! static_vcruntime = "3.0"
//! ```
//!
//! And then in your [build script] you can call [`static_vcruntime::metabuild`] like so:
//!
//! ```rust,ignore
//! fn main() {
//!     static_vcruntime::metabuild();
//! }
//! ```
//!
//! For the best compatibility it is recommended that you also set the `target-feature` flag to `+crt-static`.
//! In the same directory as your Cargo.toml, create a folder called `.cargo`. In that folder create the file `config.toml` and add the following:
//!
//! ```toml
//! # In .cargo/config.toml
//! [target.'cfg(all(windows, target_env = "msvc"))']
//! rustflags = ["-C", "target-feature=+crt-static"]
//! ```
//!
//! # Details
//!
//! What we call the "CRT" is actually three components:
//! - The C startup files. This provide startup/shutdown code.
//! - The VC runtime. In rust this is mainly used for panic handling but also provides some fundamental functions such as `memcpy`.
//! - The Universal CRT (aka UCRT). This is where most of the C standard library lives.
//!
//! Each of these can be linked either dynamically or statically, however it is usually required that if one is linked statically then they are all linked statically (and ditto for dynamica linking).
//! There is one exceptions.
//! If the VC runtime and C startup files are linked statically then the UCRT can be linked dynamically.
//!
//! The following table summarises these options:
//!
//! |                    | C startup  | VC runtime | Universal CRT |
//! | ------------------ | ---------- | ---------- | ------------- |
//! | Default            | dynamic    | dynamic    | dynamic       |
//! | `+crt-static`      | static     | static     | static        |
//! | `static_vcruntime` | static     | static     | dynamic       |
//!
//! # Issues
//!
//! If you have problems then you may need to clean the build directory before rebuilding:
//!
//! ```text
//! cargo clean
//! ```
//!
//! It is possible that some C/C++ dependencies may not work in this configuration.
//!
//! Note that Microsoft recommends all the runtime libraries be dynamically linked (which is the default).
//!
//! [Details]: #details
//! [build script]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
//! [`static_vcruntime::metabuild`]: metabuild

use std::env;

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
/// You can restrict it to only be enabled on specific profiles, such as `release`.
///
/// ```rust,ignore
/// fn main() {
///     if std::env::var("PROFILE").as_deref() == Ok("release") {
///         static_vcruntime::metabuild();
///     }
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
