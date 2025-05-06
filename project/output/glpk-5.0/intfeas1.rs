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
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_printf(fmt: *const i8, _: ...);
    fn _glp_npp_create_wksp() -> *mut NPP;
    fn _glp_npp_load_prob(
        npp: *mut NPP,
        orig: *mut glp_prob,
        names: i32,
        sol: i32,
        scaling: i32,
    );
    fn _glp_npp_build_prob(npp: *mut NPP, prob: *mut glp_prob);
    fn _glp_npp_postprocess(npp: *mut NPP, prob: *mut glp_prob);
    fn _glp_npp_unload_sol(npp: *mut NPP, orig: *mut glp_prob);
    fn _glp_npp_delete_wksp(npp: *mut NPP);
    fn glp_add_rows(P: *mut glp_prob, nrs: i32) -> i32;
    fn glp_set_row_bnds(
        P: *mut glp_prob,
        i: i32,
        type_0: i32,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_set_mat_row(
        P: *mut glp_prob,
        i: i32,
        len: i32,
        ind: *const i32,
        val: *const libc::c_double,
    );
    fn glp_del_rows(P: *mut glp_prob, nrs: i32, num: *const i32);
    fn glp_delete_prob(P: *mut glp_prob);
    fn glp_create_prob() -> *mut glp_prob;
    fn _glp_npp_sat_encode_prob(npp: *mut NPP) -> i32;
    fn glp_minisat1(P: *mut glp_prob) -> i32;
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
pub unsafe extern "C" fn glp_intfeas1(
    mut P: *mut glp_prob,
    mut use_bound: i32,
    mut obj_bound: i32,
) -> i32 {
    let mut current_block: u64;
    let mut npp: *mut NPP = 0 as *mut NPP;
    let mut mip: *mut glp_prob = 0 as *mut glp_prob;
    let mut obj_ind: *mut i32 = 0 as *mut i32;
    let mut obj_val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut obj_row: i32 = 0 as i32;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut obj_len: i32 = 0;
    let mut temp: i32 = 0;
    let mut ret: i32 = 0;
    if !((*P).tree).is_null() {
        (glp_error_(b"api/intfeas1.c\0" as *const u8 as *const i8, 40 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_intfeas1: operation not allowed\n\0" as *const u8 as *const i8);
    }
    (*P).mip_stat = 1 as i32;
    (*P).mip_obj = 0.0f64;
    j = 1 as i32;
    loop {
        if !(j <= (*P).n) {
            current_block = 15089075282327824602;
            break;
        }
        let mut col: *mut GLPCOL = *((*P).col).offset(j as isize);
        if !((*col).kind == 2 as i32 && (*col).lb == 0.0f64 && (*col).ub == 1.0f64
            || (*col).type_0 == 5 as i32)
        {
            glp_printf(
                b"glp_intfeas1: column %d: non-binary non-fixed variable not allowed\n\0"
                    as *const u8 as *const i8,
                j,
            );
            ret = 0x12 as i32;
            current_block = 6112804666749291094;
            break;
        } else {
            temp = (*col).lb as i32;
            if temp as libc::c_double != (*col).lb {
                if (*col).type_0 == 5 as i32 {
                    glp_printf(
                        b"glp_intfeas1: column %d: fixed value %g is non-integer or out of range\n\0"
                            as *const u8 as *const i8,
                        j,
                        (*col).lb,
                    );
                } else {
                    glp_printf(
                        b"glp_intfeas1: column %d: lower bound %g is non-integer or out of range\n\0"
                            as *const u8 as *const i8,
                        j,
                        (*col).lb,
                    );
                }
                ret = 0x12 as i32;
                current_block = 6112804666749291094;
                break;
            } else {
                temp = (*col).ub as i32;
                if temp as libc::c_double != (*col).ub {
                    glp_printf(
                        b"glp_intfeas1: column %d: upper bound %g is non-integer or out of range\n\0"
                            as *const u8 as *const i8,
                        j,
                        (*col).ub,
                    );
                    ret = 0x12 as i32;
                    current_block = 6112804666749291094;
                    break;
                } else if (*col).type_0 == 4 as i32 && (*col).lb > (*col).ub {
                    glp_printf(
                        b"glp_intfeas1: column %d: lower bound %g is greater than upper bound %g\n\0"
                            as *const u8 as *const i8,
                        j,
                        (*col).lb,
                        (*col).ub,
                    );
                    ret = 0x4 as i32;
                    current_block = 6112804666749291094;
                    break;
                } else {
                    j += 1;
                    j;
                }
            }
        }
    }
    match current_block {
        15089075282327824602 => {
            i = 1 as i32;
            's_110: loop {
                if !(i <= (*P).m) {
                    current_block = 17784502470059252271;
                    break;
                }
                let mut row: *mut GLPROW = *((*P).row).offset(i as isize);
                let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
                aij = (*row).ptr;
                while !aij.is_null() {
                    temp = (*aij).val as i32;
                    if temp as libc::c_double != (*aij).val {
                        glp_printf(
                            b"glp_intfeas1: row = %d, column %d: constraint coefficient %g is non-integer or out of range\n\0"
                                as *const u8 as *const i8,
                            i,
                            (*(*aij).col).j,
                            (*aij).val,
                        );
                        ret = 0x12 as i32;
                        current_block = 6112804666749291094;
                        break 's_110;
                    } else {
                        aij = (*aij).r_next;
                    }
                }
                temp = (*row).lb as i32;
                if temp as libc::c_double != (*row).lb {
                    if (*row).type_0 == 5 as i32 {
                        glp_printf(
                            b"glp_intfeas1: row = %d: fixed value %g is non-integer or out of range\n\0"
                                as *const u8 as *const i8,
                            i,
                            (*row).lb,
                        );
                    } else {
                        glp_printf(
                            b"glp_intfeas1: row = %d: lower bound %g is non-integer or out of range\n\0"
                                as *const u8 as *const i8,
                            i,
                            (*row).lb,
                        );
                    }
                    ret = 0x12 as i32;
                    current_block = 6112804666749291094;
                    break;
                } else {
                    temp = (*row).ub as i32;
                    if temp as libc::c_double != (*row).ub {
                        glp_printf(
                            b"glp_intfeas1: row = %d: upper bound %g is non-integer or out of range\n\0"
                                as *const u8 as *const i8,
                            i,
                            (*row).ub,
                        );
                        ret = 0x12 as i32;
                        current_block = 6112804666749291094;
                        break;
                    } else if (*row).type_0 == 4 as i32 && (*row).lb > (*row).ub {
                        glp_printf(
                            b"glp_intfeas1: row %d: lower bound %g is greater than upper bound %g\n\0"
                                as *const u8 as *const i8,
                            i,
                            (*row).lb,
                            (*row).ub,
                        );
                        ret = 0x4 as i32;
                        current_block = 6112804666749291094;
                        break;
                    } else {
                        i += 1;
                        i;
                    }
                }
            }
            match current_block {
                6112804666749291094 => {}
                _ => {
                    if use_bound == 0 {
                        current_block = 13321564401369230990;
                    } else {
                        temp = (*P).c0 as i32;
                        if temp as libc::c_double != (*P).c0 {
                            glp_printf(
                                b"glp_intfeas1: objective constant term %g is non-integer or out of range\n\0"
                                    as *const u8 as *const i8,
                                (*P).c0,
                            );
                            ret = 0x12 as i32;
                            current_block = 6112804666749291094;
                        } else {
                            j = 1 as i32;
                            loop {
                                if !(j <= (*P).n) {
                                    current_block = 13321564401369230990;
                                    break;
                                }
                                temp = (**((*P).col).offset(j as isize)).coef as i32;
                                if temp as libc::c_double
                                    != (**((*P).col).offset(j as isize)).coef
                                {
                                    glp_printf(
                                        b"glp_intfeas1: column %d: objective coefficient is non-integer or out of range\n\0"
                                            as *const u8 as *const i8,
                                        j,
                                        (**((*P).col).offset(j as isize)).coef,
                                    );
                                    ret = 0x12 as i32;
                                    current_block = 6112804666749291094;
                                    break;
                                } else {
                                    j += 1;
                                    j;
                                }
                            }
                        }
                    }
                    match current_block {
                        6112804666749291094 => {}
                        _ => {
                            obj_ind = glp_alloc(
                                1 as i32 + (*P).n,
                                ::core::mem::size_of::<i32>() as u64 as i32,
                            ) as *mut i32;
                            obj_val = glp_alloc(
                                1 as i32 + (*P).n,
                                ::core::mem::size_of::<libc::c_double>() as u64 as i32,
                            ) as *mut libc::c_double;
                            obj_len = 0 as i32;
                            *obj_ind.offset(0 as i32 as isize) = 0 as i32;
                            *obj_val.offset(0 as i32 as isize) = (*P).c0;
                            (*P).c0 = 0.0f64;
                            j = 1 as i32;
                            while j <= (*P).n {
                                if (**((*P).col).offset(j as isize)).coef != 0.0f64 {
                                    obj_len += 1;
                                    obj_len;
                                    *obj_ind.offset(obj_len as isize) = j;
                                    *obj_val.offset(obj_len as isize) = (**((*P).col)
                                        .offset(j as isize))
                                        .coef;
                                    (**((*P).col).offset(j as isize)).coef = 0.0f64;
                                }
                                j += 1;
                                j;
                            }
                            if use_bound == 0 {
                                glp_printf(
                                    b"Will search for ANY feasible solution\n\0" as *const u8
                                        as *const i8,
                                );
                            } else {
                                glp_printf(
                                    b"Will search only for solution not worse than %d\n\0"
                                        as *const u8 as *const i8,
                                    obj_bound,
                                );
                                obj_row = glp_add_rows(P, 1 as i32);
                                glp_set_mat_row(
                                    P,
                                    obj_row,
                                    obj_len,
                                    obj_ind as *const i32,
                                    obj_val as *const libc::c_double,
                                );
                                if (*P).dir == 1 as i32 {
                                    glp_set_row_bnds(
                                        P,
                                        obj_row,
                                        3 as i32,
                                        0.0f64,
                                        obj_bound as libc::c_double
                                            - *obj_val.offset(0 as i32 as isize),
                                    );
                                } else if (*P).dir == 2 as i32 {
                                    glp_set_row_bnds(
                                        P,
                                        obj_row,
                                        2 as i32,
                                        obj_bound as libc::c_double
                                            - *obj_val.offset(0 as i32 as isize),
                                        0.0f64,
                                    );
                                } else {
                                    (P != P
                                        || {
                                            glp_assert_(
                                                b"P != P\0" as *const u8 as *const i8,
                                                b"api/intfeas1.c\0" as *const u8 as *const i8,
                                                180 as i32,
                                            );
                                            1 as i32 != 0
                                        }) as i32;
                                }
                            }
                            glp_printf(
                                b"Translating to CNF-SAT...\n\0" as *const u8 as *const i8,
                            );
                            glp_printf(
                                b"Original problem has %d row%s, %d column%s, and %d non-zero%s\n\0"
                                    as *const u8 as *const i8,
                                (*P).m,
                                if (*P).m == 1 as i32 {
                                    b"\0" as *const u8 as *const i8
                                } else {
                                    b"s\0" as *const u8 as *const i8
                                },
                                (*P).n,
                                if (*P).n == 1 as i32 {
                                    b"\0" as *const u8 as *const i8
                                } else {
                                    b"s\0" as *const u8 as *const i8
                                },
                                (*P).nnz,
                                if (*P).nnz == 1 as i32 {
                                    b"\0" as *const u8 as *const i8
                                } else {
                                    b"s\0" as *const u8 as *const i8
                                },
                            );
                            npp = _glp_npp_create_wksp();
                            _glp_npp_load_prob(npp, P, 0 as i32, 3 as i32, 0 as i32);
                            ret = _glp_npp_sat_encode_prob(npp);
                            if !(ret == 0 as i32) {
                                if ret == 0xa as i32 {
                                    glp_printf(
                                        b"PROBLEM HAS NO INTEGER FEASIBLE SOLUTION\n\0" as *const u8
                                            as *const i8,
                                    );
                                } else if ret == 0x13 as i32 {
                                    glp_printf(
                                        b"glp_intfeas1: translation to SAT-CNF failed because of integer overflow\n\0"
                                            as *const u8 as *const i8,
                                    );
                                } else {
                                    (ret != ret
                                        || {
                                            glp_assert_(
                                                b"ret != ret\0" as *const u8 as *const i8,
                                                b"api/intfeas1.c\0" as *const u8 as *const i8,
                                                200 as i32,
                                            );
                                            1 as i32 != 0
                                        }) as i32;
                                }
                            }
                            if !(ret != 0 as i32) {
                                mip = glp_create_prob();
                                _glp_npp_build_prob(npp, mip);
                                ret = glp_minisat1(mip);
                                if !((*mip).mip_stat == 5 as i32
                                    || (*mip).mip_stat == 2 as i32)
                                {
                                    (*P).mip_stat = (*mip).mip_stat;
                                } else {
                                    _glp_npp_postprocess(npp, mip);
                                    glp_delete_prob(mip);
                                    mip = 0 as *mut glp_prob;
                                    _glp_npp_unload_sol(npp, P);
                                    (*P).mip_stat = 2 as i32;
                                    i = 1 as i32;
                                    while i <= (*P).m {
                                        let mut row_0: *mut GLPROW = 0 as *mut GLPROW;
                                        let mut aij_0: *mut GLPAIJ = 0 as *mut GLPAIJ;
                                        let mut sum: libc::c_double = 0.;
                                        row_0 = *((*P).row).offset(i as isize);
                                        sum = 0.0f64;
                                        aij_0 = (*row_0).ptr;
                                        while !aij_0.is_null() {
                                            sum += (*aij_0).val * (*(*aij_0).col).mipx;
                                            aij_0 = (*aij_0).r_next;
                                        }
                                        (sum == (*row_0).mipx
                                            || {
                                                glp_assert_(
                                                    b"sum == row->mipx\0" as *const u8 as *const i8,
                                                    b"api/intfeas1.c\0" as *const u8 as *const i8,
                                                    229 as i32,
                                                );
                                                1 as i32 != 0
                                            }) as i32;
                                        if (*row_0).type_0 == 2 as i32
                                            || (*row_0).type_0 == 4 as i32
                                            || (*row_0).type_0 == 5 as i32
                                        {
                                            (sum >= (*row_0).lb
                                                || {
                                                    glp_assert_(
                                                        b"sum >= row->lb\0" as *const u8 as *const i8,
                                                        b"api/intfeas1.c\0" as *const u8 as *const i8,
                                                        232 as i32,
                                                    );
                                                    1 as i32 != 0
                                                }) as i32;
                                        }
                                        if (*row_0).type_0 == 3 as i32
                                            || (*row_0).type_0 == 4 as i32
                                            || (*row_0).type_0 == 5 as i32
                                        {
                                            (sum <= (*row_0).ub
                                                || {
                                                    glp_assert_(
                                                        b"sum <= row->ub\0" as *const u8 as *const i8,
                                                        b"api/intfeas1.c\0" as *const u8 as *const i8,
                                                        235 as i32,
                                                    );
                                                    1 as i32 != 0
                                                }) as i32;
                                        }
                                        i += 1;
                                        i;
                                    }
                                    (*P).mip_obj = *obj_val.offset(0 as i32 as isize);
                                    k = 1 as i32;
                                    while k <= obj_len {
                                        (*P).mip_obj
                                            += *obj_val.offset(k as isize)
                                                * (**((*P).col)
                                                    .offset(*obj_ind.offset(k as isize) as isize))
                                                    .mipx;
                                        k += 1;
                                        k;
                                    }
                                    glp_printf(
                                        b"Objective value = %17.9e\n\0" as *const u8 as *const i8,
                                        (*P).mip_obj,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if !mip.is_null() {
        glp_delete_prob(mip);
    }
    if !npp.is_null() {
        _glp_npp_delete_wksp(npp);
    }
    if obj_row > 0 as i32 {
        let mut ind: [i32; 2] = [0; 2];
        ind[1 as i32 as usize] = obj_row;
        glp_del_rows(P, 1 as i32, ind.as_mut_ptr() as *const i32);
    }
    if !obj_ind.is_null() {
        (*P).c0 = *obj_val.offset(0 as i32 as isize);
        k = 1 as i32;
        while k <= obj_len {
            (**((*P).col).offset(*obj_ind.offset(k as isize) as isize)).coef = *obj_val
                .offset(k as isize);
            k += 1;
            k;
        }
        glp_free(obj_ind as *mut libc::c_void);
        glp_free(obj_val as *mut libc::c_void);
    }
    return ret;
}