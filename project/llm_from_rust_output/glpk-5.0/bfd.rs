use std::mem;
use std::ptr;
use std::os::raw::{c_int, c_double, c_void, c_char};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct glp_bfcp {
    pub msg_lev: c_int,
    pub type_: c_int,
    pub lu_size: c_int,
    pub piv_tol: c_double,
    pub piv_lim: c_int,
    pub suhl: c_int,
    pub eps_tol: c_double,
    pub max_gro: c_double,
    pub nfs_max: c_int,
    pub upd_tol: c_double,
    pub nrs_max: c_int,
    pub rs_size: c_int,
    pub foo_bar: [c_double; 38],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FVS {
    pub n: c_int,
    pub nnz: c_int,
    pub ind: *mut c_int,
    pub vec: *mut c_double,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BFD {
    pub valid: c_int,
    pub type_: c_int,
    pub u: BFDUnion,
    pub parm: glp_bfcp,
    pub upd_cnt: c_int,
    pub b_norm: c_double,
    pub i_norm: c_double,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub union BFDUnion {
    pub none: *mut c_void,
    pub fhvi: *mut FHVINT,
    pub scfi: *mut SCFINT,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SCFINT {
    pub valid: c_int,
    pub scf: SCF,
    pub u: SCFINTUnion,
    pub w1: *mut c_double,
    pub w2: *mut c_double,
    pub w3: *mut c_double,
    pub w4: *mut c_double,
    pub w5: *mut c_double,
    pub nn_max: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub union SCFINTUnion {
    pub lufi: *mut LUFINT,
    pub btfi: *mut BTFINT,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BTFINT {
    pub n_max: c_int,
    pub valid: c_int,
    pub sva: *mut SVA,
    pub btf: *mut BTF,
    pub sgf: *mut SGF,
    pub sva_n_max: c_int,
    pub sva_size: c_int,
    pub delta_n0: c_int,
    pub delta_n: c_int,
    pub sgf_piv_tol: c_double,
    pub sgf_piv_lim: c_int,
    pub sgf_suhl: c_int,
    pub sgf_eps_tol: c_double,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SGF {
    pub luf: *mut LUF,
    pub rs_head: *mut c_int,
    pub rs_prev: *mut c_int,
    pub rs_next: *mut c_int,
    pub cs_head: *mut c_int,
    pub cs_prev: *mut c_int,
    pub cs_next: *mut c_int,
    pub vr_max: *mut c_double,
    pub flag: *mut c_char,
    pub work: *mut c_double,
    pub updat: c_int,
    pub piv_tol: c_double,
    pub piv_lim: c_int,
    pub suhl: c_int,
    pub eps_tol: c_double,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LUF {
    pub n: c_int,
    pub sva: *mut SVA,
    pub fr_ref: c_int,
    pub fc_ref: c_int,
    pub vr_ref: c_int,
    pub vr_piv: *mut c_double,
    pub vc_ref: c_int,
    pub pp_ind: *mut c_int,
    pub pp_inv: *mut c_int,
    pub qq_ind: *mut c_int,
    pub qq_inv: *mut c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SVA {
    pub n_max: c_int,
    pub n: c_int,
    pub ptr: *mut c_int,
    pub len: *mut c_int,
    pub cap: *mut c_int,
    pub size: c_int,
    pub m_ptr: c_int,
    pub r_ptr: c_int,
    pub head: c_int,
    pub tail: c_int,
    pub prev: *mut c_int,
    pub next: *mut c_int,
    pub ind: *mut c_int,
    pub val: *mut c_double,
    pub talky: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BTF {
    pub n: c_int,
    pub sva: *mut SVA,
    pub pp_ind: *mut c_int,
    pub pp_inv: *mut c_int,
    pub qq_ind: *mut c_int,
    pub qq_inv: *mut c_int,
    pub num: c_int,
    pub beg: *mut c_int,
    pub ar_ref: c_int,
    pub ac_ref: c_int,
    pub fr_ref: c_int,
    pub fc_ref: c_int,
    pub vr_ref: c_int,
    pub vr_piv: *mut c_double,
    pub vc_ref: c_int,
    pub p1_ind: *mut c_int,
    pub p1_inv: *mut c_int,
    pub q1_ind: *mut c_int,
    pub q1_inv: *mut c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LUFINT {
    pub n_max: c_int,
    pub valid: c_int,
    pub sva: *mut SVA,
    pub luf: *mut LUF,
    pub sgf: *mut SGF,
    pub sva_n_max: c_int,
    pub sva_size: c_int,
    pub delta_n0: c_int,
    pub delta_n: c_int,
    pub sgf_updat: c_int,
    pub sgf_piv_tol: c_double,
    pub sgf_piv_lim: c_int,
    pub sgf_suhl: c_int,
    pub sgf_eps_tol: c_double,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SCF {
    pub n: c_int,
    pub n0: c_int,
    pub type_: c_int,
    pub a0: SCFUnion,
    pub nn_max: c_int,
    pub nn: c_int,
    pub sva: *mut SVA,
    pub rr_ref: c_int,
    pub ss_ref: c_int,
    pub ifu: IFU,
    pub pp_ind: *mut c_int,
    pub pp_inv: *mut c_int,
    pub qq_ind: *mut c_int,
    pub qq_inv: *mut c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IFU {
    pub n_max: c_int,
    pub n: c_int,
    pub f: *mut c_double,
    pub u: *mut c_double,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub union SCFUnion {
    pub luf: *mut LUF,
    pub btf: *mut BTF,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FHVINT {
    pub valid: c_int,
    pub fhv: FHV,
    pub lufi: *mut LUFINT,
    pub nfs_max: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FHV {
    pub luf: *mut LUF,
    pub nfs_max: c_int,
    pub nfs: c_int,
    pub hh_ind: *mut c_int,
    pub hh_ref: c_int,
    pub p0_ind: *mut c_int,
    pub p0_inv: *mut c_int,
}

#[repr(C)]
#[derive(Debug)]
pub struct bfd_info {
    pub bfd: *mut BFD,
    pub col: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_int, *mut c_double) -> c_int>,
    pub info: *mut c_void,
}

extern "C" {
    fn glp_assert_(expr: *const c_char, file: *const c_char, line: c_int);
    fn glp_alloc(n: c_int, size: c_int) -> *mut c_void;
    fn glp_free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn _glp_fhvint_estimate(fi: *mut FHVINT) -> c_double;
    fn _glp_fhvint_factorize(
        fi: *mut FHVINT,
        n: c_int,
        col: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_int, *mut c_double) -> c_int>,
        info: *mut c_void,
    ) -> c_int;
    fn _glp_fhvint_create() -> *mut FHVINT;
    fn _glp_fhvint_delete(fi: *mut FHVINT);
    fn _glp_fhvint_ftran(fi: *mut FHVINT, x: *mut c_double);
    fn _glp_fhvint_btran(fi: *mut FHVINT, x: *mut c_double);
    fn _glp_fhvint_update(
        fi: *mut FHVINT,
        j: c_int,
        len: c_int,
        ind: *const c_int,
        val: *const c_double,
    ) -> c_int;
    fn _glp_scfint_delete(fi: *mut SCFINT);
    fn _glp_scfint_create(type_: c_int) -> *mut SCFINT;
    fn _glp_scfint_factorize(
        fi: *mut SCFINT,
        n: c_int,
        col: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_int, *mut c_double) -> c_int>,
        info: *mut c_void,
    ) -> c_int;
    fn _glp_scfint_estimate(fi: *mut SCFINT) -> c_double;
    fn _glp_scfint_update(
        fi: *mut SCFINT,
        upd: c_int,
        j: c_int,
        len: c_int,
        ind: *const c_int,
        val: *const c_double,
    ) -> c_int;
    fn _glp_scfint_btran(fi: *mut SCFINT, x: *mut c_double);
    fn _glp_scfint_ftran(fi: *mut SCFINT, x: *mut c_double);
}

#[no_mangle]
pub unsafe extern "C" fn _glp_bfd_create_it() -> *mut BFD {
    let bfd = glp_alloc(1, mem::size_of::<BFD>() as c_int) as *mut BFD;
    if bfd.is_null() {
        return ptr::null_mut();
    }

    (*bfd).valid = 0;
    (*bfd).type_ = 0;
    (*bfd).u.none = ptr::null_mut();
    _glp_bfd_set_bfcp(bfd, ptr::null());
    (*bfd).upd_cnt = 0;
    bfd
}

#[no_mangle]
pub unsafe extern "C" fn _glp_bfd_get_bfcp(bfd: *mut BFD, parm: *mut c_void) {
    if !bfd.is_null() && !parm.is_null() {
        memcpy(parm, &(*bfd).parm as *const _ as *const c_void, mem::size_of::<glp_bfcp>());
    }
}

#[no_mangle]
pub unsafe extern "C" fn _glp_bfd_set_bfcp(bfd: *mut BFD, parm: *const c_void) {
    if bfd.is_null() {
        return;
    }

    if parm.is_null() {
        memset(&mut (*bfd).parm as *mut _ as *mut c_void, 0, mem::size_of::<glp_bfcp>());
        (*bfd).parm.type_ = 1;
        (*bfd).parm.piv_tol = 0.10;
        (*bfd).parm.piv_lim = 4;
        (*bfd).parm.suhl = 1;
        (*bfd).parm.eps_tol = 2.2204460492503131e-16;
        (*bfd).parm.nfs_max = 100;
        (*bfd).parm.nrs_max = 70;
    } else {
        memcpy(&mut (*bfd).parm as *mut _ as *mut c_void, parm, mem::size_of::<glp_bfcp>());
    }
}

unsafe extern "C" fn bfd_col(
    info_: *mut c_void,
    j: c_int,
    ind: *mut c_int,
    val: *mut c_double,
) -> c_int {
    let info = info_ as *mut bfd_info;
    if info.is_null() || (*info).col.is_none() {
        return 0;
    }

    let len = ((*info).col.unwrap())((*info).info, j, ind, val);
    let mut sum = 0.0;

    for t in 1..=len {
        sum += (*val.offset(t as isize)).abs();
    }

    if (*(*info).bfd).b_norm < sum {
        (*(*info).bfd).b_norm = sum;
    }

    len
}

#[no_mangle]
pub unsafe extern "C" fn _glp_bfd_factorize(
    bfd: *mut BFD,
    m: c_int,
    col: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_int, *mut c_double) -> c_int>,
    info: *mut c_void,
) -> c_int {
    if bfd.is_null() {
        return 1;
    }

    let mut info_struct = bfd_info {
        bfd,
        col,
        info,
    };

    (*bfd).valid = 0;
    let type_ = match (*bfd).parm.type_ {
        1 => 1,
        2 | 3 | 18 | 19 => 2,
        _ => {
            glp_assert_(
                b"bfd != bfd\0".as_ptr() as *const c_char,
                b"draft/bfd.c\0".as_ptr() as *const c_char,
                159,
            );
            return 1;
        }
    };

    match (*bfd).type_ {
        0 => {}
        1 => {
            if type_ != 1 {
                (*bfd).type_ = 0;
                _glp_fhvint_delete((*bfd).u.fhvi);
                (*bfd).u.fhvi = ptr::null_mut();
            }
        }
        2 => {
            if type_ != 2 {
                (*bfd).type_ = 0;
                _glp_scfint_delete((*bfd).u.scfi);
                (*bfd).u.scfi = ptr::null_mut();
            }
        }
        _ => {
            glp_assert_(
                b"bfd != bfd\0".as_ptr() as *const c_char,
                b"draft/bfd.c\0".as_ptr() as *const c_char,
                180,
            );
            return 1;
        }
    }

    if (*bfd).type_ == 0 {
        match type_ {
            1 => {
                (*bfd).type_ = 1;
                (*bfd).u.fhvi = _glp_fhvint_create();
            }
            2 => {
                (*bfd).type_ = 2;
                (*bfd).u.scfi = if (*bfd).parm.type_ & 0x10 == 0 {
                    _glp_scfint_create(1)
                } else {
                    _glp_scfint_create(2)
                };
            }
            _ => {
                glp_assert_(
                    b"type != type\0".as_ptr() as *const c_char,
                    b"draft/bfd.c\0".as_ptr() as *const c_char,
                    199,
                );
                return 1;
            }
        }
    }

    (*bfd).i_norm = 0.0;
    (*bfd).b_norm = (*bfd).i_norm;

    let ret = match (*bfd).type_ {
        1 => {
            (*(*(*bfd).u.fhvi).lufi).sgf_piv_tol = (*bfd).parm.piv_tol;
            (*(*(*bfd).u.fhvi).lufi).sgf_piv_lim = (*bfd).parm.piv_lim;
            (*(*(*bfd).u.fhvi).lufi).sgf_suhl = (*bfd).parm.suhl;
            (*(*(*bfd).u.fhvi).lufi).sgf_eps_tol = (*bfd).parm.eps_tol;
            (*(*bfd).u.fhvi).nfs_max = (*bfd).parm.nfs_max;

            let ret = _glp_fhvint_factorize(
                (*bfd).u.fhvi,
                m,
                Some(bfd_col),
                &mut info_struct