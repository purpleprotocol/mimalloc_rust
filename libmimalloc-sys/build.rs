use std::env;

fn main() {
    let mut build = cc::Build::new();

    build.include("c_src/mimalloc/include");
    build.files(
        [
            "alloc-aligned",
            "alloc-posix",
            "alloc",
            "arena",
            "bitmap",
            "heap",
            "init",
            "options",
            "os",
            "page",
            "random",
            "region",
            "segment",
            "stats",
        ]
        .iter()
        .map(|fname| format!("c_src/mimalloc/src/{}.c", fname)),
    );

    build.define("MI_STATIC_LIB", None);

    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("target_os not defined!");

    if cfg!(feature = "override") {
        // Overriding malloc is only available on windows in shared mode, but we
        // only ever build a static lib.
        if target_os != "windows" {
            build.define("MI_MALLOC_OVERRIDE", None);
        }
    }

    if cfg!(feature = "secure") {
        build.define("MI_SECURE", "4");
    }

    let dynamic_tls = cfg!(feature = "local_dynamic_tls");

    if env::var("CARGO_CFG_TARGET_FAMILY").expect("target family not set") == "unix"
        && target_os != "haiku"
    {
        if dynamic_tls {
            build.flag_if_supported("-ftls-model=local-dynamic");
        } else {
            build.flag_if_supported("-ftls-model=initial-exec");
        }
    }

    // Remove heavy debug assertions etc
    build.define("MI_DEBUG", "0");

    if build.get_compiler().is_like_msvc() {
        build.cpp(true);
    }

    build.compile("mimalloc");
}
