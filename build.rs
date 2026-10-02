//! OxideBSD: points the linker at OpenPAM's `libpam.a` (`#[link(name = "pam")]` in
//! `src/pam/mod.rs`). OxideBSD's build passes its directory as `OXIDEBSD_LIBPAM_DIR`; any other
//! build sets nothing and links whatever `libpam` the linker finds, as before.

fn main() {
    println!("cargo:rerun-if-env-changed=OXIDEBSD_LIBPAM_DIR");
    if let Ok(dir) = std::env::var("OXIDEBSD_LIBPAM_DIR") {
        println!("cargo:rustc-link-search=native={dir}");
        println!("cargo:rerun-if-changed={dir}/libpam.a");
    }
}
