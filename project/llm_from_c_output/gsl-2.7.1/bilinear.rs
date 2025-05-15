use std::ptr;

#[repr(C)]
pub struct GslInterp2DType {
    name: *const libc::c_char,
    min_size: libc::size_t,
    alloc: Option<unsafe extern "C" fn(libc::size_t, libc::size_t) -> *mut libc::c_void>,
    init: Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_double, *const libc::c_double, *const libc::c_double, libc::size_t, libc::size_t) -> libc::c_int>,
    eval: Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_double, *const libc::c_double, *const libc::c_double, libc::size_t, libc::size_t, libc::c_double, libc::c_double, *mut libc::c_void, *mut libc::c_void, *mut libc::c_double) -> libc::c_int>,
    deriv_x: Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_double, *const libc::c_double, *const libc::c_double, libc::size_t, libc::size_t, libc::c_double, libc::c_double, *mut libc::c_void, *mut libc::c_void, *mut libc::c_double) -> libc::c_int>,
    deriv_y: Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_double, *const libc::c_double, *const libc::c_double, libc::size_t, libc::size_t, libc::c_double, libc::c_double, *mut libc::c_void, *mut libc::c_void, *mut libc::c_double) -> libc::c_int>,
    deriv_xx: Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_double, *const libc::c_double, *const libc::c_double, libc::size_t, libc::size_t, libc::c_double, libc::c_double, *mut libc::c_void, *mut libc::c_void, *mut libc::c_double) -> libc::c_int>,
    deriv_xy: Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_double, *const libc::c_double, *const libc::c_double, libc::size_t, libc::size_t, libc::c_double, libc::c_double, *mut libc::c_void, *mut libc::c_void, *mut libc::c_double) -> libc::c_int>,
    deriv_yy: Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_double, *const libc::c_double, *const libc::c_double, libc::size_t, libc::size_t, libc::c_double, libc::c_double, *mut libc::c_void, *mut libc::c_void, *mut libc::c_double) -> libc::c_int>,
    free: Option<unsafe extern "C" fn(*mut libc::c_void)>,
}

pub const GSL_SUCCESS: libc::c_int = 0;

unsafe extern "C" fn bilinear_init(
    _state: *mut libc::c_void,
    _xa: *const libc::c_double,
    _ya: *const libc::c_double,
    _za: *const libc::c_double,
    _xsize: libc::size_t,
    _ysize: libc::size_t,
) -> libc::c_int {
    GSL_SUCCESS
}

unsafe extern "C" fn bilinear_eval(
    _state: *const libc::c_void,
    xarr: *const libc::c_double,
    yarr: *const libc::c_double,
    zarr: *const libc::c_double,
    xsize: libc::size_t,
    ysize: libc::size_t,
    x: libc::c_double,
    y: libc::c_double,
    xa: *mut libc::c_void,
    ya: *mut libc::c_void,
    z: *mut libc::c_double,
) -> libc::c_int {
    let xa = if xa.is_null() {
        None
    } else {
        Some(xa)
    };
    let ya = if ya.is_null() {
        None
    } else {
        Some(ya)
    };

    let xi = if let Some(xa) = xa {
        gsl_interp_accel_find(xa, xarr, xsize, x)
    } else {
        gsl_interp_bsearch(xarr, x, 0, xsize - 1)
    };

    let yi = if let Some(ya) = ya {
        gsl_interp_accel_find(ya, yarr, ysize, y)
    } else {
        gsl_interp_bsearch(yarr, y, 0, ysize - 1)
    };

    let xmin = *xarr.offset(xi as isize);
    let xmax = *xarr.offset((xi + 1) as isize);
    let ymin = *yarr.offset(yi as isize);
    let ymax = *yarr.offset((yi + 1) as isize);
    let zminmin = *zarr.offset((yi * xsize + xi) as isize);
    let zminmax = *zarr.offset(((yi + 1) * xsize + xi) as isize);
    let zmaxmin = *zarr.offset((yi * xsize + (xi + 1)) as isize);
    let zmaxmax = *zarr.offset(((yi + 1) * xsize + (xi + 1)) as isize);
    let dx = xmax - xmin;
    let dy = ymax - ymin;
    let t = (x - xmin) / dx;
    let u = (y - ymin) / dy;
    *z = (1.0 - t) * (1.0 - u) * zminmin
        + t * (1.0 - u) * zmaxmin
        + (1.0 - t) * u * zminmax
        + t * u * zmaxmax;

    GSL_SUCCESS
}

unsafe extern "C" fn bilinear_deriv_x(
    _state: *const libc::c_void,
    xarr: *const libc::c_double,
    yarr: *const libc::c_double,
    zarr: *const libc::c_double,
    xsize: libc::size_t,
    ysize: libc::size_t,
    x: libc::c_double,
    y: libc::c_double,
    xa: *mut libc::c_void,
    ya: *mut libc::c_void,
    z_p: *mut libc::c_double,
) -> libc::c_int {
    let xa = if xa.is_null() {
        None
    } else {
        Some(xa)
    };
    let ya = if ya.is_null() {
        None
    } else {
        Some(ya)
    };

    let xi = if let Some(xa) = xa {
        gsl_interp_accel_find(xa, xarr, xsize, x)
    } else {
        gsl_interp_bsearch(xarr, x, 0, xsize - 1)
    };

    let yi = if let Some(ya) = ya {
        gsl_interp_accel_find(ya, yarr, ysize, y)
    } else {
        gsl_interp_bsearch(yarr, y, 0, ysize - 1)
    };

    let xmin = *xarr.offset(xi as isize);
    let xmax = *xarr.offset((xi + 1) as isize);
    let ymin = *yarr.offset(yi as isize);
    let ymax = *yarr.offset((yi + 1) as isize);
    let zminmin = *zarr.offset((yi * xsize + xi) as isize);
    let zminmax = *zarr.offset(((yi + 1) * xsize + xi) as isize);
    let zmaxmin = *zarr.offset((yi * xsize + (xi + 1)) as isize);
    let zmaxmax = *zarr.offset(((yi + 1) * xsize + (xi + 1)) as isize);
    let dx = xmax - xmin;
    let dy = ymax - ymin;
    let dt = 1.0 / dx;
    let u = (y - ymin) / dy;
    *z_p = dt * (-(1.0 - u) * zminmin + (1.0 - u) * zmaxmin - u * zminmax + u * zmaxmax);

    GSL_SUCCESS
}

