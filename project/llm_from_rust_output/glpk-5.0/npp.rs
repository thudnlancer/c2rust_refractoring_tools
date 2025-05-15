use std::ffi::CString;
use std::ptr;

pub type GlpErrFunc = Option<extern "C" fn(*const libc::c_char, ...)>;

#[repr(C)]
pub struct GlpProb {
    pub pool: *mut libc::c_void,
    pub tree: *mut libc::c_void,
    pub name: *mut libc::c_char,
    pub obj: *mut libc::c_char,
    pub dir: libc::c_int,
    pub c0: libc::c_double,
    pub m_max: libc::c_int,
    pub n_max: libc::c_int,
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub nnz: libc::c_int,
    pub row: *mut *mut GlpRow,
    pub col: *mut *mut GlpCol,
    pub r_tree: *mut libc::c_void,
    pub c_tree: *mut libc::c_void,
    pub valid: libc::c_int,
    pub head: *mut libc::c_int,
    pub bfd: *mut libc::c_void,
    pub pbs_stat: libc::c_int,
    pub dbs_stat: libc::c_int,
    pub obj_val: libc::c_double,
    pub it_cnt: libc::c_int,
    pub some: libc::c_int,
    pub ipt_stat: libc::c_int,
    pub ipt_obj: libc::c_double,
    pub mip_stat: libc::c_int,
    pub mip_obj: libc::c_double,
}

#[repr(C)]
pub struct GlpCol {
    pub j: libc::c_int,
    pub name: *mut libc::c_char,
    pub node: *mut libc::c_void,
    pub kind: libc::c_int,
    pub type_: libc::c_int,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub coef: libc::c_double,
    pub ptr: *mut GlpAij,
    pub sjj: libc::c_double,
    pub stat: libc::c_int,
    pub bind: libc::c_int,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
    pub pval: libc::c_double,
    pub dval: libc::c_double,
    pub mipx: libc::c_double,
}

#[repr(C)]
pub struct GlpAij {
    pub row: *mut GlpRow,
    pub col: *mut GlpCol,
    pub val: libc::c_double,
    pub r_prev: *mut GlpAij,
    pub r_next: *mut GlpAij,
    pub c_prev: *mut GlpAij,
    pub c_next: *mut GlpAij,
}

#[repr(C)]
pub struct GlpRow {
    pub i: libc::c_int,
    pub name: *mut libc::c_char,
    pub node: *mut libc::c_void,
    pub level: libc::c_int,
    pub origin: libc::c_uchar,
    pub klass: libc::c_uchar,
    pub type_: libc::c_int,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub ptr: *mut GlpAij,
    pub rii: libc::c_double,
    pub stat: libc::c_int,
    pub bind: libc::c_int,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
    pub pval: libc::c_double,
    pub dval: libc::c_double,
    pub mipx: libc::c_double,
}

#[repr(C)]
pub struct GlpPrep {
    pub orig_dir: libc::c_int,
    pub orig_m: libc::c_int,
    pub orig_n: libc::c_int,
    pub orig_nnz: libc::c_int,
    pub pool: *mut libc::c_void,
    pub name: *mut libc::c_char,
    pub obj: *mut libc::c_char,
    pub c0: libc::c_double,
    pub nrows: libc::c_int,
    pub ncols: libc::c_int,
    pub r_head: *mut NppRow,
    pub r_tail: *mut NppRow,
    pub c_head: *mut NppCol,
    pub c_tail: *mut NppCol,
    pub stack: *mut libc::c_void,
    pub top: *mut NppTse,
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub nnz: libc::c_int,
    pub row_ref: *mut libc::c_int,
    pub col_ref: *mut libc::c_int,
    pub sol: libc::c_int,
    pub scaling: libc::c_int,
    pub p_stat: libc::c_int,
    pub d_stat: libc::c_int,
    pub t_stat: libc::c_int,
    pub i_stat: libc::c_int,
    pub r_stat: *mut libc::c_char,
    pub c_stat: *mut libc::c_char,
    pub r_pi: *mut libc::c_double,
    pub c_value: *mut libc::c_double,
}

#[repr(C)]
pub struct NppTse {
    pub func: Option<extern "C" fn(*mut GlpPrep, *mut libc::c_void) -> libc::c_int>,
    pub info: *mut libc::c_void,
    pub link: *mut NppTse,
}

#[repr(C)]
pub struct NppCol {
    pub j: libc::c_int,
    pub name: *mut libc::c_char,
    pub is_int: libc::c_char,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub coef: libc::c_double,
    pub ptr: *mut NppAij,
    pub temp: libc::c_int,
    pub ll: NppColUnion,
    pub uu: NppColUnion,
    pub prev: *mut NppCol,
    pub next: *mut NppCol,
}

