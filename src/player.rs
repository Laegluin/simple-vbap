extern crate libc;
use libc::*;
use std::ptr::null;

#[derive(Debug)]
pub struct LibVlc
{
    pub instance_ptr: intptr_t,
}

impl LibVlc 
{
    pub fn new() -> LibVlc
    {
        unsafe
        {
            LibVlc
            {
                instance_ptr: libvlc_new(0, null()),
            }
        }
    }
}

impl Drop for LibVlc 
{
    fn drop(&mut self)
    {
        unsafe
        {
            libvlc_release(self.instance_ptr);
        }
    }
}


#[link(name="libvlc", kind="dylib")]
extern "C"
{
    fn libvlc_new(argc: c_int, args: *const *mut c_uchar) -> intptr_t;

    fn libvlc_release(instance_ptr: intptr_t);
}