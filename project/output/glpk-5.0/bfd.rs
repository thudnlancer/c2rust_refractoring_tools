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
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn _glp_fhvint_estimate(fi: *mut FHVINT) -> libc::c_double;
    fn _glp_fhvint_factorize(
        fi: *mut FHVINT,
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
    fn _glp_fhvint_create() -> *mut FHVINT;
    fn _glp_fhvint_delete(fi: *mut FHVINT);
    fn _glp_fhvint_ftran(fi: *mut FHVINT, x: *mut libc::c_double);
    fn _glp_fhvint_btran(fi: *mut FHVINT, x: *mut libc::c_double);
    fn _glp_fhvint_update(
        fi: *mut FHVINT,
        j: i32,
        len: i32,
        ind: *const i32,
        val: *const libc::c_double,
    ) -> i32;
    fn _glp_scfint_delete(fi: *mut SCFINT);
    fn _glp_scfint_create(type_0: i32) -> *mut SCFINT;
    fn _glp_scfint_factorize(
        fi: *mut SCFINT,
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
    fn _glp_scfint_estimate(fi: *mut SCFINT) -> libc::c_double;
    fn _glp_scfint_update(
        fi: *mut SCFINT,
        upd: i32,
        j: i32,
        len: i32,
        ind: *const i32,
        val: *const libc::c_double,
    ) -> i32;
    fn _glp_scfint_btran(fi: *mut SCFINT, x: *mut libc::c_double);
    fn _glp_scfint_ftran(fi: *mut SCFINT, x: *mut libc::c_double);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_bfcp {
    pub msg_lev: i32,
    pub type_0: i32,
    pub lu_size: i32,
    pub piv_tol: libc::c_double,
    pub piv_lim: i32,
    pub suhl: i32,
    pub eps_tol: libc::c_double,
    pub max_gro: libc::c_double,
    pub nfs_max: i32,
    pub upd_tol: libc::c_double,
    pub nrs_max: i32,
    pub rs_size: i32,
    pub foo_bar: [libc::c_double; 38],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FVS {
    pub n: i32,
    pub nnz: i32,
    pub ind: *mut i32,
    pub vec: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BFD {
    pub valid: i32,
    pub type_0: i32,
    pub u: C2RustUnnamed,
    pub parm: glp_bfcp,
    pub upd_cnt: i32,
    pub b_norm: libc::c_double,
    pub i_norm: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub none: *mut libc::c_void,
    pub fhvi: *mut FHVINT,
    pub scfi: *mut SCFINT,
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
pub struct SCF {
    pub n: i32,
    pub n0: i32,
    pub type_0: i32,
    pub a0: C2RustUnnamed_1,
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
pub struct IFU {
    pub n_max: i32,
    pub n: i32,
    pub f: *mut libc::c_double,
    pub u: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub luf: *mut LUF,
    pub btf: *mut BTF,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FHVINT {
    pub valid: i32,
    pub fhv: FHV,
    pub lufi: *mut LUFINT,
    pub nfs_max: i32,
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
pub struct bfd_info {
    pub bfd: *mut BFD,
    pub col: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            i32,
            *mut i32,
            *mut libc::c_double,
        ) -> i32,
    >,
    pub info: *mut libc::c_void,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_bfd_create_it() -> *mut BFD {
    let mut bfd: *mut BFD = 0 as *mut BFD;
    bfd = glp_alloc(1 as i32, ::core::mem::size_of::<BFD>() as u64 as i32) as *mut BFD;
    (*bfd).valid = 0 as i32;
    (*bfd).type_0 = 0 as i32;
    (*bfd).u.none = 0 as *mut libc::c_void;
    _glp_bfd_set_bfcp(bfd, 0 as *const libc::c_void);
    (*bfd).upd_cnt = 0 as i32;
    return bfd;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_bfd_get_bfcp(
    mut bfd: *mut BFD,
    mut parm: *mut libc::c_void,
) {
    memcpy(
        parm,
        &mut (*bfd).parm as *mut glp_bfcp as *const libc::c_void,
        ::core::mem::size_of::<glp_bfcp>() as u64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _glp_bfd_set_bfcp(
    mut bfd: *mut BFD,
    mut parm: *const libc::c_void,
) {
    if parm == 0 as *mut libc::c_void {
        memset(
            &mut (*bfd).parm as *mut glp_bfcp as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<glp_bfcp>() as u64,
        );
        (*bfd).parm.type_0 = 0 as i32 + 0x1 as i32;
        (*bfd).parm.piv_tol = 0.10f64;
        (*bfd).parm.piv_lim = 4 as i32;
        (*bfd).parm.suhl = 1 as i32;
        (*bfd).parm.eps_tol = 2.2204460492503131e-16f64;
        (*bfd).parm.nfs_max = 100 as i32;
        (*bfd).parm.nrs_max = 70 as i32;
    } else {
        memcpy(
            &mut (*bfd).parm as *mut glp_bfcp as *mut libc::c_void,
            parm,
            ::core::mem::size_of::<glp_bfcp>() as u64,
        );
    };
}
unsafe extern "C" fn bfd_col(
    mut info_: *mut libc::c_void,
    mut j: i32,
    mut ind: *mut i32,
    mut val: *mut libc::c_double,
) -> i32 {
    let mut info: *mut bfd_info = info_ as *mut bfd_info;
    let mut t: i32 = 0;
    let mut len: i32 = 0;
    let mut sum: libc::c_double = 0.;
    len = ((*info).col).expect("non-null function pointer")((*info).info, j, ind, val);
    sum = 0.0f64;
    t = 1 as i32;
    while t <= len {
        if *val.offset(t as isize) >= 0.0f64 {
            sum += *val.offset(t as isize);
        } else {
            sum -= *val.offset(t as isize);
        }
        t += 1;
        t;
    }
    if (*(*info).bfd).b_norm < sum {
        (*(*info).bfd).b_norm = sum;
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_bfd_factorize(
    mut bfd: *mut BFD,
    mut m: i32,
    mut col1: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            i32,
            *mut i32,
            *mut libc::c_double,
        ) -> i32,
    >,
    mut info1: *mut libc::c_void,
) -> i32 {
    let mut info: bfd_info = bfd_info {
        bfd: 0 as *mut BFD,
        col: None,
        info: 0 as *mut libc::c_void,
    };
    let mut type_0: i32 = 0;
    let mut ret: i32 = 0;
    (*bfd).valid = 0 as i32;
    match (*bfd).parm.type_0 {
        1 => {
            type_0 = 1 as i32;
        }
        2 | 3 | 18 | 19 => {
            type_0 = 2 as i32;
        }
        _ => {
            (bfd != bfd
                || {
                    glp_assert_(
                        b"bfd != bfd\0" as *const u8 as *const i8,
                        b"draft/bfd.c\0" as *const u8 as *const i8,
                        159 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    match (*bfd).type_0 {
        0 => {}
        1 => {
            if type_0 != 1 as i32 {
                (*bfd).type_0 = 0 as i32;
                _glp_fhvint_delete((*bfd).u.fhvi);
                (*bfd).u.fhvi = 0 as *mut FHVINT;
            }
        }
        2 => {
            if type_0 != 2 as i32 {
                (*bfd).type_0 = 0 as i32;
                _glp_scfint_delete((*bfd).u.scfi);
                (*bfd).u.scfi = 0 as *mut SCFINT;
            }
        }
        _ => {
            (bfd != bfd
                || {
                    glp_assert_(
                        b"bfd != bfd\0" as *const u8 as *const i8,
                        b"draft/bfd.c\0" as *const u8 as *const i8,
                        180 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    if (*bfd).type_0 == 0 as i32 {
        match type_0 {
            1 => {
                (*bfd).type_0 = 1 as i32;
                (((*bfd).u.fhvi).is_null()
                    || {
                        glp_assert_(
                            b"bfd->u.fhvi == NULL\0" as *const u8 as *const i8,
                            b"draft/bfd.c\0" as *const u8 as *const i8,
                            187 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (*bfd).u.fhvi = _glp_fhvint_create();
            }
            2 => {
                (*bfd).type_0 = 2 as i32;
                (((*bfd).u.scfi).is_null()
                    || {
                        glp_assert_(
                            b"bfd->u.scfi == NULL\0" as *const u8 as *const i8,
                            b"draft/bfd.c\0" as *const u8 as *const i8,
                            192 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                if (*bfd).parm.type_0 & 0x10 as i32 == 0 {
                    (*bfd).u.scfi = _glp_scfint_create(1 as i32);
                } else {
                    (*bfd).u.scfi = _glp_scfint_create(2 as i32);
                }
            }
            _ => {
                (type_0 != type_0
                    || {
                        glp_assert_(
                            b"type != type\0" as *const u8 as *const i8,
                            b"draft/bfd.c\0" as *const u8 as *const i8,
                            199 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
        }
    }
    (*bfd).i_norm = 0.0f64;
    (*bfd).b_norm = (*bfd).i_norm;
    info.bfd = bfd;
    info.col = col1;
    info.info = info1;
    match (*bfd).type_0 {
        1 => {
            (*(*(*bfd).u.fhvi).lufi).sgf_piv_tol = (*bfd).parm.piv_tol;
            (*(*(*bfd).u.fhvi).lufi).sgf_piv_lim = (*bfd).parm.piv_lim;
            (*(*(*bfd).u.fhvi).lufi).sgf_suhl = (*bfd).parm.suhl;
            (*(*(*bfd).u.fhvi).lufi).sgf_eps_tol = (*bfd).parm.eps_tol;
            (*(*bfd).u.fhvi).nfs_max = (*bfd).parm.nfs_max;
            ret = _glp_fhvint_factorize(
                (*bfd).u.fhvi,
                m,
                Some(
                    bfd_col
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            i32,
                            *mut i32,
                            *mut libc::c_double,
                        ) -> i32,
                ),
                &mut info as *mut bfd_info as *mut libc::c_void,
            );
            if ret == 0 as i32 {
                (*bfd).i_norm = _glp_fhvint_estimate((*bfd).u.fhvi);
            } else {
                ret = 1 as i32;
            }
        }
        2 => {
            if (*(*bfd).u.scfi).scf.type_0 == 1 as i32 {
                (*(*(*bfd).u.scfi).u.lufi).sgf_piv_tol = (*bfd).parm.piv_tol;
                (*(*(*bfd).u.scfi).u.lufi).sgf_piv_lim = (*bfd).parm.piv_lim;
                (*(*(*bfd).u.scfi).u.lufi).sgf_suhl = (*bfd).parm.suhl;
                (*(*(*bfd).u.scfi).u.lufi).sgf_eps_tol = (*bfd).parm.eps_tol;
            } else if (*(*bfd).u.scfi).scf.type_0 == 2 as i32 {
                (*(*(*bfd).u.scfi).u.btfi).sgf_piv_tol = (*bfd).parm.piv_tol;
                (*(*(*bfd).u.scfi).u.btfi).sgf_piv_lim = (*bfd).parm.piv_lim;
                (*(*(*bfd).u.scfi).u.btfi).sgf_suhl = (*bfd).parm.suhl;
                (*(*(*bfd).u.scfi).u.btfi).sgf_eps_tol = (*bfd).parm.eps_tol;
            } else {
                (bfd != bfd
                    || {
                        glp_assert_(
                            b"bfd != bfd\0" as *const u8 as *const i8,
                            b"draft/bfd.c\0" as *const u8 as *const i8,
                            238 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            (*(*bfd).u.scfi).nn_max = (*bfd).parm.nrs_max;
            ret = _glp_scfint_factorize(
                (*bfd).u.scfi,
                m,
                Some(
                    bfd_col
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            i32,
                            *mut i32,
                            *mut libc::c_double,
                        ) -> i32,
                ),
                &mut info as *mut bfd_info as *mut libc::c_void,
            );
            if ret == 0 as i32 {
                (*bfd).i_norm = _glp_scfint_estimate((*bfd).u.scfi);
            } else {
                ret = 1 as i32;
            }
        }
        _ => {
            (bfd != bfd
                || {
                    glp_assert_(
                        b"bfd != bfd\0" as *const u8 as *const i8,
                        b"draft/bfd.c\0" as *const u8 as *const i8,
                        249 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    if ret == 0 as i32 {
        let mut cond: libc::c_double = 0.;
        (*bfd).valid = 1 as i32;
    }
    (*bfd).upd_cnt = 0 as i32;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_bfd_condest(mut bfd: *mut BFD) -> libc::c_double {
    let mut cond: libc::c_double = 0.;
    ((*bfd).valid != 0
        || {
            glp_assert_(
                b"bfd->valid\0" as *const u8 as *const i8,
                b"draft/bfd.c\0" as *const u8 as *const i8,
                309 as i32,
            );
            1 as i32 != 0
        }) as i32;
    cond = (*bfd).b_norm * (*bfd).i_norm;
    if cond < 1.0f64 {
        cond = 1.0f64;
    }
    return cond;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_bfd_ftran(mut bfd: *mut BFD, mut x: *mut libc::c_double) {
    ((*bfd).valid != 0
        || {
            glp_assert_(
                b"bfd->valid\0" as *const u8 as *const i8,
                b"draft/bfd.c\0" as *const u8 as *const i8,
                330 as i32,
            );
            1 as i32 != 0
        }) as i32;
    match (*bfd).type_0 {
        1 => {
            _glp_fhvint_ftran((*bfd).u.fhvi, x);
        }
        2 => {
            _glp_scfint_ftran((*bfd).u.scfi, x);
        }
        _ => {
            (bfd != bfd
                || {
                    glp_assert_(
                        b"bfd != bfd\0" as *const u8 as *const i8,
                        b"draft/bfd.c\0" as *const u8 as *const i8,
                        339 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_bfd_ftran_s(mut bfd: *mut BFD, mut x: *mut FVS) {
    let mut n: i32 = (*x).n;
    let mut ind: *mut i32 = (*x).ind;
    let mut vec: *mut libc::c_double = (*x).vec;
    let mut j: i32 = 0;
    let mut nnz: i32 = 0 as i32;
    _glp_bfd_ftran(bfd, vec);
    j = n;
    while j >= 1 as i32 {
        if *vec.offset(j as isize) != 0.0f64 {
            nnz += 1;
            *ind.offset(nnz as isize) = j;
        }
        j -= 1;
        j;
    }
    (*x).nnz = nnz;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_bfd_btran(mut bfd: *mut BFD, mut x: *mut libc::c_double) {
    ((*bfd).valid != 0
        || {
            glp_assert_(
                b"bfd->valid\0" as *const u8 as *const i8,
                b"draft/bfd.c\0" as *const u8 as *const i8,
                389 as i32,
            );
            1 as i32 != 0
        }) as i32;
    match (*bfd).type_0 {
        1 => {
            _glp_fhvint_btran((*bfd).u.fhvi, x);
        }
        2 => {
            _glp_scfint_btran((*bfd).u.scfi, x);
        }
        _ => {
            (bfd != bfd
                || {
                    glp_assert_(
                        b"bfd != bfd\0" as *const u8 as *const i8,
                        b"draft/bfd.c\0" as *const u8 as *const i8,
                        398 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_bfd_btran_s(mut bfd: *mut BFD, mut x: *mut FVS) {
    let mut n: i32 = (*x).n;
    let mut ind: *mut i32 = (*x).ind;
    let mut vec: *mut libc::c_double = (*x).vec;
    let mut j: i32 = 0;
    let mut nnz: i32 = 0 as i32;
    _glp_bfd_btran(bfd, vec);
    j = n;
    while j >= 1 as i32 {
        if *vec.offset(j as isize) != 0.0f64 {
            nnz += 1;
            *ind.offset(nnz as isize) = j;
        }
        j -= 1;
        j;
    }
    (*x).nnz = nnz;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_bfd_update(
    mut bfd: *mut BFD,
    mut j: i32,
    mut len: i32,
    mut ind: *const i32,
    mut val: *const libc::c_double,
) -> i32 {
    let mut ret: i32 = 0;
    ((*bfd).valid != 0
        || {
            glp_assert_(
                b"bfd->valid\0" as *const u8 as *const i8,
                b"draft/bfd.c\0" as *const u8 as *const i8,
                440 as i32,
            );
            1 as i32 != 0
        }) as i32;
    match (*bfd).type_0 {
        1 => {
            ret = _glp_fhvint_update((*bfd).u.fhvi, j, len, ind, val);
            match ret {
                0 => {}
                1 => {
                    ret = 1 as i32;
                }
                2 | 3 => {
                    ret = 2 as i32;
                }
                4 => {
                    ret = 4 as i32;
                }
                5 => {
                    ret = 3 as i32;
                }
                _ => {
                    (ret != ret
                        || {
                            glp_assert_(
                                b"ret != ret\0" as *const u8 as *const i8,
                                b"draft/bfd.c\0" as *const u8 as *const i8,
                                462 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
            }
        }
        2 => {
            match (*bfd).parm.type_0 & 0xf as i32 {
                2 => {
                    ret = _glp_scfint_update((*bfd).u.scfi, 1 as i32, j, len, ind, val);
                }
                3 => {
                    ret = _glp_scfint_update((*bfd).u.scfi, 2 as i32, j, len, ind, val);
                }
                _ => {
                    (bfd != bfd
                        || {
                            glp_assert_(
                                b"bfd != bfd\0" as *const u8 as *const i8,
                                b"draft/bfd.c\0" as *const u8 as *const i8,
                                475 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
            }
            match ret {
                0 => {}
                1 => {
                    ret = 4 as i32;
                }
                2 => {
                    ret = 2 as i32;
                }
                _ => {
                    (ret != ret
                        || {
                            glp_assert_(
                                b"ret != ret\0" as *const u8 as *const i8,
                                b"draft/bfd.c\0" as *const u8 as *const i8,
                                488 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
            }
        }
        _ => {
            (bfd != bfd
                || {
                    glp_assert_(
                        b"bfd != bfd\0" as *const u8 as *const i8,
                        b"draft/bfd.c\0" as *const u8 as *const i8,
                        493 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    if ret != 0 as i32 {
        (*bfd).valid = 0 as i32;
    }
    if ret == 0 as i32 {
        (*bfd).upd_cnt += 1;
        (*bfd).upd_cnt;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_bfd_get_count(mut bfd: *mut BFD) -> i32 {
    return (*bfd).upd_cnt;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_bfd_delete_it(mut bfd: *mut BFD) {
    match (*bfd).type_0 {
        0 => {}
        1 => {
            _glp_fhvint_delete((*bfd).u.fhvi);
        }
        2 => {
            _glp_scfint_delete((*bfd).u.scfi);
        }
        _ => {
            (bfd != bfd
                || {
                    glp_assert_(
                        b"bfd != bfd\0" as *const u8 as *const i8,
                        b"draft/bfd.c\0" as *const u8 as *const i8,
                        532 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    glp_free(bfd as *mut libc::c_void);
}