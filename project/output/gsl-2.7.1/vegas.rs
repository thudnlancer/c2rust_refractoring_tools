#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    static mut stdout: *mut _IO_FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn gsl_pow_int(x: libc::c_double, n: i32) -> libc::c_double;
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type __off_t = i64;
pub type __off64_t = i64;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
pub type C2RustUnnamed = i32;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_rng_type {
    pub name: *const i8,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: Option<unsafe extern "C" fn(*mut libc::c_void, u64) -> ()>,
    pub get: Option<unsafe extern "C" fn(*mut libc::c_void) -> u64>,
    pub get_double: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_rng {
    pub type_0: *const gsl_rng_type,
    pub state: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_monte_function_struct {
    pub f: Option<
        unsafe extern "C" fn(
            *mut libc::c_double,
            size_t,
            *mut libc::c_void,
        ) -> libc::c_double,
    >,
    pub dim: size_t,
    pub params: *mut libc::c_void,
}
pub type gsl_monte_function = gsl_monte_function_struct;
pub type C2RustUnnamed_0 = i32;
pub const GSL_VEGAS_MODE_STRATIFIED: C2RustUnnamed_0 = -1;
pub const GSL_VEGAS_MODE_IMPORTANCE_ONLY: C2RustUnnamed_0 = 0;
pub const GSL_VEGAS_MODE_IMPORTANCE: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_monte_vegas_state {
    pub dim: size_t,
    pub bins_max: size_t,
    pub bins: u32,
    pub boxes: u32,
    pub xi: *mut libc::c_double,
    pub xin: *mut libc::c_double,
    pub delx: *mut libc::c_double,
    pub weight: *mut libc::c_double,
    pub vol: libc::c_double,
    pub x: *mut libc::c_double,
    pub bin: *mut i32,
    pub box_0: *mut i32,
    pub d: *mut libc::c_double,
    pub alpha: libc::c_double,
    pub mode: i32,
    pub verbose: i32,
    pub iterations: u32,
    pub stage: i32,
    pub jac: libc::c_double,
    pub wtd_int_sum: libc::c_double,
    pub sum_wgts: libc::c_double,
    pub chi_sum: libc::c_double,
    pub chisq: libc::c_double,
    pub result: libc::c_double,
    pub sigma: libc::c_double,
    pub it_start: u32,
    pub it_num: u32,
    pub samples: u32,
    pub calls_per_box: u32,
    pub ostream: *mut FILE,
}
pub type coord = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_monte_vegas_params {
    pub alpha: libc::c_double,
    pub iterations: size_t,
    pub stage: i32,
    pub mode: i32,
    pub verbose: i32,
    pub ostream: *mut FILE,
}
#[inline]
unsafe extern "C" fn gsl_rng_uniform_pos(mut r: *const gsl_rng) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    loop {
        x = ((*(*r).type_0).get_double).expect("non-null function pointer")((*r).state);
        if !(x == 0 as i32 as libc::c_double) {
            break;
        }
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_monte_vegas_integrate(
    mut f: *mut gsl_monte_function,
    mut xl: *mut libc::c_double,
    mut xu: *mut libc::c_double,
    mut dim: size_t,
    mut calls: size_t,
    mut r: *mut gsl_rng,
    mut state: *mut gsl_monte_vegas_state,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
) -> i32 {
    let mut cum_int: libc::c_double = 0.;
    let mut cum_sig: libc::c_double = 0.;
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut it: size_t = 0;
    if dim != (*state).dim {
        gsl_error(
            b"number of dimensions must match allocated size\0" as *const u8
                as *const i8,
            b"vegas.c\0" as *const u8 as *const i8,
            121 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    i = 0 as i32 as size_t;
    while i < dim {
        if *xu.offset(i as isize) <= *xl.offset(i as isize) {
            gsl_error(
                b"xu must be greater than xl\0" as *const u8 as *const i8,
                b"vegas.c\0" as *const u8 as *const i8,
                128 as i32,
                GSL_EINVAL as i32,
            );
            return GSL_EINVAL as i32;
        }
        if *xu.offset(i as isize) - *xl.offset(i as isize) > 1.7976931348623157e+308f64 {
            gsl_error(
                b"Range of integration is too large, please rescale\0" as *const u8
                    as *const i8,
                b"vegas.c\0" as *const u8 as *const i8,
                134 as i32,
                GSL_EINVAL as i32,
            );
            return GSL_EINVAL as i32;
        }
        i = i.wrapping_add(1);
        i;
    }
    if (*state).stage == 0 as i32 {
        init_grid(state, xl, xu, dim);
        if (*state).verbose >= 0 as i32 {
            print_lim(state, xl, xu, dim);
        }
    }
    if (*state).stage <= 1 as i32 {
        (*state).wtd_int_sum = 0 as i32 as libc::c_double;
        (*state).sum_wgts = 0 as i32 as libc::c_double;
        (*state).chi_sum = 0 as i32 as libc::c_double;
        (*state).it_num = 1 as i32 as u32;
        (*state).samples = 0 as i32 as u32;
        (*state).chisq = 0 as i32 as libc::c_double;
    }
    if (*state).stage <= 2 as i32 {
        let mut bins: u32 = (*state).bins_max as u32;
        let mut boxes: u32 = 1 as i32 as u32;
        if (*state).mode != GSL_VEGAS_MODE_IMPORTANCE_ONLY as i32 {
            boxes = floor(
                pow(calls as libc::c_double / 2.0f64, 1.0f64 / dim as libc::c_double),
            ) as u32;
            (*state).mode = GSL_VEGAS_MODE_IMPORTANCE as i32;
            if (2 as i32 as u32).wrapping_mul(boxes) as u64 >= (*state).bins_max {
                let mut box_per_bin: i32 = (if (boxes as u64)
                    .wrapping_div((*state).bins_max) > 1 as i32 as u64
                {
                    (boxes as u64).wrapping_div((*state).bins_max)
                } else {
                    1 as i32 as u64
                }) as i32;
                bins = (if (boxes.wrapping_div(box_per_bin as u32) as u64)
                    < (*state).bins_max
                {
                    boxes.wrapping_div(box_per_bin as u32) as u64
                } else {
                    (*state).bins_max
                }) as u32;
                boxes = (box_per_bin as u32).wrapping_mul(bins);
                (*state).mode = GSL_VEGAS_MODE_STRATIFIED as i32;
            }
        }
        let mut tot_boxes: libc::c_double = gsl_pow_int(
            boxes as libc::c_double,
            dim as i32,
        );
        (*state).calls_per_box = (if calls as libc::c_double / tot_boxes
            > 2 as i32 as libc::c_double
        {
            calls as libc::c_double / tot_boxes
        } else {
            2 as i32 as libc::c_double
        }) as u32;
        calls = ((*state).calls_per_box as libc::c_double * tot_boxes) as size_t;
        (*state).jac = (*state).vol * pow(bins as libc::c_double, dim as libc::c_double)
            / calls as libc::c_double;
        (*state).boxes = boxes;
        if bins != (*state).bins {
            resize_grid(state, bins);
            if (*state).verbose > 1 as i32 {
                print_grid(state, dim);
            }
        }
        if (*state).verbose >= 0 as i32 {
            print_head(
                state,
                dim,
                calls,
                (*state).it_num,
                (*state).bins,
                (*state).boxes,
            );
        }
    }
    (*state).it_start = (*state).it_num;
    cum_int = 0.0f64;
    cum_sig = 0.0f64;
    it = 0 as i32 as size_t;
    while it < (*state).iterations as u64 {
        let mut intgrl: libc::c_double = 0.0f64;
        let mut intgrl_sq: libc::c_double = 0.0f64;
        let mut tss: libc::c_double = 0.0f64;
        let mut wgt: libc::c_double = 0.;
        let mut var: libc::c_double = 0.;
        let mut sig: libc::c_double = 0.;
        let mut calls_per_box: size_t = (*state).calls_per_box as size_t;
        let mut jacbin: libc::c_double = (*state).jac;
        let mut x: *mut libc::c_double = (*state).x;
        let mut bin: *mut coord = (*state).bin;
        (*state).it_num = ((*state).it_start as u64).wrapping_add(it) as u32;
        reset_grid_values(state);
        init_box_coord(state, (*state).box_0);
        loop {
            let mut m: libc::c_double = 0 as i32 as libc::c_double;
            let mut q: libc::c_double = 0 as i32 as libc::c_double;
            let mut f_sq_sum: libc::c_double = 0.0f64;
            k = 0 as i32 as size_t;
            while k < calls_per_box {
                let mut fval: libc::c_double = 0.;
                let mut bin_vol: libc::c_double = 0.;
                random_point(
                    x,
                    bin,
                    &mut bin_vol,
                    (*state).box_0 as *const coord,
                    xl as *const libc::c_double,
                    xu as *const libc::c_double,
                    state,
                    r,
                );
                ::core::ptr::write_volatile(
                    &mut fval as *mut libc::c_double,
                    jacbin * bin_vol
                        * (Some(((*f).f).expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(x, (*f).dim, (*f).params),
                );
                let mut d: libc::c_double = fval - m;
                ::core::ptr::write_volatile(
                    &mut m as *mut libc::c_double,
                    ::core::ptr::read_volatile::<
                        libc::c_double,
                    >(&m as *const libc::c_double) + d / (k as libc::c_double + 1.0f64),
                );
                ::core::ptr::write_volatile(
                    &mut q as *mut libc::c_double,
                    ::core::ptr::read_volatile::<
                        libc::c_double,
                    >(&q as *const libc::c_double)
                        + d * d * (k as libc::c_double / (k as libc::c_double + 1.0f64)),
                );
                if (*state).mode != GSL_VEGAS_MODE_STRATIFIED as i32 {
                    let mut f_sq: libc::c_double = fval * fval;
                    accumulate_distribution(state, bin, f_sq);
                }
                k = k.wrapping_add(1);
                k;
            }
            intgrl += m * calls_per_box as libc::c_double;
            f_sq_sum = q * calls_per_box as libc::c_double;
            tss += f_sq_sum;
            if (*state).mode == GSL_VEGAS_MODE_STRATIFIED as i32 {
                accumulate_distribution(state, bin, f_sq_sum);
            }
            if !(change_box_coord(state, (*state).box_0) != 0) {
                break;
            }
        }
        var = tss / (calls_per_box as libc::c_double - 1.0f64);
        if var > 0 as i32 as libc::c_double {
            wgt = 1.0f64 / var;
        } else if (*state).sum_wgts > 0 as i32 as libc::c_double {
            wgt = (*state).sum_wgts / (*state).samples as libc::c_double;
        } else {
            wgt = 0.0f64;
        }
        intgrl_sq = intgrl * intgrl;
        sig = sqrt(var);
        (*state).result = intgrl;
        (*state).sigma = sig;
        if wgt > 0.0f64 {
            let mut sum_wgts: libc::c_double = (*state).sum_wgts;
            let mut wtd_int_sum: libc::c_double = (*state).wtd_int_sum;
            let mut m_0: libc::c_double = if sum_wgts > 0 as i32 as libc::c_double {
                wtd_int_sum / sum_wgts
            } else {
                0 as i32 as libc::c_double
            };
            let mut q_0: libc::c_double = intgrl - m_0;
            (*state).samples = ((*state).samples).wrapping_add(1);
            (*state).samples;
            (*state).sum_wgts += wgt;
            (*state).wtd_int_sum += intgrl * wgt;
            (*state).chi_sum += intgrl_sq * wgt;
            cum_int = (*state).wtd_int_sum / (*state).sum_wgts;
            cum_sig = sqrt(1 as i32 as libc::c_double / (*state).sum_wgts);
            if (*state).samples == 1 as i32 as u32 {
                (*state).chisq = 0 as i32 as libc::c_double;
            } else {
                (*state).chisq *= (*state).samples as libc::c_double - 2.0f64;
                (*state).chisq
                    += wgt / (1 as i32 as libc::c_double + wgt / sum_wgts) * q_0 * q_0;
                (*state).chisq /= (*state).samples as libc::c_double - 1.0f64;
            }
        } else {
            cum_int += (intgrl - cum_int) / (it as libc::c_double + 1.0f64);
            cum_sig = 0.0f64;
        }
        if (*state).verbose >= 0 as i32 {
            print_res(
                state,
                (*state).it_num,
                intgrl,
                sig,
                cum_int,
                cum_sig,
                (*state).chisq,
            );
            if it.wrapping_add(1 as i32 as u64) == (*state).iterations as u64
                && (*state).verbose > 0 as i32
            {
                print_grid(state, dim);
            }
        }
        if (*state).verbose > 1 as i32 {
            print_dist(state, dim);
        }
        refine_grid(state);
        if (*state).verbose > 1 as i32 {
            print_grid(state, dim);
        }
        it = it.wrapping_add(1);
        it;
    }
    (*state).stage = 1 as i32;
    *result = cum_int;
    *abserr = cum_sig;
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_monte_vegas_alloc(
    mut dim: size_t,
) -> *mut gsl_monte_vegas_state {
    let mut s: *mut gsl_monte_vegas_state = malloc(
        ::core::mem::size_of::<gsl_monte_vegas_state>() as u64,
    ) as *mut gsl_monte_vegas_state;
    if s.is_null() {
        gsl_error(
            b"failed to allocate space for vegas state struct\0" as *const u8
                as *const i8,
            b"vegas.c\0" as *const u8 as *const i8,
            390 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_monte_vegas_state;
    }
    (*s).delx = malloc(dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64))
        as *mut libc::c_double;
    if ((*s).delx).is_null() {
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for delx\0" as *const u8 as *const i8,
            b"vegas.c\0" as *const u8 as *const i8,
            398 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_monte_vegas_state;
    }
    (*s).d = malloc(
        (50 as i32 as u64)
            .wrapping_mul(dim)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*s).d).is_null() {
        free((*s).delx as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for d\0" as *const u8 as *const i8,
            b"vegas.c\0" as *const u8 as *const i8,
            407 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_monte_vegas_state;
    }
    (*s).xi = malloc(
        ((50 as i32 + 1 as i32) as u64)
            .wrapping_mul(dim)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*s).xi).is_null() {
        free((*s).d as *mut libc::c_void);
        free((*s).delx as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for xi\0" as *const u8 as *const i8,
            b"vegas.c\0" as *const u8 as *const i8,
            417 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_monte_vegas_state;
    }
    (*s).xin = malloc(
        ((50 as i32 + 1 as i32) as u64)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*s).xin).is_null() {
        free((*s).xi as *mut libc::c_void);
        free((*s).d as *mut libc::c_void);
        free((*s).delx as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for xin\0" as *const u8 as *const i8,
            b"vegas.c\0" as *const u8 as *const i8,
            428 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_monte_vegas_state;
    }
    (*s).weight = malloc(
        (50 as i32 as u64).wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*s).weight).is_null() {
        free((*s).xin as *mut libc::c_void);
        free((*s).xi as *mut libc::c_void);
        free((*s).d as *mut libc::c_void);
        free((*s).delx as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for xin\0" as *const u8 as *const i8,
            b"vegas.c\0" as *const u8 as *const i8,
            440 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_monte_vegas_state;
    }
    (*s).box_0 = malloc(dim.wrapping_mul(::core::mem::size_of::<coord>() as u64))
        as *mut coord;
    if ((*s).box_0).is_null() {
        free((*s).weight as *mut libc::c_void);
        free((*s).xin as *mut libc::c_void);
        free((*s).xi as *mut libc::c_void);
        free((*s).d as *mut libc::c_void);
        free((*s).delx as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for box\0" as *const u8 as *const i8,
            b"vegas.c\0" as *const u8 as *const i8,
            453 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_monte_vegas_state;
    }
    (*s).bin = malloc(dim.wrapping_mul(::core::mem::size_of::<coord>() as u64))
        as *mut coord;
    if ((*s).bin).is_null() {
        free((*s).box_0 as *mut libc::c_void);
        free((*s).weight as *mut libc::c_void);
        free((*s).xin as *mut libc::c_void);
        free((*s).xi as *mut libc::c_void);
        free((*s).d as *mut libc::c_void);
        free((*s).delx as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for bin\0" as *const u8 as *const i8,
            b"vegas.c\0" as *const u8 as *const i8,
            467 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_monte_vegas_state;
    }
    (*s).x = malloc(dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64))
        as *mut libc::c_double;
    if ((*s).x).is_null() {
        free((*s).bin as *mut libc::c_void);
        free((*s).box_0 as *mut libc::c_void);
        free((*s).weight as *mut libc::c_void);
        free((*s).xin as *mut libc::c_void);
        free((*s).xi as *mut libc::c_void);
        free((*s).d as *mut libc::c_void);
        free((*s).delx as *mut libc::c_void);
        free(s as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for x\0" as *const u8 as *const i8,
            b"vegas.c\0" as *const u8 as *const i8,
            482 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_monte_vegas_state;
    }
    (*s).dim = dim;
    (*s).bins_max = 50 as i32 as size_t;
    gsl_monte_vegas_init(s);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_monte_vegas_init(
    mut state: *mut gsl_monte_vegas_state,
) -> i32 {
    (*state).stage = 0 as i32;
    (*state).alpha = 1.5f64;
    (*state).verbose = -(1 as i32);
    (*state).iterations = 5 as i32 as u32;
    (*state).mode = GSL_VEGAS_MODE_IMPORTANCE as i32;
    (*state).chisq = 0 as i32 as libc::c_double;
    (*state).bins = (*state).bins_max as u32;
    (*state).ostream = stdout;
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_monte_vegas_free(mut s: *mut gsl_monte_vegas_state) {
    if s.is_null() {
        return;
    }
    free((*s).x as *mut libc::c_void);
    free((*s).delx as *mut libc::c_void);
    free((*s).d as *mut libc::c_void);
    free((*s).xi as *mut libc::c_void);
    free((*s).xin as *mut libc::c_void);
    free((*s).weight as *mut libc::c_void);
    free((*s).box_0 as *mut libc::c_void);
    free((*s).bin as *mut libc::c_void);
    free(s as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_monte_vegas_chisq(
    mut s: *const gsl_monte_vegas_state,
) -> libc::c_double {
    return (*s).chisq;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_monte_vegas_runval(
    mut s: *const gsl_monte_vegas_state,
    mut result: *mut libc::c_double,
    mut sigma: *mut libc::c_double,
) {
    *result = (*s).result;
    *sigma = (*s).sigma;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_monte_vegas_params_get(
    mut s: *const gsl_monte_vegas_state,
    mut p: *mut gsl_monte_vegas_params,
) {
    (*p).alpha = (*s).alpha;
    (*p).iterations = (*s).iterations as size_t;
    (*p).stage = (*s).stage;
    (*p).mode = (*s).mode;
    (*p).verbose = (*s).verbose;
    (*p).ostream = (*s).ostream;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_monte_vegas_params_set(
    mut s: *mut gsl_monte_vegas_state,
    mut p: *const gsl_monte_vegas_params,
) {
    (*s).alpha = (*p).alpha;
    (*s).iterations = (*p).iterations as u32;
    (*s).stage = (*p).stage;
    (*s).mode = (*p).mode;
    (*s).verbose = (*p).verbose;
    (*s).ostream = (*p).ostream;
}
unsafe extern "C" fn init_box_coord(
    mut s: *mut gsl_monte_vegas_state,
    mut box_0: *mut coord,
) {
    let mut i: size_t = 0;
    let mut dim: size_t = (*s).dim;
    i = 0 as i32 as size_t;
    while i < dim {
        *box_0.offset(i as isize) = 0 as i32;
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn change_box_coord(
    mut s: *mut gsl_monte_vegas_state,
    mut box_0: *mut coord,
) -> i32 {
    let mut j: i32 = ((*s).dim).wrapping_sub(1 as i32 as u64) as i32;
    let mut ng: i32 = (*s).boxes as i32;
    while j >= 0 as i32 {
        *box_0.offset(j as isize) = (*box_0.offset(j as isize) + 1 as i32) % ng;
        if *box_0.offset(j as isize) != 0 as i32 {
            return 1 as i32;
        }
        j -= 1;
        j;
    }
    return 0 as i32;
}
unsafe extern "C" fn init_grid(
    mut s: *mut gsl_monte_vegas_state,
    mut xl: *mut libc::c_double,
    mut xu: *mut libc::c_double,
    mut dim: size_t,
) {
    let mut j: size_t = 0;
    let mut vol: libc::c_double = 1.0f64;
    (*s).bins = 1 as i32 as u32;
    j = 0 as i32 as size_t;
    while j < dim {
        let mut dx: libc::c_double = *xu.offset(j as isize) - *xl.offset(j as isize);
        *((*s).delx).offset(j as isize) = dx;
        vol *= dx;
        *((*s).xi)
            .offset((0 as i32 as u64).wrapping_mul((*s).dim).wrapping_add(j) as isize) = 0.0f64;
        *((*s).xi)
            .offset((1 as i32 as u64).wrapping_mul((*s).dim).wrapping_add(j) as isize) = 1.0f64;
        j = j.wrapping_add(1);
        j;
    }
    (*s).vol = vol;
}
unsafe extern "C" fn reset_grid_values(mut s: *mut gsl_monte_vegas_state) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut dim: size_t = (*s).dim;
    let mut bins: size_t = (*s).bins as size_t;
    i = 0 as i32 as size_t;
    while i < bins {
        j = 0 as i32 as size_t;
        while j < dim {
            *((*s).d).offset(i.wrapping_mul((*s).dim).wrapping_add(j) as isize) = 0.0f64;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn accumulate_distribution(
    mut s: *mut gsl_monte_vegas_state,
    mut bin: *mut coord,
    mut y: libc::c_double,
) {
    let mut j: size_t = 0;
    let mut dim: size_t = (*s).dim;
    j = 0 as i32 as size_t;
    while j < dim {
        let mut i: i32 = *bin.offset(j as isize);
        *((*s).d).offset((i as u64).wrapping_mul((*s).dim).wrapping_add(j) as isize)
            += y;
        j = j.wrapping_add(1);
        j;
    }
}
unsafe extern "C" fn random_point(
    mut x: *mut libc::c_double,
    mut bin: *mut coord,
    mut bin_vol: *mut libc::c_double,
    mut box_0: *const coord,
    mut xl: *const libc::c_double,
    mut xu: *const libc::c_double,
    mut s: *mut gsl_monte_vegas_state,
    mut r: *mut gsl_rng,
) {
    let mut vol: libc::c_double = 1.0f64;
    let mut j: size_t = 0;
    let mut dim: size_t = (*s).dim;
    let mut bins: size_t = (*s).bins as size_t;
    let mut boxes: size_t = (*s).boxes as size_t;
    while if !xu.is_null() { 0 as i32 } else { 0 as i32 } != 0 {}
    j = 0 as i32 as size_t;
    while j < dim {
        let mut z: libc::c_double = (*box_0.offset(j as isize) as libc::c_double
            + gsl_rng_uniform_pos(r)) / boxes as libc::c_double * bins as libc::c_double;
        let mut k: i32 = z as i32;
        let mut y: libc::c_double = 0.;
        let mut bin_width: libc::c_double = 0.;
        *bin.offset(j as isize) = k;
        if k == 0 as i32 {
            bin_width = *((*s).xi)
                .offset(
                    (1 as i32 as u64).wrapping_mul((*s).dim).wrapping_add(j) as isize,
                );
            y = z * bin_width;
        } else {
            bin_width = *((*s).xi)
                .offset(
                    ((k + 1 as i32) as u64).wrapping_mul((*s).dim).wrapping_add(j)
                        as isize,
                )
                - *((*s).xi)
                    .offset((k as u64).wrapping_mul((*s).dim).wrapping_add(j) as isize);
            y = *((*s).xi)
                .offset((k as u64).wrapping_mul((*s).dim).wrapping_add(j) as isize)
                + (z - k as libc::c_double) * bin_width;
        }
        *x.offset(j as isize) = *xl.offset(j as isize)
            + y * *((*s).delx).offset(j as isize);
        vol *= bin_width;
        j = j.wrapping_add(1);
        j;
    }
    *bin_vol = vol;
}
unsafe extern "C" fn resize_grid(mut s: *mut gsl_monte_vegas_state, mut bins: u32) {
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut dim: size_t = (*s).dim;
    let mut pts_per_bin: libc::c_double = (*s).bins as libc::c_double
        / bins as libc::c_double;
    j = 0 as i32 as size_t;
    while j < dim {
        let mut xold: libc::c_double = 0.;
        let mut xnew: libc::c_double = 0 as i32 as libc::c_double;
        let mut dw: libc::c_double = 0 as i32 as libc::c_double;
        let mut i: i32 = 1 as i32;
        k = 1 as i32 as size_t;
        while k <= (*s).bins as u64 {
            dw += 1.0f64;
            xold = xnew;
            xnew = *((*s).xi).offset(k.wrapping_mul((*s).dim).wrapping_add(j) as isize);
            while dw > pts_per_bin {
                dw -= pts_per_bin;
                *((*s).xin).offset(i as isize) = xnew - (xnew - xold) * dw;
                i += 1;
                i;
            }
            k = k.wrapping_add(1);
            k;
        }
        k = 1 as i32 as size_t;
        while k < bins as u64 {
            *((*s).xi).offset(k.wrapping_mul((*s).dim).wrapping_add(j) as isize) = *((*s)
                .xin)
                .offset(k as isize);
            k = k.wrapping_add(1);
            k;
        }
        *((*s).xi)
            .offset((bins as u64).wrapping_mul((*s).dim).wrapping_add(j) as isize) = 1
            as i32 as libc::c_double;
        j = j.wrapping_add(1);
        j;
    }
    (*s).bins = bins;
}
unsafe extern "C" fn refine_grid(mut s: *mut gsl_monte_vegas_state) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut dim: size_t = (*s).dim;
    let mut bins: size_t = (*s).bins as size_t;
    j = 0 as i32 as size_t;
    while j < dim {
        let mut grid_tot_j: libc::c_double = 0.;
        let mut tot_weight: libc::c_double = 0.;
        let mut weight: *mut libc::c_double = (*s).weight;
        let mut oldg: libc::c_double = *((*s).d)
            .offset((0 as i32 as u64).wrapping_mul((*s).dim).wrapping_add(j) as isize);
        let mut newg: libc::c_double = *((*s).d)
            .offset((1 as i32 as u64).wrapping_mul((*s).dim).wrapping_add(j) as isize);
        *((*s).d)
            .offset((0 as i32 as u64).wrapping_mul((*s).dim).wrapping_add(j) as isize) = (oldg
            + newg) / 2 as i32 as libc::c_double;
        grid_tot_j = *((*s).d)
            .offset((0 as i32 as u64).wrapping_mul((*s).dim).wrapping_add(j) as isize);
        i = 1 as i32 as size_t;
        while i < bins.wrapping_sub(1 as i32 as u64) {
            let mut rc: libc::c_double = oldg + newg;
            oldg = newg;
            newg = *((*s).d)
                .offset(
                    i
                        .wrapping_add(1 as i32 as u64)
                        .wrapping_mul((*s).dim)
                        .wrapping_add(j) as isize,
                );
            *((*s).d).offset(i.wrapping_mul((*s).dim).wrapping_add(j) as isize) = (rc
                + newg) / 3 as i32 as libc::c_double;
            grid_tot_j
                += *((*s).d).offset(i.wrapping_mul((*s).dim).wrapping_add(j) as isize);
            i = i.wrapping_add(1);
            i;
        }
        *((*s).d)
            .offset(
                bins.wrapping_sub(1 as i32 as u64).wrapping_mul((*s).dim).wrapping_add(j)
                    as isize,
            ) = (newg + oldg) / 2 as i32 as libc::c_double;
        grid_tot_j
            += *((*s).d)
                .offset(
                    bins
                        .wrapping_sub(1 as i32 as u64)
                        .wrapping_mul((*s).dim)
                        .wrapping_add(j) as isize,
                );
        tot_weight = 0 as i32 as libc::c_double;
        i = 0 as i32 as size_t;
        while i < bins {
            *weight.offset(i as isize) = 0 as i32 as libc::c_double;
            if *((*s).d).offset(i.wrapping_mul((*s).dim).wrapping_add(j) as isize)
                > 0 as i32 as libc::c_double
            {
                oldg = grid_tot_j
                    / *((*s).d)
                        .offset(i.wrapping_mul((*s).dim).wrapping_add(j) as isize);
                *weight.offset(i as isize) = pow(
                    (oldg - 1 as i32 as libc::c_double) / oldg / log(oldg),
                    (*s).alpha,
                );
            }
            tot_weight += *weight.offset(i as isize);
            i = i.wrapping_add(1);
            i;
        }
        let mut pts_per_bin: libc::c_double = tot_weight / bins as libc::c_double;
        let mut xold: libc::c_double = 0.;
        let mut xnew: libc::c_double = 0 as i32 as libc::c_double;
        let mut dw: libc::c_double = 0 as i32 as libc::c_double;
        i = 1 as i32 as size_t;
        k = 0 as i32 as size_t;
        while k < bins {
            dw += *weight.offset(k as isize);
            xold = xnew;
            xnew = *((*s).xi)
                .offset(
                    k
                        .wrapping_add(1 as i32 as u64)
                        .wrapping_mul((*s).dim)
                        .wrapping_add(j) as isize,
                );
            while dw > pts_per_bin {
                dw -= pts_per_bin;
                *((*s).xin).offset(i as isize) = xnew
                    - (xnew - xold) * dw / *weight.offset(k as isize);
                i = i.wrapping_add(1);
                i;
            }
            k = k.wrapping_add(1);
            k;
        }
        k = 1 as i32 as size_t;
        while k < bins {
            *((*s).xi).offset(k.wrapping_mul((*s).dim).wrapping_add(j) as isize) = *((*s)
                .xin)
                .offset(k as isize);
            k = k.wrapping_add(1);
            k;
        }
        *((*s).xi).offset(bins.wrapping_mul((*s).dim).wrapping_add(j) as isize) = 1
            as i32 as libc::c_double;
        j = j.wrapping_add(1);
        j;
    }
}
unsafe extern "C" fn print_lim(
    mut state: *mut gsl_monte_vegas_state,
    mut xl: *mut libc::c_double,
    mut xu: *mut libc::c_double,
    mut dim: u64,
) {
    let mut j: u64 = 0;
    fprintf(
        (*state).ostream,
        b"The limits of integration are:\n\0" as *const u8 as *const i8,
    );
    j = 0 as i32 as u64;
    while j < dim {
        fprintf(
            (*state).ostream,
            b"\nxl[%lu]=%f    xu[%lu]=%f\0" as *const u8 as *const i8,
            j,
            *xl.offset(j as isize),
            j,
            *xu.offset(j as isize),
        );
        j = j.wrapping_add(1);
        j;
    }
    fprintf((*state).ostream, b"\n\0" as *const u8 as *const i8);
    fflush((*state).ostream);
}
unsafe extern "C" fn print_head(
    mut state: *mut gsl_monte_vegas_state,
    mut num_dim: u64,
    mut calls: u64,
    mut it_num: u32,
    mut bins: u32,
    mut boxes: u32,
) {
    fprintf(
        (*state).ostream,
        b"\nnum_dim=%lu, calls=%lu, it_num=%d, max_it_num=%d \0" as *const u8
            as *const i8,
        num_dim,
        calls,
        it_num,
        (*state).iterations,
    );
    fprintf(
        (*state).ostream,
        b"verb=%d, alph=%.2f,\nmode=%d, bins=%d, boxes=%d\n\0" as *const u8 as *const i8,
        (*state).verbose,
        (*state).alpha,
        (*state).mode,
        bins,
        boxes,
    );
    fprintf(
        (*state).ostream,
        b"\n       single.......iteration                   \0" as *const u8 as *const i8,
    );
    fprintf(
        (*state).ostream,
        b"accumulated......results   \n\0" as *const u8 as *const i8,
    );
    fprintf(
        (*state).ostream,
        b"iteration     integral    sigma             integral   \0" as *const u8
            as *const i8,
    );
    fprintf(
        (*state).ostream,
        b"      sigma     chi-sq/it\n\n\0" as *const u8 as *const i8,
    );
    fflush((*state).ostream);
}
unsafe extern "C" fn print_res(
    mut state: *mut gsl_monte_vegas_state,
    mut itr: u32,
    mut res: libc::c_double,
    mut err: libc::c_double,
    mut cum_res: libc::c_double,
    mut cum_err: libc::c_double,
    mut chi_sq: libc::c_double,
) {
    fprintf(
        (*state).ostream,
        b"%4d        %6.4e %10.2e          %6.4e      %8.2e  %10.2e\n\0" as *const u8
            as *const i8,
        itr,
        res,
        err,
        cum_res,
        cum_err,
        chi_sq,
    );
    fflush((*state).ostream);
}
unsafe extern "C" fn print_dist(mut state: *mut gsl_monte_vegas_state, mut dim: u64) {
    let mut i: u64 = 0;
    let mut j: u64 = 0;
    let mut p: i32 = (*state).verbose;
    if p < 1 as i32 {
        return;
    }
    j = 0 as i32 as u64;
    while j < dim {
        fprintf((*state).ostream, b"\n axis %lu \n\0" as *const u8 as *const i8, j);
        fprintf((*state).ostream, b"      x   g\n\0" as *const u8 as *const i8);
        i = 0 as i32 as u64;
        while i < (*state).bins as u64 {
            fprintf(
                (*state).ostream,
                b"weight [%11.2e , %11.2e] = \0" as *const u8 as *const i8,
                *((*state).xi)
                    .offset(i.wrapping_mul((*state).dim).wrapping_add(j) as isize),
                *((*state).xi)
                    .offset(
                        i
                            .wrapping_add(1 as i32 as u64)
                            .wrapping_mul((*state).dim)
                            .wrapping_add(j) as isize,
                    ),
            );
            fprintf(
                (*state).ostream,
                b" %11.2e\n\0" as *const u8 as *const i8,
                *((*state).d)
                    .offset(i.wrapping_mul((*state).dim).wrapping_add(j) as isize),
            );
            i = i.wrapping_add(1);
            i;
        }
        fprintf((*state).ostream, b"\n\0" as *const u8 as *const i8);
        j = j.wrapping_add(1);
        j;
    }
    fprintf((*state).ostream, b"\n\0" as *const u8 as *const i8);
    fflush((*state).ostream);
}
unsafe extern "C" fn print_grid(mut state: *mut gsl_monte_vegas_state, mut dim: u64) {
    let mut i: u64 = 0;
    let mut j: u64 = 0;
    let mut p: i32 = (*state).verbose;
    if p < 1 as i32 {
        return;
    }
    j = 0 as i32 as u64;
    while j < dim {
        fprintf((*state).ostream, b"\n axis %lu \n\0" as *const u8 as *const i8, j);
        fprintf((*state).ostream, b"      x   \n\0" as *const u8 as *const i8);
        i = 0 as i32 as u64;
        while i <= (*state).bins as u64 {
            fprintf(
                (*state).ostream,
                b"%11.2e\0" as *const u8 as *const i8,
                *((*state).xi)
                    .offset(i.wrapping_mul((*state).dim).wrapping_add(j) as isize),
            );
            if i.wrapping_rem(5 as i32 as u64) == 4 as i32 as u64 {
                fprintf((*state).ostream, b"\n\0" as *const u8 as *const i8);
            }
            i = i.wrapping_add(1);
            i;
        }
        fprintf((*state).ostream, b"\n\0" as *const u8 as *const i8);
        j = j.wrapping_add(1);
        j;
    }
    fprintf((*state).ostream, b"\n\0" as *const u8 as *const i8);
    fflush((*state).ostream);
}