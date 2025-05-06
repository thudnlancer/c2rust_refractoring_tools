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
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
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
#[no_mangle]
pub unsafe extern "C" fn glp_check_kkt(
    mut P: *mut glp_prob,
    mut sol: i32,
    mut cond: i32,
    mut _ae_max: *mut libc::c_double,
    mut _ae_ind: *mut i32,
    mut _re_max: *mut libc::c_double,
    mut _re_ind: *mut i32,
) {
    let mut m: i32 = (*P).m;
    let mut n: i32 = (*P).n;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut ae_ind: i32 = 0;
    let mut re_ind: i32 = 0;
    let mut e: libc::c_double = 0.;
    let mut sp: libc::c_double = 0.;
    let mut sn: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut ae_max: libc::c_double = 0.;
    let mut re_max: libc::c_double = 0.;
    if !(sol == 1 as i32 || sol == 2 as i32 || sol == 3 as i32) {
        (glp_error_(b"draft/glpapi10.c\0" as *const u8 as *const i8, 36 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_check_kkt: sol = %d; invalid solution indicator\n\0" as *const u8
                as *const i8,
            sol,
        );
    }
    if !(cond == 1 as i32 || cond == 2 as i32 || cond == 3 as i32 || cond == 4 as i32
        || cond == 5 as i32)
    {
        (glp_error_(b"draft/glpapi10.c\0" as *const u8 as *const i8, 41 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_check_kkt: cond = %d; invalid condition indicator \n\0" as *const u8
                as *const i8,
            cond,
        );
    }
    re_max = 0.0f64;
    ae_max = re_max;
    re_ind = 0 as i32;
    ae_ind = re_ind;
    if cond == 1 as i32 {
        i = 1 as i32;
        while i <= m {
            row = *((*P).row).offset(i as isize);
            sn = 0.0f64;
            sp = sn;
            if sol == 1 as i32 {
                t = (*row).prim;
            } else if sol == 2 as i32 {
                t = (*row).pval;
            } else if sol == 3 as i32 {
                t = (*row).mipx;
            } else {
                (sol != sol
                    || {
                        glp_assert_(
                            b"sol != sol\0" as *const u8 as *const i8,
                            b"draft/glpapi10.c\0" as *const u8 as *const i8,
                            58 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            if t >= 0.0f64 {
                sp += t;
            } else {
                sn -= t;
            }
            aij = (*row).ptr;
            while !aij.is_null() {
                col = (*aij).col;
                if sol == 1 as i32 {
                    t = -(*aij).val * (*col).prim;
                } else if sol == 2 as i32 {
                    t = -(*aij).val * (*col).pval;
                } else if sol == 3 as i32 {
                    t = -(*aij).val * (*col).mipx;
                } else {
                    (sol != sol
                        || {
                            glp_assert_(
                                b"sol != sol\0" as *const u8 as *const i8,
                                b"draft/glpapi10.c\0" as *const u8 as *const i8,
                                70 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
                if t >= 0.0f64 {
                    sp += t;
                } else {
                    sn -= t;
                }
                aij = (*aij).r_next;
            }
            e = fabs(sp - sn);
            if ae_max < e {
                ae_max = e;
                ae_ind = i;
            }
            e /= 1.0f64 + sp + sn;
            if re_max < e {
                re_max = e;
                re_ind = i;
            }
            i += 1;
            i;
        }
    } else if cond == 2 as i32 {
        i = 1 as i32;
        while i <= m {
            row = *((*P).row).offset(i as isize);
            if sol == 1 as i32 {
                t = (*row).prim;
            } else if sol == 2 as i32 {
                t = (*row).pval;
            } else if sol == 3 as i32 {
                t = (*row).mipx;
            } else {
                (sol != sol
                    || {
                        glp_assert_(
                            b"sol != sol\0" as *const u8 as *const i8,
                            b"draft/glpapi10.c\0" as *const u8 as *const i8,
                            95 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            if (*row).type_0 == 2 as i32 || (*row).type_0 == 4 as i32
                || (*row).type_0 == 5 as i32
            {
                if t < (*row).lb {
                    e = (*row).lb - t;
                    if ae_max < e {
                        ae_max = e;
                        ae_ind = i;
                    }
                    e /= 1.0f64 + fabs((*row).lb);
                    if re_max < e {
                        re_max = e;
                        re_ind = i;
                    }
                }
            }
            if (*row).type_0 == 3 as i32 || (*row).type_0 == 4 as i32
                || (*row).type_0 == 5 as i32
            {
                if t > (*row).ub {
                    e = t - (*row).ub;
                    if ae_max < e {
                        ae_max = e;
                        ae_ind = i;
                    }
                    e /= 1.0f64 + fabs((*row).ub);
                    if re_max < e {
                        re_max = e;
                        re_ind = i;
                    }
                }
            }
            i += 1;
            i;
        }
        j = 1 as i32;
        while j <= n {
            col = *((*P).col).offset(j as isize);
            if sol == 1 as i32 {
                t = (*col).prim;
            } else if sol == 2 as i32 {
                t = (*col).pval;
            } else if sol == 3 as i32 {
                t = (*col).mipx;
            } else {
                (sol != sol
                    || {
                        glp_assert_(
                            b"sol != sol\0" as *const u8 as *const i8,
                            b"draft/glpapi10.c\0" as *const u8 as *const i8,
                            136 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            if (*col).type_0 == 2 as i32 || (*col).type_0 == 4 as i32
                || (*col).type_0 == 5 as i32
            {
                if t < (*col).lb {
                    e = (*col).lb - t;
                    if ae_max < e {
                        ae_max = e;
                        ae_ind = m + j;
                    }
                    e /= 1.0f64 + fabs((*col).lb);
                    if re_max < e {
                        re_max = e;
                        re_ind = m + j;
                    }
                }
            }
            if (*col).type_0 == 3 as i32 || (*col).type_0 == 4 as i32
                || (*col).type_0 == 5 as i32
            {
                if t > (*col).ub {
                    e = t - (*col).ub;
                    if ae_max < e {
                        ae_max = e;
                        ae_ind = m + j;
                    }
                    e /= 1.0f64 + fabs((*col).ub);
                    if re_max < e {
                        re_max = e;
                        re_ind = m + j;
                    }
                }
            }
            j += 1;
            j;
        }
    } else if cond == 3 as i32 {
        j = 1 as i32;
        while j <= n {
            col = *((*P).col).offset(j as isize);
            sn = 0.0f64;
            sp = sn;
            if sol == 1 as i32 {
                t = (*col).dual - (*col).coef;
            } else if sol == 2 as i32 {
                t = (*col).dval - (*col).coef;
            } else {
                (sol != sol
                    || {
                        glp_assert_(
                            b"sol != sol\0" as *const u8 as *const i8,
                            b"draft/glpapi10.c\0" as *const u8 as *const i8,
                            178 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            if t >= 0.0f64 {
                sp += t;
            } else {
                sn -= t;
            }
            aij = (*col).ptr;
            while !aij.is_null() {
                row = (*aij).row;
                if sol == 1 as i32 {
                    t = (*aij).val * (*row).dual;
                } else if sol == 2 as i32 {
                    t = (*aij).val * (*row).dval;
                } else {
                    (sol != sol
                        || {
                            glp_assert_(
                                b"sol != sol\0" as *const u8 as *const i8,
                                b"draft/glpapi10.c\0" as *const u8 as *const i8,
                                188 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
                if t >= 0.0f64 {
                    sp += t;
                } else {
                    sn -= t;
                }
                aij = (*aij).c_next;
            }
            e = fabs(sp - sn);
            if ae_max < e {
                ae_max = e;
                ae_ind = m + j;
            }
            e /= 1.0f64 + sp + sn;
            if re_max < e {
                re_max = e;
                re_ind = m + j;
            }
            j += 1;
            j;
        }
    } else if cond == 4 as i32 {
        i = 1 as i32;
        while i <= m {
            row = *((*P).row).offset(i as isize);
            if sol == 1 as i32 {
                t = (*row).dual;
            } else if sol == 2 as i32 {
                t = (*row).dval;
            } else {
                (sol != sol
                    || {
                        glp_assert_(
                            b"sol != sol\0" as *const u8 as *const i8,
                            b"draft/glpapi10.c\0" as *const u8 as *const i8,
                            211 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            if (*P).dir == 1 as i32 {
                t = t;
            } else if (*P).dir == 2 as i32 {
                t = -t;
            } else {
                (P != P
                    || {
                        glp_assert_(
                            b"P != P\0" as *const u8 as *const i8,
                            b"draft/glpapi10.c\0" as *const u8 as *const i8,
                            218 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            if (*row).type_0 == 1 as i32 || (*row).type_0 == 2 as i32 {
                if t < 0.0f64 {
                    e = -t;
                    if ae_max < e {
                        re_max = e;
                        ae_max = re_max;
                        re_ind = i;
                        ae_ind = re_ind;
                    }
                }
            }
            if (*row).type_0 == 1 as i32 || (*row).type_0 == 3 as i32 {
                if t > 0.0f64 {
                    e = t;
                    if ae_max < e {
                        re_max = e;
                        ae_max = re_max;
                        re_ind = i;
                        ae_ind = re_ind;
                    }
                }
            }
            i += 1;
            i;
        }
        j = 1 as i32;
        while j <= n {
            col = *((*P).col).offset(j as isize);
            if sol == 1 as i32 {
                t = (*col).dual;
            } else if sol == 2 as i32 {
                t = (*col).dval;
            } else {
                (sol != sol
                    || {
                        glp_assert_(
                            b"sol != sol\0" as *const u8 as *const i8,
                            b"draft/glpapi10.c\0" as *const u8 as *const i8,
                            257 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            if (*P).dir == 1 as i32 {
                t = t;
            } else if (*P).dir == 2 as i32 {
                t = -t;
            } else {
                (P != P
                    || {
                        glp_assert_(
                            b"P != P\0" as *const u8 as *const i8,
                            b"draft/glpapi10.c\0" as *const u8 as *const i8,
                            264 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            if (*col).type_0 == 1 as i32 || (*col).type_0 == 2 as i32 {
                if t < 0.0f64 {
                    e = -t;
                    if ae_max < e {
                        re_max = e;
                        ae_max = re_max;
                        re_ind = m + j;
                        ae_ind = re_ind;
                    }
                }
            }
            if (*col).type_0 == 1 as i32 || (*col).type_0 == 3 as i32 {
                if t > 0.0f64 {
                    e = t;
                    if ae_max < e {
                        re_max = e;
                        ae_max = re_max;
                        re_ind = m + j;
                        ae_ind = re_ind;
                    }
                }
            }
            j += 1;
            j;
        }
    } else {
        (cond != cond
            || {
                glp_assert_(
                    b"cond != cond\0" as *const u8 as *const i8,
                    b"draft/glpapi10.c\0" as *const u8 as *const i8,
                    294 as i32,
                );
                1 as i32 != 0
            }) as i32;
    }
    if !_ae_max.is_null() {
        *_ae_max = ae_max;
    }
    if !_ae_ind.is_null() {
        *_ae_ind = ae_ind;
    }
    if !_re_max.is_null() {
        *_re_max = re_max;
    }
    if !_re_ind.is_null() {
        *_re_ind = re_ind;
    }
}