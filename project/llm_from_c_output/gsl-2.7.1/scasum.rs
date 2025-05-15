use std::ffi::c_void;
use std::slice;

pub fn cblas_scasum(n: i32, x: *const c_void, inc_x: i32) -> f32 {
    assert!(n >= 0, "n must be non-negative");
    assert!(inc_x > 0, "inc_x must be positive");
    
    let x_ptr = x as *const f32;
    let len = n as usize * inc_x as usize;
    let x_slice = unsafe { slice::from_raw_parts(x_ptr, len) };
    
    x_slice.chunks(inc_x as usize)
        .map(|chunk| {
            let real = chunk[0];
            let imag = if chunk.len() > 1 { chunk[1] } else { 0.0 };
            real.abs() + imag.abs()
        })
        .sum()
}