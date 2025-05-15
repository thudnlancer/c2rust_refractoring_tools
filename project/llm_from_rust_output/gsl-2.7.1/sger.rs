use std::ffi::CString;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_ORDER {
    CblasRowMajor = 101,
    CblasColMajor = 102,
}

pub fn cblas_sger(
    order: CBLAS_ORDER,
    m: i32,
    n: i32,
    alpha: f32,
    x: &[f32],
    inc_x: i32,
    y: &[f32],
    inc_y: i32,
    a: &mut [f32],
    lda: i32,
) -> Result<(), String> {
    // Parameter validation
    if m < 0 {
        return Err("M must be non-negative".to_string());
    }
    if n < 0 {
        return Err("N must be non-negative".to_string());
    }
    if inc_x == 0 {
        return Err("incX must not be zero".to_string());
    }
    if inc_y == 0 {
        return Err("incY must not be zero".to_string());
    }

    let min_lda = match order {
        CBLAS_ORDER::CblasRowMajor => n.max(1),
        CBLAS_ORDER::CblasColMajor => m.max(1),
    };
    if lda < min_lda {
        return Err("lda must be at least max(1, N) for RowMajor or max(1, M) for ColMajor".to_string());
    }

    // Check array lengths
    let x_len = if m == 0 {
        0
    } else {
        ((m - 1) * inc_x.abs()).abs() + 1) as usize
    };
    let y_len = if n == 0 {
        0
    } else {
        ((n - 1) * inc_y.abs()).abs() + 1) as usize
    };

    if x.len() < x_len {
        return Err("X array too short".to_string());
    }
    if y.len() < y_len {
        return Err("Y array too short".to_string());
    }
    if a.len() < (lda * n.max(m)) as usize {
        return Err("A array too short".to_string());
    }

    match order {
        CBLAS_ORDER::CblasRowMajor => {
            let mut ix = if inc_x > 0 { 0 } else { (m - 1) * -inc_x };
            for i in 0..m {
                let tmp = alpha * x[ix as usize];
                let mut jy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
                for j in 0..n {
                    a[(lda * i + j) as usize] += y[jy as usize] * tmp;
                    jy += inc_y;
                }
                ix += inc_x;
            }
        }
        CBLAS_ORDER::CblasColMajor => {
            let mut jy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
            for j in 0..n {
                let tmp = alpha * y[jy as usize];
                let mut ix = if inc_x > 0 { 0 } else { (m - 1) * -inc_x };
                for i in 0..m {
                    a[(i + lda * j) as usize] += x[ix as usize] * tmp;
                    ix += inc_x;
                }
                jy += inc_y;
            }
        }
    }

    Ok(())
}

#[no_mangle]
pub extern "C" fn cblas_xerbla(p: i32, rout: *const i8, form: *const i8, _: ...) {
    // This is kept as extern C for compatibility but should not be called in safe Rust
    unsafe {
        let rout_str = CString::from_raw(rout as *mut i8);
        let form_str = CString::from_raw(form as *mut i8);
        eprintln!(
            "Parameter {} was incorrect on entry to {}: {}",
            p,
            rout_str.to_string_lossy(),
            form_str.to_string_lossy()
        );
    }
}