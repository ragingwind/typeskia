extern crate napi_build;

use std::env;

fn main() {
  println!("cargo:rerun-if-changed=skia-c/skia-wrapper.cpp");
  println!("cargo:rerun-if-changed=skia-c/skia-wrapper.h");

  let mut build = cc::Build::new();
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
    .cpp(true)
    .file("libs/skia-wrapper/skia-wrapper.cpp")
    .include("libs/skia-wrapper")
    .cargo_metadata(false)
    .out_dir(&out_dir)
    .compile("skia-wrapper");


  println!("cargo:rustc-link-search={}", &out_dir);

  napi_build::setup();
}
