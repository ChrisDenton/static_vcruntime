fn main ()
{
    println!("cargo:rerun-if-env-changed=PROFILE");
    if ::std::env::var("PROFILE").map_or(false, |s| s == "release") {
        println!("cargo:rustc-cfg=release");
    }
}
