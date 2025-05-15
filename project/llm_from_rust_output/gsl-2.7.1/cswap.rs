use std::slice;

#[no_mangle]
pub extern "C" fn cblas_cswap(
    N: i32,
    X: *mut libc::c_void,
    incX: i32,
    Y: *mut libc::c_void,
    incY: i32,
) {
    if N <= 0 || incX == 0 || incY == 0 {
        return;
    }

    let x_slice = unsafe {
        slice::from_raw_parts_mut(X as *mut [f32; 2], N as usize)
    };
    let y_slice = unsafe {
        slice::from_raw_parts_mut(Y as *mut [f32; 2], N as usize)
    };

    let x_iter = if incX > 0 {
        (0..N as usize).step_by(incX as usize)
    } else {
        let start = ((N - 1) * -incX) as usize;
        (0..N as usize).rev().step_by((-incX) as usize)
    };

    let y_iter = if incY > 0 {
        (0..N as usize).step_by(incY as usize)
    } else {
        let start = ((N - 1) * -incY) as usize;
        (0..N as usize).rev().step_by((-incY) as usize)
    };

    for (x_idx, y_idx) in x_iter.zip(y_iter) {
        if x_idx < N as usize && y_idx < N as usize {
            let temp = x_slice[x_idx];
            x_slice[x_idx] = y_slice[y_idx];
            y_slice[y_idx] = temp;
        }
    }
}