pub use super::common::Args;
use crate::ffi::{CStr, OsString};

/// One-time global initialization.
pub unsafe fn init(argc: isize, argv: *const *const u8) {
    unsafe { imp::init(argc, argv) }
}

pub fn args() -> Args {
    let (argc, argv) = imp::argc_argv();

    let mut vec = Vec::with_capacity(argc as usize);

    for i in 0..argc {
        let ptr = unsafe { argv.offset(i).read() };
        if ptr.is_null() {
            break;
        }
        let cstr = unsafe { CStr::from_ptr(ptr) };
        vec.push(unsafe { OsString::from_encoded_bytes_unchecked(cstr.to_bytes().to_vec()) });
    }

    Args::new(vec)
}

mod imp {
    use crate::ffi::c_char;
    use crate::ptr;
    use crate::sync::atomic::{Atomic, AtomicIsize, AtomicPtr, Ordering};

    static ARGC: Atomic<isize> = AtomicIsize::new(0);
    static ARGV: Atomic<*mut *const u8> = AtomicPtr::new(ptr::null_mut());

    unsafe fn really_init(argc: isize, argv: *const *const u8) {
        ARGC.store(argc, Ordering::Relaxed);
        ARGV.store(argv as *mut _, Ordering::Relaxed);
    }

    #[inline(always)]
    pub unsafe fn init(argc: isize, argv: *const *const u8) {
        unsafe { really_init(argc, argv) };
    }

    pub fn argc_argv() -> (isize, *const *const c_char) {
        let argv = ARGV.load(Ordering::Relaxed);
        let argc = if argv.is_null() { 0 } else { ARGC.load(Ordering::Relaxed) };
        (argc, argv.cast())
    }
}