#[repr(C)]
pub union NppColUnion {
    pub uu: libc::c_double,
    pub neg: libc::c_int,
}

#[repr(C)]
pub struct NppAij {
    pub row: *mut NppRow,
    pub col: *mut NppCol,
    pub val: libc::c_double,
    pub r_prev: *mut NppAij,
    pub r_next: *mut NppAij,
    pub c_prev: *mut NppAij,
    pub c_next: *mut NppAij,
}

#[repr(C)]
pub struct NppRow {
    pub i: libc::c_int,
    pub name: *mut libc::c_char,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub ptr: *mut NppAij,
    pub temp: libc::c_int,
    pub prev: *mut NppRow,
    pub next: *mut NppRow,
}

extern "C" {
    fn glp_error(file: *const libc::c_char, line: libc::c_int) -> GlpErrFunc;
    fn glp_assert(expr: *const libc::c_char, file: *const libc::c_char, line: libc::c_int);
    fn glp_get_status(P: *mut GlpProb) -> libc::c_int;
    fn glp_ipt_status(P: *mut GlpProb) -> libc::c_int;
    fn glp_mip_status(P: *mut GlpProb) -> libc::c_int;
    fn _glp_npp_create_wksp() -> *mut GlpPrep;
    fn _glp_npp_load_prob(
        npp: *mut GlpPrep,
        orig: *mut GlpProb,
        names: libc::c_int,
        sol: libc::c_int,
        scaling: libc::c_int,
    );
    fn _glp_npp_delete_wksp(npp: *mut GlpPrep);
    fn _glp_npp_unload_sol(npp: *mut GlpPrep, orig: *mut GlpProb);
    fn _glp_npp_build_prob(npp: *mut GlpPrep, prob: *mut GlpProb);
    fn _glp_npp_postprocess(npp: *mut GlpPrep, prob: *mut GlpProb);
    fn _glp_npp_process_prob(npp: *mut GlpPrep, hard: libc::c_int) -> libc::c_int;
}

#[no_mangle]
pub extern "C" fn glp_npp_alloc_wksp() -> *mut GlpPrep {
    unsafe { _glp_npp_create_wksp() }
}

#[no_mangle]
pub extern "C" fn glp_npp_load_prob(
    prep: *mut GlpPrep,
    P: *mut GlpProb,
    sol: libc::c_int,
    names: libc::c_int,
) {
    unsafe {
        if (*prep).sol != 0 {
            let msg = CString::new("glp_npp_load_prob: invalid call sequence (original instance already loaded)\n").unwrap();
            glp_error(b"api/npp.c\0" as *const u8 as *const libc::c_char, 35).unwrap()(msg.as_ptr());
        }

        if !(sol == 1 || sol == 2 || sol == 3) {
            let msg = CString::new(format!("glp_npp_load_prob: sol = {}; invalid parameter\n", sol)).unwrap();
            glp_error(b"api/npp.c\0" as *const u8 as *const libc::c_char, 38).unwrap()(msg.as_ptr());
        }

        if !(names == 1 || names == 0) {
            let msg = CString::new(format!("glp_npp_load_prob: names = {}; invalid parameter\n", names)).unwrap();
            glp_error(b"api/npp.c\0" as *const u8 as *const libc::c_char, 41).unwrap()(msg.as_ptr());
        }

        _glp_npp_load_prob(prep, P, names, sol, 0);
    }
}

#[no_mangle]
pub extern "C" fn glp_npp_preprocess1(prep: *mut GlpPrep, hard: libc::c_int) -> libc::c_int {
    unsafe {
        if (*prep).sol == 0 {
            let msg = CString::new("glp_npp_preprocess1: invalid call sequence (original instance not loaded yet)\n").unwrap();
            glp_error(b"api/npp.c\0" as *const u8 as *const libc::c_char, 50).unwrap()(msg.as_ptr());
        }

        if (*prep).pool.is_null() {
            let msg = CString::new("glp_npp_preprocess1: invalid call sequence (preprocessing already finished)\n").unwrap();
            glp_error(b"api/npp.c\0" as *const u8 as *const libc::c_char, 53).unwrap()(msg.as_ptr());
        }

        if !(hard == 1 || hard == 0) {
            let msg = CString::new(format!("glp_npp_preprocess1: hard = {}; invalid parameter\n", hard)).unwrap();
            glp_error(b"api/npp.c\0" as *const u8 as *const libc::c_char, 56).unwrap()(msg.as_ptr());
        }

        _glp_npp_process_prob(prep, hard)
    }
}

