// function `common::init` is never used
#![allow(dead_code)]

#[cfg(target_feature = "atomics")]
#[path = "../wasm/atomics/futex.rs"]
pub mod futex;

#[path = "../unsupported/common.rs"]
mod common;
pub use common::*;

// SAFETY: must be called only once during runtime initialization.
// NOTE: this is not guaranteed to run, for example when Rust code is called externally.
pub unsafe fn init(argc: isize, argv: *const *const u8, _sigpipe: u8) {
    unsafe {
        crate::sys::args::init(argc, argv);
    }
}
