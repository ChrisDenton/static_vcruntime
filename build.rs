use std::{env, fs, io::Write, path::Path};

fn main() {
    println!("cargo:rerun-if-env-changed=PROFILE");

    // Early exit if not msvc
    let environment = env::var("CARGO_CFG_TARGET_ENV").unwrap();
    if environment != "msvc" { return; }

    if ::std::env::var("PROFILE").map_or(false, |s| s == "release") {
        println!("cargo:rustc-cfg=release");

        // An empty `.lib` file.
        let bytes: &[u8] = &[
            33, 60, 97, 114, 99, 104, 62, 10, 47, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
            32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
            32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 56, 32, 32, 32, 32, 32, 32, 32, 32,
            32, 96, 10, 0, 0, 0, 0, 0, 0, 0, 0, 47, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
            32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
            32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 49, 52, 32, 32, 32, 32, 32, 32, 32,
            32, 96, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        
        // Write the empty "msvcrt.lib" to the output directory.
        let out_dir = env::var("OUT_DIR").unwrap();
        let path = Path::new(&out_dir).join("msvcrt.lib");
        let f = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path);
        if let Ok(mut f) = f {
            f.write_all(bytes).unwrap();
        }
        // Add the output directory to the native library path.
        println!("cargo:rustc-link-search=native={}", out_dir);
    }
}
