#![allow(clippy::collapsible_if)]
use cmake::Config;
use std::env;

enum CMakeBuildType {
    Debug,
    Release,
    RelWithDebInfo,
    MinSizeRel,
}

/// Determine the CMake build type that will be picked by `cmake-rs`.
///
/// This is mostly pasted from `cmake-rs`:
/// https://github.com/alexcrichton/cmake-rs/blob/7f85e323/src/lib.rs#L498
fn get_cmake_build_type() -> Result<CMakeBuildType, String> {
    fn getenv(v: &str) -> Result<String, String> {
        env::var(v).map_err(|_| format!("environment variable `{}` not defined", v))
    }

    // Determine Rust's profile, optimization level, and debug info:
    #[derive(PartialEq)]
    enum RustProfile {
        Debug,
        Release,
    }
    #[derive(PartialEq, Debug)]
    enum OptLevel {
        Debug,
        Release,
        Size,
    }

    let rust_profile = match getenv("PROFILE")?.as_str() {
        "debug" => RustProfile::Debug,
        "release" | "bench" => RustProfile::Release,
        _ => RustProfile::Release,
    };

    let opt_level = match getenv("OPT_LEVEL")?.as_str() {
        "0" => OptLevel::Debug,
        "1" | "2" | "3" => OptLevel::Release,
        "s" | "z" => OptLevel::Size,
        _ => match rust_profile {
            RustProfile::Debug => OptLevel::Debug,
            RustProfile::Release => OptLevel::Release,
        },
    };

    let debug_info: bool = match getenv("DEBUG")?.as_str() {
        "false" => false,
        "true" => true,
        _ => true,
    };

    Ok(match (opt_level, debug_info) {
        (OptLevel::Debug, _) => CMakeBuildType::Debug,
        (OptLevel::Release, false) => CMakeBuildType::Release,
        (OptLevel::Release, true) => CMakeBuildType::RelWithDebInfo,
        (OptLevel::Size, _) => CMakeBuildType::MinSizeRel,
    })
}

fn main() {
    let mut cfg = &mut Config::new("c_src/mimalloc");

    if cfg!(feature = "override") {
        cfg = cfg.define("MI_OVERRIDE", "ON");
    } else {
        cfg = cfg.define("MI_OVERRIDE", "OFF");
    }

    cfg = cfg.define("MI_BUILD_TESTS", "OFF");

    if cfg!(feature = "secure") {
        cfg = cfg.define("MI_SECURE", "ON");
    } else {
        cfg = cfg.define("MI_SECURE", "OFF");
    }

    if cfg!(feature = "local_dynamic_tls") {
        cfg = cfg.define("MI_LOCAL_DYNAMIC_TLS", "ON");
    } else {
        cfg = cfg.define("MI_LOCAL_DYNAMIC_TLS", "OFF");
    }

    // Inject MI_DEBUG=0
    // This set mi_option_verbose and mi_option_show_errors options to false.
    cfg = cfg.define("mi_defines", "MI_DEBUG=0");

    let (is_debug, win_folder) = match get_cmake_build_type() {
        Ok(CMakeBuildType::Debug) => (true, "Debug"),
        Ok(CMakeBuildType::Release) => (false, "Release"),
        Ok(CMakeBuildType::RelWithDebInfo) => (false, "RelWithDebInfo"),
        Ok(CMakeBuildType::MinSizeRel) => (false, "MinSizeRel"),
        Err(e) => panic!("Cannot determine CMake build type: {}", e),
    };

    if cfg!(all(windows, target_env = "msvc")) {
        cfg = cfg.define("CMAKE_SH", "CMAKE_SH-NOTFOUND");

        // cc::get_compiler have /nologo /MD default flags that are cmake::Config
        // defaults to. Those flags prevents mimalloc from building on windows
        // extracted from default cmake configuration on windows
        if is_debug {
            // CMAKE_C_FLAGS + CMAKE_C_FLAGS_DEBUG
            cfg = cfg.cflag("/DWIN32 /D_WINDOWS /W3 /MDd /Zi /Ob0 /Od /RTC1");
        } else {
            // CMAKE_C_FLAGS + CMAKE_C_FLAGS_RELEASE
            cfg = cfg.cflag("/DWIN32 /D_WINDOWS /W3 /MD /O2 /Ob2 /DNDEBUG");
        }
    }

    let mut out_dir = "./build".to_string();
    if cfg!(all(windows, target_env = "msvc")) {
        out_dir.push('/');
        out_dir.push_str(win_folder);
    }
    let out_name = if cfg!(all(windows, target_env = "msvc")) {
        if is_debug {
            if cfg!(feature = "secure") {
                "mimalloc-static-secure-debug"
            } else {
                "mimalloc-static-debug"
            }
        } else {
            if cfg!(feature = "secure") {
                "mimalloc-static-secure"
            } else {
                "mimalloc-static"
            }
        }
    } else {
        if is_debug {
            if cfg!(feature = "secure") {
                "mimalloc-secure-debug"
            } else {
                "mimalloc-debug"
            }
        } else {
            if cfg!(feature = "secure") {
                "mimalloc-secure"
            } else {
                "mimalloc"
            }
        }
    };

    // Build mimalloc-static
    let mut dst = cfg.build_target("mimalloc-static").build();
    dst.push(out_dir);

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib={}", out_name);
}
