//! Build script for Intel IPSec Multi-Buffer Crypto Library

use std::{
    env,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

// Supported target architectures
const X86: &str = "x86";
const X86_64: &str = "x86_64";
const AARCH64: &str = "aarch64";


fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=vendor/intel-ipsec-mb/lib");
    // println!("cargo:rerun-if-changed=wrapper");
    
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    
    // Only build for supported architectures
    if !matches!(target_arch.as_str(), X86 | X86_64 | AARCH64) {
        panic!("Intel IPSec-MB is only supported on x86, x86_64, and aarch64 architectures");
    }

    let ipsec_mb_dir = check_vendored_source();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Build Intel IPSec-MB library
    build_ipsec_mb(&ipsec_mb_dir, &target_arch, &target_os, &target_env, &out_dir);
    
    // Generate Rust bindings
    generate_bindings(&ipsec_mb_dir, &out_dir);
    
    // Link the library
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=IPSec_MB");
    
}

fn check_vendored_source() -> PathBuf {
    let vendor_dir = PathBuf::from("vendor/intel-ipsec-mb");
    
    if !vendor_dir.exists() {
        panic!(
            "Intel IPSec-MB source not found at {}. \
            Please ensure the git submodule is initialized:\n\
            git submodule update --init --recursive",
            vendor_dir.display()
        );
    }
    
    let lib_dir = vendor_dir.join("lib");
    if !lib_dir.exists() {
        panic!(
            "Intel IPSec-MB lib directory not found at {}",
            lib_dir.display()
        );
    }
    
    // Check for main header file
    let header_file = lib_dir.join("intel-ipsec-mb.h");
    if !header_file.exists() {
        panic!(
            "Intel IPSec-MB header not found at {}",
            header_file.display()
        );
    }
    
    vendor_dir
}

fn build_ipsec_mb(
    ipsec_mb_dir: &Path,
    target_arch: &str,
    target_os: &str,
    _target_env: &str,
    out_dir: &Path,
) {
    // Use CMake build only
    if !cmake_build(ipsec_mb_dir, target_arch, target_os, out_dir) {
        panic!("CMake build failed. This is likely due to NASM assembly compatibility issues. \
                The Intel IPSec MB library requires NASM 2.14+ and proper assembly file support. \
                Consider using a different NASM version or building on a different system.");
    }
}

fn cmake_build(
    ipsec_mb_dir: &Path,
    target_arch: &str,
    _target_os: &str,
    out_dir: &Path,
) -> bool {
    // Check if cmake is available
    if Command::new("cmake").arg("--version").output().is_err() {
        eprintln!("cargo:warning=CMake not found. Please install CMake 3.18+ to build Intel IPSec MB library.");
        return false;
    }
    
    // Create build directory
    let build_dir = ipsec_mb_dir.join("build");
    if let Err(e) = fs::create_dir_all(&build_dir) {
        eprintln!("cargo:warning=Failed to create build directory: {}", e);
        return false;
    }
    
    let mut cmake_cmd = Command::new("cmake");
    cmake_cmd.current_dir(&build_dir);
    
    // Configure for static library build
    cmake_cmd.args(&[
        "-DBUILD_SHARED_LIBS=OFF",
        "-DBUILD_LIBRARY_ONLY=ON",
        ".."
    ]);
    
    // Configure SAFE_OPTIONS based on build profile
    let profile = env::var("PROFILE").unwrap_or_default();
    if profile == "debug" {
        cmake_cmd.arg("-DSAFE_OPTIONS=ON");
    } else {
        // Disable SAFE_OPTIONS for release builds (override the default ON)
        cmake_cmd.arg("-DSAFE_OPTIONS=OFF");
    }
    
    // Set architecture-specific options
    match target_arch {
        X86_64 => {
            // CMake will auto-detect x86_64
        }
        X86 => {
            cmake_cmd.arg("-DCMAKE_C_FLAGS=-m32");
        }
        AARCH64 => {
            // CMake will auto-detect aarch64
        }
        _ => return false,
    }
    
    // Configure
    let output = match cmake_cmd.output() {
        Ok(output) => output,
        Err(e) => {
            eprintln!("cargo:warning=Failed to run CMake configure: {}", e);
            return false;
        }
    };
    
    if !output.status.success() {
        eprintln!("cargo:warning=CMake configure failed:");
        eprintln!("cargo:warning=stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("cargo:warning=stderr: {}", String::from_utf8_lossy(&output.stderr));
        return false;
    }
    
    // Build
    let mut build_cmd = Command::new("cmake");
    build_cmd.current_dir(&build_dir);
    // build_cmd.args(&["--build", ".", "--parallel"]);
    build_cmd.args(&["--build", "."]);
    
    let output = match build_cmd.output() {
        Ok(output) => output,
        Err(e) => {
            eprintln!("cargo:warning=Failed to run CMake build: {}", e);
            return false;
        }
    };
    
    if !output.status.success() {
        eprintln!("cargo:warning=CMake build failed:");
        eprintln!("cargo:warning=stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("cargo:warning=stderr: {}", String::from_utf8_lossy(&output.stderr));
        eprintln!("cargo:warning=This is likely due to NASM assembly compatibility issues. \
                  The Intel IPSec MB library requires NASM 2.14+ with proper .note.GNU-stack support.");
        return false;
    }
    
    // Copy the built library
    let lib_path = build_dir.join("lib").join("libIPSec_MB.a");
    if lib_path.exists() {
        let dest_path = out_dir.join("libIPSec_MB.a");
        if let Err(e) = fs::copy(&lib_path, &dest_path) {
            eprintln!("cargo:warning=Failed to copy library: {}", e);
            return false;
        }
        return true;
    }
    
    eprintln!("cargo:warning=CMake build completed but library file not found at expected location: {}", lib_path.display());
    false
}

fn generate_bindings(ipsec_mb_dir: &Path, out_dir: &Path) {
    let header_path = ipsec_mb_dir.join("lib").join("intel-ipsec-mb.h");
    
    let mut builder = bindgen::Builder::default()
        .header(header_path.to_string_lossy())
        .clang_arg(format!("-I{}", ipsec_mb_dir.join("lib").display()));
    
    // Add wrapper header if it exists
    let wrapper_header = PathBuf::from("wrapper").join("wrapper.h");
    if wrapper_header.exists() {
        builder = builder
            .header(wrapper_header.to_string_lossy())
            .clang_arg("-Iwrapper");
    }
    
    let bindings = builder
        // Allow everything from the Intel IPSec MB header
        .allowlist_file(".*")
        .translate_enum_integer_types(true)
        .rustified_non_exhaustive_enum("IMB_ERR")
        .rustified_non_exhaustive_enum("IMB_ARCH")
        .rustified_non_exhaustive_enum("IMB_STATUS")
        .rustified_non_exhaustive_enum("IMB_CIPHER_DIRECTION")
        .rustified_non_exhaustive_enum("IMB_CIPHER_MODE")
        .rustified_non_exhaustive_enum("IMB_HASH_ALG")
        .rustified_non_exhaustive_enum("IMB_CHAIN_ORDER")

        // Formatting and generation options
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
    
    let bindings_path = out_dir.join("bindings.rs");
    bindings
        .write_to_file(&bindings_path)
        .expect("Couldn't write bindings!");
    
    println!("cargo:rustc-env=BINDINGS_PATH={}", bindings_path.display());
}
