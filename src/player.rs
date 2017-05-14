extern crate libc;
use libc::*;

#[link(name="libvlc", kind="dylib")]
extern "C"
{
    fn libvlc_new(argc: c_int, args: *const *mut c_uchar) -> intptr_t;
}