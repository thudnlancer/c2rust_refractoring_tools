#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type DMP;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn _glp_lux_solve(lux: *mut LUX, tr: i32, x: *mut mpq_t);
    fn _glp_lux_decomp(
        lux: *mut LUX,
        col: Option<
            unsafe extern "C" fn(*mut libc::c_void, i32, *mut i32, *mut mpq_t) -> i32,
        >,
        info: *mut libc::c_void,
    ) -> i32;
    fn _glp_lux_create(n: i32) -> *mut LUX;
    fn _glp_lux_delete(lux: *mut LUX);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpz {
    pub val: i32,
    pub ptr: *mut mpz_seg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpz_seg {
    pub d: [libc::c_ushort; 6],
    pub next: *mut mpz_seg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpq {
    pub p: mpz,
    pub q: mpz,
}
pub type mpq_t = *mut mpq;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BFX {
    pub valid: i32,
    pub lux: *mut LUX,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LUX {
    pub n: i32,
    pub pool: *mut DMP,
    pub F_row: *mut *mut LUXELM,
    pub F_col: *mut *mut LUXELM,
    pub V_piv: *mut mpq_t,
    pub V_row: *mut *mut LUXELM,
    pub V_col: *mut *mut LUXELM,
    pub P_row: *mut i32,
    pub P_col: *mut i32,
    pub Q_row: *mut i32,
    pub Q_col: *mut i32,
    pub rank: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LUXELM {
    pub i: i32,
    pub j: i32,
    pub val: mpq_t,
    pub r_prev: *mut LUXELM,
    pub r_next: *mut LUXELM,
    pub c_prev: *mut LUXELM,
    pub c_next: *mut LUXELM,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_bfx_create_binv() -> *mut BFX {
    let mut bfx: *mut BFX = 0 as *mut BFX;
    bfx = glp_alloc(1 as i32, ::core::mem::size_of::<BFX>() as u64 as i32) as *mut BFX;
    (*bfx).valid = 0 as i32;
    (*bfx).lux = 0 as *mut LUX;
    return bfx;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_bfx_factorize(
    mut binv: *mut BFX,
    mut m: i32,
    mut col: Option<
        unsafe extern "C" fn(*mut libc::c_void, i32, *mut i32, *mut mpq_t) -> i32,
    >,
    mut info: *mut libc::c_void,
) -> i32 {
    let mut ret: i32 = 0;
    (m > 0 as i32
        || {
            glp_assert_(
                b"m > 0\0" as *const u8 as *const i8,
                b"draft/bfx.c\0" as *const u8 as *const i8,
                44 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if !((*binv).lux).is_null() && (*(*binv).lux).n != m {
        _glp_lux_delete((*binv).lux);
        (*binv).lux = 0 as *mut LUX;
    }
    if ((*binv).lux).is_null() {
        (*binv).lux = _glp_lux_create(m);
    }
    ret = _glp_lux_decomp((*binv).lux, col, info);
    (*binv).valid = (ret == 0 as i32) as i32;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_bfx_ftran(
    mut binv: *mut BFX,
    mut x: *mut mpq_t,
    mut save: i32,
) {
    ((*binv).valid != 0
        || {
            glp_assert_(
                b"binv->valid\0" as *const u8 as *const i8,
                b"draft/bfx.c\0" as *const u8 as *const i8,
                58 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_lux_solve((*binv).lux, 0 as i32, x);
    (save == save
        || {
            glp_assert_(
                b"save == save\0" as *const u8 as *const i8,
                b"draft/bfx.c\0" as *const u8 as *const i8,
                60 as i32,
            );
            1 as i32 != 0
        }) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_bfx_btran(mut binv: *mut BFX, mut x: *mut mpq_t) {
    ((*binv).valid != 0
        || {
            glp_assert_(
                b"binv->valid\0" as *const u8 as *const i8,
                b"draft/bfx.c\0" as *const u8 as *const i8,
                66 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_lux_solve((*binv).lux, 1 as i32, x);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_bfx_update(mut binv: *mut BFX, mut j: i32) -> i32 {
    ((*binv).valid != 0
        || {
            glp_assert_(
                b"binv->valid\0" as *const u8 as *const i8,
                b"draft/bfx.c\0" as *const u8 as *const i8,
                73 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= j && j <= (*(*binv).lux).n
        || {
            glp_assert_(
                b"1 <= j && j <= binv->lux->n\0" as *const u8 as *const i8,
                b"draft/bfx.c\0" as *const u8 as *const i8,
                74 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_bfx_delete_binv(mut binv: *mut BFX) {
    if !((*binv).lux).is_null() {
        _glp_lux_delete((*binv).lux);
    }
    glp_free(binv as *mut libc::c_void);
}