#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[no_mangle]
pub unsafe extern "C" fn cblas_saxpy(
    N: i32,
    alpha: libc::c_float,
    mut X: *const libc::c_float,
    incX: i32,
    mut Y: *mut libc::c_float,
    incY: i32,
) {
    let mut i: i32 = 0;
    if alpha as libc::c_double == 0.0f64 {
        return;
    }
    if incX == 1 as i32 && incY == 1 as i32 {
        let m: i32 = N % 4 as i32;
        i = 0 as i32;
        while i < m {
            *Y.offset(i as isize) += alpha * *X.offset(i as isize);
            i += 1;
            i;
        }
        i = m;
        while (i + 3 as i32) < N {
            *Y.offset(i as isize) += alpha * *X.offset(i as isize);
            *Y.offset((i + 1 as i32) as isize)
                += alpha * *X.offset((i + 1 as i32) as isize);
            *Y.offset((i + 2 as i32) as isize)
                += alpha * *X.offset((i + 2 as i32) as isize);
            *Y.offset((i + 3 as i32) as isize)
                += alpha * *X.offset((i + 3 as i32) as isize);
            i += 4 as i32;
        }
    } else {
        let mut ix: i32 = if incX > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incX
        };
        let mut iy: i32 = if incY > 0 as i32 {
            0 as i32
        } else {
            (N - 1 as i32) * -incY
        };
        i = 0 as i32;
        while i < N {
            *Y.offset(iy as isize) += alpha * *X.offset(ix as isize);
            ix += incX;
            iy += incY;
            i += 1;
            i;
        }
    };
}