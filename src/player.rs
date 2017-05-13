extern crate libc;
use libc::*;

#[link(name="libvlc")]
extern "C"
{
    fn libvlc_new(argc: c_int, args: *const *const c_uchar) -> size_t;
}