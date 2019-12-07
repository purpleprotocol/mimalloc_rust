use cmake::Config;

fn main() {
    let mut cfg = &mut Config::new("c_src/mimalloc");

    cfg = cfg.define("MI_OVERRIDE", "OFF");
    cfg = cfg.define("MI_SECURE", "OFF");
    cfg = cfg.define("MI_SECURE_FULL", "OFF");
    cfg = cfg.define("MI_BUILD_TESTS", "OFF");

    if cfg!(feature = "secure") {
        cfg = cfg.define("MI_SECURE", "ON");
    }

    if cfg!(feature = "secure-full") {
        cfg = cfg.define("MI_SECURE_FULL", "ON");
    }

    // Inject MI_DEBUG=0
    // This set mi_option_verbose and mi_option_show_errors options to false.
    cfg = cfg.define("mi_defines", "MI_DEBUG=0");

    if cfg!(all(windows, target_env = "msvc")) {
        // cc::get_compiler have /nologo /MD default flags that are cmake::Config
        // defaults to. Those flags prevents mimalloc from building on windows
        // extracted from default cmake configuration on windows
        if cfg!(debug_assertions) {
            // CMAKE_C_FLAGS + CMAKE_C_FLAGS_DEBUG
            cfg = cfg.cflag("/DWIN32 /D_WINDOWS /W3 /MDd /Zi /Ob0 /Od /RTC1");
        } else {
            // CMAKE_C_FLAGS + CMAKE_C_FLAGS_RELEASE
            cfg = cfg.cflag("/DWIN32 /D_WINDOWS /W3 /MD /O2 /Ob2 /DNDEBUG");
        }
    }

    let (out_dir, out_name) = if cfg!(all(windows, target_env = "msvc")) {
        if cfg!(debug_assertions) {
            if cfg!(feature = "secure") {
                ("./build/Debug", "mimalloc-static-secure-debug")
            } else {
                ("./build/Debug", "mimalloc-static-debug")
            }
        } else {
            if cfg!(feature = "secure") {
                ("./build/Release", "mimalloc-static-secure")
            } else {
                ("./build/Release", "mimalloc-static")
            }
        }
    } else {
        if cfg!(debug_assertions) {
            if cfg!(feature = "secure") {
                ("./build", "mimalloc-secure-debug")
            } else {
                ("./build", "mimalloc-debug")
            }
        } else {
            if cfg!(feature = "secure") {
                ("./build", "mimalloc-secure")
            } else {
                ("./build", "mimalloc")
            }
        }
    };

    // Build mimalloc-static
    let mut dst = cfg.build_target("mimalloc-static").build();
    dst.push(out_dir);

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib={}", out_name);
}
