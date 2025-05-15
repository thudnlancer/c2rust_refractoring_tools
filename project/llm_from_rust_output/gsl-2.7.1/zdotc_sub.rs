use num_complex::Complex64;

#[no_mangle]
pub extern "C" fn cblas_zdotc_sub(
    n: i32,
    x: *const libc::c_void,
    incx: i32,
    y: *const libc::c_void,
    incy: i32,
    result: *mut libc::c_void,
) {
    unsafe {
        let x_ptr = x as *const Complex64;
        let y_ptr = y as *const Complex64;
        let res_ptr = result as *mut Complex64;
        
        let mut sum = Complex64::new(0.0, 0.0);
        let mut ix = if incx > 0 { 0 } else { (n - 1) * -incx };
        let mut iy = if incy > 0 { 0 } else { (n - 1) * -incy };
        
        for _ in 0..n {
            let x_val = *x_ptr.offset(ix as isize);
            let y_val = *y_ptr.offset(iy as isize);
            
            sum += x_val * y_val.conj();
            
            ix += incx;
            iy += incy;
        }
        
        *res_ptr = sum;
    }
}