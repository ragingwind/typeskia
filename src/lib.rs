#![feature(link_cfg)]
#![deny(clippy::all)]
#[allow(dead_code)]

use std::ffi::{CString};

pub mod ffi {
  use std::os::raw::c_char;

  #[link(
    name = "skia-wrapper",
    kind = "static",
    modifiers = "+bundle,+whole-archive",
    cfg(not(target_os = "windows"))
  )]
  #[link(
    name = "skia",
    kind = "static",
    modifiers = "+bundle,+whole-archive",
    cfg(not(target_os = "windows"))
  )]
  extern "C" {
    pub fn sum(a: i32, b: i32) -> i32;

    pub fn draw(width: i32, height: i32, path: *const c_char);
  }
}

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  unsafe { ffi::sum(a, b) }
}

#[napi]
pub fn draw(path: String) {
  let c_path: CString = std::ffi::CString::new(path).unwrap();
  unsafe { ffi::draw(100, 100, c_path.as_ptr()) }
}
