use cmake::Config;

fn main() {
    let dst = Config::new("c_src/mimalloc")
        .define("SECURE", "ON")             
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    //println!("cargo:rustc-link-lib=static=libmimalloc");
}