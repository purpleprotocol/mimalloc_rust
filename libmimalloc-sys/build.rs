use cmake::Config;

#[cfg(not(feature = "secure"))]
fn main() {
    let mut dst = Config::new("c_src/mimalloc")    
        .build();

    dst.push("build");

    println!("cargo:rustc-link-search=native={}", dst.display());
    if cfg!(debug_assertions) {
        println!("cargo:rustc-link-lib=static=mimalloc-debug");
    } else {
        println!("cargo:rustc-link-lib=static=mimalloc");
    }
}

#[cfg(feature = "secure")]
fn main() {
    let mut dst = Config::new("c_src/mimalloc")
        .define("SECURE", "ON")    
        .build();

    dst.push("build");
    
    println!("cargo:rustc-link-search=native={}", dst.display());
    if cfg!(debug_assertions) {
        println!("cargo:rustc-link-lib=static=mimalloc-debug");
    } else {
        println!("cargo:rustc-link-lib=static=mimalloc-secure");
    }
}