use anyhow::{format_err, Result};
use std::fs;
use std::env;
use std::path::{PathBuf, Path};


lazy_static::lazy_static! {
    static ref BINDINGS_SRC_PATH: PathBuf = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR").expect("Failed to get CARGO_MANIFEST_DIR")
    ).join("src").join("bindings.rs");
    static ref BINDINGS_TARGET_PATH: PathBuf = PathBuf::from(
        env::var("OUT_DIR").expect("Failed to get OUT_DIR")
    ).join("bindings.rs");
    static ref LIBRARY_PATH: PathBuf = PathBuf::from(
        env::var("OUT_DIR").expect("Failed to get OUT_DIR")
    ).join("yolo");
}

// Recursively copy directory
// Ref: https://stackoverflow.com/a/60406693
// Modified to remove target files right before copying to circumvent permission problems.
fn copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<(), std::io::Error> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));
    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();
    while let Some(working_path) = stack.pop() {
        println!("process: {:?}", &working_path);
        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();
        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            println!(" mkdir: {:?}", dest);
            fs::create_dir_all(&dest)?;
        }
        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        println!("  copy: {:?} -> {:?}", &path, &dest_path);
                        // Some `git` files are created with write protection, so replacing them
                        // directly can fail with a permissions error. Remove the destination file
                        // first. Ignore any errors, only the fs::copy() call is critical.
                        fs::remove_file(&dest_path).ok();
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        println!("failed: {:?}", path);
                    }
                }
            }
        }
    }
    Ok(())
}

fn guess_cmake_profile() -> &'static str {
    "Debug"
}


fn build_with_cmake<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let link = "dylib";
    let path = path.as_ref();
    copy(path, LIBRARY_PATH.as_path())?;
    let path = LIBRARY_PATH.as_path();

    let mut config = cmake::Config::new(path);
    config.uses_cxx11();
    let dst = config.build();

    // link to cucodes
    println!("cargo:rustc-link-search={}", dst.join("build").display());
    match guess_cmake_profile() {
        "Debug" => println!("cargo:rustc-link-lib={}=cucodes", link),
        _ => println!("cargo:rustc-link-lib={}=cucodes", link),
    }

    gen_bindings(path.join("src"))?;

    Ok(())
}

fn gen_bindings<P>(include_path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    bindgen::Builder::default()
        .header(
            include_path
                .as_ref()
                .join("simple_yolo.hpp")
                .to_str()
                .ok_or_else(|| format_err!("cannot create path to darknet.hpp"))?,
        )
        .clang_arg("-I/nfs/users/chenquan/packages/tensorrt_pro/data/lean/opencv-4.2.0/include/opencv4/")
        .allowlist_function("SimpleYolo::compile")
        .allowlist_function("SimpleYolo::show_boxes")
        .allowlist_function("SimpleYolo::show_mat_shape")
        .allowlist_function("SimpleYolo::create_infer")
        .allowlist_function("SimpleYolo::predict")
        .allowlist_function("SimpleYolo::reset_engine")
        .allowlist_type("SimpleYolo::Box")
        .allowlist_type("SimpleYolo::Prediction")
        .generate()
        .map_err(|_| format_err!("failed to generate bindings"))?
        .write_to_file(&*BINDINGS_TARGET_PATH)?;
    Ok(())
}

fn build_from_source() -> Result<()> {
    let src_dir: PathBuf = PathBuf::from("./libyolo");
    build_with_cmake(src_dir)?;
    Ok(())
}

fn main() -> Result<()>{
    println!("cargo:rerun-if-changed=libyolo/src/simple_yolo.hpp");
    println!("cargo:rerun-if-changed=libyolo/src/simple_yolo.cu");

    build_from_source()?;
    Ok(())
}