#[no_mangle]
pub extern "C" fn glp_npp_build_prob(prep: *mut GlpPrep, Q: *mut GlpProb) {
    unsafe {
        if (*prep).sol == 0 {
            let msg = CString::new("glp_npp_build_prob: invalid call sequence (original instance not loaded yet)\n").unwrap();
            glp_error(b"api/npp.c\0" as *const u8 as *const libc::c_char, 64).unwrap()(msg.as_ptr());
        }

        if (*prep).pool.is_null() {
            let msg = CString::new("glp_npp_build_prob: invalid call sequence (resultant instance already built)\n").unwrap();
            glp_error(b"api/npp.c\0" as *const u8 as *const libc::c_char, 67).unwrap()(msg.as_ptr());
        }

        _glp_npp_build_prob(prep, Q);
    }
}

#[no_mangle]
pub extern "C" fn glp_npp_postprocess(prep: *mut GlpPrep, Q: *mut GlpProb) {
    unsafe {
        if !(*prep).pool.is_null() {
            let msg = CString::new("glp_npp_postprocess: invalid call sequence (resultant instance not built yet)\n").unwrap();
            glp_error(b"api/npp.c\0" as *const u8 as *const libc::c_char, 76).unwrap()(msg.as_ptr());
        }

        if !((*prep).m == (*Q).m && (*prep).n == (*Q).n && (*prep).nnz == (*Q).nnz) {
            let msg = CString::new("glp_npp_postprocess: resultant instance mismatch\n").unwrap();
            glp_error(b"api/npp.c\0" as *const u8 as *const libc::c_char, 79).unwrap()(msg.as_ptr());
        }

        match (*prep).sol {
            1 => {
                if glp_get_status(Q) != 5 {
                    let msg = CString::new("glp_npp_postprocess: unable to recover non-optimal basic solution\n").unwrap();
                    glp_error(b"api/npp.c\0" as *const u8 as *const libc::c_char, 83).unwrap()(msg.as_ptr());
                }
            }
            2 => {
                if glp_ipt_status(Q) != 5 {
                    let msg = CString::new("glp_npp_postprocess: unable to recover non-optimal interior-point solution\n").unwrap();
                    glp_error(b"api/npp.c\0" as *const u8 as *const libc::c_char, 88).unwrap()(msg.as_ptr());
                }
            }
            3 => {
                if !(glp_mip_status(Q) == 5 || glp_mip_status(Q) == 2) {
                    let msg = CString::new("glp_npp_postprocess: unable to recover integer non-feasible solution\n").unwrap();
                    glp_error(b"api/npp.c\0" as *const u8 as *const libc::c_char, 94).unwrap()(msg.as_ptr());
                }
            }
            _ => {
                glp_assert(
                    b"prep != prep\0" as *const u8 as *const libc::c_char,
                    b"api/npp.c\0" as *const u8 as *const libc::c_char,
                    98,
                );
            }
        }

        _glp_npp_postprocess(prep, Q);
    }
}

#[no_mangle]
pub extern "C" fn glp_npp_obtain_sol(prep: *mut GlpPrep, P: *mut GlpProb) {
    unsafe {
        if !(*prep).pool.is_null() {
            let msg = CString::new("glp_npp_obtain_sol: invalid call sequence (resultant instance not built yet)\n").unwrap();
            glp_error(b"api/npp.c\0" as *const u8 as *const libc::c_char, 107).unwrap()(msg.as_ptr());
        }

        match (*prep).sol {
            1 => {
                if (*prep).p_stat == 0 || (*prep).d_stat == 0 {
                    let msg = CString::new("glp_npp_obtain_sol: invalid call sequence (basic solution not provided yet)\n").unwrap();
                    glp_error(b"api/npp.c\0" as *const u8 as *const libc::c_char, 112).unwrap()(msg.as_ptr());
                }
            }
            2 => {
                if (*prep).t_stat == 0 {
                    let msg = CString::new("glp_npp_obtain_sol: invalid call sequence (interior-point solution not provided yet)\n").unwrap();
                    glp_error(b"api/npp.c\0" as *const u8 as *const libc::c_char, 117).unwrap()(msg.as_ptr());
                }
            }
            3 => {
                if (*prep).i_stat == 0 {
                    let msg = CString::new("glp_npp_obtain_sol: invalid call sequence (MIP solution not provided yet)\n").unwrap();
                    glp_error(b"api/npp.c\0" as *const u8 as *const libc::c_char, 122).unwrap()(msg.as_ptr());
                }
            }
            _ => {
                glp_assert(
                    b"prep != prep\0" as *const u8 as *const libc::c_char,
                    b"api/npp.c\0" as *const u8 as *const libc::c_char,
                    126,
                );
            }
        }

        if !((*prep).orig_dir == (*P).dir
            && (*prep).orig_m == (*P).m
            && (*prep).orig_n == (*P).n
            && (*prep).orig_nnz == (*P).nnz)
        {
            let msg = CString::new("glp_npp_obtain_sol: original instance mismatch\n").unwrap();
            glp_error(b"api/n