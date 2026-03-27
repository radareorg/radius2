#[cfg(feature = "vendor-lgl")]
include!("build-vendor.rs");

fn main() {
    #[cfg(feature = "vendor-lgl")]
    {
        if std::env::var("BOOLECTOR_NO_VENDOR").map_or(true, |s| s == "0") {
            let boolector = Build::new().prerequisites().build();

            println!("cargo:vendored=1");
            println!("cargo:root={}", boolector.lib_dir().parent().unwrap().display());

            boolector.print_cargo_metadata();
        }
    }

    println!("cargo:rustc-link-lib=boolector");
}
