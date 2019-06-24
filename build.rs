use cmake::Config;

#[cfg(feature = "no_secure")]
fn main() {
    let dst = Config::new("c_src/mimalloc")    
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
}

#[cfg(not(feature = "no_secure"))]
fn main() {
    let dst = Config::new("c_src/mimalloc")
        .define("SECURE", "ON")    
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
}