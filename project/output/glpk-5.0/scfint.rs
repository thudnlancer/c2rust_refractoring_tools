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
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn _glp_sva_alloc_vecs(sva: *mut SVA, nnn: i32) -> i32;
    fn _glp_btf_estimate_norm(
        btf: *mut BTF,
        w1: *mut libc::c_double,
        w2: *mut libc::c_double,
        w3: *mut libc::c_double,
        w4: *mut libc::c_double,
    ) -> libc::c_double;
    fn _glp_luf_estimate_norm(
        luf: *mut LUF,
        w1: *mut libc::c_double,
        w2: *mut libc::c_double,
    ) -> libc::c_double;
    fn _glp_scf_a_solve(
        scf: *mut SCF,
        x: *mut libc::c_double,
        w: *mut libc::c_double,
        work1: *mut libc::c_double,
        work2: *mut libc::c_double,
        work3: *mut libc::c_double,
    );
    fn _glp_scf_at_solve(
        scf: *mut SCF,
        x: *mut libc::c_double,
        w: *mut libc::c_double,
        work1: *mut libc::c_double,
        work2: *mut libc::c_double,
        work3: *mut libc::c_double,
    );
    fn _glp_scf_update_aug(
        scf: *mut SCF,
        b: *mut libc::c_double,
        d: *mut libc::c_double,
        f: *mut libc::c_double,
        g: *mut libc::c_double,
        h: libc::c_double,
        upd: i32,
        w1: *mut libc::c_double,
        w2: *mut libc::c_double,
        w3: *mut libc::c_double,
    ) -> i32;
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
    fn _glp_btfint_create() -> *mut BTFINT;
    fn _glp_btfint_factorize(
        fi: *mut BTFINT,
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
    fn _glp_btfint_delete(fi: *mut BTFINT);
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
pub struct BTF {
    pub n: i32,
    pub sva: *mut SVA,
    pub pp_ind: *mut i32,
    pub pp_inv: *mut i32,
    pub qq_ind: *mut i32,
    pub qq_inv: *mut i32,
    pub num: i32,
    pub beg: *mut i32,
    pub ar_ref: i32,
    pub ac_ref: i32,
    pub fr_ref: i32,
    pub fc_ref: i32,
    pub vr_ref: i32,
    pub vr_piv: *mut libc::c_double,
    pub vc_ref: i32,
    pub p1_ind: *mut i32,
    pub p1_inv: *mut i32,
    pub q1_ind: *mut i32,
    pub q1_inv: *mut i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IFU {
    pub n_max: i32,
    pub n: i32,
    pub f: *mut libc::c_double,
    pub u: *mut libc::c_double,
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
pub struct SCF {
    pub n: i32,
    pub n0: i32,
    pub type_0: i32,
    pub a0: C2RustUnnamed,
    pub nn_max: i32,
    pub nn: i32,
    pub sva: *mut SVA,
    pub rr_ref: i32,
    pub ss_ref: i32,
    pub ifu: IFU,
    pub pp_ind: *mut i32,
    pub pp_inv: *mut i32,
    pub qq_ind: *mut i32,
    pub qq_inv: *mut i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub luf: *mut LUF,
    pub btf: *mut BTF,
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
pub struct BTFINT {
    pub n_max: i32,
    pub valid: i32,
    pub sva: *mut SVA,
    pub btf: *mut BTF,
    pub sgf: *mut SGF,
    pub sva_n_max: i32,
    pub sva_size: i32,
    pub delta_n0: i32,
    pub delta_n: i32,
    pub sgf_piv_tol: libc::c_double,
    pub sgf_piv_lim: i32,
    pub sgf_suhl: i32,
    pub sgf_eps_tol: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCFINT {
    pub valid: i32,
    pub scf: SCF,
    pub u: C2RustUnnamed_0,
    pub w1: *mut libc::c_double,
    pub w2: *mut libc::c_double,
    pub w3: *mut libc::c_double,
    pub w4: *mut libc::c_double,
    pub w5: *mut libc::c_double,
    pub nn_max: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub lufi: *mut LUFINT,
    pub btfi: *mut BTFINT,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_scfint_create(mut type_0: i32) -> *mut SCFINT {
    let mut fi: *mut SCFINT = 0 as *mut SCFINT;
    fi = glp_alloc(1 as i32, ::core::mem::size_of::<SCFINT>() as u64 as i32)
        as *mut SCFINT;
    memset(fi as *mut libc::c_void, 0 as i32, ::core::mem::size_of::<SCFINT>() as u64);
    (*fi).scf.type_0 = type_0;
    match (*fi).scf.type_0 {
        1 => {
            (*fi).u.lufi = _glp_lufint_create();
        }
        2 => {
            (*fi).u.btfi = _glp_btfint_create();
        }
        _ => {
            (type_0 != type_0
                || {
                    glp_assert_(
                        b"type != type\0" as *const u8 as *const i8,
                        b"bflib/scfint.c\0" as *const u8 as *const i8,
                        38 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    return fi;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_scfint_factorize(
    mut fi: *mut SCFINT,
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
    let mut nn_max: i32 = 0;
    let mut old_n0_max: i32 = 0;
    let mut n0_max: i32 = 0;
    let mut k: i32 = 0;
    let mut ret: i32 = 0;
    (n > 0 as i32
        || {
            glp_assert_(
                b"n > 0\0" as *const u8 as *const i8,
                b"bflib/scfint.c\0" as *const u8 as *const i8,
                47 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*fi).valid = 0 as i32;
    nn_max = (*fi).nn_max;
    if nn_max == 0 as i32 {
        nn_max = 100 as i32;
    }
    (nn_max > 0 as i32
        || {
            glp_assert_(
                b"nn_max > 0\0" as *const u8 as *const i8,
                b"bflib/scfint.c\0" as *const u8 as *const i8,
                53 as i32,
            );
            1 as i32 != 0
        }) as i32;
    match (*fi).scf.type_0 {
        1 => {
            old_n0_max = (*(*fi).u.lufi).n_max;
            (*(*fi).u.lufi).sva_n_max = 4 as i32 * n + 2 as i32 * nn_max;
            ret = _glp_lufint_factorize((*fi).u.lufi, n, col, info);
            n0_max = (*(*fi).u.lufi).n_max;
            (*fi).scf.sva = (*(*fi).u.lufi).sva;
            (*fi).scf.a0.luf = (*(*fi).u.lufi).luf;
        }
        2 => {
            old_n0_max = (*(*fi).u.btfi).n_max;
            (*(*fi).u.btfi).sva_n_max = 6 as i32 * n + 2 as i32 * nn_max;
            ret = _glp_btfint_factorize((*fi).u.btfi, n, col, info);
            n0_max = (*(*fi).u.btfi).n_max;
            (*fi).scf.sva = (*(*fi).u.btfi).sva;
            (*fi).scf.a0.btf = (*(*fi).u.btfi).btf;
        }
        _ => {
            (fi != fi
                || {
                    glp_assert_(
                        b"fi != fi\0" as *const u8 as *const i8,
                        b"bflib/scfint.c\0" as *const u8 as *const i8,
                        73 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    if old_n0_max < n0_max {
        if !((*fi).w1).is_null() {
            glp_free((*fi).w1 as *mut libc::c_void);
        }
        if !((*fi).w2).is_null() {
            glp_free((*fi).w2 as *mut libc::c_void);
        }
        if !((*fi).w3).is_null() {
            glp_free((*fi).w3 as *mut libc::c_void);
        }
        (*fi).w1 = glp_alloc(
            1 as i32 + n0_max,
            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
        ) as *mut libc::c_double;
        (*fi).w2 = glp_alloc(
            1 as i32 + n0_max,
            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
        ) as *mut libc::c_double;
        (*fi).w3 = glp_alloc(
            1 as i32 + n0_max,
            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
        ) as *mut libc::c_double;
    }
    if (*fi).scf.nn_max != nn_max {
        if !((*fi).scf.ifu.f).is_null() {
            glp_free((*fi).scf.ifu.f as *mut libc::c_void);
        }
        if !((*fi).scf.ifu.u).is_null() {
            glp_free((*fi).scf.ifu.u as *mut libc::c_void);
        }
        (*fi).scf.ifu.f = glp_alloc(
            nn_max * nn_max,
            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
        ) as *mut libc::c_double;
        (*fi).scf.ifu.u = glp_alloc(
            nn_max * nn_max,
            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
        ) as *mut libc::c_double;
    }
    if old_n0_max < n0_max || (*fi).scf.nn_max != nn_max {
        if !((*fi).scf.pp_ind).is_null() {
            glp_free((*fi).scf.pp_ind as *mut libc::c_void);
        }
        if !((*fi).scf.pp_inv).is_null() {
            glp_free((*fi).scf.pp_inv as *mut libc::c_void);
        }
        if !((*fi).scf.qq_ind).is_null() {
            glp_free((*fi).scf.qq_ind as *mut libc::c_void);
        }
        if !((*fi).scf.qq_inv).is_null() {
            glp_free((*fi).scf.qq_inv as *mut libc::c_void);
        }
        if !((*fi).w4).is_null() {
            glp_free((*fi).w4 as *mut libc::c_void);
        }
        if !((*fi).w5).is_null() {
            glp_free((*fi).w5 as *mut libc::c_void);
        }
        (*fi).scf.pp_ind = glp_alloc(
            1 as i32 + n0_max + nn_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*fi).scf.pp_inv = glp_alloc(
            1 as i32 + n0_max + nn_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*fi).scf.qq_ind = glp_alloc(
            1 as i32 + n0_max + nn_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*fi).scf.qq_inv = glp_alloc(
            1 as i32 + n0_max + nn_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        (*fi).w4 = glp_alloc(
            1 as i32 + n0_max + nn_max,
            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
        ) as *mut libc::c_double;
        (*fi).w5 = glp_alloc(
            1 as i32 + n0_max + nn_max,
            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
        ) as *mut libc::c_double;
    }
    (*fi).scf.n = n;
    (*fi).scf.n0 = n;
    (*fi).scf.nn_max = nn_max;
    (*fi).scf.nn = 0 as i32;
    (*fi).scf.rr_ref = _glp_sva_alloc_vecs((*fi).scf.sva, nn_max);
    (*fi).scf.ss_ref = _glp_sva_alloc_vecs((*fi).scf.sva, nn_max);
    (*fi).scf.ifu.n_max = nn_max;
    (*fi).scf.ifu.n = 0 as i32;
    k = 1 as i32;
    while k <= n {
        *((*fi).scf.pp_ind).offset(k as isize) = k;
        *((*fi).scf.pp_inv).offset(k as isize) = k;
        *((*fi).scf.qq_ind).offset(k as isize) = k;
        *((*fi).scf.qq_inv).offset(k as isize) = k;
        k += 1;
        k;
    }
    if ret == 0 as i32 {
        (*fi).valid = 1 as i32;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_scfint_update(
    mut fi: *mut SCFINT,
    mut upd: i32,
    mut j: i32,
    mut len: i32,
    mut ind: *const i32,
    mut val: *const libc::c_double,
) -> i32 {
    let mut n: i32 = (*fi).scf.n;
    let mut n0: i32 = (*fi).scf.n0;
    let mut nn: i32 = (*fi).scf.nn;
    let mut pp_ind: *mut i32 = (*fi).scf.pp_ind;
    let mut qq_ind: *mut i32 = (*fi).scf.qq_ind;
    let mut qq_inv: *mut i32 = (*fi).scf.qq_inv;
    let mut bf: *mut libc::c_double = (*fi).w4;
    let mut dg: *mut libc::c_double = (*fi).w5;
    let mut k: i32 = 0;
    let mut t: i32 = 0;
    let mut ret: i32 = 0;
    ((*fi).valid != 0
        || {
            glp_assert_(
                b"fi->valid\0" as *const u8 as *const i8,
                b"bflib/scfint.c\0" as *const u8 as *const i8,
                148 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (0 as i32 <= n && n <= n0 + nn
        || {
            glp_assert_(
                b"0 <= n && n <= n0+nn\0" as *const u8 as *const i8,
                b"bflib/scfint.c\0" as *const u8 as *const i8,
                149 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = 1 as i32;
    while k <= n0 + nn {
        *bf.offset(k as isize) = 0.0f64;
        k += 1;
        k;
    }
    t = 1 as i32;
    while t <= len {
        k = *ind.offset(t as isize);
        (1 as i32 <= k && k <= n
            || {
                glp_assert_(
                    b"1 <= k && k <= n\0" as *const u8 as *const i8,
                    b"bflib/scfint.c\0" as *const u8 as *const i8,
                    155 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*pp_ind.offset(k as isize) == k
            || {
                glp_assert_(
                    b"pp_ind[k] == k\0" as *const u8 as *const i8,
                    b"bflib/scfint.c\0" as *const u8 as *const i8,
                    157 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*bf.offset(k as isize) == 0.0f64
            || {
                glp_assert_(
                    b"bf[k] == 0.0\0" as *const u8 as *const i8,
                    b"bflib/scfint.c\0" as *const u8 as *const i8,
                    159 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*val.offset(t as isize) != 0.0f64
            || {
                glp_assert_(
                    b"val[t] != 0.0\0" as *const u8 as *const i8,
                    b"bflib/scfint.c\0" as *const u8 as *const i8,
                    160 as i32,
                );
                1 as i32 != 0
            }) as i32;
        *bf.offset(k as isize) = *val.offset(t as isize);
        t += 1;
        t;
    }
    k = 1 as i32;
    while k <= n0 + nn {
        *dg.offset(k as isize) = 0.0f64;
        k += 1;
        k;
    }
    (1 as i32 <= j && j <= n
        || {
            glp_assert_(
                b"1 <= j && j <= n\0" as *const u8 as *const i8,
                b"bflib/scfint.c\0" as *const u8 as *const i8,
                166 as i32,
            );
            1 as i32 != 0
        }) as i32;
    *dg.offset(*((*fi).scf.qq_inv).offset(j as isize) as isize) = 1 as i32
        as libc::c_double;
    ret = _glp_scf_update_aug(
        &mut (*fi).scf,
        &mut *bf.offset(0 as i32 as isize),
        &mut *dg.offset(0 as i32 as isize),
        &mut *bf.offset(n0 as isize),
        &mut *dg.offset(n0 as isize),
        0.0f64,
        upd,
        (*fi).w1,
        (*fi).w2,
        (*fi).w3,
    );
    if ret == 0 as i32 {
        let mut i1: i32 = 0;
        let mut i2: i32 = 0;
        i1 = *qq_inv.offset(j as isize);
        i2 = *qq_inv.offset((n0 + nn + 1 as i32) as isize);
        *qq_ind.offset(i1 as isize) = n0 + nn + 1 as i32;
        *qq_inv.offset((n0 + nn + 1 as i32) as isize) = i1;
        *qq_ind.offset(i2 as isize) = j;
        *qq_inv.offset(j as isize) = i2;
    } else {
        (*fi).valid = 0 as i32;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_scfint_ftran(
    mut fi: *mut SCFINT,
    mut x: *mut libc::c_double,
) {
    ((*fi).valid != 0
        || {
            glp_assert_(
                b"fi->valid\0" as *const u8 as *const i8,
                b"bflib/scfint.c\0" as *const u8 as *const i8,
                184 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_scf_a_solve(&mut (*fi).scf, x, (*fi).w4, (*fi).w5, (*fi).w1, (*fi).w2);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_scfint_btran(
    mut fi: *mut SCFINT,
    mut x: *mut libc::c_double,
) {
    ((*fi).valid != 0
        || {
            glp_assert_(
                b"fi->valid\0" as *const u8 as *const i8,
                b"bflib/scfint.c\0" as *const u8 as *const i8,
                191 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_scf_at_solve(&mut (*fi).scf, x, (*fi).w4, (*fi).w5, (*fi).w1, (*fi).w2);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_scfint_estimate(mut fi: *mut SCFINT) -> libc::c_double {
    let mut norm: libc::c_double = 0.;
    ((*fi).valid != 0
        || {
            glp_assert_(
                b"fi->valid\0" as *const u8 as *const i8,
                b"bflib/scfint.c\0" as *const u8 as *const i8,
                199 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*fi).scf.n == (*fi).scf.n0
        || {
            glp_assert_(
                b"fi->scf.n == fi->scf.n0\0" as *const u8 as *const i8,
                b"bflib/scfint.c\0" as *const u8 as *const i8,
                200 as i32,
            );
            1 as i32 != 0
        }) as i32;
    match (*fi).scf.type_0 {
        1 => {
            norm = _glp_luf_estimate_norm((*fi).scf.a0.luf, (*fi).w1, (*fi).w2);
        }
        2 => {
            norm = _glp_btf_estimate_norm(
                (*fi).scf.a0.btf,
                (*fi).w1,
                (*fi).w2,
                (*fi).w3,
                (*fi).w4,
            );
        }
        _ => {
            (fi != fi
                || {
                    glp_assert_(
                        b"fi != fi\0" as *const u8 as *const i8,
                        b"bflib/scfint.c\0" as *const u8 as *const i8,
                        210 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    return norm;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_scfint_delete(mut fi: *mut SCFINT) {
    match (*fi).scf.type_0 {
        1 => {
            _glp_lufint_delete((*fi).u.lufi);
        }
        2 => {
            _glp_btfint_delete((*fi).u.btfi);
        }
        _ => {
            (fi != fi
                || {
                    glp_assert_(
                        b"fi != fi\0" as *const u8 as *const i8,
                        b"bflib/scfint.c\0" as *const u8 as *const i8,
                        225 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    if !((*fi).scf.ifu.f).is_null() {
        glp_free((*fi).scf.ifu.f as *mut libc::c_void);
    }
    if !((*fi).scf.ifu.u).is_null() {
        glp_free((*fi).scf.ifu.u as *mut libc::c_void);
    }
    if !((*fi).scf.pp_ind).is_null() {
        glp_free((*fi).scf.pp_ind as *mut libc::c_void);
    }
    if !((*fi).scf.pp_inv).is_null() {
        glp_free((*fi).scf.pp_inv as *mut libc::c_void);
    }
    if !((*fi).scf.qq_ind).is_null() {
        glp_free((*fi).scf.qq_ind as *mut libc::c_void);
    }
    if !((*fi).scf.qq_inv).is_null() {
        glp_free((*fi).scf.qq_inv as *mut libc::c_void);
    }
    if !((*fi).w1).is_null() {
        glp_free((*fi).w1 as *mut libc::c_void);
    }
    if !((*fi).w2).is_null() {
        glp_free((*fi).w2 as *mut libc::c_void);
    }
    if !((*fi).w3).is_null() {
        glp_free((*fi).w3 as *mut libc::c_void);
    }
    if !((*fi).w4).is_null() {
        glp_free((*fi).w4 as *mut libc::c_void);
    }
    if !((*fi).w5).is_null() {
        glp_free((*fi).w5 as *mut libc::c_void);
    }
    glp_free(fi as *mut libc::c_void);
}