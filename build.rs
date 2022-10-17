extern crate napi_build;

use std::env;
use std::path;

fn main() {
  println!("cargo:rerun-if-changed=skia-c/skia-wrapper.cpp");
  println!("cargo:rerun-if-changed=skia-c/skia-wrapper.h");

  env::set_var("CC", "clang");
  env::set_var("CXX", "clang++");

  let skia_dir = env::var("SKIA_DIR").unwrap_or_else(|_| "./third-party/skia".to_owned());
  let skia_path = path::Path::new(&skia_dir);
  let skia_lib_dir = env::var("SKIA_LIB_DIR").unwrap_or_else(|_| "./third-party/skia/out/Debug".to_owned());

  let mut build = cc::Build::new();

  build.target("arm64-apple-macos");
  build
    .flag("-std=c++17")
    .flag("-fPIC")
    .flag("-fno-exceptions")
    .flag("-fno-rtti")
    .flag("-fstrict-aliasing")
    .flag("-fvisibility=hidden")
    .flag("-fvisibility-inlines-hidden")
    .flag("-fdata-sections")
    .flag("-ffunction-sections")
    .flag("-Wno-unused-function")
    .flag("-Wno-unused-parameter");
  build.cpp_set_stdlib("c++");

  println!("cargo:rustc-link-lib=c++");
  println!("cargo:rustc-link-lib=framework=ApplicationServices");

  let out_dir = env::var("OUT_DIR").unwrap();

  build
    .include("libs/skia-wrapper")
    .include(skia_path)
    .cpp(true)
    .file("libs/skia-wrapper/skia-wrapper.cpp")
    .cargo_metadata(false)
    .out_dir(&out_dir)
    .compile("skia-wrapper");

  println!("cargo:rustc-link-search={}", skia_lib_dir);
  println!("cargo:rustc-link-search={}", &out_dir);
  println!("cargo:rustc-link-lib=skshaper");

  napi_build::setup();
}
