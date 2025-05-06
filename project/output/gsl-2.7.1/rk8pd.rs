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
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
}
pub type size_t = u64;
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
pub struct gsl_odeiv_system {
    pub function: Option<
        unsafe extern "C" fn(
            libc::c_double,
            *const libc::c_double,
            *mut libc::c_double,
            *mut libc::c_void,
        ) -> i32,
    >,
    pub jacobian: Option<
        unsafe extern "C" fn(
            libc::c_double,
            *const libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_void,
        ) -> i32,
    >,
    pub dimension: size_t,
    pub params: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_odeiv_step_type {
    pub name: *const i8,
    pub can_use_dydt_in: i32,
    pub gives_exact_dydt_out: i32,
    pub alloc: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub apply: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            size_t,
            libc::c_double,
            libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
            *const libc::c_double,
            *mut libc::c_double,
            *const gsl_odeiv_system,
        ) -> i32,
    >,
    pub reset: Option<unsafe extern "C" fn(*mut libc::c_void, size_t) -> i32>,
    pub order: Option<unsafe extern "C" fn(*mut libc::c_void) -> u32>,
    pub free: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rk8pd_state_t {
    pub k: [*mut libc::c_double; 13],
    pub ytmp: *mut libc::c_double,
    pub y0: *mut libc::c_double,
}
static mut Abar: [libc::c_double; 13] = [
    14005451.0f64 / 335480064.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    -59238493.0f64 / 1068277825.0f64,
    181606767.0f64 / 758867731.0f64,
    561292985.0f64 / 797845732.0f64,
    -1041891430.0f64 / 1371343529.0f64,
    760417239.0f64 / 1151165299.0f64,
    118820643.0f64 / 751138087.0f64,
    -528747749.0f64 / 2220607170.0f64,
    1.0f64 / 4.0f64,
];
static mut A: [libc::c_double; 12] = [
    13451932.0f64 / 455176623.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    -808719846.0f64 / 976000145.0f64,
    1757004468.0f64 / 5645159321.0f64,
    656045339.0f64 / 265891186.0f64,
    -3867574721.0f64 / 1518517206.0f64,
    465885868.0f64 / 322736535.0f64,
    53011238.0f64 / 667516719.0f64,
    2.0f64 / 45.0f64,
];
static mut ah: [libc::c_double; 10] = [
    1.0f64 / 18.0f64,
    1.0f64 / 12.0f64,
    1.0f64 / 8.0f64,
    5.0f64 / 16.0f64,
    3.0f64 / 8.0f64,
    59.0f64 / 400.0f64,
    93.0f64 / 200.0f64,
    5490023248.0f64 / 9719169821.0f64,
    13.0f64 / 20.0f64,
    1201146811.0f64 / 1299019798.0f64,
];
static mut b21: libc::c_double = 1.0f64 / 18.0f64;
static mut b3: [libc::c_double; 2] = [1.0f64 / 48.0f64, 1.0f64 / 16.0f64];
static mut b4: [libc::c_double; 3] = [1.0f64 / 32.0f64, 0.0f64, 3.0f64 / 32.0f64];
static mut b5: [libc::c_double; 4] = [
    5.0f64 / 16.0f64,
    0.0f64,
    -75.0f64 / 64.0f64,
    75.0f64 / 64.0f64,
];
static mut b6: [libc::c_double; 5] = [
    3.0f64 / 80.0f64,
    0.0f64,
    0.0f64,
    3.0f64 / 16.0f64,
    3.0f64 / 20.0f64,
];
static mut b7: [libc::c_double; 6] = [
    29443841.0f64 / 614563906.0f64,
    0.0f64,
    0.0f64,
    77736538.0f64 / 692538347.0f64,
    -28693883.0f64 / 1125000000.0f64,
    23124283.0f64 / 1800000000.0f64,
];
static mut b8: [libc::c_double; 7] = [
    16016141.0f64 / 946692911.0f64,
    0.0f64,
    0.0f64,
    61564180.0f64 / 158732637.0f64,
    22789713.0f64 / 633445777.0f64,
    545815736.0f64 / 2771057229.0f64,
    -180193667.0f64 / 1043307555.0f64,
];
static mut b9: [libc::c_double; 8] = [
    39632708.0f64 / 573591083.0f64,
    0.0f64,
    0.0f64,
    -433636366.0f64 / 683701615.0f64,
    -421739975.0f64 / 2616292301.0f64,
    100302831.0f64 / 723423059.0f64,
    790204164.0f64 / 839813087.0f64,
    800635310.0f64 / 3783071287.0f64,
];
static mut b10: [libc::c_double; 9] = [
    246121993.0f64 / 1340847787.0f64,
    0.0f64,
    0.0f64,
    -37695042795.0f64 / 15268766246.0f64,
    -309121744.0f64 / 1061227803.0f64,
    -12992083.0f64 / 490766935.0f64,
    6005943493.0f64 / 2108947869.0f64,
    393006217.0f64 / 1396673457.0f64,
    123872331.0f64 / 1001029789.0f64,
];
static mut b11: [libc::c_double; 10] = [
    -1028468189.0f64 / 846180014.0f64,
    0.0f64,
    0.0f64,
    8478235783.0f64 / 508512852.0f64,
    1311729495.0f64 / 1432422823.0f64,
    -10304129995.0f64 / 1701304382.0f64,
    -48777925059.0f64 / 3047939560.0f64,
    15336726248.0f64 / 1032824649.0f64,
    -45442868181.0f64 / 3398467696.0f64,
    3065993473.0f64 / 597172653.0f64,
];
static mut b12: [libc::c_double; 11] = [
    185892177.0f64 / 718116043.0f64,
    0.0f64,
    0.0f64,
    -3185094517.0f64 / 667107341.0f64,
    -477755414.0f64 / 1098053517.0f64,
    -703635378.0f64 / 230739211.0f64,
    5731566787.0f64 / 1027545527.0f64,
    5232866602.0f64 / 850066563.0f64,
    -4093664535.0f64 / 808688257.0f64,
    3962137247.0f64 / 1805957418.0f64,
    65686358.0f64 / 487910083.0f64,
];
static mut b13: [libc::c_double; 12] = [
    403863854.0f64 / 491063109.0f64,
    0.0f64,
    0.0f64,
    -5068492393.0f64 / 434740067.0f64,
    -411421997.0f64 / 543043805.0f64,
    652783627.0f64 / 914296604.0f64,
    11173962825.0f64 / 925320556.0f64,
    -13158990841.0f64 / 6184727034.0f64,
    3936647629.0f64 / 1978049680.0f64,
    -160528059.0f64 / 685178525.0f64,
    248638103.0f64 / 1413531060.0f64,
    0.0f64,
];
unsafe extern "C" fn rk8pd_alloc(mut dim: size_t) -> *mut libc::c_void {
    let mut state: *mut rk8pd_state_t = malloc(
        ::core::mem::size_of::<rk8pd_state_t>() as u64,
    ) as *mut rk8pd_state_t;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if state.is_null() {
        gsl_error(
            b"failed to allocate space for rk8pd_state\0" as *const u8 as *const i8,
            b"rk8pd.c\0" as *const u8 as *const i8,
            182 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).ytmp = malloc(
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*state).ytmp).is_null() {
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for ytmp\0" as *const u8 as *const i8,
            b"rk8pd.c\0" as *const u8 as *const i8,
            190 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).y0 = malloc(
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*state).y0).is_null() {
        free((*state).ytmp as *mut libc::c_void);
        free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for y0\0" as *const u8 as *const i8,
            b"rk8pd.c\0" as *const u8 as *const i8,
            199 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut libc::c_void;
    }
    i = 0 as i32;
    while i < 13 as i32 {
        (*state).k[i as usize] = malloc(
            dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
        ) as *mut libc::c_double;
        if ((*state).k[i as usize]).is_null() {
            j = 0 as i32;
            while j < i {
                free((*state).k[j as usize] as *mut libc::c_void);
                j += 1;
                j;
            }
            free((*state).y0 as *mut libc::c_void);
            free((*state).ytmp as *mut libc::c_void);
            free(state as *mut libc::c_void);
            gsl_error(
                b"failed to allocate space for k's\0" as *const u8 as *const i8,
                b"rk8pd.c\0" as *const u8 as *const i8,
                215 as i32,
                GSL_ENOMEM as i32,
            );
            return 0 as *mut libc::c_void;
        }
        i += 1;
        i;
    }
    return state as *mut libc::c_void;
}
unsafe extern "C" fn rk8pd_apply(
    mut vstate: *mut libc::c_void,
    mut dim: size_t,
    mut t: libc::c_double,
    mut h: libc::c_double,
    mut y: *mut libc::c_double,
    mut yerr: *mut libc::c_double,
    mut dydt_in: *const libc::c_double,
    mut dydt_out: *mut libc::c_double,
    mut sys: *const gsl_odeiv_system,
) -> i32 {
    let mut state: *mut rk8pd_state_t = vstate as *mut rk8pd_state_t;
    let mut i: size_t = 0;
    let ytmp: *mut libc::c_double = (*state).ytmp;
    let y0: *mut libc::c_double = (*state).y0;
    let k1: *mut libc::c_double = (*state).k[0 as i32 as usize];
    let k2: *mut libc::c_double = (*state).k[1 as i32 as usize];
    let k3: *mut libc::c_double = (*state).k[2 as i32 as usize];
    let k4: *mut libc::c_double = (*state).k[3 as i32 as usize];
    let k5: *mut libc::c_double = (*state).k[4 as i32 as usize];
    let k6: *mut libc::c_double = (*state).k[5 as i32 as usize];
    let k7: *mut libc::c_double = (*state).k[6 as i32 as usize];
    let k8: *mut libc::c_double = (*state).k[7 as i32 as usize];
    let k9: *mut libc::c_double = (*state).k[8 as i32 as usize];
    let k10: *mut libc::c_double = (*state).k[9 as i32 as usize];
    let k11: *mut libc::c_double = (*state).k[10 as i32 as usize];
    let k12: *mut libc::c_double = (*state).k[11 as i32 as usize];
    let k13: *mut libc::c_double = (*state).k[12 as i32 as usize];
    memcpy(
        y0 as *mut libc::c_void,
        y as *const libc::c_void,
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    );
    if !dydt_in.is_null() {
        memcpy(
            k1 as *mut libc::c_void,
            dydt_in as *const libc::c_void,
            dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
        );
    } else {
        let mut s: i32 = (Some(((*sys).function).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(t, y as *const libc::c_double, k1, (*sys).params);
        if s != GSL_SUCCESS as i32 {
            return s;
        }
    }
    i = 0 as i32 as size_t;
    while i < dim {
        *ytmp.offset(i as isize) = *y.offset(i as isize)
            + b21 * h * *k1.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    let mut s_0: i32 = (Some(((*sys).function).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        t + ah[0 as i32 as usize] * h,
        ytmp as *const libc::c_double,
        k2,
        (*sys).params,
    );
    if s_0 != GSL_SUCCESS as i32 {
        return s_0;
    }
    i = 0 as i32 as size_t;
    while i < dim {
        *ytmp.offset(i as isize) = *y.offset(i as isize)
            + h
                * (b3[0 as i32 as usize] * *k1.offset(i as isize)
                    + b3[1 as i32 as usize] * *k2.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    let mut s_1: i32 = (Some(((*sys).function).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        t + ah[1 as i32 as usize] * h,
        ytmp as *const libc::c_double,
        k3,
        (*sys).params,
    );
    if s_1 != GSL_SUCCESS as i32 {
        return s_1;
    }
    i = 0 as i32 as size_t;
    while i < dim {
        *ytmp.offset(i as isize) = *y.offset(i as isize)
            + h
                * (b4[0 as i32 as usize] * *k1.offset(i as isize)
                    + b4[2 as i32 as usize] * *k3.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    let mut s_2: i32 = (Some(((*sys).function).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        t + ah[2 as i32 as usize] * h,
        ytmp as *const libc::c_double,
        k4,
        (*sys).params,
    );
    if s_2 != GSL_SUCCESS as i32 {
        return s_2;
    }
    i = 0 as i32 as size_t;
    while i < dim {
        *ytmp.offset(i as isize) = *y.offset(i as isize)
            + h
                * (b5[0 as i32 as usize] * *k1.offset(i as isize)
                    + b5[2 as i32 as usize] * *k3.offset(i as isize)
                    + b5[3 as i32 as usize] * *k4.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    let mut s_3: i32 = (Some(((*sys).function).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        t + ah[3 as i32 as usize] * h,
        ytmp as *const libc::c_double,
        k5,
        (*sys).params,
    );
    if s_3 != GSL_SUCCESS as i32 {
        return s_3;
    }
    i = 0 as i32 as size_t;
    while i < dim {
        *ytmp.offset(i as isize) = *y.offset(i as isize)
            + h
                * (b6[0 as i32 as usize] * *k1.offset(i as isize)
                    + b6[3 as i32 as usize] * *k4.offset(i as isize)
                    + b6[4 as i32 as usize] * *k5.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    let mut s_4: i32 = (Some(((*sys).function).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        t + ah[4 as i32 as usize] * h,
        ytmp as *const libc::c_double,
        k6,
        (*sys).params,
    );
    if s_4 != GSL_SUCCESS as i32 {
        return s_4;
    }
    i = 0 as i32 as size_t;
    while i < dim {
        *ytmp.offset(i as isize) = *y.offset(i as isize)
            + h
                * (b7[0 as i32 as usize] * *k1.offset(i as isize)
                    + b7[3 as i32 as usize] * *k4.offset(i as isize)
                    + b7[4 as i32 as usize] * *k5.offset(i as isize)
                    + b7[5 as i32 as usize] * *k6.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    let mut s_5: i32 = (Some(((*sys).function).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        t + ah[5 as i32 as usize] * h,
        ytmp as *const libc::c_double,
        k7,
        (*sys).params,
    );
    if s_5 != GSL_SUCCESS as i32 {
        return s_5;
    }
    i = 0 as i32 as size_t;
    while i < dim {
        *ytmp.offset(i as isize) = *y.offset(i as isize)
            + h
                * (b8[0 as i32 as usize] * *k1.offset(i as isize)
                    + b8[3 as i32 as usize] * *k4.offset(i as isize)
                    + b8[4 as i32 as usize] * *k5.offset(i as isize)
                    + b8[5 as i32 as usize] * *k6.offset(i as isize)
                    + b8[6 as i32 as usize] * *k7.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    let mut s_6: i32 = (Some(((*sys).function).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        t + ah[6 as i32 as usize] * h,
        ytmp as *const libc::c_double,
        k8,
        (*sys).params,
    );
    if s_6 != GSL_SUCCESS as i32 {
        return s_6;
    }
    i = 0 as i32 as size_t;
    while i < dim {
        *ytmp.offset(i as isize) = *y.offset(i as isize)
            + h
                * (b9[0 as i32 as usize] * *k1.offset(i as isize)
                    + b9[3 as i32 as usize] * *k4.offset(i as isize)
                    + b9[4 as i32 as usize] * *k5.offset(i as isize)
                    + b9[5 as i32 as usize] * *k6.offset(i as isize)
                    + b9[6 as i32 as usize] * *k7.offset(i as isize)
                    + b9[7 as i32 as usize] * *k8.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    let mut s_7: i32 = (Some(((*sys).function).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        t + ah[7 as i32 as usize] * h,
        ytmp as *const libc::c_double,
        k9,
        (*sys).params,
    );
    if s_7 != GSL_SUCCESS as i32 {
        return s_7;
    }
    i = 0 as i32 as size_t;
    while i < dim {
        *ytmp.offset(i as isize) = *y.offset(i as isize)
            + h
                * (b10[0 as i32 as usize] * *k1.offset(i as isize)
                    + b10[3 as i32 as usize] * *k4.offset(i as isize)
                    + b10[4 as i32 as usize] * *k5.offset(i as isize)
                    + b10[5 as i32 as usize] * *k6.offset(i as isize)
                    + b10[6 as i32 as usize] * *k7.offset(i as isize)
                    + b10[7 as i32 as usize] * *k8.offset(i as isize)
                    + b10[8 as i32 as usize] * *k9.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    let mut s_8: i32 = (Some(((*sys).function).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        t + ah[8 as i32 as usize] * h,
        ytmp as *const libc::c_double,
        k10,
        (*sys).params,
    );
    if s_8 != GSL_SUCCESS as i32 {
        return s_8;
    }
    i = 0 as i32 as size_t;
    while i < dim {
        *ytmp.offset(i as isize) = *y.offset(i as isize)
            + h
                * (b11[0 as i32 as usize] * *k1.offset(i as isize)
                    + b11[3 as i32 as usize] * *k4.offset(i as isize)
                    + b11[4 as i32 as usize] * *k5.offset(i as isize)
                    + b11[5 as i32 as usize] * *k6.offset(i as isize)
                    + b11[6 as i32 as usize] * *k7.offset(i as isize)
                    + b11[7 as i32 as usize] * *k8.offset(i as isize)
                    + b11[8 as i32 as usize] * *k9.offset(i as isize)
                    + b11[9 as i32 as usize] * *k10.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    let mut s_9: i32 = (Some(((*sys).function).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        t + ah[9 as i32 as usize] * h,
        ytmp as *const libc::c_double,
        k11,
        (*sys).params,
    );
    if s_9 != GSL_SUCCESS as i32 {
        return s_9;
    }
    i = 0 as i32 as size_t;
    while i < dim {
        *ytmp.offset(i as isize) = *y.offset(i as isize)
            + h
                * (b12[0 as i32 as usize] * *k1.offset(i as isize)
                    + b12[3 as i32 as usize] * *k4.offset(i as isize)
                    + b12[4 as i32 as usize] * *k5.offset(i as isize)
                    + b12[5 as i32 as usize] * *k6.offset(i as isize)
                    + b12[6 as i32 as usize] * *k7.offset(i as isize)
                    + b12[7 as i32 as usize] * *k8.offset(i as isize)
                    + b12[8 as i32 as usize] * *k9.offset(i as isize)
                    + b12[9 as i32 as usize] * *k10.offset(i as isize)
                    + b12[10 as i32 as usize] * *k11.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    let mut s_10: i32 = (Some(((*sys).function).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(t + h, ytmp as *const libc::c_double, k12, (*sys).params);
    if s_10 != GSL_SUCCESS as i32 {
        return s_10;
    }
    i = 0 as i32 as size_t;
    while i < dim {
        *ytmp.offset(i as isize) = *y.offset(i as isize)
            + h
                * (b13[0 as i32 as usize] * *k1.offset(i as isize)
                    + b13[3 as i32 as usize] * *k4.offset(i as isize)
                    + b13[4 as i32 as usize] * *k5.offset(i as isize)
                    + b13[5 as i32 as usize] * *k6.offset(i as isize)
                    + b13[6 as i32 as usize] * *k7.offset(i as isize)
                    + b13[7 as i32 as usize] * *k8.offset(i as isize)
                    + b13[8 as i32 as usize] * *k9.offset(i as isize)
                    + b13[9 as i32 as usize] * *k10.offset(i as isize)
                    + b13[10 as i32 as usize] * *k11.offset(i as isize)
                    + b13[11 as i32 as usize] * *k12.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    let mut s_11: i32 = (Some(((*sys).function).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(t + h, ytmp as *const libc::c_double, k13, (*sys).params);
    if s_11 != GSL_SUCCESS as i32 {
        return s_11;
    }
    i = 0 as i32 as size_t;
    while i < dim {
        let ksum8: libc::c_double = Abar[0 as i32 as usize] * *k1.offset(i as isize)
            + Abar[5 as i32 as usize] * *k6.offset(i as isize)
            + Abar[6 as i32 as usize] * *k7.offset(i as isize)
            + Abar[7 as i32 as usize] * *k8.offset(i as isize)
            + Abar[8 as i32 as usize] * *k9.offset(i as isize)
            + Abar[9 as i32 as usize] * *k10.offset(i as isize)
            + Abar[10 as i32 as usize] * *k11.offset(i as isize)
            + Abar[11 as i32 as usize] * *k12.offset(i as isize)
            + Abar[12 as i32 as usize] * *k13.offset(i as isize);
        *y.offset(i as isize) += h * ksum8;
        i = i.wrapping_add(1);
        i;
    }
    if !dydt_out.is_null() {
        let mut s_12: i32 = (Some(((*sys).function).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(t + h, y as *const libc::c_double, dydt_out, (*sys).params);
        if s_12 != GSL_SUCCESS as i32 {
            memcpy(
                y as *mut libc::c_void,
                y0 as *const libc::c_void,
                dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
            );
            return s_12;
        }
    }
    i = 0 as i32 as size_t;
    while i < dim {
        let ksum8_0: libc::c_double = Abar[0 as i32 as usize] * *k1.offset(i as isize)
            + Abar[5 as i32 as usize] * *k6.offset(i as isize)
            + Abar[6 as i32 as usize] * *k7.offset(i as isize)
            + Abar[7 as i32 as usize] * *k8.offset(i as isize)
            + Abar[8 as i32 as usize] * *k9.offset(i as isize)
            + Abar[9 as i32 as usize] * *k10.offset(i as isize)
            + Abar[10 as i32 as usize] * *k11.offset(i as isize)
            + Abar[11 as i32 as usize] * *k12.offset(i as isize)
            + Abar[12 as i32 as usize] * *k13.offset(i as isize);
        let ksum7: libc::c_double = A[0 as i32 as usize] * *k1.offset(i as isize)
            + A[5 as i32 as usize] * *k6.offset(i as isize)
            + A[6 as i32 as usize] * *k7.offset(i as isize)
            + A[7 as i32 as usize] * *k8.offset(i as isize)
            + A[8 as i32 as usize] * *k9.offset(i as isize)
            + A[9 as i32 as usize] * *k10.offset(i as isize)
            + A[10 as i32 as usize] * *k11.offset(i as isize)
            + A[11 as i32 as usize] * *k12.offset(i as isize);
        *yerr.offset(i as isize) = h * (ksum7 - ksum8_0);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn rk8pd_reset(mut vstate: *mut libc::c_void, mut dim: size_t) -> i32 {
    let mut state: *mut rk8pd_state_t = vstate as *mut rk8pd_state_t;
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 13 as i32 {
        memset(
            (*state).k[i as usize] as *mut libc::c_void,
            0 as i32,
            dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
        );
        i += 1;
        i;
    }
    memset(
        (*state).y0 as *mut libc::c_void,
        0 as i32,
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    );
    memset(
        (*state).ytmp as *mut libc::c_void,
        0 as i32,
        dim.wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    );
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn rk8pd_order(mut vstate: *mut libc::c_void) -> u32 {
    let mut state: *mut rk8pd_state_t = vstate as *mut rk8pd_state_t;
    state = 0 as *mut rk8pd_state_t;
    return 8 as i32 as u32;
}
unsafe extern "C" fn rk8pd_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut rk8pd_state_t = vstate as *mut rk8pd_state_t;
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 13 as i32 {
        free((*state).k[i as usize] as *mut libc::c_void);
        i += 1;
        i;
    }
    free((*state).y0 as *mut libc::c_void);
    free((*state).ytmp as *mut libc::c_void);
    free(state as *mut libc::c_void);
}
static mut rk8pd_type: gsl_odeiv_step_type = {
    let mut init = gsl_odeiv_step_type {
        name: b"rk8pd\0" as *const u8 as *const i8,
        can_use_dydt_in: 1 as i32,
        gives_exact_dydt_out: 1 as i32,
        alloc: Some(rk8pd_alloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        apply: Some(
            rk8pd_apply
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    size_t,
                    libc::c_double,
                    libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *const libc::c_double,
                    *mut libc::c_double,
                    *const gsl_odeiv_system,
                ) -> i32,
        ),
        reset: Some(
            rk8pd_reset as unsafe extern "C" fn(*mut libc::c_void, size_t) -> i32,
        ),
        order: Some(rk8pd_order as unsafe extern "C" fn(*mut libc::c_void) -> u32),
        free: Some(rk8pd_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
#[no_mangle]
pub static mut gsl_odeiv_step_rk8pd: *const gsl_odeiv_step_type = unsafe {
    &rk8pd_type as *const gsl_odeiv_step_type
};