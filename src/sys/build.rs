#[cfg(feature = "generate")]
extern crate bindgen;
extern crate pkg_config as pkg;
extern crate semver_parser;

use semver_parser::version;
use semver_parser::version::Version;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn main() {
    let src_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let src_dir = Path::new(&src_dir);
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    let ffi_header = src_dir.join("ffi.h");
    let ffi_rs = out_dir.join("ffi.rs");

    let libvpx = pkg::probe_library("vpx").unwrap();

    println!("rerun-if-changed={}", ffi_header.display());
    for dir in &libvpx.include_paths {
        println!("rerun-if-changed={}", dir.display());
    }

    let gen_dir = src_dir.join("generated");
    let exact_file = gen_dir.join(format!("vpx-ffi-{}.rs", libvpx.version));

    if cfg!(feature = "generate") || !copy_pregenerated(&gen_dir, &ffi_rs, &exact_file, &libvpx.version) {
        generate_bindings(&ffi_header, &libvpx.include_paths, &ffi_rs, &exact_file, &libvpx.version);
    }
}

fn parse(version: &str) -> Result<Version, String> {
    let version = version.split('-').next().unwrap();
    match version::parse(version) {
        Ok(ver) => Ok(ver),
        Err(err) => {
            if let Ok(zero_ver) = version::parse(&format!("{}.0", version)) {
                Ok(zero_ver)
            } else {
                Err(err)
            }
        },
    }
}

fn copy_pregenerated(gen_dir: &Path, ffi_rs: &Path, exact_file: &Path, version: &str) -> bool {
    let wanted_semver = match parse(version) {
        Ok(ver) => ver,
        Err(err) => {
            println!("cargo:warning=libvpx has unsupported version {} {}", version, err);
            return false;
        },
    };

    if exact_file.exists() && fs::copy(&exact_file, ffi_rs).is_ok() {
        return true;
    }

    let closest_match = fs::read_dir(gen_dir).unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter_map(|path| {
            let diff = path.file_stem().and_then(|base_name| {
                let base_name = base_name.to_string_lossy();
                if !base_name.starts_with("vpx-ffi-") {
                    return None
                }
                let ver = match parse(&base_name["vpx-ffi-".len()..]) {
                    Ok(ver) => ver,
                    Err(err) => {
                        println!("cargo:warning=Ignored pregenerated {} {}", err, base_name);
                        return None;
                    },
                };
                if ver.major != wanted_semver.major {
                    return None;
                }
                if ver.major == 0 && ver.minor != wanted_semver.minor {
                    return None;
                }
                let mut diff = (ver.minor as i32 - wanted_semver.minor as i32).abs();
                if ver.minor > wanted_semver.minor {
                    diff += 2;
                }
                Some(diff)
            });
            diff.map(|diff| (path, diff))
        })
        .min_by_key(|pair| pair.1);

    if let Some((path, diff)) = closest_match {
        if diff > 0 {
            println!("\ncargo:warning=Using {} for libvpx version {}. If you get compile errors, try generating exact version.", path.file_name().unwrap().to_string_lossy(), version);
        }
        if fs::copy(path, ffi_rs).is_ok() {
            return true;
        }
    }

    println!("\ncargo:warning=Bindings for vpx version {} are not bundled with libvpx-native-sys yet (expected in {})", version, exact_file.display());
    false
}

#[cfg(not(feature = "generate"))]
fn generate_bindings(_ffi_header: &Path, _include_paths: &[PathBuf], _ffi_rs: &Path, _exact_file: &Path, version: &str) {
    panic!("The 'generate' feature of libvpx-native-sys is disabled, and we don't have pre-generated bindings for libvpx version {}. Enable 'generate' feature of the 'libvpx-native-sys' crate", version);
}

#[cfg(feature = "generate")]
fn generate_bindings(ffi_header: &Path, include_paths: &[PathBuf], ffi_rs: &Path, exact_file: &Path, _version: &str) {
    let mut b = bindgen::builder().header(ffi_header.to_str().unwrap())
        .rust_target(bindgen::RustTarget::Stable_1_25)
        .whitelist_type("^[vV].*")
        .whitelist_var("^[vV].*")
        .whitelist_function("^[vV].*")
        .rustified_enum("^v.*")
        .trust_clang_mangling(false)
        .layout_tests(false) // breaks 32/64-bit compat
        .generate_comments(false); // vpx comments have prefix /*!\

    for dir in include_paths {
        b = b.clang_arg(format!("-I{}", dir.display()));
    }

    b.generate().unwrap()
        .write_to_file(ffi_rs).unwrap();
    fs::copy(ffi_rs, exact_file).ok(); // ignore failure
}
