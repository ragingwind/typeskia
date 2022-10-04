#![feature(link_cfg)]
#![deny(clippy::all)]

pub mod ffi {
  #[link(
    name = "skia-wrapper",
    kind = "static",
    modifiers = "+bundle,+whole-archive",
    cfg(not(target_os = "windows"))
  )]
  extern "C" {
    pub fn sum(a: i32, b: i32) -> i32;
  }
}

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  unsafe { ffi::sum(a, b) }
}
