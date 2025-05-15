use num_complex::Complex32;

#[no_mangle]
pub extern "C" fn cblas_cdotc_sub(
    n: i32,
    x: *const libc::c_void,
    inc_x: i32,
    y: *const libc::c_void,
    inc_y: i32,
    result: *mut libc::c_void,
) {
    let x_slice = unsafe {
        std::slice::from_raw_parts(x as *const Complex32, n as usize)
    };
    let y_slice = unsafe {
        std::slice::from_raw_parts(y as *const Complex32, n as usize)
    };
    let result_slice = unsafe {
        std::slice::from_raw_parts_mut(result as *mut Complex32, 1)
    };

    let dot = if inc_x == 1 && inc_y == 1 {
        x_slice.iter()
            .zip(y_slice.iter())
            .map(|(a, b)| a.conj() * b)
            .sum()
    } else {
        let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x } as usize;
        let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y } as usize;
        let x_step = inc_x as usize;
        let y_step = inc_y as usize;
        
        (0..n as usize).map(|_| {
            let res = x_slice[ix].conj() * y_slice[iy];
            ix = ix.wrapping_add(x_step);
            iy = iy.wrapping_add(y_step);
            res
        }).sum()
    };

    result_slice[0] = dot;
}