unsafe extern "C" fn bilinear_deriv_y(
    _state: *const libc::c_void,
    xarr: *const libc::c_double,
    yarr: *const libc::c_double,
    zarr: *const libc::c_double,
    xsize: libc::size_t,
    ysize: libc::size_t,
    x: libc::c_double,
    y: libc::c_double,
    xa: *mut libc::c_void,
    ya: *mut libc::c_void,
    z_p: *mut libc::c_double,
) -> libc::c_int {
    let xa = if xa.is_null() {
        None
    } else {
        Some(xa)
    };
    let ya = if ya.is_null() {
        None
    } else {
        Some(ya)
    };

    let xi = if let Some(xa) = xa {
        gsl_interp_accel_find(xa, xarr, xsize, x)
    } else {
        gsl_interp_bsearch(xarr, x, 0, xsize - 1)
    };

    let yi = if let Some(ya) = ya {
        gsl_interp_accel_find(ya, yarr, ysize, y)
    } else {
        gsl_interp_bsearch(yarr, y, 0, ysize - 1)
    };

    let xmin = *xarr.offset(xi as isize);
    let xmax = *xarr.offset((xi + 1) as isize);
    let ymin = *yarr.offset(yi as isize);
    let ymax = *yarr.offset((yi + 1) as isize);
    let zminmin = *zarr.offset((yi * xsize + xi) as isize);
    let zminmax = *zarr.offset(((yi + 1) * xsize + xi) as isize);
    let zmaxmin = *zarr.offset((yi * xsize + (xi + 1)) as isize);
    let zmaxmax = *zarr.offset(((yi + 1) * xsize + (xi + 1)) as isize);
    let dx = xmax - xmin;
    let dy = ymax - ymin;
    let t = (x - xmin) / dx;
    let du = 1.0 / dy;
    *z_p = du * (-(1.0 - t) * zminmin - t * zmaxmin + (1.0 - t) * zminmax + t * zmaxmax);

    GSL_SUCCESS
}

unsafe extern "C" fn bilinear_deriv2(
    _state: *const libc::c_void,
    _xarr: *const libc::c_double,
    _yarr: *const libc::c_double,
    _zarr: *const libc::c_double,
    _xsize: libc::size_t,
    _ysize: libc::size_t,
    _x: libc::c_double,
    _y: libc::c_double,
    _xa: *mut libc::c_void,
    _ya: *mut libc::c_void,
    z_pp: *mut libc::c_double,
) -> libc::c_int {
    *z_pp = 0.0;
    GSL_SUCCESS
}

unsafe extern "C" fn bilinear_derivxy(
    _state: *const libc::c_void,
    xarr: *const libc::c_double,
    yarr: *const libc::c_double,
    zarr: *const libc::c_double,
    xsize: libc::size_t,
    ysize: libc::size_t,
    x: libc::c_double,
    y: libc::c_double,
    xa: *mut libc::c_void,
    ya: *mut libc::c_void,
    z_pp: *mut libc::c_double,
) -> libc::c_int {
    let xa = if xa.is_null() {
        None
    } else {
        Some(xa)
    };
    let ya = if ya.is_null() {
        None
    } else {
        Some(ya)
    };

    let xi = if let Some(xa) = xa {
        gsl_interp_accel_find(xa, xarr, xsize, x)
    } else {
        gsl_interp_bsearch(xarr, x, 0, xsize - 1)
    };

    let yi = if let Some(ya) = ya {
        gsl_interp_accel_find(ya, yarr, ysize, y)
    } else {
        gsl_interp_bsearch(yarr, y, 0, ysize - 1)
    };

    let xmin = *xarr.offset(xi as isize);
    let xmax = *xarr.offset((xi + 1) as isize);
    let ymin = *yarr.offset(yi as isize);
    let ymax = *yarr.offset((yi + 1) as isize);
    let zminmin = *zarr.offset((yi * xsize + xi) as isize);
    let zminmax = *zarr.offset(((yi + 1) * xsize + xi) as isize);
    let zmaxmin = *zarr.offset((yi * xsize + (xi + 1)) as isize);
    let zmaxmax = *zarr.offset(((yi + 1) * xsize + (xi + 1)) as isize);
    let dx = xmax - xmin;
    let dy = ymax - ymin;
    let dt = 1.0 / dx;
    let du = 1.0 / dy;
    *z_pp = dt * du * (zminmin - zmaxmin - zminmax + zmaxmax);

    GSL_SUCCESS
}

#[no_mangle]
pub static gsl_interp2d_bilinear: *const GslInterp2DType = &GslInterp2DType {
    name: b"bilinear\0".as_ptr() as *const libc::c_char,
    min_size: 2,
    alloc: None,
    init: Some(bilinear_init),
    eval: Some(bilinear_eval),
    deriv_x: Some(bilinear_deriv_x),
    deriv_y: Some(bilinear_deriv_y),
    deriv_xx: Some(bilinear_deriv2),
    deriv_xy: Some(bilinear_derivxy),
    deriv_yy: Some(bilinear_deriv2),
    free: None,
};