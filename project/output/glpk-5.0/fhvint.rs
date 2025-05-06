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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn _glp_luf_f_solve(luf: *mut LUF, x: *mut libc::c_double);
    fn _glp_luf_ft_solve(luf: *mut LUF, x: *mut libc::c_double);
    fn _glp_luf_v_solve(luf: *mut LUF, b: *mut libc::c_double, x: *mut libc::c_double);
    fn _glp_luf_vt_solve(luf: *mut LUF, b: *mut libc::c_double, x: *mut libc::c_double);
    fn _glp_luf_estimate_norm(
        luf: *mut LUF,
        w1: *mut libc::c_double,
        w2: *mut libc::c_double,
    ) -> libc::c_double;
    fn _glp_fhv_ft_update(
        fhv: *mut FHV,
        q: i32,
        aq_len: i32,
        aq_ind: *const i32,
        aq_val: *const libc::c_double,
        ind: *mut i32,
        val: *mut libc::c_double,
        work: *mut libc::c_double,
    ) -> i32;
    fn _glp_fhv_ht_solve(fhv: *mut FHV, x: *mut libc::c_double);
    fn _glp_sva_alloc_vecs(sva: *mut SVA, nnn: i32) -> i32;
    fn _glp_fhv_h_solve(fhv: *mut FHV, x: *mut libc::c_double);
    fn _glp_lufint_create() -> *mut LUFINT;
    fn _glp_lufint_factorize(
        fi: *mut LUFINT,
        n: i32,
        col: Option<
            unsafe extern "C" fn(
                *mut libc::c_void,
                i32,
                *mut i32,
                *mut libc::c_double,
            ) -> i32,
        >,
        info: *mut libc::c_void,
    ) -> i32;
    fn _glp_lufint_delete(fi: *mut LUFINT);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SVA {
    pub n_max: i32,
    pub n: i32,
    pub ptr: *mut i32,
    pub len: *mut i32,
    pub cap: *mut i32,
    pub size: i32,
    pub m_ptr: i32,
    pub r_ptr: i32,
    pub head: i32,
    pub tail: i32,
    pub prev: *mut i32,
    pub next: *mut i32,
    pub ind: *mut i32,
    pub val: *mut libc::c_double,
    pub talky: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LUF {
    pub n: i32,
    pub sva: *mut SVA,
    pub fr_ref: i32,
    pub fc_ref: i32,
    pub vr_ref: i32,
    pub vr_piv: *mut libc::c_double,
    pub vc_ref: i32,
    pub pp_ind: *mut i32,
    pub pp_inv: *mut i32,
    pub qq_ind: *mut i32,
    pub qq_inv: *mut i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FHV {
    pub luf: *mut LUF,
    pub nfs_max: i32,
    pub nfs: i32,
    pub hh_ind: *mut i32,
    pub hh_ref: i32,
    pub p0_ind: *mut i32,
    pub p0_inv: *mut i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SGF {
    pub luf: *mut LUF,
    pub rs_head: *mut i32,
    pub rs_prev: *mut i32,
    pub rs_next: *mut i32,
    pub cs_head: *mut i32,
    pub cs_prev: *mut i32,
    pub cs_next: *mut i32,
    pub vr_max: *mut libc::c_double,
    pub flag: *mut i8,
    pub work: *mut libc::c_double,
    pub updat: i32,
    pub piv_tol: libc::c_double,
    pub piv_lim: i32,
    pub suhl: i32,
    pub eps_tol: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LUFINT {
    pub n_max: i32,
    pub valid: i32,
    pub sva: *mut SVA,
    pub luf: *mut LUF,
    pub sgf: *mut SGF,
    pub sva_n_max: i32,
    pub sva_size: i32,
    pub delta_n0: i32,
    pub delta_n: i32,
    pub sgf_updat: i32,
    pub sgf_piv_tol: libc::c_double,
    pub sgf_piv_lim: i32,
    pub sgf_suhl: i32,
    pub sgf_eps_tol: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FHVINT {
    pub valid: i32,
    pub fhv: FHV,
    pub lufi: *mut LUFINT,
    pub nfs_max: i32,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_fhvint_create() -> *mut FHVINT {
    let mut fi: *mut FHVINT = 0 as *mut FHVINT;
    fi = glp_alloc(1 as i32, ::core::mem::size_of::<FHVINT>() as u64 as i32)
        as *mut FHVINT;
    memset(fi as *mut libc::c_void, 0 as i32, ::core::mem::size_of::<FHVINT>() as u64);
    (*fi).lufi = _glp_lufint_create();
    return fi;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_fhvint_factorize(
    mut fi: *mut FHVINT,
    mut n: i32,
    mut col: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            i32,
            *mut i32,
            *mut libc::c_double,
        ) -> i32,
    >,
    mut info: *mut libc::c_void,
) -> i32 {
    let mut nfs_max: i32 = 0;
    let mut old_n_max: i32 = 0;
    let mut n_max: i32 = 0;
    let mut k: i32 = 0;
    let mut ret: i32 = 0;
    (n > 0 as i32
        || {
            glp_assert_(
                b"n > 0\0" as *const u8 as *const i8,
                b"bflib/fhvint.c\0" as *const u8 as *const i8,
                38 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*fi).valid = 0 as i32;
    nfs_max = (*fi).nfs_max;
    if nfs_max == 0 as i32 {
        nfs_max = 100 as i32;
    }
    (nfs_max > 0 as i32
        || {
            glp_assert_(
                b"nfs_max > 0\0" as *const u8 as *const i8,
                b"bflib/fhvint.c\0" as *const u8 as *const i8,
                44 as i32,
            );
            1 as i32 != 0
        }) as i32;
    old_n_max = (*(*fi).lufi).n_max;
    (*(*fi).lufi).sva_n_max = 4 as i32 * n + nfs_max;
    (*(*fi).lufi).sgf_updat = 1 as i32;
    ret = _glp_lufint_factorize((*fi).lufi, n, col, info);
    n_max = (*(*fi).lufi).n_max;
    if (*fi).fhv.nfs_max != nfs_max {
        if !((*fi).fhv.hh_ind).is_null() {
            glp_free((*fi).fhv.hh_ind as *mut libc::c_void);
        }
        (*fi).fhv.hh_ind = glp_alloc(
            1 as i32 + nfs_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
    }
    if old_n_max < n_max {
        if !((*fi).fhv.p0_ind).is_null() {
            glp_free((*fi).fhv.p0_ind as *mut libc::c_void);
        }
        if !((*fi).fhv.p0_inv).is_null() {
            glp_free((*fi).fhv.p0_inv as *mut libc::c_void);
        }
        (*fi).fhv.p0_ind = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*fi).fhv.p0_inv = glp_alloc(
            1 as i32 + n_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
    }
    (*fi).fhv.luf = (*(*fi).lufi).luf;
    (*fi).fhv.nfs_max = nfs_max;
    (*fi).fhv.nfs = 0 as i32;
    (*fi).fhv.hh_ref = _glp_sva_alloc_vecs((*(*fi).lufi).sva, nfs_max);
    k = 1 as i32;
    while k <= n {
        *((*fi).fhv.p0_ind).offset(k as isize) = *((*(*fi).fhv.luf).pp_ind)
            .offset(k as isize);
        *((*fi).fhv.p0_inv).offset(k as isize) = *((*(*fi).fhv.luf).pp_inv)
            .offset(k as isize);
        k += 1;
        k;
    }
    if ret == 0 as i32 {
        (*fi).valid = 1 as i32;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_fhvint_update(
    mut fi: *mut FHVINT,
    mut j: i32,
    mut len: i32,
    mut ind: *const i32,
    mut val: *const libc::c_double,
) -> i32 {
    let mut sgf: *mut SGF = (*(*fi).lufi).sgf;
    let mut ind1: *mut i32 = (*sgf).rs_next;
    let mut val1: *mut libc::c_double = (*sgf).vr_max;
    let mut work: *mut libc::c_double = (*sgf).work;
    let mut ret: i32 = 0;
    ((*fi).valid != 0
        || {
            glp_assert_(
                b"fi->valid\0" as *const u8 as *const i8,
                b"bflib/fhvint.c\0" as *const u8 as *const i8,
                90 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ret = _glp_fhv_ft_update(&mut (*fi).fhv, j, len, ind, val, ind1, val1, work);
    if ret != 0 as i32 {
        (*fi).valid = 0 as i32;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_fhvint_ftran(
    mut fi: *mut FHVINT,
    mut x: *mut libc::c_double,
) {
    let mut fhv: *mut FHV = &mut (*fi).fhv;
    let mut luf: *mut LUF = (*fhv).luf;
    let mut n: i32 = (*luf).n;
    let mut pp_ind: *mut i32 = (*luf).pp_ind;
    let mut pp_inv: *mut i32 = (*luf).pp_inv;
    let mut sgf: *mut SGF = (*(*fi).lufi).sgf;
    let mut work: *mut libc::c_double = (*sgf).work;
    ((*fi).valid != 0
        || {
            glp_assert_(
                b"fi->valid\0" as *const u8 as *const i8,
                b"bflib/fhvint.c\0" as *const u8 as *const i8,
                106 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*luf).pp_ind = (*fhv).p0_ind;
    (*luf).pp_inv = (*fhv).p0_inv;
    _glp_luf_f_solve(luf, x);
    (*luf).pp_ind = pp_ind;
    (*luf).pp_inv = pp_inv;
    _glp_fhv_h_solve(fhv, x);
    _glp_luf_v_solve(luf, x, work);
    memcpy(
        &mut *x.offset(1 as i32 as isize) as *mut libc::c_double as *mut libc::c_void,
        &mut *work.offset(1 as i32 as isize) as *mut libc::c_double
            as *const libc::c_void,
        (n as u64).wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    );
}
#[no_mangle]
pub unsafe extern "C" fn _glp_fhvint_btran(
    mut fi: *mut FHVINT,
    mut x: *mut libc::c_double,
) {
    let mut fhv: *mut FHV = &mut (*fi).fhv;
    let mut luf: *mut LUF = (*fhv).luf;
    let mut n: i32 = (*luf).n;
    let mut pp_ind: *mut i32 = (*luf).pp_ind;
    let mut pp_inv: *mut i32 = (*luf).pp_inv;
    let mut sgf: *mut SGF = (*(*fi).lufi).sgf;
    let mut work: *mut libc::c_double = (*sgf).work;
    ((*fi).valid != 0
        || {
            glp_assert_(
                b"fi->valid\0" as *const u8 as *const i8,
                b"bflib/fhvint.c\0" as *const u8 as *const i8,
                129 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_luf_vt_solve(luf, x, work);
    _glp_fhv_ht_solve(fhv, work);
    (*luf).pp_ind = (*fhv).p0_ind;
    (*luf).pp_inv = (*fhv).p0_inv;
    _glp_luf_ft_solve(luf, work);
    (*luf).pp_ind = pp_ind;
    (*luf).pp_inv = pp_inv;
    memcpy(
        &mut *x.offset(1 as i32 as isize) as *mut libc::c_double as *mut libc::c_void,
        &mut *work.offset(1 as i32 as isize) as *mut libc::c_double
            as *const libc::c_void,
        (n as u64).wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    );
}
#[no_mangle]
pub unsafe extern "C" fn _glp_fhvint_estimate(mut fi: *mut FHVINT) -> libc::c_double {
    let mut norm: libc::c_double = 0.;
    ((*fi).valid != 0
        || {
            glp_assert_(
                b"fi->valid\0" as *const u8 as *const i8,
                b"bflib/fhvint.c\0" as *const u8 as *const i8,
                146 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*fi).fhv.nfs == 0 as i32
        || {
            glp_assert_(
                b"fi->fhv.nfs == 0\0" as *const u8 as *const i8,
                b"bflib/fhvint.c\0" as *const u8 as *const i8,
                147 as i32,
            );
            1 as i32 != 0
        }) as i32;
    norm = _glp_luf_estimate_norm(
        (*fi).fhv.luf,
        (*(*(*fi).lufi).sgf).vr_max,
        (*(*(*fi).lufi).sgf).work,
    );
    return norm;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_fhvint_delete(mut fi: *mut FHVINT) {
    _glp_lufint_delete((*fi).lufi);
    if !((*fi).fhv.hh_ind).is_null() {
        glp_free((*fi).fhv.hh_ind as *mut libc::c_void);
    }
    if !((*fi).fhv.p0_ind).is_null() {
        glp_free((*fi).fhv.p0_ind as *mut libc::c_void);
    }
    if !((*fi).fhv.p0_inv).is_null() {
        glp_free((*fi).fhv.p0_inv as *mut libc::c_void);
    }
    glp_free(fi as *mut libc::c_void);
}