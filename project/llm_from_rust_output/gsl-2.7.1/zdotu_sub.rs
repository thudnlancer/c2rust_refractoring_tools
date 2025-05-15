use num_complex::Complex64;

#[no_mangle]
pub extern "C" fn cblas_zdotu_sub(
    n: i32,
    x: *const libc::c_void,
    incx: i32,
    y: *const libc::c_void,
    incy: i32,
    result: *mut libc::c_void,
) {
    let x_slice = unsafe {
        std::slice::from_raw_parts(
            x as *const Complex64,
            if n == 0 { 0 } else { ((n - 1) * incx.abs()).abs() as usize + 1 },
        )
    };
    let y_slice = unsafe {
        std::slice::from_raw_parts(
            y as *const Complex64,
            if n == 0 { 0 } else { ((n - 1) * incy.abs()).abs() as usize + 1 },
        )
    };

    let mut sum = Complex64::new(0.0, 0.0);
    let mut ix = if incx > 0 { 0 } else { (n - 1) * -incx } as usize;
    let mut iy = if incy > 0 { 0 } else { (n - 1) * -incy } as usize;

    for _ in 0..n {
        let x_val = if incx == 0 { x_slice[0] } else { x_slice[ix] };
        let y_val = if incy == 0 { y_slice[0] } else { y_slice[iy] };
        
        sum += x_val * y_val.conj();
        
        if incx != 0 {
            ix = ix.wrapping_add(incx as usize);
        }
        if incy != 0 {
            iy = iy.wrapping_add(incy as usize);
        }
    }

    unsafe {
        let res_ptr = result as *mut [f64; 2];
        *res_ptr = [sum.re, sum.im];
    }
}