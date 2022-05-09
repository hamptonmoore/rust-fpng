use cxx::{CxxVector};
use core::ffi::c_void;
use std::ptr;

#[cxx::bridge(namespace = "fpng")]
pub mod ffi {
    unsafe extern "C++" {
        include!("rust-fpng/include/fpng.h");
        unsafe fn fpng_encode_image_to_memory(pImage: *mut u32, width: u32, height: u32, num_chans: u32, out_buf: &CxxVector<u8>, flags: u32) -> bool;
    }
}
   


fn main() {
    let input = 4;
    let mut buffer: Vec<u8> = vec![255; 10*10*4];
    let mut png: Vec<u8> = vec![];
    unsafe {
        ffi::fpng_encode_image_to_memory(&mut (&buffer[..]), 10, 10, 4, &png, 0);
    }
}