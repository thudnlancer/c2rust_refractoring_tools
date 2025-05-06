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
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_get_status(P: *mut glp_prob) -> i32;
    fn glp_ipt_status(P: *mut glp_prob) -> i32;
    fn glp_mip_status(P: *mut glp_prob) -> i32;
    fn _glp_npp_create_wksp() -> *mut NPP;
    fn _glp_npp_load_prob(
        npp: *mut NPP,
        orig: *mut glp_prob,
        names: i32,
        sol: i32,
        scaling: i32,
    );
    fn _glp_npp_delete_wksp(npp: *mut NPP);
    fn _glp_npp_unload_sol(npp: *mut NPP, orig: *mut glp_prob);
    fn _glp_npp_build_prob(npp: *mut NPP, prob: *mut glp_prob);
    fn _glp_npp_postprocess(npp: *mut NPP, prob: *mut glp_prob);
    fn _glp_npp_process_prob(npp: *mut NPP, hard: i32) -> i32;
}
pub type glp_errfunc = Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_prob {
    pub pool: *mut DMP,
    pub tree: *mut glp_tree,
    pub name: *mut i8,
    pub obj: *mut i8,
    pub dir: i32,
    pub c0: libc::c_double,
    pub m_max: i32,
    pub n_max: i32,
    pub m: i32,
    pub n: i32,
    pub nnz: i32,
    pub row: *mut *mut GLPROW,
    pub col: *mut *mut GLPCOL,
    pub r_tree: *mut AVL,
    pub c_tree: *mut AVL,
    pub valid: i32,
    pub head: *mut i32,
    pub bfd: *mut BFD,
    pub pbs_stat: i32,
    pub dbs_stat: i32,
    pub obj_val: libc::c_double,
    pub it_cnt: i32,
    pub some: i32,
    pub ipt_stat: i32,
    pub ipt_obj: libc::c_double,
    pub mip_stat: i32,
    pub mip_obj: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPCOL {
    pub j: i32,
    pub name: *mut i8,
    pub node: *mut AVLNODE,
    pub kind: i32,
    pub type_0: i32,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub coef: libc::c_double,
    pub ptr: *mut GLPAIJ,
    pub sjj: libc::c_double,
    pub stat: i32,
    pub bind: i32,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
    pub pval: libc::c_double,
    pub dval: libc::c_double,
    pub mipx: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPAIJ {
    pub row: *mut GLPROW,
    pub col: *mut GLPCOL,
    pub val: libc::c_double,
    pub r_prev: *mut GLPAIJ,
    pub r_next: *mut GLPAIJ,
    pub c_prev: *mut GLPAIJ,
    pub c_next: *mut GLPAIJ,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPROW {
    pub i: i32,
    pub name: *mut i8,
    pub node: *mut AVLNODE,
    pub level: i32,
    pub origin: u8,
    pub klass: u8,
    pub type_0: i32,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub ptr: *mut GLPAIJ,
    pub rii: libc::c_double,
    pub stat: i32,
    pub bind: i32,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
    pub pval: libc::c_double,
    pub dval: libc::c_double,
    pub mipx: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_prep {
    pub orig_dir: i32,
    pub orig_m: i32,
    pub orig_n: i32,
    pub orig_nnz: i32,
    pub pool: *mut DMP,
    pub name: *mut i8,
    pub obj: *mut i8,
    pub c0: libc::c_double,
    pub nrows: i32,
    pub ncols: i32,
    pub r_head: *mut NPPROW,
    pub r_tail: *mut NPPROW,
    pub c_head: *mut NPPCOL,
    pub c_tail: *mut NPPCOL,
    pub stack: *mut DMP,
    pub top: *mut NPPTSE,
    pub m: i32,
    pub n: i32,
    pub nnz: i32,
    pub row_ref: *mut i32,
    pub col_ref: *mut i32,
    pub sol: i32,
    pub scaling: i32,
    pub p_stat: i32,
    pub d_stat: i32,
    pub t_stat: i32,
    pub i_stat: i32,
    pub r_stat: *mut i8,
    pub c_stat: *mut i8,
    pub r_pi: *mut libc::c_double,
    pub c_value: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPTSE {
    pub func: Option<unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32>,
    pub info: *mut libc::c_void,
    pub link: *mut NPPTSE,
}
pub type NPP = glp_prep;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPCOL {
    pub j: i32,
    pub name: *mut i8,
    pub is_int: i8,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub coef: libc::c_double,
    pub ptr: *mut NPPAIJ,
    pub temp: i32,
    pub ll: C2RustUnnamed_0,
    pub uu: C2RustUnnamed,
    pub prev: *mut NPPCOL,
    pub next: *mut NPPCOL,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub uu: libc::c_double,
    pub neg: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ll: libc::c_double,
    pub pos: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPAIJ {
    pub row: *mut NPPROW,
    pub col: *mut NPPCOL,
    pub val: libc::c_double,
    pub r_prev: *mut NPPAIJ,
    pub r_next: *mut NPPAIJ,
    pub c_prev: *mut NPPAIJ,
    pub c_next: *mut NPPAIJ,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPROW {
    pub i: i32,
    pub name: *mut i8,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub ptr: *mut NPPAIJ,
    pub temp: i32,
    pub prev: *mut NPPROW,
    pub next: *mut NPPROW,
}
#[no_mangle]
pub unsafe extern "C" fn glp_npp_alloc_wksp() -> *mut glp_prep {
    let mut prep: *mut glp_prep = 0 as *mut glp_prep;
    prep = _glp_npp_create_wksp();
    return prep;
}
#[no_mangle]
pub unsafe extern "C" fn glp_npp_load_prob(
    mut prep: *mut glp_prep,
    mut P: *mut glp_prob,
    mut sol: i32,
    mut names: i32,
) {
    if (*prep).sol != 0 as i32 {
        (glp_error_(b"api/npp.c\0" as *const u8 as *const i8, 35 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_npp_load_prob: invalid call sequence (original instance already loaded)\n\0"
                as *const u8 as *const i8,
        );
    }
    if !(sol == 1 as i32 || sol == 2 as i32 || sol == 3 as i32) {
        (glp_error_(b"api/npp.c\0" as *const u8 as *const i8, 38 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_npp_load_prob: sol = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            sol,
        );
    }
    if !(names == 1 as i32 || names == 0 as i32) {
        (glp_error_(b"api/npp.c\0" as *const u8 as *const i8, 41 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_npp_load_prob: names = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            names,
        );
    }
    _glp_npp_load_prob(prep, P, names, sol, 0 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn glp_npp_preprocess1(
    mut prep: *mut glp_prep,
    mut hard: i32,
) -> i32 {
    if (*prep).sol == 0 as i32 {
        (glp_error_(b"api/npp.c\0" as *const u8 as *const i8, 50 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_npp_preprocess1: invalid call sequence (original instance not loaded yet)\n\0"
                as *const u8 as *const i8,
        );
    }
    if ((*prep).pool).is_null() {
        (glp_error_(b"api/npp.c\0" as *const u8 as *const i8, 53 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_npp_preprocess1: invalid call sequence (preprocessing already finished)\n\0"
                as *const u8 as *const i8,
        );
    }
    if !(hard == 1 as i32 || hard == 0 as i32) {
        (glp_error_(b"api/npp.c\0" as *const u8 as *const i8, 56 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_npp_preprocess1: hard = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            hard,
        );
    }
    return _glp_npp_process_prob(prep, hard);
}
#[no_mangle]
pub unsafe extern "C" fn glp_npp_build_prob(
    mut prep: *mut glp_prep,
    mut Q: *mut glp_prob,
) {
    if (*prep).sol == 0 as i32 {
        (glp_error_(b"api/npp.c\0" as *const u8 as *const i8, 64 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_npp_build_prob: invalid call sequence (original instance not loaded yet)\n\0"
                as *const u8 as *const i8,
        );
    }
    if ((*prep).pool).is_null() {
        (glp_error_(b"api/npp.c\0" as *const u8 as *const i8, 67 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_npp_build_prob: invalid call sequence (resultant instance already built)\n\0"
                as *const u8 as *const i8,
        );
    }
    _glp_npp_build_prob(prep, Q);
}
#[no_mangle]
pub unsafe extern "C" fn glp_npp_postprocess(
    mut prep: *mut glp_prep,
    mut Q: *mut glp_prob,
) {
    if !((*prep).pool).is_null() {
        (glp_error_(b"api/npp.c\0" as *const u8 as *const i8, 76 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_npp_postprocess: invalid call sequence (resultant instance not built yet)\n\0"
                as *const u8 as *const i8,
        );
    }
    if !((*prep).m == (*Q).m && (*prep).n == (*Q).n && (*prep).nnz == (*Q).nnz) {
        (glp_error_(b"api/npp.c\0" as *const u8 as *const i8, 79 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_npp_postprocess: resultant instance mismatch\n\0" as *const u8
                as *const i8,
        );
    }
    match (*prep).sol {
        1 => {
            if glp_get_status(Q) != 5 as i32 {
                (glp_error_(b"api/npp.c\0" as *const u8 as *const i8, 83 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_npp_postprocess: unable to recover non-optimal basic solution\n\0"
                        as *const u8 as *const i8,
                );
            }
        }
        2 => {
            if glp_ipt_status(Q) != 5 as i32 {
                (glp_error_(b"api/npp.c\0" as *const u8 as *const i8, 88 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_npp_postprocess: unable to recover non-optimal interior-point solution\n\0"
                        as *const u8 as *const i8,
                );
            }
        }
        3 => {
            if !(glp_mip_status(Q) == 5 as i32 || glp_mip_status(Q) == 2 as i32) {
                (glp_error_(b"api/npp.c\0" as *const u8 as *const i8, 94 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_npp_postprocess: unable to recover integer non-feasible solution\n\0"
                        as *const u8 as *const i8,
                );
            }
        }
        _ => {
            (prep != prep
                || {
                    glp_assert_(
                        b"prep != prep\0" as *const u8 as *const i8,
                        b"api/npp.c\0" as *const u8 as *const i8,
                        98 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    _glp_npp_postprocess(prep, Q);
}
#[no_mangle]
pub unsafe extern "C" fn glp_npp_obtain_sol(
    mut prep: *mut glp_prep,
    mut P: *mut glp_prob,
) {
    if !((*prep).pool).is_null() {
        (glp_error_(b"api/npp.c\0" as *const u8 as *const i8, 107 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_npp_obtain_sol: invalid call sequence (resultant instance not built yet)\n\0"
                as *const u8 as *const i8,
        );
    }
    match (*prep).sol {
        1 => {
            if (*prep).p_stat == 0 as i32 || (*prep).d_stat == 0 as i32 {
                (glp_error_(b"api/npp.c\0" as *const u8 as *const i8, 112 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_npp_obtain_sol: invalid call sequence (basic solution not provided yet)\n\0"
                        as *const u8 as *const i8,
                );
            }
        }
        2 => {
            if (*prep).t_stat == 0 as i32 {
                (glp_error_(b"api/npp.c\0" as *const u8 as *const i8, 117 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_npp_obtain_sol: invalid call sequence (interior-point solution not provided yet)\n\0"
                        as *const u8 as *const i8,
                );
            }
        }
        3 => {
            if (*prep).i_stat == 0 as i32 {
                (glp_error_(b"api/npp.c\0" as *const u8 as *const i8, 122 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_npp_obtain_sol: invalid call sequence (MIP solution not provided yet)\n\0"
                        as *const u8 as *const i8,
                );
            }
        }
        _ => {
            (prep != prep
                || {
                    glp_assert_(
                        b"prep != prep\0" as *const u8 as *const i8,
                        b"api/npp.c\0" as *const u8 as *const i8,
                        126 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    if !((*prep).orig_dir == (*P).dir && (*prep).orig_m == (*P).m
        && (*prep).orig_n == (*P).n && (*prep).orig_nnz == (*P).nnz)
    {
        (glp_error_(b"api/npp.c\0" as *const u8 as *const i8, 130 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_npp_obtain_sol: original instance mismatch\n\0" as *const u8
                as *const i8,
        );
    }
    _glp_npp_unload_sol(prep, P);
}
#[no_mangle]
pub unsafe extern "C" fn glp_npp_free_wksp(mut prep: *mut glp_prep) {
    _glp_npp_delete_wksp(prep);
}