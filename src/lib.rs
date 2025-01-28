#![deny(clippy::all)]

mod binding;
mod segfault_handler;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn cause_sigsegv() {
  let ptr = std::ptr::null_mut::<u8>();
  unsafe {
    *ptr = 1;
  }
}

#[napi]
pub fn register() -> () {
  segfault_handler::register_segfault_handler();
}
