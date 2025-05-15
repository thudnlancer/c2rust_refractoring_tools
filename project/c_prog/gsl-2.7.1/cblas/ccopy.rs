use ::libc;
#[no_mangle]
pub unsafe extern "C" fn cblas_ccopy(
    N: libc::c_int,
    mut X: *const libc::c_void,
    incX: libc::c_int,
    mut Y: *mut libc::c_void,
    incY: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut ix: libc::c_int = if incX > 0 as libc::c_int {
        0 as libc::c_int
    } else {
        (N - 1 as libc::c_int) * -incX
    };
    let mut iy: libc::c_int = if incY > 0 as libc::c_int {
        0 as libc::c_int
    } else {
        (N - 1 as libc::c_int) * -incY
    };
    i = 0 as libc::c_int;
    while i < N {
        *(Y as *mut libc::c_float)
            .offset(
                (2 as libc::c_int * iy) as isize,
            ) = *(X as *const libc::c_float).offset((2 as libc::c_int * ix) as isize);
        *(Y as *mut libc::c_float)
            .offset(
                (2 as libc::c_int * iy + 1 as libc::c_int) as isize,
            ) = *(X as *const libc::c_float)
            .offset((2 as libc::c_int * ix + 1 as libc::c_int) as isize);
        ix += incX;
        iy += incY;
        i += 1;
        i;
    }
}
